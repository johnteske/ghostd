use std::sync::{Arc, Mutex};
use warp::Filter;

// mod tmp_state;

include!(concat!(env!("OUT_DIR"), "/html.rs"));

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new("".to_string()));
    // let tx = tmp_state::start(Arc::clone(&state));

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
        get_value(state.clone()).or(post_value(state.clone()))
    }

    fn get_value(
        state: State,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("value")
            .and(warp::get()) //.map(|| "bruh")
            .and(with_db(state))
            .and_then(handlers::get_value)
    }

    fn post_value(
        state: State,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("value")
            .and(warp::post())
            // Only accept bodies smaller than 16kb
            .and(warp::body::content_length_limit(1024 * 16))
            .and(with_db(state))
            .and_then(handlers::post_value)
        //.map(|| "post")
    }
}

mod handlers {
    use super::State;
    use std::convert::Infallible;

    pub async fn get_value(state: State) -> Result<impl warp::Reply, Infallible> {
        Ok("todo, bruh")
    }

    pub async fn post_value(state: State) -> Result<impl warp::Reply, Infallible> {
        Ok("todo, bruh")
    }
}
