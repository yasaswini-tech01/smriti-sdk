use reqwest::Client;
use dotenvy::dotenv;
use std::env;

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
            base_url,
            http_client: Client::new(),
        }
    }

}