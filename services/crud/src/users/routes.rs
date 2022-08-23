use actix_web::{delete, get, post, put, web, HttpResponse};
use auth::{belongs_to_account, higher_role, require_role, ReqUser};
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
async fn find(id: web::Path<String>, req_user: Option<ReqUser>) -> Result<HttpResponse, ApiError> {
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
    if !belongs_to_account(&req_user, &new_user.account_id)
        || !require_role(&req_user, Role::Moderator)
    {
        return Err(ApiError::forbidden());
    }
    use models::users::dsl::*;

    let new_user = new_user.into_inner();

    // must have higher role than user you are trying to create
    if !higher_role(&req_user, new_user.role.clone()) {
        return Err(ApiError::forbidden());
    }

    new_user.validate()?;
    let hashed_pass = web::block(move || hash(new_user.password, bcrypt::DEFAULT_COST)).await??;
    // If req came from user, use their account id instead of whatever they set
    let with_hash = if let Some(req_account_id) = req_user.map(|req_user| req_user.account_id) {
        NewUser {
            id: nanoid!(10),
            password: hashed_pass,
            role: Role::User,
            account_id: req_account_id,
            ..new_user
        }
    } else {
        NewUser {
            id: nanoid!(10),
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
    id: web::Path<String>,
    user: web::Json<UpdateUser>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();
    let updated_id = id.clone();
    let to_be_updated = web::block(move || User::find_by_id(updated_id)).await??;
    if !belongs_to_account(&req_user, &to_be_updated.account_id)
        || !require_role(&req_user, Role::Moderator)
    {
        return Err(ApiError::forbidden());
    }

    let hashing_pass = user.password.clone();
    // if password is being updated, hash it
    let hashed_pass = web::block(move || {
        hashing_pass.map(|password| hash(password, bcrypt::DEFAULT_COST).unwrap())
    })
    .await?;
    // TODO log user out when updated
    let update_set: UpdateUser = UpdateUser {
        account_id: None,
        password: hashed_pass,
        ..user.into_inner()
    };

    // must have higher role than what you want to update to
    // except owner can change anyone, including themselves
    if let Some(new_role) = update_set.role.clone() {
        if !require_role(&req_user, Role::Owner) && !higher_role(&req_user, new_role.clone()) {
            return Err(ApiError::forbidden());
        }
        // user cannot change their own role
        if let Some(req_user) = req_user {
            if id == req_user.id {
                return Err(ApiError::forbidden());
            }
        }
    }

    update_set.validate()?;

    let user = web::block(move || User::update(id, update_set)).await??;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(
    target: web::Path<String>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let target = target.into_inner();

    let deleted_id = target.clone();
    let user = web::block(move || User::find_by_id(deleted_id)).await??;
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

#[put("/users/{id}/toggle-status")]
async fn toggle_status(
    id: web::Path<String>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();
    let updated_id = id.clone();
    let to_be_updated = web::block(move || User::find_by_id(updated_id)).await??;
    if !belongs_to_account(&req_user, &to_be_updated.account_id)
        || !higher_role(&req_user, to_be_updated.role)
    {
        return Err(ApiError::forbidden());
    }

    let update_set: UpdateUser = UpdateUser {
        active: Some(!to_be_updated.active),
        ..Default::default()
    };
    update_set.validate()?;

    let user = web::block(move || User::update(id, update_set)).await??;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}/transfer-owner")]
async fn transfer_owner(
    target: web::Path<String>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let target = target.into_inner();
    let updated_id = target.clone();
    let to_be_updated = web::block(move || User::find_by_id(updated_id)).await??;
    if !belongs_to_account(&req_user, &to_be_updated.account_id)
        || !require_role(&req_user, Role::Owner)
    {
        return Err(ApiError::forbidden());
    }

    web::block::<_, Result<(), ApiError>>(move || {
        use models::users::dsl::*;
        let conn = db::connection()?;
        // update both users in transaction
        conn.transaction::<(), ApiError, _>(|| {
            // TODO log both users out
            diesel::update(users.filter(id.eq(target)))
                .set(UpdateUser {
                    role: Some(Role::Owner),
                    ..Default::default()
                })
                .execute(&conn)?;

            if let Some(req_user) = req_user {
                diesel::update(users.filter(id.eq(req_user.id)))
                    .set(UpdateUser {
                        role: Some(Role::Admin),
                        ..Default::default()
                    })
                    .execute(&conn)?;
            }

            Ok(())
        })
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(toggle_status);
    config.service(transfer_owner);
}
