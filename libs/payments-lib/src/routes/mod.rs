/// Accepts POST requests
pub mod create_usage_record {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CreateUsageRecordParams {
        pub sub_id: String,
        /// Should be either "instances" or "users"
        pub resource: String,
        pub number: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CreateUsageRecordResponse;

    /// Accepts POST requests
    pub const ROUTE: &str = "/create-usage-record";
}

pub mod subscribe {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SubscribeParams {
        pub account: models::Account,
        pub payment_method_id: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SubscribeResponse {
        pub sub_id: String,
        pub client_secret: String,
    }

    pub const ROUTE: &str = "/subscribe";
}

pub mod customer {
    pub type CustomerParams = models::Account;
    pub type CustomerResponse = String;

    pub mod update {
        pub type UpdateCustomerParams = models::UpdateAccount;
        pub type UpdateCustomerResponse = ();

        pub fn route(id: &str) -> String {
            "/customer/".to_string() + id
        }
    }

    pub const ROUTE: &str = "/customer";
}

pub mod is_subbed {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct IsSubbedQuery {
        pub sub_id: String,
    }

    pub type IsSubbedResponse = String;

    pub const ROUTE: &str = "/is-subbed";
}
