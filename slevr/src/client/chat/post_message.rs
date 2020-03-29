use crate::SlackApiClient;
use hyper::{Body, Method, Request};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Result};

#[derive(Serialize, Debug)]
pub struct ChatMessage {
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name.
    pub channel: String,
    /// Text to display: More info - https://api.slack.com/methods/chat.postMessage#text_usage
    pub text: String,
    // pub attachments: Option<Vec<HashMap<String, String>>>,
    // pub blocks: ???
    /// Emoji to use as the icon for this message. Overrides icon_url.
    /// Must be used in conjunction with as_user set to false, otherwise ignored.
    pub icon_emoji: Option<String>,
    /// URL to an image to use as the icon for this message. Must be used in conjunction
    /// with as_user set to false, otherwise ignored.
    pub icon_url: Option<String>,
    /// Find and link channel names and usernames. default true?
    pub link_names: Option<bool>,
    /// Disable Slack markup parsing by setting to false. Enabled by default.
    pub mrkdnw: Option<bool>,
    // pub parse: Option<String?>
    /// Used in conjunction with thread_ts and indicates whether reply should be made visible
    /// to everyone in the channel or conversation. Defaults to false.
    pub reply_broadcast: Option<bool>,
    /// Provide another message's ts value to make this message a reply. Avoid using a reply's
    /// ts value; use its parent instead.
    pub thread_ts: Option<String>,
    /// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
}

impl Default for ChatMessage {
    fn default() -> Self {
        ChatMessage {
            channel: "".to_string(),
            text: "".to_string(),
            icon_emoji: None,
            icon_url: None,
            link_names: None,
            mrkdnw: None,
            reply_broadcast: None,
            thread_ts: None,
            unfurl_links: None,
            unfurl_media: None,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ChatMessageResponse {
    Error {
        error: String,
    },
    Ok {
        channel: String,
        ts: String,
        message: MessageResponse,
    },
}
#[derive(Deserialize, Debug)]
pub struct MessageResponse {
    text: String,
    username: String,
    bot_id: String,
    r#type: String,
    subtype: String,
    ts: String,
}

impl SlackApiClient {
    pub async fn chat_post_message(&self, message: ChatMessage) -> Result<ChatMessageResponse> {
        let request = Request::builder()
            .method(Method::POST)
            .uri("https://slack.com/api/chat.postMessage")
            .header("content-type", "application/json")
            .header("Authorization", format!("Bearer {}", self.oauth))
            .body(Body::from(to_string(&message).unwrap()))
            .unwrap();
        debug!("request - {:#?}", request);
        let resp = self.client.request(request).await.unwrap();
        debug!("response - {:#?}", resp);
        // let (parts, body): (_, Body) = resp.into_parts();
        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let c_m_r: Result<ChatMessageResponse> = serde_json::from_slice(&body);
        debug!("response - {:#?}", c_m_r);
        c_m_r
    }
}
