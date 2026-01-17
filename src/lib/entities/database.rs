//! Database management module for persistent game data storage.
//!
//! This module handles SQLite database initialization, table creation,
//! and provides constructors for both shared and user-specific databases.

use rusqlite::{Connection, Result};
use std::path::{self, Path};

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
    /// let db = Database::new("data/my_game.db").expect("Failed to create database");
    /// ```
    pub fn new(db_path: &str) -> Result<Self> {
        // Check if database file already exists
        let db_exists = Path::new(db_path).exists();
        let conn = Connection::open(db_path)?;

        if !db_exists {
            println!("Creating new Database! At {}", db_path);
            Self::create_tables(&conn)?;
        } else {
            println!("opening existing databse databse at {}", db_path);
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
    /// // Creates database at "saves/Veteran_Investigator.db"
    /// let db = Database::_new_fn_name("saves/", "Veteran Investigator")
    ///     .expect("Failed to create character database");
    /// ```
    ///
    /// # Note
    ///
    /// The underscore prefix should be removed once a proper function name is chosen.
    /// Consider renaming to `new_for_user` or `new_with_name`.
    pub fn _new_fn_name(db_path: &str, name: &str) -> Result<Self> {
        let full_name_string_path: String = Database::name_combiner(db_path, name);

        let full_name_string_path_exists: bool = Path::new(&full_name_string_path).exists();
        let conn = Connection::open(full_name_string_path)?;


        if !full_name_string_path_exists {
            println!("Creating new Database! At {}", db_path);
            Self::create_tables(&conn)?;
        } else {
            println!("opening existing databse databse at {}", db_path);
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
    /// let path = Database::name_combiner("data/saves/", "My Character.db");
    /// assert_eq!(path, "data/saves/My_Character.db");
    /// ```
    pub fn name_combiner(word1: &str, word2: &str) -> String {
        let parts = [word1, word2];

        let combined: String = parts.iter()
            .flat_map(|s| s.chars())
            .map(|c| if c == ' ' { '_' } else { c })
            .collect::<String>();
        
        combined
    }

    /// Initializes database tables for a new database.
    ///
    /// This private method is called when a new database is created. It executes
    /// SQL statements to create the necessary table schema.
    ///
    /// Currently creates:
    /// - `test` table: A basic test table with id (primary key) and name fields
    ///
    /// # Arguments
    ///
    /// * `conn` - Reference to the SQLite connection where tables should be created
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if table creation fails.
    ///
    /// # Note
    ///
    /// This is currently a placeholder with only a test table. In production,
    /// this should create tables for characters, inventory, game state, etc.
    fn create_tables(conn: &Connection) -> Result<()> {
        conn.execute("CREATE TABLE test(id INTEGER PRIMARY KEY, name TEXT NOT NULL)", [],)?;
        println!("Tables created!");
        Ok(())
    }
}
