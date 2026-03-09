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

        let base_url = env::var("SMRITI_BASE_URL")
            .expect("SMRITI_BASE_URL must be set in .env");

        Self {
            base_url,
            http_client: Client::new(),
        }
    }

}