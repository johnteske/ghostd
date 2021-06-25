#![deny(warnings)]

include!(concat!(env!("OUT_DIR"), "/html.rs"));

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;
use warp::Filter;

mod api;
mod expiration_checker;
mod state;
use state::State;

const TTL: Duration = Duration::from_secs(5);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let db = Arc::new(Mutex::new(State::new(TTL)));

    expiration_checker::run(Arc::clone(&db));

    let api = api::create(db);
    let html = warp::get().map(|| warp::reply::html(HTML));
    let routes = api.or(html);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
