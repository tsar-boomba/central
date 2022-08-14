use actix_web::web;
use diesel::prelude::*;
use models::types::InstanceStatus;
use serde::{Deserialize, Serialize};

use crate::{api_error::ApiError, db};

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    pub instances: i64,
    pub users: i64,
}
pub async fn usage(target: String) -> Result<Usage, ApiError> {
	let users_acct_id = target.clone();
    let num_users = web::block::<_, Result<i64, ApiError>>(move || {
        use models::users::dsl::*;
        Ok(users
            .count()
            .filter(account_id.eq(users_acct_id))
            .get_result::<i64>(&db::connection()?)?)
    })
    .await??;

    let instances_acct_id = target.clone();
    let num_instances = web::block::<_, Result<i64, ApiError>>(move || {
        use models::instances::dsl::*;
        Ok(instances
            .count()
            .filter(account_id.eq(instances_acct_id))
            .filter(status.eq_any(vec![
                InstanceStatus::Ok,
                InstanceStatus::Unhealthy,
                InstanceStatus::Configured,
            ]))
            .get_result::<i64>(&db::connection()?)?)
    })
    .await??;

    Ok(Usage {
        instances: num_instances,
        users: num_users,
    })
}
