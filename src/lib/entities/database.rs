//! Database management module for persistent game data storage.
//! This module handles SQLite database initialization, table creation,
//! and provides constructors for both shared and user-specific databases.
use rusqlite::{Connection, Result};
use std::path::Path;
use uuid::Uuid;

/// Wrapper around a SQLite database connection for game data persistence.
///
/// The Database struct manages SQLite connections and provides methods for
/// initializing databases. It supports both fixed-path databases and
/// dynamically-named user/character-specific databases.
pub struct Database {
    /// The underlying SQLite connection
    conn: Connection,
}

impl Database {
    /// Creates or opens a database at the specified path.
    ///
    /// If the database file doesn't exist, this method will:
    /// 1. Create a new database file at the specified path
    /// 2. Initialize tables by calling `create_tables`
    /// 3. Print a confirmation message
    ///
    /// If the database file already exists, it will simply open the existing database.
    ///
    /// # Arguments
    ///
    /// * `db_path` - The file path where the database should be created or opened
    ///
    /// # Returns
    ///
    /// Returns a `Result<Database>` containing the initialized database or an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttdigirpg::entities::database::Database;
    ///
    /// // Use :memory: for in-memory database in examples/tests
    /// let db = Database::new(":memory:").expect("Failed to create database");
    /// // Database created successfully if we get here
    /// ```
    pub fn new(db_path: &str) -> Result<Self> {
        // Check if database file already exists
        let db_exists = Path::new(db_path).exists();
        let conn = Connection::open(db_path)?;

        // Enable foreign key constraints
        conn.execute("PRAGMA foreign_keys = ON", [])?;

        if !db_exists {
            println!("Creating new Database! At {}", db_path);
            Self::create_tables(&conn)?;
        } else {
            println!("Opening existing database at {}", db_path);
        }

        Ok(Database { conn })
    }

    /// Creates or opens a user/character-specific database.
    ///
    /// This constructor generates a unique database path by combining the base path
    /// with a user or character name. The name is sanitized (spaces replaced with
    /// underscores) and appended to create a unique database file for each user.
    ///
    /// This allows each player or character to have their own isolated database
    /// for storing personal game state, inventory, progress, etc.
    ///
    /// # Arguments
    ///
    /// * `db_path` - The base directory path where the database should be created
    /// * `name` - The user or character name used to generate a unique database filename
    ///
    /// # Returns
    ///
    /// Returns a `Result<Database>` containing the initialized database or an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttdigirpg::entities::database::Database;
    ///
    /// // Use :memory: for in-memory database in examples/tests
    /// let db = Database::new_with_name(":memory:", "Veteran Investigator")
    ///     .expect("Failed to create character database");
    /// // Database created successfully with sanitized name
    /// ```
    ///
    pub fn new_with_name(db_path: &str, name: &str) -> Result<Self> {
        let full_name_string_path: String = Database::name_combiner(db_path, name);

        let full_name_string_path_exists: bool = Path::new(&full_name_string_path).exists();
        let conn = Connection::open(full_name_string_path)?;

        // Enable foreign key constraints
        conn.execute("PRAGMA foreign_keys = ON", [])?;

        if !full_name_string_path_exists {
            println!("Creating new Database! At {}", db_path);
            Self::create_tables(&conn)?;
        } else {
            println!("Opening existing database at {}", db_path);
        }

        Ok(Database { conn })
    }

    /// Combines two strings into a valid file path by concatenating and sanitizing.
    ///
    /// This utility function takes two string slices and:
    /// 1. Concatenates all characters from both strings together
    /// 2. Replaces any space characters with underscores
    /// 3. Returns the sanitized combined string suitable for use in file paths
    ///
    /// This is used to create database filenames by combining a base path with
    /// a user/character name.
    ///
    /// # Arguments
    ///
    /// * `word1` - The first string (typically a directory path)
    /// * `word2` - The second string (typically a user/character name)
    ///
    /// # Returns
    ///
    /// A new `String` containing the concatenated and sanitized result.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttdigirpg::entities::database::Database;
    ///
    /// let path = Database::name_combiner("data/saves/", "My Character");
    /// assert_eq!(path, "data/saves/My_Character.db");
    /// ```
    pub fn name_combiner(word1: &str, word2: &str) -> String {
        let parts = [word1, word2];

        let combined: String = parts
            .iter()
            .flat_map(|s| s.chars())
            .map(|c| if c == ' ' { '_' } else { c })
            .collect::<String>();

        println!("{combined}");

        if combined.ends_with(".db") {
            combined
        } else {
            combined + ".db"
        }
    }

    /// Initializes database tables for a new database.
    ///
    /// This private method is called when a new database is created. It executes
    /// SQL statements to create the necessary table schema.
    ///
    /// Creates three tables:
    /// - `characters`: Stores character data with game context and flexible JSON data
    /// - `character_objects`: Tracks ownership/associations between characters and objects
    /// - `objects`: Defines object templates with flexible JSON properties
    ///
    /// # Arguments
    ///
    /// * `conn` - Reference to the SQLite connection where tables should be created
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if table creation fails.
    fn create_tables(conn: &Connection) -> Result<()> {
        // Characters table - stores character data with game context
        conn.execute(
            "CREATE TABLE characters (
                uuid TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                game TEXT NOT NULL,
                data TEXT,
                PRIMARY KEY (name, game)
            )",
            [],
        )?;

        // Objects table - defines what objects are (templates/definitions)
        conn.execute(
            "CREATE TABLE objects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                properties TEXT
            )",
            [],
        )?;

        // Character objects table - tracks ownership/associations
        conn.execute(
            "CREATE TABLE character_objects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                game TEXT NOT NULL,
                character_name TEXT NOT NULL,
                object_id INTEGER NOT NULL,
                quantity INTEGER DEFAULT 1,
                FOREIGN KEY (object_id) REFERENCES objects(id) ON DELETE CASCADE,
                FOREIGN KEY (character_name, game) REFERENCES characters(name, game) ON DELETE CASCADE
            )",
            [],
        )?;

        println!("Tables created successfully!");
        println!("  - characters: Stores character data");
        println!("  - objects: Stores object definitions");
        println!("  - character_objects: Tracks character ownership");
        Ok(())
    }

    // ==================== CHARACTER METHODS ====================

    /// Inserts a new character into the database.
    ///
    /// # Arguments
    ///
    /// * `name` - The character's name
    /// * `game` - The game this character belongs to
    /// * `data` - Optional JSON string containing character data (stats, attributes, etc.)
    ///
    /// # Returns
    ///
    /// Returns the UUID of the newly inserted character, or an error if insertion fails
    /// (e.g., if a character with the same name already exists in this game).
    ///
    /// # Examples
    ///
    /// ```
    /// use ttdigirpg::entities::database::Database;
    ///
    /// let db = Database::new(":memory:").unwrap();
    /// let uuid = db.insert_character("Alice", "Knives Out", Some("{\"level\": 5}")).unwrap();
    /// ```
    pub fn insert_character(&self, name: &str, game: &str, data: Option<&str>) -> Result<String> {
        let uuid = Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO characters (uuid, name, game, data) VALUES (?1, ?2, ?3, ?4)",
            (&uuid, name, game, data),
        )?;
        Ok(uuid)
    }

    /// Retrieves a character from the database.
    ///
    /// # Arguments
    ///
    /// * `name` - The character's name
    /// * `game` - The game this character belongs to
    ///
    /// # Returns
    ///
    /// Returns `Some((uuid, name, game, data))` if found, or `None` if not found.
    pub fn get_character(
        &self,
        name: &str,
        game: &str,
    ) -> Result<Option<(String, String, String, Option<String>)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT uuid, name, game, data FROM characters WHERE name = ?1 AND game = ?2")?;

        let mut rows = stmt.query((name, game))?;

        if let Some(row) = rows.next()? {
            Ok(Some((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
        } else {
            Ok(None)
        }
    }

    /// Updates a character's data in the database.
    ///
    /// # Arguments
    ///
    /// * `name` - The character's name
    /// * `game` - The game this character belongs to
    /// * `data` - New JSON string containing character data
    ///
    /// # Returns
    ///
    /// Returns the number of rows updated (should be 1 if successful, 0 if character not found).
    pub fn update_character(&self, name: &str, game: &str, data: &str) -> Result<usize> {
        Ok(self.conn.execute(
            "UPDATE characters SET data = ?1 WHERE name = ?2 AND game = ?3",
            (data, name, game),
        )?)
    }

    /// Deletes a character from the database.
    ///
    /// # Arguments
    ///
    /// * `name` - The character's name
    /// * `game` - The game this character belongs to
    ///
    /// # Returns
    ///
    /// Returns the number of rows deleted (should be 1 if successful, 0 if character not found).
    pub fn delete_character(&self, name: &str, game: &str) -> Result<usize> {
        Ok(self.conn.execute(
            "DELETE FROM characters WHERE name = ?1 AND game = ?2",
            (name, game),
        )?)
    }

    // ==================== OBJECT METHODS ====================

    /// Inserts a new object definition into the database.
    ///
    /// # Arguments
    ///
    /// * `name` - The object's name (e.g., "Car Factory", "Sword")
    /// * `obj_type` - The object's type (e.g., "building", "organization", "item")
    /// * `properties` - Optional JSON string containing object properties
    ///
    /// # Returns
    ///
    /// Returns the ID of the newly inserted object.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttdigirpg::entities::database::Database;
    ///
    /// let db = Database::new(":memory:").unwrap();
    /// let id = db.insert_object(
    ///     "Car Factory",
    ///     "building",
    ///     Some("{\"production_rate\": 10, \"cost\": 1000}")
    /// ).unwrap();
    /// ```
    pub fn insert_object(
        &self,
        name: &str,
        obj_type: &str,
        properties: Option<&str>,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO objects (name, type, properties) VALUES (?1, ?2, ?3)",
            (name, obj_type, properties),
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Retrieves an object definition by ID.
    ///
    /// # Arguments
    ///
    /// * `object_id` - The object's ID
    ///
    /// # Returns
    ///
    /// Returns `Some((id, name, type, properties))` if found, or `None` if not found.
    pub fn get_object(
        &self,
        object_id: i64,
    ) -> Result<Option<(i64, String, String, Option<String>)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, type, properties FROM objects WHERE id = ?1")?;

        let mut rows = stmt.query([object_id])?;

        if let Some(row) = rows.next()? {
            Ok(Some((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
        } else {
            Ok(None)
        }
    }

    /// Updates an object's properties.
    ///
    /// # Arguments
    ///
    /// * `object_id` - The object's ID
    /// * `properties` - New JSON string containing object properties
    ///
    /// # Returns
    ///
    /// Returns the number of rows updated (should be 1 if successful, 0 if object not found).
    pub fn update_object(&self, object_id: i64, properties: &str) -> Result<usize> {
        Ok(self.conn.execute(
            "UPDATE objects SET properties = ?1 WHERE id = ?2",
            (properties, object_id),
        )?)
    }

    /// Deletes an object definition from the database.
    ///
    /// # Arguments
    ///
    /// * `object_id` - The object's ID
    ///
    /// # Returns
    ///
    /// Returns the number of rows deleted (should be 1 if successful, 0 if object not found).
    pub fn delete_object(&self, object_id: i64) -> Result<usize> {
        Ok(self
            .conn
            .execute("DELETE FROM objects WHERE id = ?1", [object_id])?)
    }

    // ==================== CHARACTER OBJECT (OWNERSHIP) METHODS ====================

    /// Adds an object to a character's inventory/associations.
    ///
    /// # Arguments
    ///
    /// * `game` - The game context
    /// * `character_name` - The character's name
    /// * `object_id` - The ID of the object to add
    /// * `quantity` - How many of this object to add (default: 1)
    ///
    /// # Returns
    ///
    /// Returns the ID of the newly created association.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttdigirpg::entities::database::Database;
    ///
    /// let db = Database::new(":memory:").unwrap();
    /// db.insert_character("Bob", "Knives Out", None).unwrap();
    /// let factory_id = db.insert_object("Car Factory", "building", None).unwrap();
    /// db.add_object_to_character("Knives Out", "Bob", factory_id, 1).unwrap();
    /// ```
    pub fn add_object_to_character(
        &self,
        game: &str,
        character_name: &str,
        object_id: i64,
        quantity: i32,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO character_objects (game, character_name, object_id, quantity) VALUES (?1, ?2, ?3, ?4)",
            (game, character_name, object_id, quantity),
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Removes an object from a character's inventory/associations.
    ///
    /// # Arguments
    ///
    /// * `game` - The game context
    /// * `character_name` - The character's name
    /// * `object_id` - The ID of the object to remove
    ///
    /// # Returns
    ///
    /// Returns the number of rows deleted.
    pub fn remove_object_from_character(
        &self,
        game: &str,
        character_name: &str,
        object_id: i64,
    ) -> Result<usize> {
        Ok(self.conn.execute(
            "DELETE FROM character_objects WHERE game = ?1 AND character_name = ?2 AND object_id = ?3",
            (game, character_name, object_id),
        )?)
    }

    /// Updates the quantity of an object in a character's inventory.
    ///
    /// # Arguments
    ///
    /// * `game` - The game context
    /// * `character_name` - The character's name
    /// * `object_id` - The ID of the object
    /// * `quantity` - The new quantity
    ///
    /// # Returns
    ///
    /// Returns the number of rows updated.
    pub fn update_object_quantity(
        &self,
        game: &str,
        character_name: &str,
        object_id: i64,
        quantity: i32,
    ) -> Result<usize> {
        Ok(self.conn.execute(
            "UPDATE character_objects SET quantity = ?1 WHERE game = ?2 AND character_name = ?3 AND object_id = ?4",
            (quantity, game, character_name, object_id),
        )?)
    }

    /// Gets all objects owned by a character.
    ///
    /// # Arguments
    ///
    /// * `game` - The game context
    /// * `character_name` - The character's name
    ///
    /// # Returns
    ///
    /// Returns a vector of tuples containing (object_id, object_name, object_type, quantity, properties).
    pub fn get_character_objects(
        &self,
        game: &str,
        character_name: &str,
    ) -> Result<Vec<(i64, String, String, i32, Option<String>)>> {
        let mut stmt = self.conn.prepare(
            "SELECT o.id, o.name, o.type, co.quantity, o.properties
             FROM character_objects co
             JOIN objects o ON co.object_id = o.id
             WHERE co.game = ?1 AND co.character_name = ?2",
        )?;

        let rows = stmt.query_map((game, character_name), |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        let mut objects = Vec::new();
        for row in rows {
            objects.push(row?);
        }

        Ok(objects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== HELPER FUNCTIONS ====================

    /// Helper function to create a fresh in-memory database for testing.
    /// Using :memory: creates a temporary database that's destroyed after the test.
    fn setup_test_db() -> Database {
        Database::new(":memory:").expect("Failed to create test database")
    }

    // ==================== CONSTRUCTOR TESTS ====================

    #[test]
    fn test_database_new_creates_tables() {
        // Test that new() successfully creates a database with tables
        let db = setup_test_db();

        // If we can insert a character, the tables were created successfully
        let result = db.insert_character("Test Character", "Test Game", None);
        assert!(result.is_ok(), "Should be able to insert into newly created database");
    }

    #[test]
    fn test_name_combiner_basic() {
        // Test basic string concatenation
        let result = Database::name_combiner("path/", "file");
        assert_eq!(result, "path/file.db");
    }

    #[test]
    fn test_name_combiner_with_spaces() {
        // Test that spaces are replaced with underscores
        let result = Database::name_combiner("data/saves/", "My Character");
        assert_eq!(result, "data/saves/My_Character.db");
    }

    #[test]
    fn test_name_combiner_multiple_spaces() {
        // Test handling of multiple spaces
        let result = Database::name_combiner("saves/", "Veteran  Investigator  2");
        assert_eq!(result, "saves/Veteran__Investigator__2.db");
    }

    // ==================== CHARACTER METHOD TESTS ====================

    #[test]
    fn test_insert_character_basic() {
        let db = setup_test_db();

        // Insert a character and verify we get a valid UUID
        let uuid = db.insert_character("Alice", "Knives Out", None)
            .expect("Failed to insert character");

        assert!(!uuid.is_empty(), "UUID should not be empty");
        assert!(Uuid::parse_str(&uuid).is_ok(), "Should return a valid UUID");
    }

    #[test]
    fn test_insert_character_with_data() {
        let db = setup_test_db();

        let json_data = r#"{"level": 5, "class": "warrior"}"#;
        let uuid = db.insert_character("Bob", "RPG Game", Some(json_data))
            .expect("Failed to insert character with data");

        assert!(!uuid.is_empty(), "Should return a valid UUID");
    }

    #[test]
    fn test_insert_duplicate_character_fails() {
        let db = setup_test_db();

        // Insert first character successfully
        db.insert_character("Alice", "Knives Out", None)
            .expect("First insert should succeed");

        // Try to insert duplicate (same name + game) - should fail
        let result = db.insert_character("Alice", "Knives Out", None);
        assert!(result.is_err(), "Duplicate character should fail due to UNIQUE constraint");
    }

    #[test]
    fn test_insert_same_name_different_game_succeeds() {
        let db = setup_test_db();

        // Same character name in different games should be allowed
        db.insert_character("Alice", "Game1", None)
            .expect("First insert should succeed");

        let result = db.insert_character("Alice", "Game2", None);
        assert!(result.is_ok(), "Same name in different game should succeed");
    }

    #[test]
    fn test_get_character_exists() {
        let db = setup_test_db();

        let json_data = r#"{"level": 5}"#;
        db.insert_character("Alice", "Knives Out", Some(json_data))
            .expect("Failed to insert character");

        // Retrieve the character
        let result = db.get_character("Alice", "Knives Out")
            .expect("Query failed");

        assert!(result.is_some(), "Character should be found");

        let (uuid, name, game, data) = result.unwrap();
        assert!(!uuid.is_empty(), "UUID should not be empty");
        assert!(Uuid::parse_str(&uuid).is_ok(), "Should have a valid UUID");
        assert_eq!(name, "Alice");
        assert_eq!(game, "Knives Out");
        assert_eq!(data, Some(json_data.to_string()));
    }

    #[test]
    fn test_get_character_not_exists() {
        let db = setup_test_db();

        let result = db.get_character("NonExistent", "Test Game")
            .expect("Query should not fail");

        assert!(result.is_none(), "Non-existent character should return None");
    }

    #[test]
    fn test_update_character() {
        let db = setup_test_db();

        // Insert character
        db.insert_character("Alice", "Knives Out", Some(r#"{"level": 1}"#))
            .expect("Failed to insert character");

        // Update the character data
        let updated_data = r#"{"level": 10, "class": "mage"}"#;
        let rows_affected = db.update_character("Alice", "Knives Out", updated_data)
            .expect("Failed to update character");

        assert_eq!(rows_affected, 1, "Should update exactly 1 row");

        // Verify the update
        let result = db.get_character("Alice", "Knives Out")
            .expect("Failed to get character");
        let (_, _, _, data) = result.unwrap();
        assert_eq!(data, Some(updated_data.to_string()));
    }

    #[test]
    fn test_update_nonexistent_character() {
        let db = setup_test_db();

        let rows_affected = db.update_character("Ghost", "Test Game", "{}")
            .expect("Update should not fail");

        assert_eq!(rows_affected, 0, "Updating non-existent character should affect 0 rows");
    }

    #[test]
    fn test_delete_character() {
        let db = setup_test_db();

        // Insert and then delete
        db.insert_character("ToDelete", "Test Game", None)
            .expect("Failed to insert character");

        let rows_affected = db.delete_character("ToDelete", "Test Game")
            .expect("Failed to delete character");

        assert_eq!(rows_affected, 1, "Should delete exactly 1 row");

        // Verify deletion
        let result = db.get_character("ToDelete", "Test Game")
            .expect("Query failed");
        assert!(result.is_none(), "Character should be gone");
    }

    #[test]
    fn test_delete_nonexistent_character() {
        let db = setup_test_db();

        let rows_affected = db.delete_character("Ghost", "Test Game")
            .expect("Delete should not fail");

        assert_eq!(rows_affected, 0, "Deleting non-existent character should affect 0 rows");
    }

    // ==================== OBJECT METHOD TESTS ====================

    #[test]
    fn test_insert_object() {
        let db = setup_test_db();

        let id = db.insert_object("Sword", "weapon", Some(r#"{"damage": 10}"#))
            .expect("Failed to insert object");

        assert_eq!(id, 1, "First object should have ID 1");
    }

    #[test]
    fn test_get_object_exists() {
        let db = setup_test_db();

        let props = r#"{"damage": 10}"#;
        let inserted_id = db.insert_object("Sword", "weapon", Some(props))
            .expect("Failed to insert object");

        let result = db.get_object(inserted_id)
            .expect("Query failed");

        assert!(result.is_some());
        let (id, name, obj_type, properties) = result.unwrap();
        assert_eq!(id, inserted_id);
        assert_eq!(name, "Sword");
        assert_eq!(obj_type, "weapon");
        assert_eq!(properties, Some(props.to_string()));
    }

    #[test]
    fn test_get_object_not_exists() {
        let db = setup_test_db();

        let result = db.get_object(999)
            .expect("Query should not fail");

        assert!(result.is_none());
    }

    #[test]
    fn test_update_object() {
        let db = setup_test_db();

        let id = db.insert_object("Sword", "weapon", Some(r#"{"damage": 10}"#))
            .expect("Failed to insert object");

        let new_props = r#"{"damage": 20, "durability": 100}"#;
        let rows_affected = db.update_object(id, new_props)
            .expect("Failed to update object");

        assert_eq!(rows_affected, 1);

        let result = db.get_object(id).expect("Query failed");
        let (_, _, _, properties) = result.unwrap();
        assert_eq!(properties, Some(new_props.to_string()));
    }

    #[test]
    fn test_delete_object() {
        let db = setup_test_db();

        let id = db.insert_object("Sword", "weapon", None)
            .expect("Failed to insert object");

        let rows_affected = db.delete_object(id)
            .expect("Failed to delete object");

        assert_eq!(rows_affected, 1);

        let result = db.get_object(id).expect("Query failed");
        assert!(result.is_none());
    }

    // ==================== CHARACTER OBJECT (OWNERSHIP) TESTS ====================

    #[test]
    fn test_add_object_to_character() {
        let db = setup_test_db();

        // Setup: create character and object
        db.insert_character("Alice", "Test Game", None).unwrap();
        let sword_id = db.insert_object("Sword", "weapon", None).unwrap();

        // Add object to character
        let association_id = db.add_object_to_character("Test Game", "Alice", sword_id, 1)
            .expect("Failed to add object to character");

        assert!(association_id > 0);
    }

    #[test]
    fn test_get_character_objects() {
        let db = setup_test_db();

        // Setup
        db.insert_character("Alice", "Test Game", None).unwrap();
        let sword_id = db.insert_object("Sword", "weapon", Some(r#"{"damage": 10}"#)).unwrap();
        let shield_id = db.insert_object("Shield", "armor", Some(r#"{"defense": 5}"#)).unwrap();

        // Add multiple objects
        db.add_object_to_character("Test Game", "Alice", sword_id, 1).unwrap();
        db.add_object_to_character("Test Game", "Alice", shield_id, 2).unwrap();

        // Get all objects
        let objects = db.get_character_objects("Test Game", "Alice")
            .expect("Failed to get character objects");

        assert_eq!(objects.len(), 2, "Character should have 2 objects");

        // Verify first object (Sword)
        let (id, name, obj_type, quantity, properties) = &objects[0];
        assert_eq!(*id, sword_id);
        assert_eq!(name, "Sword");
        assert_eq!(obj_type, "weapon");
        assert_eq!(*quantity, 1);
        assert_eq!(properties, &Some(r#"{"damage": 10}"#.to_string()));

        // Verify second object (Shield)
        let (id, name, obj_type, quantity, _) = &objects[1];
        assert_eq!(*id, shield_id);
        assert_eq!(name, "Shield");
        assert_eq!(obj_type, "armor");
        assert_eq!(*quantity, 2);
    }

    #[test]
    fn test_get_character_objects_empty() {
        let db = setup_test_db();

        db.insert_character("Alice", "Test Game", None).unwrap();

        let objects = db.get_character_objects("Test Game", "Alice")
            .expect("Failed to get character objects");

        assert_eq!(objects.len(), 0, "New character should have no objects");
    }

    #[test]
    fn test_update_object_quantity() {
        let db = setup_test_db();

        // Setup
        db.insert_character("Alice", "Test Game", None).unwrap();
        let potion_id = db.insert_object("Potion", "consumable", None).unwrap();
        db.add_object_to_character("Test Game", "Alice", potion_id, 5).unwrap();

        // Update quantity
        let rows_affected = db.update_object_quantity("Test Game", "Alice", potion_id, 10)
            .expect("Failed to update quantity");

        assert_eq!(rows_affected, 1);

        // Verify
        let objects = db.get_character_objects("Test Game", "Alice").unwrap();
        let (_, _, _, quantity, _) = &objects[0];
        assert_eq!(*quantity, 10);
    }

    #[test]
    fn test_remove_object_from_character() {
        let db = setup_test_db();

        // Setup
        db.insert_character("Alice", "Test Game", None).unwrap();
        let sword_id = db.insert_object("Sword", "weapon", None).unwrap();
        db.add_object_to_character("Test Game", "Alice", sword_id, 1).unwrap();

        // Remove object
        let rows_affected = db.remove_object_from_character("Test Game", "Alice", sword_id)
            .expect("Failed to remove object");

        assert_eq!(rows_affected, 1);

        // Verify removal
        let objects = db.get_character_objects("Test Game", "Alice").unwrap();
        assert_eq!(objects.len(), 0, "Character should have no objects after removal");
    }

    // ==================== INTEGRATION TESTS ====================

    #[test]
    fn test_full_character_lifecycle() {
        // Test a complete workflow: create, read, update, delete
        let db = setup_test_db();

        // Create
        let id = db.insert_character("Hero", "Epic Quest", Some(r#"{"level": 1}"#))
            .expect("Failed to insert");

        // Read
        let character = db.get_character("Hero", "Epic Quest")
            .expect("Failed to get")
            .expect("Character should exist");
        assert_eq!(character.0, id);

        // Update
        db.update_character("Hero", "Epic Quest", r#"{"level": 50}"#)
            .expect("Failed to update");

        // Verify update
        let updated = db.get_character("Hero", "Epic Quest")
            .unwrap()
            .unwrap();
        assert_eq!(updated.3, Some(r#"{"level": 50}"#.to_string()));

        // Delete
        db.delete_character("Hero", "Epic Quest")
            .expect("Failed to delete");

        // Verify deletion
        let deleted = db.get_character("Hero", "Epic Quest").unwrap();
        assert!(deleted.is_none());
    }

    #[test]
    fn test_inventory_management() {
        // Test a realistic inventory scenario
        let db = setup_test_db();

        // Create character
        db.insert_character("Adventurer", "Dungeon Crawler", None).unwrap();

        // Create various items
        let sword_id = db.insert_object("Iron Sword", "weapon", Some(r#"{"damage": 15}"#)).unwrap();
        let potion_id = db.insert_object("Health Potion", "consumable", Some(r#"{"heal": 50}"#)).unwrap();
        let gold_id = db.insert_object("Gold Coins", "currency", None).unwrap();

        // Add items to inventory
        db.add_object_to_character("Dungeon Crawler", "Adventurer", sword_id, 1).unwrap();
        db.add_object_to_character("Dungeon Crawler", "Adventurer", potion_id, 5).unwrap();
        db.add_object_to_character("Dungeon Crawler", "Adventurer", gold_id, 100).unwrap();

        // Check inventory
        let inventory = db.get_character_objects("Dungeon Crawler", "Adventurer").unwrap();
        assert_eq!(inventory.len(), 3, "Should have 3 different item types");

        // Use potions (decrease quantity)
        db.update_object_quantity("Dungeon Crawler", "Adventurer", potion_id, 3).unwrap();

        // Verify potion quantity
        let updated_inventory = db.get_character_objects("Dungeon Crawler", "Adventurer").unwrap();
        let potion_entry = updated_inventory.iter()
            .find(|(id, _, _, _, _)| *id == potion_id)
            .expect("Potion should exist");
        assert_eq!(potion_entry.3, 3);

        // Sell sword (remove from inventory)
        db.remove_object_from_character("Dungeon Crawler", "Adventurer", sword_id).unwrap();

        // Verify sword is gone
        let final_inventory = db.get_character_objects("Dungeon Crawler", "Adventurer").unwrap();
        assert_eq!(final_inventory.len(), 2, "Should have 2 items after selling sword");
    }

    #[test]
    fn test_foreign_key_cascade_delete() {
        // Test that deleting a character cascades to character_objects
        let db = setup_test_db();

        // Create a character
        db.insert_character("TestChar", "TestGame", None).unwrap();

        // Create an object
        let obj_id = db.insert_object("Sword", "weapon", None).unwrap();

        // Add object to character
        db.add_object_to_character("TestGame", "TestChar", obj_id, 1).unwrap();

        // Verify the object exists in character_objects
        let objects_before = db.get_character_objects("TestGame", "TestChar").unwrap();
        assert_eq!(objects_before.len(), 1, "Should have 1 object before delete");

        // Delete the character - should cascade and delete character_objects
        db.delete_character("TestChar", "TestGame").unwrap();

        // Verify the character_objects record was deleted via cascade
        let objects_after = db.get_character_objects("TestGame", "TestChar").unwrap();
        assert_eq!(objects_after.len(), 0, "Character objects should be cascade deleted");
    }
}
