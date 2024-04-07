#[derive(Debug)]
pub enum ShodanError {
    ShodanClientError(String),
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for ShodanError {
    fn from(e: reqwest::Error) -> Self {
        ShodanError::ReqwestError(e)
    }
}
