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

#[cfg(feature = "diesel")]
pub mod schema {
    use diesel::table;

    table! {
        instances {
            id -> Text,
            created_at -> Timestamp,
            updated_at -> Timestamp,
            account_id -> Text,
            db_url -> Text,
            url -> Text,
            business_name -> Text,
            short_name -> Text,
            address -> Text,
            city -> Text,
            zip_code -> Text,
            phone_number -> Text,
            rate_conf_email -> Text,
            instance_name -> Nullable<Text>,
            top_terms -> Nullable<Text>,
            bottom_terms -> Nullable<Array<Text>>,
        }
    }
}

pub mod model {
    #[cfg(feature = "diesel")]
    use super::schema::instances;
    #[cfg(feature = "diesel")]
    use crate::Account;
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Serialize};

    instance_models!(Account);
}
