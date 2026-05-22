use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;

async fn test() -> &'static str {
    "Hello from Backend!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/test", get(test))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    println!("Listening on http://localhost:5000");

    axum::serve(listener, app).await.unwrap();
}
