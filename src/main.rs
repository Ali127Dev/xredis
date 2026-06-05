use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:6379").await?;

    println!("Listening on 6379");

    loop {
        let (socket, addr) = listener.accept().await?;

        println!("New connection: {}", addr);

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("client disconnected");
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {}", message);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
