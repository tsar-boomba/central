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
                #[serde(skip)]
                stripe_id: Option<String>,
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
            state -> Text,
        }
    }
}


pub mod model {
    #[cfg(feature = "diesel")]
    use super::schema::accounts;
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Serialize};

    account_models!();
}
