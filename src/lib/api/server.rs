use axum::{routing::post, Router, http::{Method, header}};
use tower_http::cors::CorsLayer;

use super::handlers;

/// Runs the API server for testing purposes
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if server runs successfully, Err otherwise
pub async fn run_api_server() -> Result<(), Box<dyn std::error::Error>> {
    // Set up CORS for localhost
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:30000".parse()?,
            "http://127.0.0.1:30000".parse()?,
        ])
        .allow_methods([Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    // Build the router with our test endpoint
    let app = Router::new()
        .route("/api/test/echo", post(handlers::test_echo))
        .route("/api/db/create", post(handlers::create_db))
        .layer(cors);

    // Bind to localhost:8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    println!("Test API server running on http://127.0.0.1:8080");
    println!("Endpoints:");
    println!("  POST /api/test/echo - Echo back any JSON data");
    println!("\nPress Ctrl+C to stop the server");

    // Run the server
    axum::serve(listener, app).await?;

    Ok(())
}
