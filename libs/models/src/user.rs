/// If using this macro include function in scope named "skip_serialize_pass" with this type signature (value: &String) -> bool.
/// Also, import Role and Resources enums from types
macro_rules! user_models {
    ($parent:ident) => {
        child_model! {
            String, NaiveDateTime, "users", NewUser, UpdateUser, "server gen", $parent,
            User {
                #[serde(default)]
                account_id: String,
                #[validate(length(min = 1))]
                username: String,
                #[validate(length(min = 1))]
                first_name: String,
                #[validate(length(min = 1))]
                last_name: String,
                #[serde(skip_serializing_if = "skip_serialize_pass")]
                password: String,
                active: bool,
                instances: Vec<String>,
                create_perms: Vec<Resource>,
                update_perms: Vec<Resource>,
                delete_perms: Vec<Resource>,
                role: Role,
                notes: Option<String>,
            }
        }
    };
}

#[cfg(feature = "diesel")]
pub mod schema {
    use diesel::table;

    table! {
        use diesel::sql_types::*;
        use crate::types::resource_sql::Resource;
        use crate::types::role_sql::Role;

        users {
            id -> Text,
            created_at -> Timestamp,
            updated_at -> Timestamp,
            account_id -> Text,
            username -> Text,
            first_name -> Text,
            last_name -> Text,
            password -> Text,
            active -> Bool,
            instances -> Array<Text>,
            create_perms -> Array<Resource>,
            update_perms -> Array<Resource>,
            delete_perms -> Array<Resource>,
            role -> Role,
            notes -> Nullable<Text>,
        }
    }
}

pub mod model {
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[cfg(feature = "diesel")]
    use super::schema::users;
    use crate::types::*;
    #[cfg(feature = "diesel")]
    use crate::Account;
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    static SKIP_SERIALIZE_PASS: AtomicUsize = AtomicUsize::new(1);

    pub fn dont_skip_pass() {
        SKIP_SERIALIZE_PASS.swap(0, Ordering::SeqCst);
    }

    pub fn skip_pass() {
        SKIP_SERIALIZE_PASS.swap(1, Ordering::SeqCst);
    }

    fn skip_serialize_pass<T>(_: T) -> bool {
        SKIP_SERIALIZE_PASS.load(Ordering::SeqCst) == 1
    }

    user_models!(Account);
}
