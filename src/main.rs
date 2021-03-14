// https://github.com/hyperium/hyper/blob/master/examples/single_threaded.rs
// https://github.com/hyperium/hyper/blob/master/examples/web_api.rs
#![deny(warnings)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Response, Server, StatusCode};
use tokio::sync::mpsc;

static NOTFOUND: &[u8] = b"Not Found";

#[derive(Debug)]
enum Message {
    Start,
    Stop,
}

fn main() {
    // Configure a runtime that runs everything on the current thread
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build runtime");

    // Combine it with a `LocalSet,  which means it can spawn !Send futures...
    let local = tokio::task::LocalSet::new();
    local.block_on(&rt, run());
}

async fn run() {
    let (tx, mut rx) = mpsc::channel::<Message>(32);

    tokio::task::spawn_local(async move {
        while let Some(message) = rx.recv().await {
            println!("GOT = {:?}", message);
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let make_service = make_service_fn(move |_| {
        let tx = tx.clone();

        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let tx = tx.clone();
                async move {
                    match (req.method(), req.uri().path()) {
                        (&Method::GET, "/value") => {
                            Ok::<_, Error>(Response::new(Body::from("get")))
                        }
                        (&Method::POST, "/value") => {
                            tx.send(Message::Start).await.unwrap();
                            // TODO set the timeout here, although that means the thread can't be
                            // dropped from here
                            Ok::<_, Error>(Response::new(Body::from("post")))
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

    let server = Server::bind(&addr).executor(LocalExec).serve(make_service);

    println!("Listening on http://{}", addr);

    // The server would block on current thread to await !Send futures.
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

// Since the Server needs to spawn some background tasks, we needed
// to configure an Executor that can spawn !Send futures...
#[derive(Clone, Copy, Debug)]
struct LocalExec;

impl<F> hyper::rt::Executor<F> for LocalExec
where
    F: std::future::Future + 'static, // not requiring `Send`
{
    fn execute(&self, fut: F) {
        // This will spawn into the currently running `LocalSet`.
        tokio::task::spawn_local(fut);
    }
}
