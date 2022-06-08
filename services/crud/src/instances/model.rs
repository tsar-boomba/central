use super::instances;
use crate::accounts::model::Account;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{api_error::ApiError, db, ID_SIZE};

child_model! {
    String, NaiveDateTime, "instances", NewInstance, "Server generated id", Account,
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

use self::instances::dsl::*;
impl Instance {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;
        let result = instances.load::<Self>(&conn)?;

        Ok(result)
    }

    pub fn find_by_id(target: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let result = instances.filter(id.eq(target)).get_result::<Self>(&conn)?;

        Ok(result)
    }

    pub fn insert(new: NewInstance) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let with_id = NewInstance {
            id: nanoid!(ID_SIZE),
            ..new
        };
        let result = diesel::insert_into(instances)
            .values(&with_id)
            .get_result::<Self>(&conn)?;

        Ok(result)
    }

    pub fn update(target: String, new_vals: NewInstance) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let result = diesel::update(instances)
            .filter(id.eq(target))
            .set(new_vals)
            .get_result(&conn)?;

        Ok(result)
    }

    pub fn delete(target: String) -> Result<usize, ApiError> {
        let conn = db::connection()?;
        let result = diesel::delete(instances.filter(id.eq(target))).execute(&conn)?;

        Ok(result)
    }
}
