use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};

use crate::protocol::parse;
use crate::worker::Request;

pub async fn handle_connection(mut socket: TcpStream, tx: mpsc::Sender<Request>) {
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
