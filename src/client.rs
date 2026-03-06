use reqwest::Client;
#[derive(Debug)]
pub struct SmrithiClient {
    pub base_url: String,
    pub client: Client,
}

impl SmrithiClient {
    pub fn new(base_url: String,) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }
 
}
