use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Couldn't parse URL: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Shodan API error: {0}")]
    Shodan(String),

    #[error("Caught reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}
