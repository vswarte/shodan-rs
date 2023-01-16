use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ShodanClientResponse<T> {
    Error(ErrorResponse),
    Response(T),
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
}
