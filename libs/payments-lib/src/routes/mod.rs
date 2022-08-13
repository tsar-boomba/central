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

pub mod subscription {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateSubscriptionParams {
        pub account: models::Account,
        pub payment_method_id: String,
    }
    pub type CreateSubscriptionResponse = ();

    pub type UpdateSubscriptionParams = CreateSubscriptionParams;
    // not used in backend at all
    pub type UpdateSubscriptionResponse = ();

    pub const ROUTE: &str = "/subscription";
}

pub mod customer {
    pub type CustomerParams = models::Account;
    pub type CustomerResponse = String;

    pub type UpdateCustomerParams = models::UpdateAccount;
    pub type UpdateCustomerResponse = ();

    pub fn id_route(id: &str) -> String {
        "/customer/".to_string() + id
    }

    pub const ROUTE: &str = "/customer";
}

pub mod sub_status {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct IsSubbedQuery {
        pub sub_id: String,
    }

    pub type IsSubbedResponse = String;

    pub const ROUTE: &str = "/sub-status";
}
