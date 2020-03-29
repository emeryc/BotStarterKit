use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub(crate) struct SNSMessage {
    pub(crate) records: Vec<Record>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub(crate) struct Record {
    pub(crate) event_version: String,
    pub(crate) event_subscription_arn: String,
    pub(crate) event_source: String,
    pub(crate) sns: Sns,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub(crate) struct Sns {
    pub(crate) signature_version: String,
    pub(crate) timestamp: String,
    pub(crate) signature: String,
    pub(crate) signing_cert_url: String,
    pub(crate) message_id: String,
    pub(crate) message: String,
    pub(crate) message_attributes: Value,
    pub(crate) r#type: String,
    pub(crate) unsubscribe_url: String,
    pub(crate) topic_arn: String,
    pub(crate) subject: Option<String>,
}
