use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};

use crate::protocol::parse;
use crate::worker::Request;

pub async fn handle_connection(
    mut socket: TcpStream,
    tx: mpsc::Sender<Request>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut temp = [0; 1024];

    loop {
        let n = socket.read(&mut temp).await?;
        if n == 0 {
            break Ok(());
        }

        buffer.extend_from_slice(&temp[..n]);

        while let Some(pos) = buffer.iter().position(|b| *b == b'\n') {
            let line = buffer.drain(..=pos).collect::<Vec<_>>();

            let line = &line[..line.len() - 1];

            let message = String::from_utf8_lossy(line);

            if let Some(cmd) = parse(&message) {
                let (resp_tx, resp_rx) = oneshot::channel();

                let req = Request {
                    cmd,

                    respond_to: resp_tx,
                };

                tx.send(req).await?;

                if let Ok(Some(resp)) = resp_rx.await {
                    socket.write_all(resp.as_bytes()).await?;

                    socket.write_all(b"\n").await?;
                }
            }
        }
    }
}
