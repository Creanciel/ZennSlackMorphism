use slack_morphism::{
    SlackApiToken, SlackApiTokenValue, SlackChannelId, SlackClient, SlackMessageContent,
    api::SlackApiChatPostMessageRequest, prelude::SlackClientHyperConnector,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Client の作成
    let client = SlackClient::new(SlackClientHyperConnector::new()?);

    // 環境変数から Token を取得
    let token_value = SlackApiTokenValue::new(std::env::var("SLACK_TOKEN")?);
    let token = SlackApiToken::new(token_value);

    // セッション構築
    let session = client.open_session(&token);

    // Channel ID を環境変数から取得
    let channel_id = SlackChannelId(std::env::var("CHANNEL_ID")?);

    // 送信する内容を作成 (Legacy)
    // REST API を直叩きするときは以下のようなフォーマット
    // {
    //     "text": "Hello"
    // }
    let content = SlackMessageContent::new().with_text("Hello".to_string());
    let request = SlackApiChatPostMessageRequest::new(channel_id.clone(), content);

    // 送信
    session.chat_post_message(&request).await?;

    Ok(())
}
