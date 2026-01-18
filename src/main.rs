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
    use ttdigirpg::api::server::run_api_server;

    let db_path = "src/database/game_data.db";
    if let Err(e) = run_api_server(db_path).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
