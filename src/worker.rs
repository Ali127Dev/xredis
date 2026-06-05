use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

use crate::protocol::Command;

pub struct Request {
    pub cmd: Command,
    pub respond_to: oneshot::Sender<Option<String>>,
}

pub async fn worker(mut rx: mpsc::Receiver<Request>) {
    let mut db: HashMap<String, String> = HashMap::new();

    while let Some(req) = rx.recv().await {
        let response = match req.cmd {
            Command::Ping => Some("PONG".to_string()),

            Command::Set { key, value } => {
                db.insert(key, value);
                Some("OK".to_string())
            }

            Command::Get { key } => db.get(&key).cloned(),
        };

        let _ = req.respond_to.send(response);
    }
}
