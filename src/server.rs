use crate::handler::handle_connection;
use tokio::net::TcpListener;

const PORT: u32 = 4221;

pub async fn run() {
    let addr = format!("127.0.0.1:{}", PORT);

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server listening on 127.0.0.1:4221");

    loop {
        let (stream, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            match handle_connection(stream).await {
                Ok(_) => println!("new client: {:?}", addr),
                Err(e) => eprintln!("Failed to handle connection: {}", e),
            }
        });
    }
}
