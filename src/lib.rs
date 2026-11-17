use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

pub async fn run() -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server is running on http://localhost:3000");
    axum::serve(listener, app()).await
}

// Router creating separated for unit testing
pub fn app() -> Router {
    Router::new().route("/health_check", get(health_check))
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
