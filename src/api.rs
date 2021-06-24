use bytes::Bytes;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use warp::{http::StatusCode, Filter, Rejection, Reply};

use super::state::Message;

pub fn create(
    tx: Sender<Message>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let with_tx = warp::any().map(move || tx.clone());
    let body_bytes = warp::body::content_length_limit(1024 * 16).and(warp::body::bytes());

    let get = warp::get().and(with_tx.clone()).and_then(get_handler);

    let post = warp::post()
        .and(with_tx)
        .and(body_bytes)
        .and_then(post_handler);

    let handlers = get.or(post);

    warp::path("value").and(handlers)
}

async fn get_handler(tx: Sender<Message>) -> Result<impl Reply, Rejection> {
    let (resp_tx, resp_rx) = oneshot::channel();

    tx.send(Message::Get { resp: resp_tx })
        .await
        .expect("tx failed");

    let res = resp_rx.await.expect("rx failed");
    Ok(res)
}

async fn post_handler(tx: Sender<Message>, bytes: Bytes) -> Result<impl Reply, Rejection> {
    let (resp_tx, resp_rx) = oneshot::channel();

    let body = String::from_utf8(bytes.to_vec()).expect("parsing body failed");

    tx.send(Message::Set {
        value: body,
        resp: resp_tx,
    })
    .await
    .expect("tx failed");

    let _ = resp_rx.await.expect("rx failed");
    Ok(StatusCode::NO_CONTENT)
}
