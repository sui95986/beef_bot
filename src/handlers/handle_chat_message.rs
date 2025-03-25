use serde_json::Value;

use crate::clients::TwitchApiClient;

pub async fn handle(json: &Value, twitch_api_client: &TwitchApiClient) {
    let event = &json["payload"]["event"];
    let chatter = &event["chatter_user_name"].as_str().unwrap();
    let message = &event["message"]["text"].as_str().unwrap();
    println!("{}: {}", chatter, message);
    twitch_api_client
        .send_chat_message(String::from("Hello world"))
        .await;
}
