use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;
use warp::Filter;

mod tmp_state;

include!(concat!(env!("OUT_DIR"), "/html.rs"));

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new("".to_string()));
    let tx = tmp_state::start(Arc::clone(&state));

    let index = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::html(HTML));

    let get_value = warp::path("value").and(warp::get()).map(|| "bruh");

    let post_value = warp::path("value")
        .and(warp::post())
        // Only accept bodies smaller than 16kb
        .and(warp::body::content_length_limit(1024 * 16))
        .and_then(post);

    let routes = index.or(get_value).or(post_value);
    warp::serve(routes).run(([127, 0, 0, 1], 4321)).await;
}

pub async fn post() -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}
type State = Arc<Mutex<String>>;
fn with_db(db: State) -> impl Filter<Extract = (State,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
