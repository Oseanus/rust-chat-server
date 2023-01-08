use tokio::{
    net::TcpListener, 
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt}
};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        
        tokio::spawn(async move {
            let (read, mut writer) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();
            
            loop{
                let bytes_read = reader.read_line(&mut line).await.unwrap();
    
                if bytes_read == 0 {
                    break;
                }
    
                writer.write_all(&line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
