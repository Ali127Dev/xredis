mod connection;
mod protocol;
mod worker;

use tokio::net::TcpListener;

use crate::connection::handle_connection;
use crate::worker::Request;
use tokio::sync::mpsc;
use worker::worker;

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
