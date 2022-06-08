use super::users;
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    accounts::model::Account, api_error::ApiError, db, sql_types::Resource, sql_types::Role,
};

fn skip_serialize_pass(_: &String) -> bool {
    !cfg!(test)
}

child_model! {
    i32, NaiveDateTime, "users", NewUser, Account,
    User {
        account_id: String,
        username: String,
        first_name: String,
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

use self::users::dsl::*;
impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;
        let result = users.load::<Self>(&conn)?;

        Ok(result)
    }

    pub fn find_by_id(target: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let result = users.filter(id.eq(target)).get_result::<Self>(&conn)?;

        Ok(result)
    }

    pub fn insert(new: NewUser) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let with_hash = NewUser {
            password: hash(new.password, DEFAULT_COST)?,
            role: Role::User,
            ..new
        };
        let result = diesel::insert_into(users)
            .values(&with_hash)
            .get_result::<Self>(&conn)?;

        Ok(result)
    }

    pub fn update(target: i32, new_vals: NewUser) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let result = diesel::update(users.filter(id.eq(target)))
            .set(new_vals)
            .get_result(&conn)?;

        Ok(result)
    }

    pub fn delete(target: i32) -> Result<usize, ApiError> {
        let conn = db::connection()?;
        let result = diesel::delete(users.filter(id.eq(target))).execute(&conn)?;

        Ok(result)
    }
}
