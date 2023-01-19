use crate::response::ShodanClientResponse;
use crate::ShodanClient;
use serde::Deserialize;
use crate::error::ShodanError;

pub trait Account {
    fn get_account_profile(
        &self,
    ) -> Result<AccountProfileResponse, ShodanError>;
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
    ) -> Result<AccountProfileResponse, ShodanError> {
        Self::fetch(self.build_request_url("/account/profile", None))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::account::Account;
    use crate::response::ShodanClientResponse;
    use crate::tests::get_test_api_key;
    use crate::ShodanClient;

    #[test]
    fn can_get_account_profile() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_account_profile().unwrap();

        // This endpoint is heavily rate limited so we should be good with either
        // response or API error

    }
}
