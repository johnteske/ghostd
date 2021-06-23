#![deny(warnings)]

include!(concat!(env!("OUT_DIR"), "/html.rs"));

use tokio::time::Duration;
use warp::Filter;

mod api;
mod state;

const TTL: Duration = Duration::from_secs(5);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (_, tx) = state::start(TTL);

    let api = api::create(tx);
    let html = warp::get().map(|| warp::reply::html(HTML));
    let routes = api.or(html);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
