use std::sync::{Arc, Mutex};
use warp::Filter;

// mod tmp_state;

include!(concat!(env!("OUT_DIR"), "/html.rs"));

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new("".to_string()));
    // let tx = tmp_state::start(Arc::clone(&state));

    // TODO add all assets
    let index = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::html(HTML));

    let routes = index.or(filters::api(state));
    warp::serve(routes).run(([127, 0, 0, 1], 4321)).await;
}

type State = Arc<Mutex<String>>;

fn with_db(db: State) -> impl Filter<Extract = (State,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

mod filters {
    use super::handlers;
    use super::{with_db, State};
    use warp::Filter;

    pub fn api(
        state: State,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("value").and(get_value(state.clone()).or(post_value(state.clone())))
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
        state: State,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::post()
            // Only accept bodies smaller than 16kb
            .and(warp::body::content_length_limit(1024 * 16))
            .and(with_db(state))
            .and(warp::body::bytes())
            .and_then(handlers::post_value)
    }
}

mod handlers {
    use super::State;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn get_value(state: State) -> Result<impl warp::Reply, Infallible> {
        let value = state.lock().unwrap();
        Ok(value.to_string())
    }

    pub async fn post_value(
        state: State,
        bytes: bytes::Bytes,
    ) -> Result<impl warp::Reply, Infallible> {
        // TODO is not temporary
        let mut value = state.lock().unwrap();
        let s = String::from_utf8(bytes.to_vec()).expect("found invalid UTF-8");
        *value = s;
        Ok(StatusCode::RESET_CONTENT)
    }
}
