use tokio::sync::mpsc;

use std::sync::{Arc, Mutex};
use warp::Filter;

mod tmp_state;

include!(concat!(env!("OUT_DIR"), "/html.rs"));

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new("".to_string()));

    let tx = tmp_state::start(Arc::clone(&state));

    // TODO add all assets
    let index = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::html(HTML));

    let routes = index.or(filters::api(state, tx));
    warp::serve(routes).run(([127, 0, 0, 1], 4321)).await;
}

type State = Arc<Mutex<String>>;

fn with_db(db: State) -> impl Filter<Extract = (State,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

type TX = mpsc::UnboundedSender<String>;
fn with_tx(tx: TX) -> impl Filter<Extract = (TX,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || tx.clone())
}

mod filters {
    use super::handlers;
    use super::{with_db, with_tx, State, TX};
    use warp::Filter;

    pub fn api(
        state: State,
        tx: TX,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("value").and(get_value(state.clone()).or(post_value(tx.clone())))
    }

    // GET /value
    fn get_value(
        state: State,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(with_db(state))
            .and_then(handlers::get_value)
    }

    // POST /value
    fn post_value(
        tx: super::TX,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::post()
            // Only accept bodies smaller than 16kb
            .and(warp::body::content_length_limit(1024 * 16))
            .and(with_tx(tx))
            .and(warp::body::bytes())
            .and_then(handlers::post_value)
    }
}

mod handlers {
    use super::{State, TX};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn get_value(state: State) -> Result<impl warp::Reply, Infallible> {
        let value = state.lock().unwrap();
        Ok(value.to_string())
    }

    pub async fn post_value(tx: TX, bytes: bytes::Bytes) -> Result<impl warp::Reply, Infallible> {
        // TODO is not temporary
        let body = String::from_utf8(bytes.to_vec()).expect("found invalid UTF-8");
        tx.send(body.to_string()).unwrap();
        Ok(StatusCode::RESET_CONTENT)
    }
}
