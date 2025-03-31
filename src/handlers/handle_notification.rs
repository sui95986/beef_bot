use serde_json::Value;

use crate::clients::TwitchApiClient;
use dotenv_codegen::dotenv;

pub struct NotificationHandler {
    twitch_api_client: TwitchApiClient,
}

impl NotificationHandler {
    pub fn new(twitch_api_client: TwitchApiClient) -> NotificationHandler {
        NotificationHandler { twitch_api_client }
    }

    pub async fn handle_chat_message(&self, json: &Value) {
        let bot_user_id = dotenv!("BOT_USER_ID");
        let event = &json["payload"]["event"];
        let chatter = event["chatter_user_name"].as_str().unwrap();
        let chatter_user_id = event["chatter_user_id"].as_str().unwrap();
        let message = event["message"]["text"].as_str().unwrap();
        println!("{}: {}", chatter, message);
        if chatter_user_id != bot_user_id && message.starts_with("!") {
            let string_message = String::from(message);
            let mut parts = string_message.splitn(2, " ");
            let cmd = parts.next().unwrap_or("");
            let message = parts.next().unwrap_or("");
            self.handle_command(cmd, message, chatter).await;
        };
    }

    pub async fn handle_command(&self, cmd: &str, message: &str, chatter_user_name: &str) {
        match cmd {
            "!love" => {
                self.twitch_api_client
                    .send_chat_message(format!(
                        "Hi @{}, I love you and your beef cheeks",
                        chatter_user_name
                    ))
                    .await;
            }
            "!test" => {
                self.twitch_api_client
                    .send_chat_message(format!(
                        "{} typed the command: {} with rest of command: {}",
                        chatter_user_name, cmd, message
                    ))
                    .await;
            }
            "!analdischarge" => {
                self.twitch_api_client
                    .send_chat_message("You should probably put on a diaper".to_string())
                    .await;
            }
            _ => {
                self.twitch_api_client
                    .send_chat_message("unknown command".to_string())
                    .await;
            }
        }
    }

    pub async fn handle_ad_break_begin(&self, json: &Value) {
        self.twitch_api_client
            .send_chat_message("Ad break starting!".to_owned())
            .await;
        println!("{}", json);
    }
}
