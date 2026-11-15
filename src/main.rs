use axum::{Router, extract::Path, routing::get};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(greet))
        .route("/{name}", get(greet));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server is running on http://localhost:3000");

    axum::serve(listener, app).await
}

async fn greet(req: Option<Path<String>>) -> String {
    let name = match req {
        Some(Path(val)) => val,
        None => "World".to_string(),
    };
    format!("Hello {}!", name)
}
