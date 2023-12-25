use slack_chat_api::{Conversations, SlackApi};
use tokio;

#[tokio::main]
async fn main() {
    let token = "your-slack-bot-token"; // Replace with your actual token
    let channel_id = "target-channel-id"; // Replace with your actual channel ID

    let slack = SlackApi::new(token);

    match slack.conversations_history(channel_id, None, None, None, None, None, None).await {
        Ok(response) => {
            for message in response.messages {
                println!("{:?}", message);
            }
        },
        Err(error) => {
            eprintln!("Error fetching channel history: {:?}", error);
        }
    }
}
