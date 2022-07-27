use diesel::prelude::*;
use payments_lib::routes::create_usage_record;
use std::sync::{Arc, Mutex};

use actix_web::{
    dev::{Service, ServiceResponse},
    test, web, App, HttpResponse, HttpServer,
};

use crate::{
    accounts::model::{Account, NewAccount},
    api_error::ApiError,
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

pub async fn mock_payments() -> std::io::Result<()> {
    std::env::set_var("PAYMENTS_URI", "http://127.0.0.1:6666");

    async fn create_usage_record_handler() -> Result<HttpResponse, ApiError> {
        println!("Payments received request.");
        Ok(HttpResponse::Ok().finish())
    }

    HttpServer::new(|| {
        App::new().route(
            create_usage_record::ROUTE,
            web::post().to(create_usage_record_handler),
        )
    })
    .bind(("127.0.0.1", 6666))?
    .run()
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
            stripe_id: Some("something".into()),
            state: "nc".into(),
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
