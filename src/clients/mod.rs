use crate::deciders::message_decider::MessageDecider;
use crate::setup::validate_oauth_token;
use dotenv_codegen::dotenv;
use futures_util::StreamExt;
use serde_json::{Value, json};
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;

pub struct WebSocketClient {
    message_decider: MessageDecider,
}

pub struct TwitchApiClient {
    api_token: String,
    client_id: String,
    bot_user_id: String,
    broadcaster_id: String,
    reqwest_client: reqwest::Client,
}

impl WebSocketClient {
    pub fn new(message_decider: MessageDecider) -> WebSocketClient {
        WebSocketClient { message_decider }
    }

    pub async fn start(&self) {
        validate_oauth_token().await;
        let url = "wss://eventsub.wss.twitch.tv/ws?keepalive_timeout_seconds=30"
            .into_client_request()
            .unwrap();

        let (ws_stream, _) = connect_async(url).await.expect("Faled to connect");

        let (mut _write, read) = ws_stream.split();

        let read_future = read.for_each(|message| async {
            match message {
                Ok(msg) => {
                    self.message_decider.decide(msg).await;
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
        let request_body = self.get_subscribe_to_chat_request_body(session_id);

        self.reqwest_client
            .post("https://api.twitch.tv/helix/eventsub/subscriptions")
            .bearer_auth(&self.api_token)
            .header("Client-Id", &self.client_id)
            .json(&request_body)
            .send()
            .await
    }

    fn get_subscribe_to_chat_request_body(&self, session_id: &Value) -> Value {
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

    pub async fn send_chat_message(&self, arg: String) {
        println!("Sending chat message: {}", arg);
        let request_body = self.get_send_chat_message_request_body(arg);
        if let Err(e) = self
            .reqwest_client
            .post("https://api.twitch.tv/helix/chat/messages")
            .bearer_auth(&self.api_token)
            .header("Client-Id", &self.client_id)
            .json(&request_body)
            .send()
            .await
        {
            eprintln!("Error sending chat message {}", e);
        }
    }

    fn get_send_chat_message_request_body(&self, chat_message: String) -> Value {
        let json = json!({
            "broadcaster_id": self.broadcaster_id,
            "sender_id": self.bot_user_id,
            "message": chat_message,
        });
        json
    }
}
