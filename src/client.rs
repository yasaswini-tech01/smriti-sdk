use reqwest::Client;

#[derive(Clone)]
pub struct SmritiClient {
    pub base_url: String,
    pub http_client: Client,
}

impl SmritiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client: Client::new(),
        }
    }
}