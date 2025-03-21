use serde_json::Value;

pub async fn handle(json: &Value) {
    let event = &json["payload"]["event"];
    let chatter = &event["chatter_user_name"].as_str().unwrap();
    let message = &event["message"]["text"].as_str().unwrap();
    println!("{}: {}", chatter, message);
}
