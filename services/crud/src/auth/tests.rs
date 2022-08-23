use crate::{
    auth::*,
    db, tests,
    users
};
use models::{User, NewUser};
use actix_http::StatusCode;
use actix_web::{cookie::Cookie, test};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;

#[actix_web::test]
async fn login() {
    let (default1, _default2) = users::tests::defaults("auth login");

    let app = tests::init(routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: User = diesel::insert_into(models::users::table)
        .values(NewUser {
            password: hash(default1.password.clone(), DEFAULT_COST)
                .expect("Failed to hash password."),
            ..default1.clone()
        })
        .get_result::<User>(&conn)
        .expect("couldn't insert");

    // try to login with creds
    drop(conn);
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(Login {
            account_id: result1.account_id,
            username: result1.username,
            password: default1.password,
        })
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let conn = db::connection().unwrap();

    users::tests::remove(result1.id, &conn);
}

#[actix_web::test]
async fn authenticate() {
    let (default1, _default2) = users::tests::defaults("auth authenticate");

    let app = tests::init(routes::init_routes).await;

    // send req with valid cookie expect success response
    let req = test::TestRequest::get()
        .uri("/authenticate")
        .cookie(
            Cookie::build(
                "at",
                sign("1".into(), default1.account_id.clone()).expect("Failed to sign jwt."),
            )
            .path("/")
            .finish(),
        )
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    // send req with invalid cookie expect unauthorized response
    let req = test::TestRequest::get()
        .uri("/authenticate")
        .cookie(Cookie::build("at", "not valid :P").path("/").finish())
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn verify() {
    let (default1, _default2) = users::tests::defaults("auth verify");

    let app = tests::init(routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: User = diesel::insert_into(models::users::table)
        .values(NewUser {
            password: hash(default1.password.clone(), DEFAULT_COST)
                .expect("Failed to hash password."),
            ..default1.clone()
        })
        .get_result::<User>(&conn)
        .expect("couldn't insert");

    // try to login with creds
    drop(conn);
    let req = test::TestRequest::get()
        .uri("/verify")
        .cookie(
            Cookie::build(
                "at",
                sign(result1.id.clone(), result1.account_id.clone()).expect("Failed to sign jwt."),
            )
            .path("/")
            .finish(),
        )
        .to_request();

    let resp: User = test::call_and_read_body_json(&app, req).await;

    users::tests::compare(&resp, &default1);

    let conn = db::connection().unwrap();

    users::tests::remove(result1.id, &conn);
}
