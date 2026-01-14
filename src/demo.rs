use crate::entities::character::Character;

pub fn demo() {
    println!("=== TTRPG System Demo ===\n");

    // Check for database and create new databse if no database found on start.
    

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

    println!("\n\n=== Demo Complete ===");
}
