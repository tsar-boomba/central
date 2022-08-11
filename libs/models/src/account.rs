macro_rules! account_models {
    () => {
        model! {
            String, NaiveDateTime, "accounts", NewAccount, UpdateAccount, "server gen",
            Account {
                #[validate(length(min = 1))]
                address: String,
                #[validate(regex = "crate::EMAIL_RE")]
                email: String,
                #[validate(length(min = 1))]
                business_name: String,
                #[validate(length(min = 1))]
                short_name: String,
                #[validate(length(min = 1))]
                city: String,
                #[validate(regex = "crate::ZIP_RE")]
                zip_code: String,
                #[validate(regex = "crate::PHONE_RE")]
                phone_number: String,
                stripe_id: Option<String>,
                sub_id: Option<String>,
                #[validate(custom = "crate::validate_state")]
                state: String,
            }
        }
    };
}

#[cfg(feature = "diesel")]
pub mod schema {
    use diesel::table;

    table! {
        accounts {
            id -> Text,
            created_at -> Timestamp,
            updated_at -> Timestamp,
            address -> Text,
            email -> Text,
            business_name -> Text,
            short_name -> Text,
            city -> Text,
            zip_code -> Text,
            phone_number -> Text,
            stripe_id -> Nullable<Text>,
            sub_id -> Nullable<Text>,
            state -> Text,
        }
    }
}

pub mod model {
    #[cfg(feature = "diesel")]
    use super::schema::accounts;
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    account_models!();
}
