#[macro_export]
macro_rules! account_models {
    () => {
        model! {
            String, NaiveDateTime, "accounts", NewAccount, "server gen",
            Account {
                address: String,
                email: String,
                business_name: String,
                short_name: String,
                city: String,
                zip_code: String,
                phone_number: String,
                stripe_id: Option<String>,
                state: String,
            }
        }
    };
}
