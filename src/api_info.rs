use crate::response::ShodanClientResponse;
use crate::ShodanClient;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiInfoResponse {
    pub scan_credits: u32,
    pub usage_limits: ApiInfoResponseUsageLimits,
    pub plan: String,
    pub https: bool,
    pub unlocked: bool,
    pub query_credits: u32,
    pub monitored_ips: Option<u32>,
    pub unlocked_left: u32,
    pub telnet: bool,
}

#[derive(Deserialize, Debug)]
pub struct ApiInfoResponseUsageLimits {
    pub scan_credits: i32,
    pub query_credits: i32,
    pub monitored_ips: i32,
}

trait ApiInfo {
    fn get_api_info(&self) -> Result<ShodanClientResponse<ApiInfoResponse>, reqwest::Error>;
}

impl ApiInfo for ShodanClient {
    fn get_api_info(&self) -> Result<ShodanClientResponse<ApiInfoResponse>, reqwest::Error> {
        let url = self.build_request_url("/api-info", None);

        let res = reqwest::blocking::get(url)?.json::<ShodanClientResponse<ApiInfoResponse>>()?;

        Ok(res)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::api_info::ApiInfo;
    use crate::response::ShodanClientResponse;
    use crate::tests::get_test_api_key;
    use crate::ShodanClient;

    #[test]
    fn can_request_api_info() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_api_info().unwrap();

        assert!(matches!(response, ShodanClientResponse::Response { .. }));
    }
}
