use rusqlite::{Connection, Result};
use std::path::{self, Path};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        // crates a new database if the database does not exist?
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

    pub fn name_combiner(word1: &str, word2: &str) -> String {
        let parts = [word1, word2];

        let combined: String = parts.iter()
            .flat_map(|s| s.chars())
            .map(|c| if c == ' ' { '_' } else { c })
            .collect::<String>();
        
        combined
    }

    fn create_tables(conn: &Connection) -> Result<()> {
        conn.execute("CREATE TABLE test(id INTEGER PRIMARY KEY, name TEXT NOT NULL)", [],)?;
        println!("Tables created!");
        Ok(())
    }
}
