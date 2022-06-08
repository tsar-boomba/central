use super::{
    model::{Account, NewAccount},
    schema::accounts::dsl::*,
};
use crate::{db, json::DeleteBody, tests, ID_SIZE};
use actix_web::test;
use diesel::prelude::*;

fn compare(got: &Account, exp: &NewAccount) {
    assert_eq!(got.address, exp.address);
    assert_eq!(got.email, exp.email);
    assert_eq!(got.business_name, exp.business_name);
    assert_eq!(got.short_name, exp.short_name);
    assert_eq!(got.city, exp.city);
    assert_eq!(got.zip_code, exp.zip_code);
    assert_eq!(got.phone_number, exp.phone_number);
}

fn defaults(test_name: String /*fk: String*/) -> (NewAccount, NewAccount) {
    (
        NewAccount {
            id: nanoid!(ID_SIZE),
            address: "123 alphabet street".into(),
            email: format!("{}@mail.com", test_name.clone()).into(),
            business_name: "hatfield llc".into(),
            short_name: "hatfield".into(),
            city: "charlotte".into(),
            zip_code: "28254".into(),
            phone_number: "704-805-1261".into(),
        },
        NewAccount {
            id: nanoid!(ID_SIZE),
            address: "456 beta lane".into(),
            email: format!("{}2@mail.com", test_name).into(),
            business_name: "logh llc".into(),
            short_name: "lggh".into(),
            city: "charlotte".into(),
            zip_code: "28254".into(),
            phone_number: "980-335-6090".into(),
        },
    )
}

fn remove(target: String, conn: &db::PoolConn) {
    diesel::delete(accounts.filter(id.eq(target)))
        .execute(conn)
        .expect("couldn't delete test account from table");
}

#[actix_web::test]
async fn get() {
    let (default1, default2) = defaults("accounts-get".into());

    let app = tests::init(super::routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: Account = diesel::insert_into(accounts)
        .values(&default1)
        .get_result::<Account>(&conn)
        .expect("couldn't insert");

    let result2: Account = diesel::insert_into(accounts)
        .values(&default2)
        .get_result::<Account>(&conn)
        .expect("couldn't insert");

    // get back instance 1, expect to be same
    drop(conn);
    let req = test::TestRequest::get()
        .uri(&format!("/accounts/{}", result1.id))
        .to_request();

    let resp: Account = test::call_and_read_body_json(&app, req).await;

    compare(&resp, &default1);

    // Check for both inserted records with the get all route
    let req = test::TestRequest::get().uri("/accounts").to_request();

    let resp: Vec<Account> = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    assert!(resp.len() >= 2);

    remove(result1.id, &conn);
    remove(result2.id, &conn);
}

#[actix_web::test]
async fn post() {
    let (default1, _default2) = defaults("accounts-post".into());

    let app = tests::init(super::routes::init_routes).await;

    // Insert a instance
    let req = test::TestRequest::post()
        .uri("/accounts")
        .set_json(&default1)
        .to_request();

    let resp: Account = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    // Check in db for inserted record
    let got = accounts
        .filter(id.eq(&resp.id))
        .get_result::<Account>(&conn)
        .expect("Could not find record.");

    // make sure inserted is same as what we gave the route
    compare(&got, &default1);

    // reset db by removing test record
    remove(resp.id, &conn);
}

#[actix_web::test]
async fn put() {
    let (default1, default2) = defaults("accounts-put".into());

    let app = tests::init(super::routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: Account = diesel::insert_into(accounts)
        .values(&default1)
        .get_result::<Account>(&conn)
        .expect("couldn't insert");

    // update record 1, to be record 2's values
    let req = test::TestRequest::put()
        .uri(&format!("/accounts/{}", result1.id))
        .set_json(&default2)
        .to_request();

    // call route
    drop(conn);
    let _res: Account = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    let result1: Account = accounts
        .filter(id.eq(result1.id))
        .get_result(&conn)
        .expect("Failed to get account");

    // make sure record 1 got updated
    compare(&result1, &default2);

    remove(result1.id, &conn);
}

#[actix_web::test]
async fn delete() {
    let (default1, default2) = defaults("accounts-delete".into());

    let app = tests::init(super::routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: Account = diesel::insert_into(accounts)
        .values(&default1)
        .get_result::<Account>(&conn)
        .expect("couldn't insert");

    // update record 1, to be record 2's values
    let req = test::TestRequest::delete()
        .uri(&format!("/accounts/{}", result1.id))
        .set_json(&default2)
        .to_request();

    // call route
    drop(conn);
    let resp: DeleteBody = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    assert_eq!(resp.affected, 1);

    let result1: Option<Account> = accounts
        .filter(id.eq(result1.id))
        .get_result(&conn)
        .optional()
        .expect("failed to convert to to option");

    assert!(result1 == None);
}
