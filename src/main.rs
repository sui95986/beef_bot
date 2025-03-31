mod brain;
mod clients;
mod deciders;
mod handlers;
mod setup;

use brain::Brain;
use clients::{TwitchApiClient, WebSocketClient};
use deciders::message_decider::MessageDecider;
use dotenv::dotenv;
use handlers::NotificationHandler;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Sets up vars from .env file
    loop {
        let brain = Brain::new().await;
        let twitch_api_client = TwitchApiClient::new();
        let chat_message_handler = NotificationHandler::new(twitch_api_client, brain);
        let message_decider = MessageDecider::new(chat_message_handler);
        let client = WebSocketClient::new(message_decider);
        client.start().await;
    }
}
