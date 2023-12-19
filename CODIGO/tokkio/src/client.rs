use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr).await.unwrap();

    loop {
        // Send a single line to the server (modify this section according to your needs)
        let data_to_send = "Hello, Server!\n";
        if let Err(e) = stream.write_all(data_to_send.as_bytes()).await {
            eprintln!("Error writing to stream: {:?}", e);
            return;
        }

        // Read the response from the server (modify this section according to your needs)
        let mut reader = BufReader::new(&mut stream);
        let mut response = String::new();
        if let Err(e) = reader.read_line(&mut response).await {
            eprintln!("Error reading line from stream: {:?}", e);
            return;
        }

        // Handle the received response (modify this section according to your needs)
        println!("Received response from server: {}", response);
    }
}
