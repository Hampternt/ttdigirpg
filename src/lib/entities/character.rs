/// Represents a character in the TTRPG system
///
/// The character system is inspired by World of Darkness, with three core attributes
/// and skills divided into three categories. All stats default to 1 and use u32 to
/// allow flexibility without artificial caps (though 1-5 is the typical range).
#[derive(Debug, Clone)]
pub struct Character {
    /// The character's name
    pub name: String,

    // Core Attributes
    /// Physical attribute - represents body, strength, and coordination (default: 1)
    pub physical: u32,
    /// Social attribute - represents charisma, manipulation, and appearance (default: 1)
    pub social: u32,
    /// Mental attribute - represents intelligence, wits, and perception (default: 1)
    pub mental: u32,

    // Talents (innate abilities)
    /// Athletics talent - running, jumping, climbing, sports (default: 1)
    pub athletics: u32,
    /// Awareness talent - noticing details, alertness, perception (default: 1)
    pub awareness: u32,
    /// Brawl talent - unarmed combat, hand-to-hand fighting (default: 1)
    pub brawl: u32,
    /// Streetwise talent - urban survival, underworld knowledge (default: 1)
    pub streetwise: u32,

    // Skills (trained abilities)
    /// Combat skill - armed combat, weapon proficiency (default: 1)
    pub combat: u32,
    /// Stealth skill - sneaking, hiding, moving silently (default: 1)
    pub stealth: u32,
    /// Survival skill - wilderness navigation, tracking, foraging (default: 1)
    pub survival: u32,
    /// Performance skill - acting, music, public speaking (default: 1)
    pub performance: u32,

    // Knowledges (academic abilities)
    /// Academics knowledge - education, research, humanities (default: 1)
    pub academics: u32,
    /// Science knowledge - natural sciences, medicine, technology (default: 1)
    pub science: u32,
    /// Investigation knowledge - research, puzzle-solving, forensics (default: 1)
    pub investigation: u32,
    /// Occult knowledge - supernatural lore, mysticism, hidden knowledge (default: 1)
    pub occult: u32,
}

impl Character {
    /// Creates a new character with the given name and all stats defaulting to 1
    pub fn new(name: String) -> Self {
        Character {
            name,
            // Attributes
            physical: 1,
            social: 1,
            mental: 1,
            // Talents
            athletics: 1,
            awareness: 1,
            brawl: 1,
            streetwise: 1,
            // Skills
            combat: 1,
            stealth: 1,
            survival: 1,
            performance: 1,
            // Knowledges
            academics: 1,
            science: 1,
            investigation: 1,
            occult: 1,
        }
    }

    /// Displays the character sheet in a formatted terminal output.
    ///
    /// Renders a visually appealing character sheet to stdout using Unicode
    /// box-drawing characters. The sheet displays:
    /// - Character name
    /// - Core attributes (Physical, Social, Mental)
    /// - Talents (innate abilities)
    /// - Skills (trained abilities)
    /// - Knowledges (academic abilities)
    ///
    /// Each stat is shown with visual dot indicators (●) and numeric values.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttdigirpg::entities::character::Character;
    ///
    /// let mut character = Character::new("Eldric".to_string());
    /// character.physical = 4;
    /// character.display(); // Prints formatted character sheet to terminal
    /// ```
    pub fn display(&self) {
        println!("╔════════════════════════════════════════╗");
        println!("║  CHARACTER SHEET                       ║");
        println!("╠════════════════════════════════════════╣");
        println!("║  Name: {:<32}║", self.name);
        println!("╠════════════════════════════════════════╣");
        println!("║  ATTRIBUTES                            ║");
        println!("╠════════════════════════════════════════╣");
        println!("║  Physical: {:<28}║", self.format_dots(self.physical));
        println!("║  Social:   {:<28}║", self.format_dots(self.social));
        println!("║  Mental:   {:<28}║", self.format_dots(self.mental));
        println!("╠════════════════════════════════════════╣");
        println!("║  TALENTS (Innate)                      ║");
        println!("╠════════════════════════════════════════╣");
        println!("║  Athletics:   {:<25}║", self.format_dots(self.athletics));
        println!("║  Awareness:   {:<25}║", self.format_dots(self.awareness));
        println!("║  Brawl:       {:<25}║", self.format_dots(self.brawl));
        println!("║  Streetwise:  {:<25}║", self.format_dots(self.streetwise));
        println!("╠════════════════════════════════════════╣");
        println!("║  SKILLS (Trained)                      ║");
        println!("╠════════════════════════════════════════╣");
        println!("║  Combat:      {:<25}║", self.format_dots(self.combat));
        println!("║  Stealth:     {:<25}║", self.format_dots(self.stealth));
        println!("║  Survival:    {:<25}║", self.format_dots(self.survival));
        println!("║  Performance: {:<25}║", self.format_dots(self.performance));
        println!("╠════════════════════════════════════════╣");
        println!("║  KNOWLEDGES (Academic)                 ║");
        println!("╠════════════════════════════════════════╣");
        println!("║  Academics:     {:<23}║", self.format_dots(self.academics));
        println!("║  Science:       {:<23}║", self.format_dots(self.science));
        println!("║  Investigation: {:<23}║", self.format_dots(self.investigation));
        println!("║  Occult:        {:<23}║", self.format_dots(self.occult));
        println!("╚════════════════════════════════════════╝");
    }

    /// Formats a stat value as visual dots with the numeric value.
    ///
    /// This helper method converts a numeric stat value into a string representation
    /// using filled circle characters (●) followed by the number in parentheses.
    ///
    /// # Arguments
    ///
    /// * `value` - The numeric stat value to format
    ///
    /// # Returns
    ///
    /// A formatted string like "●●● (3)" for a value of 3.
    ///
    /// # Examples
    ///
    /// ```
    /// // A character with investigation = 5 would display as:
    /// // "●●●●● (5)"
    /// ```
    fn format_dots(&self, value: u32) -> String {
        let dots = "●".repeat(value as usize);
        format!("{} ({})", dots, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_character_defaults() {
        let character = Character::new("Test Character".to_string());

        // All attributes should default to 1
        assert_eq!(character.physical, 1);
        assert_eq!(character.social, 1);
        assert_eq!(character.mental, 1);

        // All talents should default to 1
        assert_eq!(character.athletics, 1);
        assert_eq!(character.awareness, 1);
        assert_eq!(character.brawl, 1);
        assert_eq!(character.streetwise, 1);

        // All skills should default to 1
        assert_eq!(character.combat, 1);
        assert_eq!(character.stealth, 1);
        assert_eq!(character.survival, 1);
        assert_eq!(character.performance, 1);

        // All knowledges should default to 1
        assert_eq!(character.academics, 1);
        assert_eq!(character.science, 1);
        assert_eq!(character.investigation, 1);
        assert_eq!(character.occult, 1);
    }

    #[test]
    fn test_character_modification() {
        let mut character = Character::new("Skilled Fighter".to_string());

        // Modify some stats
        character.physical = 4;
        character.combat = 5;
        character.brawl = 3;

        assert_eq!(character.physical, 4);
        assert_eq!(character.combat, 5);
        assert_eq!(character.brawl, 3);
    }
}
