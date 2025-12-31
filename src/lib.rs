/// Main library module for the TTRPG system
///
/// This library provides the core systems for a World of Darkness-inspired
/// tabletop RPG implemented as a terminal application in Rust.

// Use path attributes to organize code in lib/ subdirectory
#[path = "lib/entities/mod.rs"]
pub mod entities;

#[path = "lib/systems/mod.rs"]
pub mod systems;
