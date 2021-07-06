use tokio::time::{sleep, Duration};

use super::state::Db;

pub fn run(db: Db) {
    tokio::task::spawn(async move {
        loop {
            sleep(Duration::from_secs(5)).await;
            let mut state = db.lock().await;
            state.clear_if_expired();
        }
    });
}
