use axum::{routing::post, Router, http::{Method, header}};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use crate::entities::database::Database;
use super::handlers;

/// Runs the API server for FoundryVTT integration
///
/// # Arguments
/// * `db_path` - Path to the SQLite database file
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if server runs successfully, Err otherwise
pub async fn run_api_server(db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the database
    let db = Database::new(db_path)?;
    let db = Arc::new(Mutex::new(db));

    // Set up CORS for FoundryVTT (localhost only)
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:30000".parse()?,
            "http://127.0.0.1:30000".parse()?,
        ])
        .allow_methods([Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    // Build the router with our endpoint
    let app = Router::new()
        .route("/api/character/controls", post(handlers::update_controls))
        .layer(cors)
        .with_state(db);

    // Bind to localhost:8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    println!("FoundryVTT API server running on http://127.0.0.1:8080");
    println!("Endpoints:");
    println!("  POST /api/character/controls - Update character controls");
    println!("\nPress Ctrl+C to stop the server");

    // Run the server
    axum::serve(listener, app).await?;

    Ok(())
}
