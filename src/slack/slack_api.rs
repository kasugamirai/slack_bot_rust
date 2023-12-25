use reqwest::Error;
use serde::Deserialize;

const SLACK_API_URL: &str = "https://slack.com/api/";

#[derive(Deserialize)]
struct Message {
    // Define message structure
}

pub async fn get_channel_history(token: &str, channel_id: &str) -> Result<Vec<Message>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}conversations.history", SLACK_API_URL))
        .bearer_auth(token)
        .form(&[("channel", channel_id)])
        .send()
        .await?
        .json::<Vec<Message>>()
        .await?;

    Ok(response)
}
