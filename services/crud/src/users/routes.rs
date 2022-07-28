use actix_web::{delete, get, post, put, web, HttpResponse};
use bcrypt::hash;
use diesel::prelude::*;
use models::types::Role;
use models::{Account, NewUser, User, Model};
use payments_lib::routes::create_usage_record;

use crate::{
    api_error::ApiError, auth::Claim, belongs_to_account, db, json::DeleteBody, PAYMENTS_URI,
};

#[get("/users")]
async fn find_all(jwt: Option<Claim>) -> Result<HttpResponse, ApiError> {
    let users = web::block(User::find_all).await??;

    let users = if let Some(jwt) = jwt {
        // if filter func returns true item will be allowed into the iterator
        // so if the account ids match or if the user is admin of site they will see the user
        users
            .into_iter()
            .filter(|x| x.account_id == jwt.account_id || jwt.account_id == "admin")
            .collect()
    } else {
        users
    };

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn find(id: web::Path<i32>, jwt: Option<Claim>) -> Result<HttpResponse, ApiError> {
    let user = web::block(move || User::find_by_id(id.into_inner())).await??;

    if !belongs_to_account(&jwt, &user.account_id) {
        return Err(ApiError::forbidden());
    }

    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(
    new_user: web::Json<NewUser>,
    jwt: Option<Claim>,
) -> Result<HttpResponse, ApiError> {
    if !belongs_to_account(&jwt, &new_user.account_id) {
        return Err(ApiError::forbidden());
    }
    use models::users::dsl::*;

    let new_user = new_user.into_inner();
    let hashed_pass =
        web::block(move || hash(new_user.password.clone(), bcrypt::DEFAULT_COST)).await??;
    let with_hash = NewUser {
        password: hashed_pass,
        role: Role::User,
        ..new_user
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
                        if None == owner.stripe_id {
                            // err variant causes rollback
                            return Err(ApiError::new(
                                400,
                                "Cannot create users while not subscribed.".into(),
                            ));
                        }

                        let num_user = users
                            .count()
                            .filter(account_id.eq(owner.id.clone()))
                            .get_result::<i64>(&conn)?;

                        let res = update_user_usage(&owner, num_user)?;

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
    user: web::Json<NewUser>,
    jwt: Option<Claim>,
) -> Result<HttpResponse, ApiError> {
    if !belongs_to_account(&jwt, &user.account_id) {
        return Err(ApiError::forbidden());
    }

    let user = web::block(move || User::update(id.into_inner(), user.into_inner())).await??;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(target: web::Path<i32>, jwt: Option<Claim>) -> Result<HttpResponse, ApiError> {
    let target = target.into_inner();

    let user = web::block(move || User::find_by_id(target)).await??;
    if !belongs_to_account(&jwt, &user.account_id) {
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
                    if None == owner.stripe_id {
                        // err variant causes rollback
                        return Err(ApiError::new(
                            400,
                            "Cannot delete users while not subscribed.".into(),
                        ));
                    }

                    let num_user = users
                        .count()
                        .filter(account_id.eq(owner.id.clone()))
                        .get_result::<i64>(&conn)?;

                    let res = update_user_usage(&owner, num_user)?;

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

fn update_user_usage(
    owner: &Account,
    new_value: i64,
) -> Result<reqwest::blocking::Response, ApiError> {
    let client = reqwest::blocking::Client::new();

    return Ok(client
        .post(PAYMENTS_URI.to_string() + create_usage_record::ROUTE)
        .json(&create_usage_record::CreateUsageRecordParams {
            stripe_id: owner.stripe_id.clone().unwrap(),
            number: new_value.try_into().unwrap(),
            resource: "users".into(),
        })
        .send()?);
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
