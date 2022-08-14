use models::{NewInstance, Instance, instances::dsl::*, UpdateInstance, types::InstanceStatus};
use crate::{db, json::DeleteBody, tests::{self, mock_instance_deploy}, ID_SIZE};
use actix_web::test;
use diesel::prelude::*;

fn compare(got: &Instance, exp: &NewInstance) {
    assert_eq!(got.account_id, exp.account_id);
    assert_eq!(got.business_name, exp.business_name);
    assert_eq!(got.short_name, exp.short_name);
    assert_eq!(got.address1, exp.address1);
    assert_eq!(got.address2, exp.address2);
    assert_eq!(got.city, exp.city);
    assert_eq!(got.zip_code, exp.zip_code);
    assert_eq!(got.phone_number, exp.phone_number);
    assert_eq!(got.email, exp.email);
    assert_eq!(got.name, exp.name);
    assert_eq!(got.state, exp.state);
}

fn defaults(test_name: String) -> (NewInstance, NewInstance) {
    (
        NewInstance {
            id: nanoid!(ID_SIZE),
            account_id: "test".into(),
            url: Some("yumyum.milkyweb.app".into()),
            business_name: "hatfield llc".into(),
            short_name: "hatfield".into(),
            address1: "123 alphabet street".into(),
            address2: None,
            city: "charlotte".into(),
            status: InstanceStatus::Ok,
            zip_code: "28254".into(),
            state: "NC".into(),
            phone_number: "704-805-1261".into(),
            email: "igamble@gmail.com".into(),
            name: test_name.clone() + "our-deployment".into(),
            top_text: Some("you must not be sus!".into()),
            env_id: Some("augagjaijg".into()),
            key: Some("agouajpigjaipsjg".into()),
            bottom_text: None,
        },
        NewInstance {
            id: nanoid!(ID_SIZE),
            account_id: "test".into(),
            url: None,
            business_name: "logh llc".into(),
            short_name: "hatfield".into(),
            address1: "456 beta lane".into(),
            address2: Some("Suite 512".into()),
            city: "charlotte".into(),
            status: InstanceStatus::Deploying,
            zip_code: "28254".into(),
            state: "NC".into(),
            phone_number: "980-335-6090".into(),
            email: "ugamble@gmail.com".into(),
            name: test_name + "load-mgner".into(),
            env_id: None,
            key: None,
            top_text: None,
            bottom_text: Some("You will pay us $1000".into()),
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
    let (default1, default2) = defaults("get".into());

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
    actix_web::rt::spawn(mock_instance_deploy());
    let (default1, _default2) = defaults("post".into());

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
    let (default1, _default2) = defaults("put".into());

    let app = tests::init(super::routes::init_routes).await;
    let conn = db::connection().unwrap();

    let result1: Instance = diesel::insert_into(instances)
        .values(&default1)
        .get_result::<Instance>(&conn)
        .expect("couldn't insert");

    // update record 1, to be record 2's values
    let req = test::TestRequest::put()
        .uri(&format!("/instances/{}", result1.id))
        .set_json(&UpdateInstance { city: Some("austin".into()), ..Default::default() })
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
    assert_eq!(result1.city, "austin");

    remove(result1.id, &conn);
}

#[actix_web::test]
async fn delete() {
    let (default1, default2) = defaults("delete".into());

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
