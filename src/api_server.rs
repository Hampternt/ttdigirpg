use axum::{
    routing::post,
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{CorsLayer, Any};

use ttdigirpg::entities::database::Database;
use ttdigirpg::api::handlers;

#[tokio::main]
async fn main() {
    // Initialize the database
    let db_path = "src/database/game_data.db";
    let db = Database::new(db_path).expect("Failed to initialize database");
    let db = Arc::new(Mutex::new(db));

    // Set up CORS to allow requests from FoundryVTT (localhost)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the router with our endpoint
    let app = Router::new()
        .route("/api/character/controls", post(handlers::update_controls))
        .layer(cors)
        .with_state(db);

    // Bind to localhost:8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to port 8080");

    println!("FoundryVTT API server running on http://127.0.0.1:8080");
    println!("Endpoints:");
    println!("  POST /api/character/controls - Update character controls");
    println!("\nPress Ctrl+C to stop the server");

    // Run the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
