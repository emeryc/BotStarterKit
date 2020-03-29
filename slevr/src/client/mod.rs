use hyper::{client::HttpConnector, Body, Client};
use hyper_rustls::HttpsConnector;

pub mod chat;

pub struct SlackApiClient {
    client: Client<HttpsConnector<HttpConnector>>,
    oauth: String,
}

impl SlackApiClient {
    pub fn new(oauth: &str) -> Self {
        SlackApiClient {
            client: Client::builder().build::<_, Body>(HttpsConnector::new()),
            oauth: oauth.to_string(),
        }
    }
}
