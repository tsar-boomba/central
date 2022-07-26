use super::{
    model::{NewUser, User},
    schema::users::dsl::*,
    *,
};
use crate::{db, json::DeleteBody, tests};
use actix_web::test;
use diesel::prelude::*;
use models::types::Role;

pub fn compare(got: &User, exp: &NewUser) {
    assert_eq!(got.username, exp.username);
    assert_eq!(got.first_name, exp.first_name);
    assert_eq!(got.last_name, exp.last_name);
    assert_eq!(got.active, exp.active);
}

pub fn defaults(test_name: &str) -> (NewUser, NewUser) {
    (
        model::NewUser {
            account_id: "test".into(),
            username: test_name.into(),
            first_name: "Test".into(),
            last_name: "User".into(),
            password: "pretend this is hashed".into(),
            active: true,
            instances: vec!["hatfield".into()],
            create_perms: vec!["load".into()],
            update_perms: vec!["load".into()],
            delete_perms: vec!["load".into()],
            role: Role::Admin,
            notes: None,
        },
        model::NewUser {
            account_id: "test".into(),
            username: format!("{}2", test_name),
            first_name: "Test2".into(),
            last_name: "User2".into(),
            password: "pretend this is hashed2".into(),
            active: false,
            instances: vec!["log gh".into()],
            create_perms: vec!["carrier".into()],
            update_perms: vec!["carrier".into()],
            delete_perms: vec!["carrier".into()],
            role: Role::User,
            notes: Some("good employee".into()),
        },
    )
}

pub fn remove(target: i32, conn: &db::PoolConn) {
    diesel::delete(users.filter(id.eq(target)))
        .execute(conn)
        .expect("couldn't delete test user from table");
}

#[actix_web::test]
async fn get() {
    let (default1, default2) = defaults("users get");

    let app = tests::init(super::routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: User = diesel::insert_into(users)
        .values(&default1)
        .get_result::<User>(&conn)
        .expect("couldn't insert");

    let result2: User = diesel::insert_into(users)
        .values(&default2)
        .get_result::<User>(&conn)
        .expect("couldn't insert");

    // get back user 1, expect to be same
    let req = test::TestRequest::get()
        .uri(&format!("/users/{}", result1.id))
        .to_request();

    drop(conn);
    let resp: model::User = test::call_and_read_body_json(&app, req).await;

    compare(&resp, &default1);

    // Check for both inserted records with the get all route
    let req = test::TestRequest::get().uri("/users").to_request();

    let resp: Vec<model::User> = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    assert!(resp.len() >= 2);

    remove(result1.id, &conn);
    remove(result2.id, &conn);
}

#[actix_web::test]
async fn post() {
    let (default1, _default2) = defaults("users post");

    let app = tests::init(super::routes::init_routes).await;

    // Insert a user
    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&default1)
        .to_request();

    let resp: model::User = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    // Check in db for inserted record
    let got = users
        .filter(id.eq(resp.id))
        .get_result::<User>(&conn)
        .expect("Could not find record.");

    // make sure inserted is same as what we gave the route
    compare(&got, &default1);

    // reset db by removing test record
    remove(resp.id, &conn);
}

#[actix_web::test]
async fn put() {
    let (default1, default2) = defaults("users put");

    let app = tests::init(super::routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: User = diesel::insert_into(users)
        .values(&default1)
        .get_result::<User>(&conn)
        .expect("couldn't insert");

    println!("{:?}", result1);

    // update record 1, to be record 2's values
    let req = test::TestRequest::put()
        .uri(&format!("/users/{}", result1.id))
        .set_json(&default2)
        .to_request();

    // call route
    drop(conn);
    let _res: User = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    let result1: User = users
        .filter(id.eq(result1.id))
        .get_result(&conn)
        .expect("Failed to get user");

    // make sure record 1 got updated
    compare(&result1, &default2);

    remove(result1.id, &conn);
}

#[actix_web::test]
async fn delete() {
    let (default1, default2) = defaults("users delete");

    let app = tests::init(super::routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: User = diesel::insert_into(users)
        .values(&default1)
        .get_result::<User>(&conn)
        .expect("couldn't insert");

    println!("{:?}", result1);

    // update record 1, to be record 2's values
    let req = test::TestRequest::delete()
        .uri(&format!("/users/{}", result1.id))
        .set_json(&default2)
        .to_request();

    // call route
    drop(conn);
    let resp: DeleteBody = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    assert_eq!(resp.affected, 1);

    let result1: Option<User> = users
        .filter(id.eq(result1.id))
        .get_result(&conn)
        .optional()
        .expect("failed to convert to to option");

    assert!(result1 == None);
}
