use slack_morphism::{
    SlackApiToken, SlackApiTokenValue, SlackChannelId, SlackClient, SlackMessageContent,
    api::SlackApiChatPostMessageRequest,
    blocks::{SlackBlock, SlackBlockPlainText, SlackBlockText, SlackSectionBlock},
    prelude::SlackClientHyperConnector,
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

    // Block API で送信する内容を作成
    // REST API を直叩きするときは以下のようなフォーマット
    // {
    //     "blocks": [
    //         {
    //             "type": "section",
    //             "text": {
    //                 "type": "plain_text",
    //                 "text": "Hello"
    //             }
    //         }
    //     ]
    // }
    let blocks: Vec<SlackBlock> = vec![SlackBlock::Section(SlackSectionBlock::new().with_text(
        SlackBlockText::Plain(SlackBlockPlainText::new("Hello".to_owned())),
    ))];
    let content = SlackMessageContent::new().with_blocks(blocks);
    let request = SlackApiChatPostMessageRequest::new(channel_id, content);

    // 送信
    session.chat_post_message(&request).await?;

    Ok(())
}
