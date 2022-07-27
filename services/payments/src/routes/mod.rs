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

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubscribeParams {
        pub account: crate::crud_models::Account,
        /// Card number
        pub number: String,
        pub exp_year: i32,
        pub exp_month: i32,
        /// 3 numbers on the back
        pub cvc: String,
    }

	pub type SubscribeResponse = String;

    pub const ROUTE: &str = "/subscribe";
}
