use axum::response::IntoResponse;
use axum::{Router, http::StatusCode, routing::get};

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/health_check", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server is running on http://localhost:3000");

    axum::serve(listener, app).await
}
