use models::{NewInstance, Instance, instances::dsl::*};
use crate::{db, json::DeleteBody, tests, ID_SIZE};
use actix_web::test;
use diesel::prelude::*;

fn compare(got: &Instance, exp: &NewInstance) {
    assert_eq!(got.account_id, exp.account_id);
    assert_eq!(got.db_url, exp.db_url);
    assert_eq!(got.url, exp.url);
    assert_eq!(got.business_name, exp.business_name);
    assert_eq!(got.short_name, exp.short_name);
    assert_eq!(got.address, exp.address);
    assert_eq!(got.city, exp.city);
    assert_eq!(got.zip_code, exp.zip_code);
    assert_eq!(got.phone_number, exp.phone_number);
    assert_eq!(got.rate_conf_email, exp.rate_conf_email);
    assert_eq!(got.instance_name, exp.instance_name);
}

fn defaults(/*fk: String*/) -> (NewInstance, NewInstance) {
    (
        NewInstance {
            id: nanoid!(ID_SIZE),
            account_id: "test".into(),
            db_url: "postgres".into(),
            url: "https://instance.loadmngr.com".into(),
            business_name: "hatfield llc".into(),
            short_name: "hatfield".into(),
            address: "123 alphabet street".into(),
            city: "charlotte".into(),
            zip_code: "28254".into(),
            phone_number: "704-805-1261".into(),
            rate_conf_email: "igamble@gmail.com".into(),
            instance_name: Some("our deployment".into()),
            top_terms: Some("you must not be sus!".into()),
            bottom_terms: None,
        },
        NewInstance {
            id: nanoid!(ID_SIZE),
            account_id: "test".into(),
            db_url: "postgres".into(),
            url: "https://instance.loadmngr.com".into(),
            business_name: "logh llc".into(),
            short_name: "hatfield".into(),
            address: "456 beta lane".into(),
            city: "charlotte".into(),
            zip_code: "28254".into(),
            phone_number: "980-335-6090".into(),
            rate_conf_email: "ugamble@gmail.com".into(),
            instance_name: None,
            top_terms: None,
            bottom_terms: Some(vec!["You will pay us $1000".into()]),
        },
    )
}

fn remove(target: String, conn: &db::PoolConn) {
    diesel::delete(instances.filter(id.eq(target)))
        .execute(conn)
        .expect("couldn't delete test instance from table");
}

#[actix_web::test]
async fn get() {
    let (default1, default2) = defaults();

    let app = tests::init(super::routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: Instance = diesel::insert_into(instances)
        .values(&default1)
        .get_result::<Instance>(&conn)
        .expect("couldn't insert");

    let result2: Instance = diesel::insert_into(instances)
        .values(&default2)
        .get_result::<Instance>(&conn)
        .expect("couldn't insert");

    // get back instance 1, expect to be same
    drop(conn);
    let req = test::TestRequest::get()
        .uri(&format!("/instances/{}", result1.id))
        .to_request();

    let resp: Instance = test::call_and_read_body_json(&app, req).await;

    compare(&resp, &default1);

    // Check for both inserted records with the get all route
    let req = test::TestRequest::get().uri("/instances").to_request();

    let resp: Vec<Instance> = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    assert!(resp.len() >= 2);

    remove(result1.id, &conn);
    remove(result2.id, &conn);
}

#[actix_web::test]
async fn post() {
    let (default1, _default2) = defaults();

    let app = tests::init(super::routes::init_routes).await;

    // Insert a instance
    let req = test::TestRequest::post()
        .uri("/instances")
        .set_json(&default1)
        .to_request();

    let resp: Instance = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    // Check in db for inserted record
    let got = instances
        .filter(id.eq(&resp.id))
        .get_result::<Instance>(&conn)
        .expect("Could not find record.");

    // make sure inserted is same as what we gave the route
    compare(&got, &default1);

    // reset db by removing test record
    remove(resp.id, &conn);
}

#[actix_web::test]
async fn put() {
    let (default1, default2) = defaults();

    let app = tests::init(super::routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: Instance = diesel::insert_into(instances)
        .values(&default1)
        .get_result::<Instance>(&conn)
        .expect("couldn't insert");

    // update record 1, to be record 2's values
    let req = test::TestRequest::put()
        .uri(&format!("/instances/{}", result1.id))
        .set_json(&default2)
        .to_request();

    // call route
    drop(conn);
    let _res: Instance = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    let result1: Instance = instances
        .filter(id.eq(result1.id))
        .get_result(&conn)
        .expect("Failed to get instance");

    // make sure record 1 got updated
    compare(&result1, &default2);

    remove(result1.id, &conn);
}

#[actix_web::test]
async fn delete() {
    let (default1, default2) = defaults();

    let app = tests::init(super::routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: Instance = diesel::insert_into(instances)
        .values(&default1)
        .get_result::<Instance>(&conn)
        .expect("couldn't insert");

    // update record 1, to be record 2's values
    let req = test::TestRequest::delete()
        .uri(&format!("/instances/{}", result1.id))
        .set_json(&default2)
        .to_request();

    // call route
    drop(conn);
    let resp: DeleteBody = test::call_and_read_body_json(&app, req).await;
    let conn = db::connection().unwrap();

    assert_eq!(resp.affected, 1);

    let result1: Option<Instance> = instances
        .filter(id.eq(result1.id))
        .get_result(&conn)
        .optional()
        .expect("failed to convert to to option");

    assert!(result1 == None);
}
