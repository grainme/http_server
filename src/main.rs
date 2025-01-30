mod handler;
mod response;
mod server;

#[tokio::main]
async fn main() {
    server::run().await;
}
