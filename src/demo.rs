//! Demo module showcasing the TTRPG system capabilities.
//!
//! This module demonstrates character creation with both default and customized stats,
//! as well as database initialization.

use crate::entities::character::Character;
use crate::entities::database::Database;

/// Runs a demonstration of the TTRPG system.
///
/// This function showcases:
/// - Database initialization (creates or opens existing database)
/// - Creating characters with default stats (all attributes start at 1)
/// - Customizing character stats by direct field modification
/// - Displaying formatted character sheets to the terminal
///
/// The demo creates two main characters:
/// 1. A default character with all stats at 1
/// 2. A "Veteran Investigator" with customized stats emphasizing investigation skills
///
/// # Panics
///
/// Panics if the database cannot be initialized at the specified path.
pub fn demo() {
    println!("=== TTRPG System Demo ===\n");

    // Check for database and create new databse if no database found on start.
    let _db = Database::new("src/database/game_data.db").expect("Failed to initialize database");

    // Create a default character
    let default_char = Character::new("Default Character".to_string());
    println!("Created a new character with default stats:\n");
    default_char.display();

    println!("\n\n");

    // Create a customized character
    let mut skilled_char = Character::new("Veteran Investigator".to_string());

    // Set attributes (above average investigator)
    skilled_char.physical = 2;
    skilled_char.social = 3;
    skilled_char.mental = 4;

    // Set talents
    skilled_char.athletics = 2;
    skilled_char.awareness = 4;
    skilled_char.brawl = 2;
    skilled_char.streetwise = 3;

    // Set skills
    skilled_char.combat = 2;
    skilled_char.stealth = 3;
    skilled_char.survival = 2;
    skilled_char.performance = 2;

    // Set knowledges (investigator specialty)
    skilled_char.academics = 3;
    skilled_char.science = 3;
    skilled_char.investigation = 5;
    skilled_char.occult = 4;

    println!("Created a customized character:\n");
    skilled_char.display();

    let default_char = Character::new("Default Character".to_string());
    println!("Created a new character with default stats:\n");
    default_char.display();

    println!("\n\n=== Demo Complete ===");
}
