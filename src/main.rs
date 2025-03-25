mod clients;
mod deciders;
mod handlers;
mod setup;

use clients::{TwitchApiClient, WebSocketClient};
use deciders::message_decider::MessageDecider;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Sets up vars from .env file
    loop {
        let twitch_api_client = TwitchApiClient::new();
        let message_decider = MessageDecider::new(twitch_api_client);
        let client = WebSocketClient::new(message_decider);
        client.start().await;
    }
}
