macro_rules! instance_models {
    ($parent:ident) => {
        child_model! {
            String, NaiveDateTime, "instances", NewInstance, UpdateInstance, "sever gen", $parent,
            Instance {
                account_id: String,
                business_name: String,
                short_name: String,
                address: String,
                city: String,
                zip_code: String,
                phone_number: String,
                rate_conf_email: String,
                name: String,
                status: InstanceStatus,
                #[serde(skip)]
                key: Option<String>,
                #[serde(skip)]
                env_id: Option<String>,
                #[serde(skip)]
                url: Option<String>,
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
        use diesel::sql_types::*;
        use crate::types::instance_status_sql::InstanceStatus;

        instances {
            id -> Text,
            created_at -> Timestamp,
            updated_at -> Timestamp,
            account_id -> Text,
            business_name -> Text,
            short_name -> Text,
            address -> Text,
            city -> Text,
            zip_code -> Text,
            phone_number -> Text,
            rate_conf_email -> Text,
            name -> Text,
            status -> InstanceStatus,
            key -> Nullable<Text>,
            env_id -> Nullable<Text>,
            url -> Nullable<Text>,
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
    use crate::types::InstanceStatus;
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Serialize};

    instance_models!(Account);
}
