use std::sync::Arc;
use tokio::sync::Mutex;

use bytes::Bytes;
use warp::{http::StatusCode, Filter, Rejection, Reply};

use super::state::State;

type Db = Arc<Mutex<State>>;

pub fn create(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db = warp::any().map(move || Arc::clone(&db));

    let body_bytes = warp::body::content_length_limit(1024 * 16).and(warp::body::bytes());

    let get = warp::get().and(db.clone()).and_then(get_handler);

    let post = warp::post().and(db).and(body_bytes).and_then(post_handler);

    let handlers = get.or(post);

    warp::path("value").and(handlers)
}

async fn get_handler(db: Db) -> Result<impl Reply, Rejection> {
    let state = db.lock().await;
    let value = state.get();
    Ok(value.clone().into_response())
}

async fn post_handler(db: Db, bytes: Bytes) -> Result<impl Reply, Rejection> {
    let mut state = db.lock().await;

    let body = match String::from_utf8(bytes.to_vec()) {
        Ok(body) => body,
        Err(_) => return Ok(StatusCode::BAD_REQUEST),
    };

    state.set(body);

    Ok(StatusCode::NO_CONTENT)
}
