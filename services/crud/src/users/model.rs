use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use models::{types::Role, Model, NewUser};
use models::{User, UpdateUser};
use models::users::dsl::*;

use crate::{api_error::ApiError, db};

impl Model<String, NewUser, UpdateUser, ApiError> for User {
    fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;
        let result = users.load::<Self>(&conn)?;

        Ok(result)
    }

    fn find_by_id(target: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let result = users.filter(id.eq(target)).get_result::<Self>(&conn)?;

        Ok(result)
    }

    fn insert(new: NewUser) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let with_hash = NewUser {
            id: nanoid!(10),
            password: hash(new.password, DEFAULT_COST)?,
            role: Role::User,
            ..new
        };
        let result = diesel::insert_into(users)
            .values(&with_hash)
            .get_result::<Self>(&conn)?;

        Ok(result)
    }

    fn update(target: String, new_vals: UpdateUser) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let result = diesel::update(users.filter(id.eq(target)))
            .set(new_vals)
            .get_result(&conn)?;

        Ok(result)
    }

    fn delete(target: String) -> Result<usize, ApiError> {
        let conn = db::connection()?;
        let result = diesel::delete(users.filter(id.eq(target))).execute(&conn)?;

        Ok(result)
    }
}
