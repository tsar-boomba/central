#[macro_use]
extern crate models;

pub mod routes;

pub mod crud_models {
    use chrono::NaiveDateTime;
    use models::account_models;
    use serde::{Deserialize, Serialize};
    account_models!();
}
