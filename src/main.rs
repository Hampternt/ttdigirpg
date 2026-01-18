//! Entry point for the TTRPG terminal application.
//!
//! This binary can run either the API server for FoundryVTT integration
//! or a demo of the game system.

use std::env;
use ttdigirpg::demo::demo;

/// Application entry point that can launch either the API server or demo.
///
/// Usage:
///   cargo run           - Runs the API server (default)
///   cargo run -- --demo - Runs the character creation demo
///   cargo run -- --server - Explicitly runs the API server
fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for command line arguments
    let mode = if args.len() > 1 {
        args[1].as_str()
    } else {
        "--server" // Default to server mode
    };

    match mode {
        "--demo" => {
            println!("Running demo mode...\n");
            demo();
        }
        "--server" => {
            println!("Starting API server mode...\n");
            // Run the server by spawning the api_server binary logic
            run_server();
        }
        _ => {
            eprintln!("Unknown argument: {}", mode);
            eprintln!("Usage:");
            eprintln!("  cargo run           - Run API server (default)");
            eprintln!("  cargo run -- --demo - Run character demo");
            eprintln!("  cargo run -- --server - Run API server explicitly");
            std::process::exit(1);
        }
    }
}

/// Runs the API server for FoundryVTT integration
#[tokio::main]
async fn run_server() {
    use axum::{
        routing::post,
        Router,
    };
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tower_http::cors::{CorsLayer, Any};
    use ttdigirpg::entities::database::Database;
    use ttdigirpg::api::handlers;

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
