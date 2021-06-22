//#![deny(warnings)]

use tokio::time::Duration;
use warp::Filter;

mod state;

const TTL: Duration = Duration::from_secs(5);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (_, tx) = state::start(TTL);

    let get = warp::path("value")
        .and(warp::get())
        .and_then(|| async move {
            if true {
                Ok(format!("asd"))
            } else {
                Err(warp::reject::not_found())
            }
        });

    warp::serve(get).run(([127, 0, 0, 1], 3000)).await;
}
