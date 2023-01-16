use serde::{Deserialize};
use crate::ShodanClient;
use crate::response::ShodanClientResponse;

pub trait Account {
    fn get_account_profile(
        &self,
    ) -> Result<ShodanClientResponse<AccountProfileResponse>, reqwest::Error>;
}

#[derive(Deserialize, Debug)]
pub struct AccountProfileResponse {
    pub member: bool,
    pub credits: u32,
    pub display_name: Option<String>,
    pub created: String,
}

impl Account for ShodanClient {
    fn get_account_profile(
        &self,
    ) -> Result<ShodanClientResponse<AccountProfileResponse>, reqwest::Error> {
        let url = self.build_request_url("/account/profile", None);

        let res = reqwest::blocking::get(url)?
            .json::<ShodanClientResponse<AccountProfileResponse>>()?;

        Ok(res)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::ShodanClient;
    use crate::account::Account;
    use crate::tests::get_test_api_key;
    use crate::response ::ShodanClientResponse;

    #[test]
    fn can_request_account_profile() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_account_profile().unwrap();

        assert!(matches!(response, ShodanClientResponse::Response{ .. }));
    }
}
