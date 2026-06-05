use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:6379").await?;

    println!("Listening on 6379");

    let store = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    loop {
        let store = store.clone();

        let (socket, addr) = listener.accept().await?;

        println!("New connection: {}", addr);

        tokio::spawn(async move {
            handle_connection(socket, store).await;
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}

enum Command {
    Ping,
    Set { key: String, value: String },
    Get { key: String },
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

fn execute(cmd: Command, store: &Arc<Mutex<HashMap<String, String>>>) -> Option<String> {
    match cmd {
        Command::Ping => Some("PONG".to_string()),
        Command::Set { key, value } => {
            let mut db = store.lock().unwrap();
            db.insert(key, value);
            Some("OK".to_string())
        }
        Command::Get { key } => {
            let db = store.lock().unwrap();
            db.get(&key).cloned()
        }
    }
}

async fn handle_connection(mut socket: TcpStream, store: Arc<Mutex<HashMap<String, String>>>) {
    let mut buffer = [0; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("client disconnected");
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                if let Some(cmd) = parse(&message) {
                    if let Some(resp) = execute(cmd,&store) {
                        socket.write_all(resp.as_bytes()).await.unwrap();
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
