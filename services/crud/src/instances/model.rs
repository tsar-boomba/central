use diesel::prelude::*;
use models::{Instance, Model, NewInstance, UpdateInstance};
use models::instances::dsl::*;

use crate::{api_error::ApiError, db, ID_SIZE};

impl Model<String, NewInstance, UpdateInstance, ApiError> for Instance {
    fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;
        let result = instances.load::<Self>(&conn)?;

        Ok(result)
    }

    fn find_by_id(target: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let result = instances.filter(id.eq(target)).get_result::<Self>(&conn)?;

        Ok(result)
    }

    fn insert(new: NewInstance) -> Result<Self, ApiError> {
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

    fn update(target: String, new_vals: UpdateInstance) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let result = diesel::update(instances)
            .filter(id.eq(target))
            .set(new_vals)
            .get_result(&conn)?;

        Ok(result)
    }

    fn delete(target: String) -> Result<usize, ApiError> {
        let conn = db::connection()?;
        let result = diesel::delete(instances.filter(id.eq(target))).execute(&conn)?;

        Ok(result)
    }
}
