//#![deny(warnings)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Response, Server, StatusCode};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

mod state;
use state::State;

const TTL: Duration = Duration::from_secs(5);
static NOTFOUND: &[u8] = b"Not Found";

type Responder<T> = oneshot::Sender<T>;

#[derive(Debug)]
enum Message {
    Get { resp: Responder<String> },
    Set { value: String, resp: Responder<()> },
    Check { resp: Responder<()> },
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // TODO this channel is used for state--how to communicate that?
    let (tx, mut rx) = mpsc::channel::<Message>(32);
    let mut state = State::new(TTL);

    tokio::task::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                Message::Get { resp } => {
                    let value = state.get();
                    let _ = resp.send(value.to_owned());
                }
                Message::Set { value, resp } => {
                    state.set(value);
                    let _ = resp.send(());
                }
                Message::Check { resp } => {
                    state.clear_if_expired();
                    let _ = resp.send(());
                }
            }
        }
    });

    let timer_tx = tx.clone();
    tokio::task::spawn(async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            let (resp_tx, resp_rx) = oneshot::channel();
            timer_tx
                .send(Message::Check { resp: resp_tx })
                .await
                .unwrap();
            let res = resp_rx.await;
            println!("GOT = {:?}", res);
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    // TODO impl hyper::Service for ___
    let make_service = make_service_fn(move |_| {
        let tx = tx.clone();

        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let tx = tx.clone();
                async move {
                    match (req.method(), req.uri().path()) {
                        (&Method::GET, "/value") => {
                            let (resp_tx, resp_rx) = oneshot::channel();

                            tx.send(Message::Get { resp: resp_tx }).await.unwrap();
                            let res = resp_rx.await;
                            println!("GOT = {:?}", res);

                            Ok::<_, Error>(Response::new(Body::from(res.unwrap())))
                        }
                        (&Method::POST, "/value") => {
                            let bytes = hyper::body::to_bytes(req.into_body()).await?;
                            let body = String::from_utf8(bytes.to_vec())
                                .expect("response was not valid utf-8");
                            println!("BODY = {:?}", body);

                            let (resp_tx, resp_rx) = oneshot::channel();
                            tx.send(Message::Set {
                                value: body.to_string(),
                                resp: resp_tx,
                            })
                            .await
                            .unwrap();

                            let res = resp_rx.await;
                            println!("GOT = {:?}", res);

                            Ok::<_, Error>(Response::new(Body::from("")))
                        }
                        _ => Ok(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(NOTFOUND.into())
                            .unwrap()),
                    }
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
