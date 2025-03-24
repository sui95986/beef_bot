mod clients;
mod deciders;
mod handlers;
mod setup;

use clients::WebSocketClient;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Sets up vars from .env file
    loop {
        let client = WebSocketClient::new();
        client.start().await;
    }
}
