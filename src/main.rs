mod clients;
mod deciders;
mod handlers;
mod setup;

use clients::{TwitchApiClient, WebSocketClient};
use deciders::message_decider::MessageDecider;
use dotenv::dotenv;
use handlers::ChatMessageHandler;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Sets up vars from .env file
    loop {
        let twitch_api_client = TwitchApiClient::new();
        let chat_message_handler = ChatMessageHandler::new(twitch_api_client);
        let message_decider = MessageDecider::new(chat_message_handler);
        let client = WebSocketClient::new(message_decider);
        client.start().await;
    }
}
