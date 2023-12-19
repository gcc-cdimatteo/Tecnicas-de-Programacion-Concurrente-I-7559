use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let stream_clone = &mut stream;
        loop {
            tokio::spawn(handle_client(stream_clone));
        }
    }
}

async fn handle_client(stream: &mut TcpStream) {
    let mut reader = BufReader::new(&mut stream);

    // Read a single line from the client
    let mut line = String::new();
    match reader.read_line(&mut line).await {
        Ok(0) => {
            println!("Connection closed by client");
            return;
        }
        Ok(_) => {
            // Handle the received line (modify this section according to your needs)
            println!("Received line from client: {}", line);

            // Send a response back to the client (modify this section according to your needs)
            if let Err(e) = stream
                .write_all(format!("Hi Client, I received from you: {}", line).as_bytes())
                .await
            {
                eprintln!("Error writing to stream: {:?}", e);
                return;
            }
        }
        Err(e) => {
            eprintln!("Error reading line from stream: {:?}", e);
            return;
        }
    }
}
