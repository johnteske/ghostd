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

    if tx.send(Message::Get { resp: resp_tx }).await.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    match resp_rx.await {
        Ok(res) => Ok(res.into_response()),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

async fn post_handler(tx: Sender<Message>, bytes: Bytes) -> Result<impl Reply, Rejection> {
    let (resp_tx, resp_rx) = oneshot::channel();

    let body = match String::from_utf8(bytes.to_vec()) {
        Ok(body) => body,
        Err(_) => return Ok(StatusCode::BAD_REQUEST),
    };

    if tx
        .send(Message::Set {
            value: body,
            resp: resp_tx,
        })
        .await
        .is_err()
    {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR);
    }

    match resp_rx.await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
