use crate::clients::TwitchApiClient;
use serde_json::Value;

pub async fn handle(json: &Value) {
    let session_id = &json["payload"]["session"]["id"];

    println!("The session Id is: {}", session_id);

    let client = TwitchApiClient::new();

    // Subscribe to chat message events
    let result = client.subscribe_to_chat_events(session_id);

    let response = result
        .await
        .expect("Call to subscribe to chat messages errored");

    println!(
        "Response when register to listen to chat messages: {}",
        response.status(),
        // response.text().await.unwrap()
    );

    let ad_break_result = client.subscribe_to_ad_break_begin_events(session_id);

    let ad_break_response = ad_break_result
        .await
        .expect("Call to subscribe to ad break begin events errored");

    println!(
        "Response when register to listen to ad break begin events: {}",
        ad_break_response.status(),
        // ad_break_response.text().await.unwrap()
    );
}
