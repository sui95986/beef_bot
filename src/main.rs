mod deciders;
mod handlers;
mod setup;

use crate::deciders::message_decider;
use dotenv::dotenv;
use futures_util::StreamExt;
use setup::validate_oauth_token;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Sets up vars from .env file
    validate_oauth_token().await;

    let url = "wss://eventsub.wss.twitch.tv/ws?keepalive_timeout_seconds=120"
        .into_client_request()
        .unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Faled to connect");
    println!("Websocket handshake has been successfully completed");

    let (mut _write, read) = ws_stream.split();

    let read_future = read.for_each(|message| async {
        match message {
            Ok(msg) => {
                message_decider::decide(msg).await;
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
            }
        }
    });
    read_future.await;
}
