// mod server;

use tokio::{
    net::TcpListener, 
    io::{AsyncReadExt, AsyncWriteExt}
};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();

    let (mut socket, socketAddr) = listener.accept().await.unwrap();

    loop{
        let mut buffer = [0u8; 1024];
        let bytes_read = socket.read(&mut buffer).await.unwrap();

        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }
}
