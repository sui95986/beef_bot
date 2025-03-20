use serde_json::Value;

pub fn handle_welcome_message(json: &Value) {
    let session_id = &json["payload"]["session"]["id"];
    println!("The session Id is: {}", session_id);
}

