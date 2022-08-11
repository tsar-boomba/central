macro_rules! instance_models {
    ($parent:ident) => {
        child_model! {
            String, NaiveDateTime, "instances", NewInstance, UpdateInstance, "sever gen", $parent,
            Instance {
                account_id: String,
                #[validate(length(min = 1))]
                business_name: String,
                #[validate(length(min = 1))]
                short_name: String,
                #[validate(length(min = 1))]
                address: String,
                #[validate(length(min = 1))]
                city: String,
                #[validate(regex = "crate::ZIP_RE")]
                zip_code: String,
                #[validate(custom = "crate::validate_state")]
                state: String,
                #[validate(regex = "crate::PHONE_RE")]
                phone_number: String,
                #[validate(regex = "crate::EMAIL_RE")]
                rate_conf_email: String,
                #[validate(length(min = 1))]
                name: String,
                status: InstanceStatus,
                #[serde(skip)]
                key: Option<String>,
                #[serde(skip)]
                env_id: Option<String>,
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
            state -> Text,
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
    use crate::types::InstanceStatus;
    #[cfg(feature = "diesel")]
    use crate::Account;
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    instance_models!(Account);
}
