use crate::deciders::message_decider;
use crate::setup::validate_oauth_token;
use dotenv_codegen::dotenv;
use futures_util::StreamExt;
use serde_json::{Value, json};
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;

pub struct WebSocketClient;
pub struct TwitchApiClient {
    api_token: String,
    client_id: String,
    bot_user_id: String,
    broadcaster_id: String,
    reqwest_client: reqwest::Client,
}

impl WebSocketClient {
    pub fn new() -> WebSocketClient {
        WebSocketClient {}
    }

    pub async fn start(self) {
        validate_oauth_token().await;
        let url = "wss://eventsub.wss.twitch.tv/ws?keepalive_timeout_seconds=30"
            .into_client_request()
            .unwrap();

        let (ws_stream, _) = connect_async(url).await.expect("Faled to connect");

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
}

impl TwitchApiClient {
    pub fn new() -> TwitchApiClient {
        let client = reqwest::Client::new();
        TwitchApiClient {
            api_token: dotenv!("OAUTH_TOKEN").to_string(),
            client_id: dotenv!("CLIENT_ID").to_string(),
            bot_user_id: dotenv!("BOT_USER_ID").to_string(),
            broadcaster_id: dotenv!("BROADCASTER_ID").to_string(),
            reqwest_client: client,
        }
    }

    pub async fn subscribe_to_chat_events(
        &self,
        session_id: &Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let request_body = self.get_request_body(&session_id);
        let result = self
            .reqwest_client
            .post("https://api.twitch.tv/helix/eventsub/subscriptions")
            .bearer_auth(&self.api_token)
            .header("Client-Id", &self.client_id)
            .json(&request_body)
            .send()
            .await;
        result
    }

    fn get_request_body(&self, session_id: &Value) -> Value {
        let json = json!({
            "type": "channel.chat.message",
            "version": "1",
            "condition": {
                "broadcaster_user_id": self.broadcaster_id,
                "user_id": self.bot_user_id,
            },
            "transport": {
                "method": "websocket",
                "session_id": session_id
            }
        });
        json
    }
}
