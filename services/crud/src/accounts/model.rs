use super::accounts;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{api_error::ApiError, db, ID_SIZE};

model! {
    String, NaiveDateTime, "accounts", NewAccount, ReqAccount,
    Account {
        address: String,
        email: String,
        business_name: String,
        short_name: String,
        city: String,
        zip_code: String,
        phone_number: String,
    }
}

use self::accounts::dsl::*;
impl Account {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;
        let result = accounts.load::<Self>(&conn)?;

        Ok(result)
    }

    pub fn find_by_id(target: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let result = accounts.filter(id.eq(target)).get_result::<Self>(&conn)?;

        Ok(result)
    }

    pub fn insert(new: NewAccount) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let with_id = NewAccount {
            id: nanoid!(ID_SIZE),
            ..new
        };
        let result = diesel::insert_into(accounts)
            .values(&with_id)
            .get_result::<Self>(&conn)?;

        Ok(result)
    }

    pub fn update(target: String, new_vals: NewAccount) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let result = diesel::update(accounts)
            .filter(id.eq(target))
            .set(new_vals)
            .get_result(&conn)?;

        Ok(result)
    }

    pub fn delete(target: String) -> Result<usize, ApiError> {
        let conn = db::connection()?;
        let result = diesel::delete(accounts.filter(id.eq(target))).execute(&conn)?;

        Ok(result)
    }
}
