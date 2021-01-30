use tokio::net::TcpListener;

use std::io;

mod messages;
mod server;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:6667").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        //process_socket(socket).await;
    }
    Ok(())
}
