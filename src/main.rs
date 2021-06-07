//#![deny(warnings)]

use tokio::time::Duration;

mod state;

mod server;

const TTL: Duration = Duration::from_secs(5);
static NOTFOUND: &[u8] = b"Not Found";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (_, tx) = state::start(TTL);

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = server::run(addr, tx);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
