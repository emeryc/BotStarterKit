use http::Response;
use lambda::handler_fn;
use lambda_http::{Body, LambdaRequest, LambdaResponse, Request};
use log::debug;
use rusoto_core::region::Region;
use rusoto_sns::{ListTopicsInput, PublishInput, Sns, SnsClient};
use serde_json;
use slevr::OuterEvent;
use tokio;

use simple_logger;

// fn main() -> Result<(), Box<dyn Error>> {
//     simple_logger::init_with_level(log::Level::Debug)?;
//     lambda!(hello);

//     Ok(())
// }

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Debug)?;
    let func = handler_fn(func);
    lambda::run(func).await?;
    Ok(())
}

async fn func(request: LambdaRequest<'_>) -> Result<LambdaResponse, Error> {
    let request: Request = request.into();
    let body = request.body();
    debug!("{:?}", body);
    let response = match body {
        Body::Text(body) => match serde_json::from_str::<OuterEvent>(body).unwrap() {
            OuterEvent::UrlVerification { challenge, .. } => {
                debug!("Got a challenge, responding accordingly?");
                Response::builder().status(200).body(challenge).unwrap()
            }
            s => {
                debug!("Not a UrlVerification, looks like: {:?}", s);
                let sns = SnsClient::new(Region::UsWest1);
                let topics = sns.list_topics(ListTopicsInput { next_token: None }).await;
                debug!("Topics - {:?}", topics);
                let topic = topics?.topics.and_then(|a| {
                    a.into_iter()
                        .filter(|topic| match topic.topic_arn.as_ref() {
                            Some(t) => t.contains("slack_incoming_messages"),
                            None => false,
                        })
                        .last()
                        .and_then(|t| t.topic_arn)
                });

                let publish = PublishInput {
                    message: body.to_string(),
                    message_attributes: None,
                    message_structure: None,
                    topic_arn: topic,
                    phone_number: None,
                    subject: None,
                    target_arn: None,
                };

                debug!("Publishing - {:?}", publish);

                let published = sns.publish(publish).await;

                debug!("Published - {:?}", published);

                match published {
                    Ok(_) => Response::builder()
                        .status(200)
                        .body("".to_string())
                        .unwrap(),
                    Err(err) => Response::builder()
                        .status(500)
                        .body(format!("{:?}", err))
                        .unwrap(),
                }
            }
        },
        _ => Response::builder()
            .status(400)
            .body("Need a text body".into())
            .unwrap(),
    };

    debug!("Response - {:?}", response);

    Ok(LambdaResponse::from_response(false, response))
}
