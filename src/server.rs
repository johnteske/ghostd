use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Response, Server, StatusCode};
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use super::state::Message;

static NOTFOUND: &[u8] = b"Not Found";

async fn make_service() -> hyper::service::make::MakeServiceFn {
make_service_fn(move |_| {
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
    })
}

pub async fn run(
    addr: SocketAddr,
    tx: Sender<Message>,
) -> hyper::Server<hyper::server::conn::AddrIncoming, ()> {
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

    Server::bind(&addr).serve(make_service)
}
