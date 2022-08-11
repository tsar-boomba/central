use actix_web::{delete, get, post, put, web, HttpResponse};
use auth::{belongs_to_account, ReqUser};
use bcrypt::hash;
use diesel::prelude::*;
use models::types::Role;
use models::{Account, Model, NewUser, UpdateUser, User, Validate};

use crate::update_usage;
use crate::{api_error::ApiError, db, json::DeleteBody};

#[get("/users")]
async fn find_all(req_user: Option<ReqUser>) -> Result<HttpResponse, ApiError> {
    let users = web::block(User::find_all).await??;

    let users = if let Some(req_user) = req_user {
        // if filter func returns true item will be allowed into the iterator
        // so if the account ids match or if the user is admin of site they will see the user
        users
            .into_iter()
            .filter(|x| x.account_id == req_user.account_id || req_user.account_id == "admin")
            .collect()
    } else {
        users
    };

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn find(id: web::Path<i32>, req_user: Option<ReqUser>) -> Result<HttpResponse, ApiError> {
    let user = web::block(move || User::find_by_id(id.into_inner())).await??;

    if !belongs_to_account(&req_user, &user.account_id) {
        return Err(ApiError::forbidden());
    }

    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(
    new_user: web::Json<NewUser>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    if !belongs_to_account(&req_user, &new_user.account_id) {
        return Err(ApiError::forbidden());
    }
    use models::users::dsl::*;

    let new_user = new_user.into_inner();
    new_user.validate()?;
    let hashed_pass =
        web::block(move || hash(new_user.password.clone(), bcrypt::DEFAULT_COST)).await??;
    // If req came from user, use their account id instead of whatever they set
    let with_hash = if let Some(req_account_id) = req_user.map(|req_user| req_user.account_id) {
        NewUser {
            password: hashed_pass,
            role: Role::User,
            account_id: req_account_id,
            ..new_user
        }
    } else {
        NewUser {
            password: hashed_pass,
            role: Role::User,
            ..new_user
        }
    };

    let owner_id = with_hash.account_id.clone();
    // must get it manually so tests can pass when we have only 1 connection
    let owner = web::block::<_, Result<Account, ApiError>>(move || {
        use models::accounts::dsl::*;
        let conn = db::connection()?;

        let result = accounts.filter(id.eq(owner_id)).first::<Account>(&conn)?;
        drop(conn);

        Ok(result)
    })
    .await?;

    let conn = db::connection()?;

    let result: Result<User, ApiError> = web::block(move || {
        conn.transaction(|| {
            let user: Result<User, ApiError> = diesel::insert_into(users)
                .values(&with_hash)
                .get_result::<User>(&conn)
                .map_err(|e| ApiError::from(e));

            match user {
                Ok(user) => match owner {
                    Ok(owner) => {
                        if None == owner.sub_id {
                            // err variant causes rollback
                            return Err(ApiError::not_subbed());
                        }

                        let num_user = users
                            .count()
                            .filter(account_id.eq(owner.id.clone()))
                            .get_result::<i64>(&conn)?;

                        let res = update_usage(&owner, "users".into(), num_user)?;

                        println!("{:?}", res);

                        match res.error_for_status() {
                            Ok(_) => Ok(user),
                            Err(_) => Err(ApiError::new(
                                500,
                                "Failed to update user subscription with Stripe.".into(),
                            )),
                        }
                    }
                    Err(_) => Err(ApiError::server_err()),
                },
                Err(err) => {
                    if err.status_code == 409 {
                        // there was a conflict only possibility is username
                        Err(ApiError::new(409, "Username must be unique.".into()))
                    } else {
                        Err(err)
                    }
                }
            }
        })
    })
    .await?;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => Err(err),
    }
}

#[put("/users/{id}")]
async fn update(
    id: web::Path<i32>,
    user: web::Json<UpdateUser>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();
    let to_be_updated = web::block(move || User::find_by_id(id)).await??;
    if !belongs_to_account(&req_user, &to_be_updated.account_id) {
        return Err(ApiError::forbidden());
    }

    let update_set: UpdateUser = UpdateUser {
        account_id: None,
        ..user.into_inner()
    };
    update_set.validate()?;

    let user = web::block(move || User::update(id, update_set)).await??;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(
    target: web::Path<i32>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let target = target.into_inner();

    let user = web::block(move || User::find_by_id(target)).await??;
    if !belongs_to_account(&req_user, &user.account_id) {
        return Err(ApiError::forbidden());
    }
    use models::users::dsl::*;

    let owner_id = user.account_id.clone();
    let owner = web::block::<_, Result<Account, ApiError>>(move || {
        use models::accounts::dsl::*;
        let conn = db::connection()?;

        let result = accounts.filter(id.eq(owner_id)).first::<Account>(&conn)?;
        drop(conn);

        Ok(result)
    })
    .await?;

    let conn = db::connection()?;

    let result: Result<usize, ApiError> = web::block(move || {
        conn.transaction(|| {
            let affected = diesel::delete(users.filter(id.eq(target))).execute(&conn)?;

            match owner {
                Ok(owner) => {
                    if None == owner.sub_id {
                        // err variant causes rollback
                        return Err(ApiError::not_subbed());
                    }

                    let num_user = users
                        .count()
                        .filter(account_id.eq(owner.id.clone()))
                        .get_result::<i64>(&conn)?;

                    let res = update_usage(&owner, "users".into(), num_user)?;

                    match res.error_for_status() {
                        Ok(_) => Ok(affected),
                        Err(_) => Err(ApiError::new(
                            500,
                            "Failed to update user subscription with Stripe.".into(),
                        )),
                    }
                }
                Err(_) => Err(ApiError::server_err()),
            }
        })
    })
    .await?;

    match result {
        Ok(affected) => Ok(HttpResponse::Ok().json(DeleteBody::new(affected.try_into().unwrap()))),
        Err(err) => Err(err),
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
