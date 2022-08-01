use std::future::{ready, Ready};

use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::{Error, HttpMessage};
use futures_util::future::LocalBoxFuture;

pub struct Authorize;

// middleware factory boiler plate 🤮
impl<S, B> Transform<S, ServiceRequest> for Authorize
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizeMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizeMiddleware { service }))
    }
}
pub struct AuthorizeMiddleware<S> {
    service: S,
}

// actual middleware
impl<S, B> Service<ServiceRequest> for AuthorizeMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let user = request.headers().get("user");
        // Check if token
        let authed_token = auth::extract(
            user.map(|v| v.to_str().unwrap().to_string())
                .unwrap_or_default(),
        );

        // add claims into request extensions if they are there
        request.extensions_mut().insert(authed_token);

        let res = self.service.call(request);

        Box::pin(async move { res.await })
    }
}
