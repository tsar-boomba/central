#[macro_export]
macro_rules! instance_models {
    ($parent:ident) => {
        child_model! {
            String, NaiveDateTime, "instances", NewInstance, "sever gen", $parent,
            Instance {
                account_id: String,
                db_url: String,
                url: String,
                business_name: String,
                short_name: String,
                address: String,
                city: String,
                zip_code: String,
                phone_number: String,
                rate_conf_email: String,
                instance_name: Option<String>,
                top_terms: Option<String>,
                bottom_terms: Option<Vec<String>>,
            }
        }
    };
}
