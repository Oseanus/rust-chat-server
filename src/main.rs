use tokio::{
    net::TcpListener, 
    sync::broadcast,
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt},
};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    let (tx, _) = broadcast::channel(10);

    loop {
        let (mut socket, client_addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // Spawn for each new cient an own task
        tokio::spawn(async move {
            let (read, mut writer) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();
            
            loop{
                tokio::select! {
                    // Reads line
                    // Returns a future
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        // Sending message as broadcast
                        tx.send((line.clone(), client_addr)).unwrap();
                        line.clear();
                    }

                    // Receive message
                    // Returns a future
                    result = rx.recv() => {
                        let (message, other_addr) = result.unwrap();

                        if client_addr != other_addr {
                            writer.write_all(&message.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
