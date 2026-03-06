use thiserror::Error;
#[derive(Error,Debug)]
pub enum SdkError{
    #[error("Request error")]
    Reqwest(#[from]reqwest::Error),
}

