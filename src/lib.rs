use axum::{http::StatusCode, response::IntoResponse, Router, routing::get};

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn run() -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server is running on http://localhost:3000");
    axum::serve(listener, app()).await
}

// Router creating separated for unit testing
pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
}