use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Duration, Instant};

type Responder<T> = oneshot::Sender<T>;

#[derive(Debug)]
pub enum Message {
    Get { resp: Responder<String> },
    Set { value: String, resp: Responder<()> },
    Check { resp: Responder<()> },
}

// TODO max access count?
//
struct State {
    value: String,
    timestamp: Option<Instant>,
    max_elapsed: Duration,
}

impl State {
    pub fn new(max_elapsed: Duration) -> State {
        State {
            value: String::new(),
            timestamp: None,
            max_elapsed,
        }
    }
    pub fn get(&self) -> &String {
        &self.value
    }
    pub fn set(&mut self, new_value: String) {
        self.timestamp = Some(Instant::now());
        self.value = new_value;
    }
    fn clear(&mut self) {
        self.timestamp = None;
        self.value.clear();
    }
    pub fn clear_if_expired(&mut self) {
        if let Some(ts) = self.timestamp {
            if ts.elapsed() >= self.max_elapsed {
                self.clear();
            }
        }
    }
}

pub fn start(max_elapsed: Duration) -> (tokio::task::JoinHandle<()>, Sender<Message>) {
    let mut state = State::new(max_elapsed);
    let (tx, mut rx) = mpsc::channel::<Message>(4);

    let timer_tx = tx.clone();
    tokio::task::spawn(async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            let (resp_tx, resp_rx) = oneshot::channel();
            timer_tx
                .send(Message::Check { resp: resp_tx })
                .await
                .unwrap();
            let res = resp_rx.await;
            println!("GOT = {:?}", res);
        }
    });

    let handle = tokio::task::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                Message::Get { resp } => {
                    let value = state.get();
                    let _ = resp.send(value.to_owned());
                }
                Message::Set { value, resp } => {
                    state.set(value);
                    let _ = resp.send(());
                }
                Message::Check { resp } => {
                    state.clear_if_expired();
                    let _ = resp.send(());
                }
            }
        }
    });

    (handle, tx)
}
