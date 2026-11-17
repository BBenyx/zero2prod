/*
Our statement: When we receive a GET request for /health_check we return a 200 OK response with no body
 */
use axum::body::{Body, to_bytes};
use axum::http::Request;
use tower::ServiceExt;

#[tokio::test]
async fn health_check_works() {
    let app = zero2prod::app();

    // Oneshot will one send one request without actually starting an HTTP server
    let response = app
        .oneshot(Request::get("/health_check").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert!(response.status().is_success());

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Response body  read failure.");

    assert!(body.is_empty());

    // Sources: github.com/tokio-rs/axum/blob/main/examples/testing/
    // https://docs.rs/axum/latest/axum/body/fn.to_bytes.html
}
