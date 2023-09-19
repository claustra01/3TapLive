use anyhow::Result;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct TokenResponse {
  token: String,
}

pub async fn get_token(channel_name: String) -> Result<String> {
  let url = env::var("TOKEN_SERVER_URL").expect("Failed to find TOKEN_SERVER_URL in .env");
  let client = reqwest::Client::new();
  let res = client.get(format!("{}/token?name={}", url, channel_name))
    .send().await?
    .text().await?;
  let token_response: TokenResponse = serde_json::from_str::<TokenResponse>(&res).unwrap();
  Ok(token_response.token)
}
