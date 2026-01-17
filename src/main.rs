//! Entry point for the TTRPG terminal application.
//!
//! This binary runs a demonstration of the game system including
//! character creation and database initialization.

use ttdigirpg::demo::demo;

/// Application entry point that launches the demo.
fn main() {
    demo();
}
