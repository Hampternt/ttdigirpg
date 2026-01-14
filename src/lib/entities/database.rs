use rusqlite::{Connection, Result};
use std::path::Path;

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

    fn create_tables(conn: &Connection) -> Result<()> {
        conn.execute("CREATE TABLE test(id INTEGER PRIMARY KEY, name TEXT NOT NULL)", [],)?;
        println!("Tables created!");
        Ok(())
    }
}
