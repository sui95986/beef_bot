use dotenv_codegen::dotenv;

pub async fn validate_oauth_token() {
    let api_token = dotenv!("OAUTH_TOKEN");
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
