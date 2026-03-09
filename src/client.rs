use reqwest::Client;
use dotenv::dotenv;
use std::env;
#[derive(Clone)]
pub struct SmritiClient {
    pub base_url: String,
    pub http_client: Client,
}
impl SmritiClient {
    pub fn new() -> Self {
        dotenv().ok();
        let base_url =
            env::var("SMRITI_END_POINT").expect("SMRITI_END_POINT not set");
        Self {
            base_url: base_url.to_string(),
            http_client: Client::new(),
        }
    }
}