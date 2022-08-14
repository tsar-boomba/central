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
                address1: String,
                #[validate(length(min = 1))]
                address2: Option<String>,
                #[validate(length(min = 1))]
                city: String,
                #[validate(regex = "crate::ZIP_RE")]
                zip_code: String,
                #[validate(custom = "crate::validate_state")]
                state: String,
                #[validate(regex = "crate::PHONE_RE")]
                phone_number: String,
                #[validate(regex = "crate::EMAIL_RE")]
                email: String,
                #[validate(regex = "crate::INSTANCE_NAME_RE")]
                name: String,
                status: InstanceStatus,
                #[serde(skip)]
                key: Option<String>,
                #[serde(skip)]
                env_id: Option<String>,
                url: Option<String>,
                #[validate(length(min = 1))]
                top_text: Option<String>,
                #[validate(length(min = 1))]
                bottom_text: Option<String>,
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
            address1 -> Text,
            address2 -> Nullable<Text>,
            city -> Text,
            zip_code -> Text,
            state -> Text,
            phone_number -> Text,
            email -> Text,
            name -> Text,
            status -> InstanceStatus,
            key -> Nullable<Text>,
            env_id -> Nullable<Text>,
            url -> Nullable<Text>,
            top_text -> Nullable<Text>,
            bottom_text -> Nullable<Text>,
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
