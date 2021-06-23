#![deny(warnings)]

use tokio::time::Duration;

mod api;
mod state;

const TTL: Duration = Duration::from_secs(5);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (_, tx) = state::start(TTL);

    let api = api::create(tx);

    warp::serve(api).run(([127, 0, 0, 1], 3000)).await;
}
