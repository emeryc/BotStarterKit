use futures::future::join_all;
use lambda::handler_fn;
use log::debug;
use rusoto_core::Region;
use rusoto_secretsmanager::{GetSecretValueRequest, SecretsManager, SecretsManagerClient};
use serde_json::{from_str, Value};
use simple_logger;
use slevr::{InnerEvent, OuterEvent, SlackApiClient};
use tokio;

mod sns;
use sns::SNSMessage;

mod dynamo;
use dynamo::EchoTabel;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda::run(handler_fn(func)).await?;
    Ok(())
}

async fn func(message: SNSMessage) -> Result<String, Error> {
    debug!("{:#?}", message);

    let slack_client = SlackApiClient::new(&{
        let secrets_manager = SecretsManagerClient::new(Region::UsWest1);
        secrets_manager
            .get_secret_value(GetSecretValueRequest {
                secret_id: "SlackClientSecret08B328AB-55ENGjJh0rOK".to_string(),
                version_id: None,
                version_stage: None,
            })
            .await?
            .secret_string
            .unwrap()
    });

    let echo_tabel = EchoTabel::new();

    let slack_message_str = &message.records.first().unwrap().sns.message[..];
    let slack_message = serde_json::from_str::<OuterEvent>(slack_message_str);

    if let Ok(OuterEvent::EventCallback {
        event:
            InnerEvent::Message {
                channel,
                user,
                text,
                channel_type,
                ..
            },
        ..
    }) = slack_message
    {
        match (&channel_type[..], &text[..]) {
            ("im", "echo all") => {
                let result = slack_client
                    .chat_post_message(slevr::chat::post_message::ChatMessage {
                        channel,
                        text: match echo_tabel.add_listener(user).await {
                            Ok(_) => "I\'ll now echo everything to you",
                            Err(_) => "Got an error",
                        }
                        .to_string(),
                        ..Default::default()
                    })
                    .await;
                debug!("{:?}", result);
            }
            ("im", "echo none") => {
                // remove user from dynamo!
                let result = slack_client
                    .chat_post_message(slevr::chat::post_message::ChatMessage {
                        channel,
                        text: match echo_tabel.remove_listener(user).await {
                            Ok(_) => "You have been unsubscribed",
                            Err(_) => "Couldn't remove you",
                        }
                        .to_string(),
                        ..Default::default()
                    })
                    .await;
                debug!("{:?}", result);
            }
            ("im", "help") => {
                let result = slack_client
                    .chat_post_message(slevr::chat::post_message::ChatMessage {
                        channel,
                        text: " You can ask me to send you messages. 
                        I can either echo everything to you, by you IMing me `echo all`
                        or I can send nothing to you, if you IM me `echo none`."
                            .to_string(),
                        ..Default::default()
                    })
                    .await;
                debug!("{:?}", result);
            }
            _ => (),
        }
    }
    let val: Value = from_str(slack_message_str).unwrap();
    let messages = echo_tabel
        .get_listeners()
        .await
        .into_iter()
        .map(|user| {
            let chat_message = slevr::chat::post_message::ChatMessage {
                channel: user,
                text: format!("```{:#?}```", val),
                ..Default::default()
            };
            slack_client.chat_post_message(chat_message)
        })
        .collect::<Vec<_>>();
    join_all(messages).await;

    Ok("Success".into())
}
