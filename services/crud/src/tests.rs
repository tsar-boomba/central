use actix_web::{
    dev::{Service, ServiceResponse},
    test, web, App, HttpResponse, HttpServer,
};
use diesel::prelude::*;
use models::{Account, NewAccount};
use payments_lib::routes::create_usage_record;
use std::sync::{Arc, Mutex};

use crate::{api_error::ApiError, auth, db};

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

    let aws_creds = aws_config::load_from_env().await;
    let eb_client = aws_sdk_elasticbeanstalk::Client::new(&aws_creds);
    let r53_client = aws_sdk_route53::Client::new(&aws_creds);
    let sns_client = aws_sdk_sns::Client::new(&aws_creds);

    let app_data = crate::AppData {
        eb_client,
        r53_client,
        sns_client,
    };

    // serialize password when doing tests
    models::dont_skip_pass();
    test::init_service(
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .wrap(auth::middleware::Authorize)
            .configure(init_routes),
    )
    .await
}

pub async fn mock_payments() -> std::io::Result<()> {
    std::env::set_var("PAYMENTS_URI", "http://127.0.0.1:6666");

    async fn create_usage_record_handler() -> Result<HttpResponse, ApiError> {
        Ok(HttpResponse::Ok().finish())
    }

    async fn update_customer() -> Result<HttpResponse, ApiError> {
        Ok(HttpResponse::Ok().finish())
    }

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .route(
                create_usage_record::ROUTE,
                web::post().to(create_usage_record_handler),
            )
            .route("/customer/{id}", web::put().to(update_customer))
    })
    .bind(("127.0.0.1", 6666))?
    .run()
    .await
}

pub async fn mock_instance_deploy() -> std::io::Result<()> {
    std::env::set_var("INSTANCES_URI", "http://127.0.0.1:7777");

    async fn deploy() -> Result<HttpResponse, ApiError> {
        Ok(HttpResponse::Ok().finish())
    }

    HttpServer::new(|| App::new().route("/", web::post().to(deploy)))
        .bind(("127.0.0.1", 7777))?
        .run()
        .await
}

/// Creates an account to ensure foreign keys are satisfied during testing
pub fn test_account() {
    use models::accounts::dsl::*;
    diesel::insert_into(accounts)
        .values(NewAccount {
            id: "test".into(),
            address1: "testys".into(),
            address2: None,
            business_name: "tests".into(),
            city: "testcity".into(),
            email: "test@testys.test".into(),
            phone_number: "999-999-0000".into(),
            short_name: "test".into(),
            zip_code: "28282".into(),
            stripe_id: Some("something".into()),
            sub_id: Some("totally_subbed".into()),
            state: "NC".into(),
        })
        .on_conflict_do_nothing()
        .get_result::<Account>(&db::connection().unwrap())
        .ok();
}

// pub fn remove_test_account() {
//     use crate::accounts::accounts::dsl::*;
//     diesel::delete(accounts.filter(id.eq("test")))
//         .execute(&db::connection().unwrap())
//         .unwrap();
// }
