mod services;

#[macro_use(lazy_static)]
extern crate lazy_static;

use axum::http::{HeaderValue, Request, Response};
use hyper::{
    body,
    client::HttpConnector,
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Method, StatusCode, Uri,
};
use services::{crud, payments};
use std::{
    convert::Infallible,
    net::{IpAddr, SocketAddr},
};

type Client = hyper::client::Client<HttpConnector, Body>;

lazy_static! {
    static ref PROD: bool = std::env::var("RUST_ENV").unwrap_or("dev".into()) == "prod";
}

async fn app(port: u16) {
    let main_client = Client::new();

    let make_service = make_service_fn(|conn: &AddrStream| {
        let client_ip = conn.remote_addr().ip();
        let temp_client = main_client.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let client = temp_client.clone();

                // for logging
                let req_uri = req.uri().clone();
                let req_method = req.method().clone();
                let ua_header = req.headers().get("User-Agent").cloned();

                let res = async move {
                    let res = match req.method() {
                        &Method::OPTIONS => {
                            corsify(
                                Response::builder()
                                    .status(StatusCode::OK)
                                    .body(Body::empty())
                                    .unwrap(),
                            )
                            .await
                        }
                        _ => corsify(crate::handle(client_ip, client, req).await.unwrap()).await,
                    }
                    .unwrap();
                    log(req_uri, req_method, ua_header.as_ref(), &res);
                    Ok::<Response<Body>, Infallible>(res)
                };
                res
            }))
        }
    });

    let addr = SocketAddr::from((if *PROD { [0, 0, 0, 0] } else { [127, 0, 0, 1] }, port));
    println!("reverse proxy listening on {}", addr);
    axum::Server::bind(&addr).serve(make_service).await.unwrap()
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    app(4000).await
}

async fn handle(
    client_ip: IpAddr,
    client: Client,
    // NOTE: Make sure to put the request extractor last because once the request
    // is extracted, extensions can't be extracted anymore.
    mut req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().to_owned();

    if path.starts_with(payments::PATH_BASE) {
        let path_query = service_path_query("/payments", &mut req, path);

        let uri = format!("http://127.0.0.1:6000{}", path_query);

        *req.uri_mut() = Uri::try_from(uri).unwrap();

        // TODO make payments service
        payments::proxy(client_ip, client, req, path_query).await
    } else if path.starts_with(crud::PATH_BASE) {
        // will forward requests to crud/auth service
        crud::proxy(client_ip, client, req, path).await
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!(
                r#"{{"message": "uri: {} is not valid"}}"#,
                path
            )))
            .unwrap())
    }
}

fn service_path_query(service_path: &str, req: &mut Request<Body>, path: String) -> String {
    let truncated_path = path.replacen(service_path, "", 1);

    // if it is empty add on a "/" so it is the base path
    let truncated_path = if truncated_path.len() == 0 {
        "/".to_string()
    } else {
        truncated_path
    };

    let path_query = req
        .uri()
        .query()
        .map(|query| format!("{}?{}", truncated_path, query))
        .unwrap_or(truncated_path);

    path_query.into()
}

pub async fn proxy_call(
    client_ip: IpAddr,
    forward_uri: &str,
    request: Request<Body>,
) -> Response<Body> {
    match hyper_reverse_proxy::call(client_ip, forward_uri, request).await {
        Ok(response) => response,
        _ => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap(),
    }
}

pub async fn authorize_req(client: &Client, req: &Request<Body>) -> Option<auth::ReqUser> {
    let req_cookies = req.headers().get("Cookie");

    let req_cookies = req_cookies?;
    let auth_req = Request::builder()
        .method(Method::GET)
        .uri(crud::URI.clone() + "/verify")
        .header("Cookie", req_cookies)
        .body(Body::empty())
        .unwrap();

    match client.request(auth_req).await {
        Ok(res) => {
            let body = body::to_bytes(res.into_body()).await.unwrap();
            let opt = serde_json::from_slice(&body);
            opt.ok()
        }
        _ => None,
    }
}

pub async fn corsify(mut res: Response<Body>) -> Result<Response<Body>, Infallible> {
    let new_headers = res.headers_mut();
    new_headers.append(
        "Access-Control-Allow-Origin",
        if *PROD {
            "https://dashboard.milkyweb.app"
        } else {
            "http://localhost:3000"
        }
        .parse::<HeaderValue>()
        .unwrap(),
    );
    new_headers.append(
        "Access-Control-Allow-Headers",
        "Cookie, Content-Type".parse::<HeaderValue>().unwrap(),
    );
    new_headers.append(
        "Access-Control-Allow-Methods",
        "POST, GET, DELETE, PUT, OPTIONS"
            .parse::<HeaderValue>()
            .unwrap(),
    );
    new_headers.append(
        "Access-Control-Allow-Credentials",
        "true".parse::<HeaderValue>().unwrap(),
    );

    Ok(res)
}

// TODO make better logger
pub fn log(uri: Uri, method: Method, ua_header: Option<&HeaderValue>, res: &Response<Body>) {
    let ua = ua_header
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or_default();
    tracing::info!(r#"[Logger] {} {} {} "{}""#, res.status(), method, uri, ua);
}

pub fn error_body(message: impl Into<String>) -> String {
    format!(r#"{{"message": {}}}"#, message.into())
}

#[cfg(test)]
mod tests;
