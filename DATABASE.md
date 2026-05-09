# Database Documentation

This document describes the SQLite database structure, database types, and available methods for the ttdigirpg project.

## Database Types

The project supports two types of database creation:

### 1. Fixed-Path Database
Created via `Database::new(db_path: &str)`

**Use case**: Shared databases, API databases, or fixed-location game data.

**Example**:
```rust
let db = Database::new("src/database/game_data.db")?;
let api_db = Database::new("src/database/test_api.db")?;
```

### 2. User/Character-Specific Database
Created via `Database::new_with_name(db_path: &str, name: &str)`

**Use case**: Player-specific save files or character-specific data isolation.

**Behavior**:
- Combines base path with character/user name
- Sanitizes the name (replaces spaces with underscores)
- Automatically appends `.db` extension if not present

**Example**:
```rust
// Creates: data/saves/Veteran_Investigator.db
let db = Database::new_with_name("data/saves/", "Veteran Investigator")?;
```

## Database Schema

All databases are created with three interconnected tables and foreign key constraints enabled.

### Table: `characters`

Stores character data with game context and flexible JSON properties.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `uuid` | TEXT | NOT NULL, UNIQUE | UUID v4 identifier for the character |
| `name` | TEXT | NOT NULL | Character's name |
| `game` | TEXT | NOT NULL | Game/campaign context |
| `data` | TEXT | NULL | Optional JSON string for character stats, attributes, etc. |
| **PRIMARY KEY** | - | (name, game) | Composite primary key - unique character per game |

**Key Design Decisions**:
- Composite primary key allows same character name across different games
- UUID provides globally unique identifier
- Flexible JSON `data` field stores arbitrary character information

### Table: `objects`

Defines object templates/definitions (items, buildings, organizations, etc.)

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | INTEGER | PRIMARY KEY AUTOINCREMENT | Auto-incrementing object ID |
| `name` | TEXT | NOT NULL | Object name (e.g., "Sword", "Car Factory") |
| `type` | TEXT | NOT NULL | Object type (e.g., "weapon", "building", "item") |
| `properties` | TEXT | NULL | Optional JSON string for object properties |

**Key Design Decisions**:
- Objects are templates/definitions, not instances
- Flexible JSON `properties` field stores type-specific data
- Type categorization allows filtering and grouping

### Table: `character_objects`

Junction table tracking character ownership/associations with objects.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | INTEGER | PRIMARY KEY AUTOINCREMENT | Auto-incrementing association ID |
| `game` | TEXT | NOT NULL | Game context (must match character's game) |
| `character_name` | TEXT | NOT NULL | Character name (foreign key) |
| `object_id` | INTEGER | NOT NULL | Object ID (foreign key) |
| `quantity` | INTEGER | DEFAULT 1 | How many of this object the character has |
| **FOREIGN KEY** | - | (character_name, game) → characters(name, game) | References character with CASCADE DELETE |
| **FOREIGN KEY** | - | (object_id) → objects(id) | References object with CASCADE DELETE |

**Key Design Decisions**:
- Many-to-many relationship between characters and objects
- Quantity field supports stackable items
- Cascade delete ensures cleanup when characters or objects are removed
- Game context ensures associations are scoped properly

## Database Methods

All methods return `Result<T, rusqlite::Error>` for error handling.

### Database Initialization

#### `Database::new(db_path: &str) -> Result<Database>`

Creates or opens a database at the specified path.

**Behavior**:
1. Checks if database file exists
2. If not, creates new database and calls `create_tables()`
3. If exists, opens existing database
4. Enables foreign key constraints via `PRAGMA foreign_keys = ON`

**Returns**: Initialized `Database` struct wrapping the connection.

#### `Database::new_with_name(db_path: &str, name: &str) -> Result<Self>`

Creates or opens a user/character-specific database.

**Behavior**:
1. Calls `name_combiner()` to generate unique path
2. Same initialization logic as `new()`

**Returns**: Initialized `Database` struct.

#### `Database::name_combiner(word1: &str, word2: &str) -> String`

Utility function for path generation.

**Behavior**:
1. Concatenates both strings
2. Replaces spaces with underscores
3. Appends `.db` if not already present

**Example**:
```rust
// Returns: "data/saves/My_Character.db"
Database::name_combiner("data/saves/", "My Character");
```

---

## Character Methods

### Insert

#### `insert_character(&self, name: &str, game: &str, data: Option<&str>) -> Result<String>`

Inserts a new character into the database.

**Parameters**:
- `name`: Character's name
- `game`: Game/campaign identifier
- `data`: Optional JSON string with character stats/attributes

**Returns**: UUID string of the newly created character.

**SQL Executed**:
```sql
INSERT INTO characters (uuid, name, game, data) VALUES (?1, ?2, ?3, ?4)
```

**Example**:
```rust
let uuid = db.insert_character(
    "Thorin Oakenshield",
    "Middle Earth Campaign",
    Some(r#"{"level": 5, "class": "warrior"}"#)
)?;
```

**Notes**:
- Generates UUID v4 automatically
- Fails if character with same (name, game) already exists (UNIQUE constraint)

### Read

#### `get_character(&self, name: &str, game: &str) -> Result<Option<(String, String, String, Option<String>)>>`

Retrieves a character from the database.

**Parameters**:
- `name`: Character's name
- `game`: Game identifier

**Returns**: `Option` containing tuple `(uuid, name, game, data)` if found, `None` if not found.

**SQL Executed**:
```sql
SELECT uuid, name, game, data FROM characters WHERE name = ?1 AND game = ?2
```

**Example**:
```rust
if let Some((uuid, name, game, data)) = db.get_character("Thorin Oakenshield", "Middle Earth Campaign")? {
    println!("Found character: {} (UUID: {})", name, uuid);
}
```

### Update

#### `update_character(&self, name: &str, game: &str, data: &str) -> Result<usize>`

Updates a character's data field.

**Parameters**:
- `name`: Character's name
- `game`: Game identifier
- `data`: New JSON string (replaces existing data completely)

**Returns**: Number of rows updated (1 if successful, 0 if character not found).

**SQL Executed**:
```sql
UPDATE characters SET data = ?1 WHERE name = ?2 AND game = ?3
```

**Example**:
```rust
let rows_updated = db.update_character(
    "Thorin Oakenshield",
    "Middle Earth Campaign",
    r#"{"level": 10, "class": "warrior", "hp": 150}"#
)?;
```

### Delete

#### `delete_character(&self, name: &str, game: &str) -> Result<usize>`

Deletes a character from the database.

**Parameters**:
- `name`: Character's name
- `game`: Game identifier

**Returns**: Number of rows deleted (1 if successful, 0 if character not found).

**SQL Executed**:
```sql
DELETE FROM characters WHERE name = ?1 AND game = ?2
```

**Cascade Behavior**:
- Automatically deletes all entries in `character_objects` for this character (foreign key cascade).

---

## Object Methods

### Insert

#### `insert_object(&self, name: &str, obj_type: &str, properties: Option<&str>) -> Result<i64>`

Inserts a new object definition into the database.

**Parameters**:
- `name`: Object name (e.g., "Iron Sword", "Car Factory")
- `obj_type`: Object type/category (e.g., "weapon", "building", "consumable")
- `properties`: Optional JSON string with object-specific properties

**Returns**: Auto-generated object ID.

**SQL Executed**:
```sql
INSERT INTO objects (name, type, properties) VALUES (?1, ?2, ?3)
```

**Example**:
```rust
let sword_id = db.insert_object(
    "Iron Sword",
    "weapon",
    Some(r#"{"damage": 15, "durability": 100}"#)
)?;
```

### Read

#### `get_object(&self, object_id: i64) -> Result<Option<(i64, String, String, Option<String>)>>`

Retrieves an object definition by ID.

**Parameters**:
- `object_id`: The object's ID

**Returns**: `Option` containing tuple `(id, name, type, properties)` if found, `None` if not found.

**SQL Executed**:
```sql
SELECT id, name, type, properties FROM objects WHERE id = ?1
```

### Update

#### `update_object(&self, object_id: i64, properties: &str) -> Result<usize>`

Updates an object's properties.

**Parameters**:
- `object_id`: The object's ID
- `properties`: New JSON string (replaces existing properties completely)

**Returns**: Number of rows updated (1 if successful, 0 if object not found).

**SQL Executed**:
```sql
UPDATE objects SET properties = ?1 WHERE id = ?2
```

### Delete

#### `delete_object(&self, object_id: i64) -> Result<usize>`

Deletes an object definition from the database.

**Parameters**:
- `object_id`: The object's ID

**Returns**: Number of rows deleted (1 if successful, 0 if object not found).

**SQL Executed**:
```sql
DELETE FROM objects WHERE id = ?1
```

**Cascade Behavior**:
- Automatically deletes all entries in `character_objects` referencing this object.

---

## Character-Object Association Methods

These methods manage the many-to-many relationship between characters and objects (inventory, ownership, etc.)

### Add Association

#### `add_object_to_character(&self, game: &str, character_name: &str, object_id: i64, quantity: i32) -> Result<i64>`

Adds an object to a character's inventory/associations.

**Parameters**:
- `game`: Game context
- `character_name`: Character's name
- `object_id`: ID of the object to add
- `quantity`: How many to add (e.g., 1 sword, 50 gold coins)

**Returns**: Auto-generated association ID.

**SQL Executed**:
```sql
INSERT INTO character_objects (game, character_name, object_id, quantity)
VALUES (?1, ?2, ?3, ?4)
```

**Example**:
```rust
// Give character 5 health potions
db.add_object_to_character("Dungeon Crawler", "Adventurer", potion_id, 5)?;
```

### Query Associations

#### `get_character_objects(&self, game: &str, character_name: &str) -> Result<Vec<(i64, String, String, i32, Option<String>)>>`

Retrieves all objects owned by a character with a JOIN query.

**Parameters**:
- `game`: Game context
- `character_name`: Character's name

**Returns**: Vector of tuples containing `(object_id, name, type, quantity, properties)`.

**SQL Executed**:
```sql
SELECT o.id, o.name, o.type, co.quantity, o.properties
FROM character_objects co
JOIN objects o ON co.object_id = o.id
WHERE co.game = ?1 AND co.character_name = ?2
```

**Example**:
```rust
let inventory = db.get_character_objects("Dungeon Crawler", "Adventurer")?;
for (id, name, obj_type, quantity, props) in inventory {
    println!("{} x{} ({})", name, quantity, obj_type);
}
```

### Update Association

#### `update_object_quantity(&self, game: &str, character_name: &str, object_id: i64, quantity: i32) -> Result<usize>`

Updates the quantity of an object in a character's inventory.

**Parameters**:
- `game`: Game context
- `character_name`: Character's name
- `object_id`: Object ID
- `quantity`: New quantity value

**Returns**: Number of rows updated (1 if successful, 0 if association not found).

**SQL Executed**:
```sql
UPDATE character_objects SET quantity = ?1
WHERE game = ?2 AND character_name = ?3 AND object_id = ?4
```

**Example**:
```rust
// Character used 2 potions, update from 5 to 3
db.update_object_quantity("Dungeon Crawler", "Adventurer", potion_id, 3)?;
```

### Remove Association

#### `remove_object_from_character(&self, game: &str, character_name: &str, object_id: i64) -> Result<usize>`

Removes an object from a character's inventory/associations.

**Parameters**:
- `game`: Game context
- `character_name`: Character's name
- `object_id`: Object ID to remove

**Returns**: Number of rows deleted (1+ if successful, 0 if association not found).

**SQL Executed**:
```sql
DELETE FROM character_objects
WHERE game = ?1 AND character_name = ?2 AND object_id = ?3
```

**Example**:
```rust
// Character sells their sword
db.remove_object_from_character("Dungeon Crawler", "Adventurer", sword_id)?;
```

---

## Data Flow Examples

### Creating a Character with Inventory

```rust
// 1. Initialize database
let db = Database::new("src/database/game_data.db")?;

// 2. Create character
let uuid = db.insert_character(
    "Gandalf",
    "Lord of the Rings",
    Some(r#"{"class": "wizard", "level": 20}"#)
)?;

// 3. Define objects
let staff_id = db.insert_object("Staff of Power", "weapon", Some(r#"{"magic": 100}"#))?;
let potion_id = db.insert_object("Mana Potion", "consumable", Some(r#"{"restore": 50}"#))?;

// 4. Add objects to character
db.add_object_to_character("Lord of the Rings", "Gandalf", staff_id, 1)?;
db.add_object_to_character("Lord of the Rings", "Gandalf", potion_id, 10)?;

// 5. Query inventory
let inventory = db.get_character_objects("Lord of the Rings", "Gandalf")?;
```

### Updating Character Progress

```rust
// 1. Get current character data
if let Some((uuid, name, game, data)) = db.get_character("Gandalf", "Lord of the Rings")? {
    // 2. Parse existing JSON, modify, and update
    let updated_data = r#"{"class": "wizard", "level": 21, "exp": 15000}"#;
    db.update_character(&name, &game, updated_data)?;

    // 3. Use a potion (decrease quantity)
    db.update_object_quantity(&game, &name, potion_id, 9)?;
}
```

### Foreign Key Cascade Example

```rust
// When you delete a character...
db.delete_character("Gandalf", "Lord of the Rings")?;

// All associated character_objects entries are automatically deleted!
// No orphaned inventory records remain.
```

---

## Important Notes

### Foreign Key Constraints

The database uses `PRAGMA foreign_keys = ON` to enforce referential integrity:

1. **Deleting a character** automatically deletes all their `character_objects` entries
2. **Deleting an object** automatically deletes all `character_objects` references to it
3. Cannot create `character_objects` entries for non-existent characters or objects

### JSON Data Storage

Both `characters.data` and `objects.properties` store JSON as TEXT:
- Flexible schema-less data storage
- Application is responsible for JSON parsing/validation
- Allows different character types with different stat structures

### Composite Primary Key (characters)

The `(name, game)` composite key means:
- Same character name can exist in different games: ✅
- Duplicate character in same game: ❌

### Error Handling

All methods return `Result<T, rusqlite::Error>`. Common errors:
- **UNIQUE constraint violation**: Duplicate (name, game) in characters
- **FOREIGN KEY constraint violation**: Invalid character_name, game, or object_id
- **Connection errors**: Database file permissions, disk space

### In-Memory Databases

For testing, use `:memory:` as the database path:
```rust
let test_db = Database::new(":memory:")?; // Temporary, destroyed after use
```

---

## Current Usage in Project

### API Handlers (src/lib/api/handlers.rs)

The API currently uses:
- `Database::new("src/database/test_api.db")` for test endpoints
- `insert_character()` in the `add_character` handler

### Demo (src/demo.rs)

The demo showcases:
- Database initialization
- Character creation
- Basic CRUD operations

### Tests (src/lib/entities/database.rs)

Comprehensive test suite covering:
- All CRUD operations
- Foreign key cascade behavior
- Composite key constraints
- Full lifecycle scenarios
