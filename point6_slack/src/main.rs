mod auth;

use auth::verify_hmac;
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
    let (parts, body) = request.into_parts();
    let response = if let Body::Text(body) = body {
        debug!("body - {:?}", body);
        match (
            parts.headers.get("X-Slack-Signature"),
            parts.headers.get("X-Slack-Request-Timestamp"),
        ) {
            (Some(sig), Some(ts)) => {
                debug!("sig - {:?}", sig);
                debug!("ts - {:?}", ts);

                if let Result::Err(err) =
                    verify_hmac(ts.to_str().unwrap(), &body[..], sig.to_str().unwrap()).await
                {
                    debug!("{}", err);
                    return Ok(LambdaResponse::from_response(
                        false,
                        Response::builder()
                            .status(403)
                            .body("".to_string())
                            .unwrap(),
                    ));
                };
            }
            _ => {
                return Ok(LambdaResponse::from_response(
                    false,
                    Response::builder()
                        .status(403)
                        .body("".to_string())
                        .unwrap(),
                ));
            }
        }

        let event = serde_json::from_str::<OuterEvent>(&body[..]).unwrap();
        match event {
            OuterEvent::UrlVerification { challenge, .. } => {
                debug!("Got a challenge, responding accordingly?");
                Response::builder().status(200).body(challenge).unwrap()
            }
            _ => {
                let published = forward_to_sns(&body[..]).await;
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
        }
    } else {
        Response::builder()
            .status(400)
            .body("".to_string())
            .unwrap()
    };

    debug!("Response - {:?}", response);

    Ok(LambdaResponse::from_response(false, response))
}

async fn forward_to_sns(body: &str) -> Result<(), String> {
    debug!("Forwarding to SNS: {:?}", body);
    let sns = SnsClient::new(Region::UsWest1);
    let topics = sns.list_topics(ListTopicsInput { next_token: None }).await;
    debug!("Topics - {:?}", topics);
    let topic = topics
        .map_err(|e| format!("Topic Error {:?}", e))?
        .topics
        .and_then(|a| {
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
    published
        .map(|_| ())
        .map_err(|err| format!("Publish Error - {:?}", err))
}
