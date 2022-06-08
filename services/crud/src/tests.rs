use diesel::prelude::*;
use std::sync::{Arc, Mutex};

use actix_web::{
    dev::{Service, ServiceResponse},
    test, web, App,
};

use crate::{
    accounts::model::{Account, NewAccount},
    auth, db,
};

lazy_static! {
    static ref INITIATED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

pub async fn init(
    init_routes: impl FnOnce(&mut web::ServiceConfig),
) -> impl Service<actix_http::Request, Response = ServiceResponse, Error = actix_web::Error> {
    let mut initiated = INITIATED.lock().unwrap();
    if *initiated == false {
        dotenv::dotenv().ok();
        env_logger::init();
        db::init();
        test_account();
        *initiated = true;
    }

    test::init_service(
        App::new()
            .wrap(auth::middleware::Authorize)
            .configure(init_routes),
    )
    .await
}

/// Creates an account to ensure foreign keys are satisfied during testing
pub fn test_account() {
    use crate::accounts::accounts::dsl::*;
    diesel::insert_into(accounts)
        .values(NewAccount {
            id: "test".into(),
            address: "testys".into(),
            business_name: "tests".into(),
            city: "testcity".into(),
            email: "test@testys.test".into(),
            phone_number: "999-999-0000".into(),
            short_name: "test".into(),
            zip_code: "28282".into(),
        })
        .on_conflict_do_nothing()
        .get_result::<Account>(&db::connection().unwrap())
        .unwrap();
}

// pub fn remove_test_account() {
//     use crate::accounts::accounts::dsl::*;
//     diesel::delete(accounts.filter(id.eq("test")))
//         .execute(&db::connection().unwrap())
//         .unwrap();
// }
