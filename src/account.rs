use crate::error::ShodanError;
use crate::ShodanClient;
use serde::Deserialize;

pub trait Account {
    fn get_account_profile(&self) -> Result<AccountProfileResponse, ShodanError>;
}

#[derive(Deserialize, Debug)]
pub struct AccountProfileResponse {
    pub member: bool,
    pub credits: u32,
    pub display_name: Option<String>,
    pub created: String,
}

impl Account for ShodanClient {
    fn get_account_profile(&self) -> Result<AccountProfileResponse, ShodanError> {
        Self::fetch(self.build_request_url("/account/profile", None))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::account::Account;
    use crate::ShodanClient;
    use crate::tests::get_test_api_key;

    #[test]
    fn can_get_account_profile() {
        let client = ShodanClient::new(get_test_api_key());
        client.get_account_profile().unwrap();
    }
}
