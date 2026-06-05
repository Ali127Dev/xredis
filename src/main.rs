use std::collections::HashMap;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::net::{TcpListener, TcpStream};

use tokio::sync::{mpsc, oneshot};

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:6379").await?;

    println!("Listening on 6379");

    let (tx, rx) = mpsc::channel::<Request>(32);

    tokio::spawn(worker(rx));

    loop {
        let (socket, addr) = listener.accept().await?;

        println!("New connection: {}", addr);

        let tx = tx.clone();

        tokio::spawn(async move {
            handle_connection(socket, tx).await;
        });
    }
}

enum Command {
    Ping,
    Set { key: String, value: String },
    Get { key: String },
}

struct Request {
    cmd: Command,
    respond_to: oneshot::Sender<Option<String>>,
}

fn parse(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["PING"] => Some(Command::Ping),
        ["SET", key, value] => Some(Command::Set {
            key: key.to_string(),
            value: value.to_string(),
        }),
        ["GET", key] => Some(Command::Get {
            key: key.to_string(),
        }),
        _ => None,
    }
}

async fn handle_connection(mut socket: TcpStream, tx: mpsc::Sender<Request>) {
    let mut buffer = [0; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);

                if let Some(cmd) = parse(&message) {
                    let (resp_tx, resp_rx) = oneshot::channel();

                    let req = Request {
                        cmd,
                        respond_to: resp_tx,
                    };
                    if tx.send(req).await.is_err() {
                        break;
                    }
                    if let Ok(Some(resp)) = resp_rx.await {
                        let _ = socket.write_all(resp.as_bytes()).await;
                    }
                }
            }
            Err(_) => break,
        }
    }
}

async fn worker(mut rx: mpsc::Receiver<Request>) {
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
