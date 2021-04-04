//#![deny(warnings)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Response, Server, StatusCode};
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

mod state;
use state::{Message, State};

const TTL: Duration = Duration::from_secs(5);
static NOTFOUND: &[u8] = b"Not Found";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let st = State::new(TTL);
    let (_, tx) = state::run(st);

    // what if state owns this and can start/stop
    // OR sleep.reset could be used
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
