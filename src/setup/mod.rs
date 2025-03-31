use dotenv_codegen::dotenv;

pub async fn validate_oauth_tokens() {
    let bot_oauth_token = dotenv!("BOT_OAUTH_TOKEN");
    validate_oauth_token(bot_oauth_token).await;
    let broadcaster_oauth_token = dotenv!("BROADCASTER_OAUTH_TOKEN");
    validate_oauth_token(broadcaster_oauth_token).await;
}

pub async fn validate_oauth_token(api_token: &str) {
    let client = reqwest::Client::new();
    let result = client
        .get("https://id.twitch.tv/oauth2/validate")
        .bearer_auth(api_token)
        .send()
        .await;

    let response = result.expect("Call to validate oauth token failed.");

    println!(
        "Oauth token validate response status code: {}",
        response.status()
    );
}
