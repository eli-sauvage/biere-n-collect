use axum::{
    extract::Request,
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::env;

pub async fn cors(request: Request, next: Next) -> Response {
    let mut response = if request.method() == axum::http::Method::OPTIONS {
        (StatusCode::OK, "").into_response()
    } else {
        next.run(request).await
    };
    let headers = response.headers_mut();

    headers.insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_str(&env::var("VITE_SITE_URL").unwrap()).unwrap(),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("Content-Type"),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET,PUT,POST,DELETE,OPTIONS,PATCH"),
    );
    headers.insert(
        "Access-Control-Allow-Credentials",
        HeaderValue::from_static("true"),
    );
    response
}
