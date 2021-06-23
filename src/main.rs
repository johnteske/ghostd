#![deny(warnings)]

use bytes::Bytes;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::time::Duration;
use warp::{Filter, Rejection, Reply};

mod state;
use state::Message;

const TTL: Duration = Duration::from_secs(5);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (_, tx) = state::start(TTL);

    let with_tx = warp::any().map(move || tx.clone());

    let get = warp::get().and(with_tx.clone()).and_then(get_handler);

    let post = warp::post()
        .and(with_tx.clone())
        .and(bytes_body())
        .and_then(post_handler);

    let api = warp::path("value").and(get.or(post));

    warp::serve(api).run(([127, 0, 0, 1], 3000)).await;
}

#[derive(Debug)]
struct TxMessageFailed;

impl warp::reject::Reject for TxMessageFailed {}

#[derive(Debug)]
struct RxMessageFailed;

impl warp::reject::Reject for RxMessageFailed {}

async fn get_handler(tx: Sender<Message>) -> Result<impl Reply, Rejection> {
    let (resp_tx, resp_rx) = oneshot::channel();

    match tx.send(Message::Get { resp: resp_tx }).await {
        Ok(_) => {}
        Err(_) => return Err(warp::reject::custom(TxMessageFailed)),
    };

    match resp_rx.await {
        Ok(res) => Ok(res),
        Err(_) => Err(warp::reject::custom(RxMessageFailed)),
    }
}

async fn post_handler(tx: Sender<Message>, bytes: Bytes) -> Result<impl Reply, Rejection> {
    let (resp_tx, resp_rx) = oneshot::channel();

    let body = String::from_utf8(bytes.to_vec()).unwrap();

    match tx
        .send(Message::Set {
            value: body,
            resp: resp_tx,
        })
        .await
    {
        Ok(_) => {}
        Err(_) => return Err(warp::reject::custom(TxMessageFailed)),
    };

    match resp_rx.await {
        Ok(_) => Ok(warp::reply()),
        Err(_) => Err(warp::reject::custom(RxMessageFailed)),
    }
}

fn bytes_body() -> impl Filter<Extract = (bytes::Bytes,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::bytes())
}
