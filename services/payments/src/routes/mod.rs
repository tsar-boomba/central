/// Accepts POST requests
pub mod create_usage_record {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CreateUsageRecordParams {
        pub stripe_id: String,
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

    pub type SubscribeParams = models::Account;

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

    pub const ROUTE: &str = "/customer";
}
