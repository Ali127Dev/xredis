use tokio::io::{AsyncReadExt, AsyncWriteExt};
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

enum Command {
    Ping,
}

fn parse(input: &str) -> Option<Command> {
    match input.trim() {
        "PING" => Some(Command::Ping),
        _ => None,
    }
}

fn execute(cmd: Command) -> Option<&'static str> {
    match cmd {
        Command::Ping => Some("PONG"),
    }
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
                if let Some(cmd) = parse(&message) {
                    if let Some(resp) = execute(cmd) {
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

fn execute_command(command: &str) -> Option<&'static str> {
    match command.trim() {
        "PING" => Some("PONG"),
        _ => None,
    }
}
