use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct GroceryItem {
    id: i32,
    name: String,
    price: f64,
    category: String,
}

fn main() -> Result<()> {
    let conn = Connection::open("grocery.db")?;

    // Step 1: Create tables if they do not exist
    setup_database(&conn)?;

    // Step 2: Insert some categories and grocery items
    let mut grocery_items = HashMap::new();
    grocery_items.insert(
        String::from("Fruits"),
        vec![("Apple", 1.2), ("Banana", 0.5)],
    );
    grocery_items.insert(
        String::from("Vegetables"),
        vec![("Carrot", 0.8), ("Lettuce", 1.0)],
    );
    insert_data(&conn, &grocery_items)?;

    // Step 3: Read and display all grocery items
    println!("All grocery items:");
    let items = read_items(&conn)?;
    for item in items {
        println!("{:?}", item);
    }

    // Step 4: Update a grocery item
    update_item_price(&conn, "Apple", 1.5)?;

    // Step 5: Delete a grocery item
    delete_item(&conn, "Banana")?;

    // Step 6: Display all grocery items after update and delete
    println!("\nGrocery items after update and delete:");
    let items = read_items(&conn)?;
    for item in items {
        println!("{:?}", item);
    }

    Ok(())
}

// Function to set up the database tables
fn setup_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS grocery_items (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price REAL NOT NULL,
            category_id INTEGER NOT NULL REFERENCES categories(id)
        )",
        [],
    )?;
    println!("Tables created successfully.");
    Ok(())
}

// Function to insert categories and grocery items into the database
fn insert_data(conn: &Connection, grocery_items: &HashMap<String, Vec<(&str, f64)>>) -> Result<()> {
    for (category, items) in grocery_items {
        conn.execute(
            "INSERT OR IGNORE INTO categories (name) VALUES (?1)",
            [&category.to_string()],
        )?;
        let category_id: i64 = conn.last_insert_rowid();

        for (name, price) in items {
            conn.execute(
                "INSERT INTO grocery_items (name, price, category_id) VALUES (?1, ?2, ?3)",
                params![name, price, category_id],
            )?;
        }
    }
    println!("Data inserted successfully.");
    Ok(())
}

// Function to read and retrieve all grocery items with their categories
fn read_items(conn: &Connection) -> Result<Vec<GroceryItem>> {
    let mut stmt = conn.prepare(
        "SELECT gi.id, gi.name, gi.price, c.name as category
         FROM grocery_items gi
         INNER JOIN categories c ON gi.category_id = c.id",
    )?;

    let items = stmt
        .query_map([], |row| {
            Ok(GroceryItem {
                id: row.get(0)?,
                name: row.get(1)?,
                price: row.get(2)?,
                category: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(items)
}

// Function to update the price of a grocery item
fn update_item_price(conn: &Connection, item_name: &str, new_price: f64) -> Result<()> {
    let rows_updated = conn.execute(
        "UPDATE grocery_items SET price = ?1 WHERE name = ?2",
        params![new_price, item_name],
    )?;
    if rows_updated == 0 {
        println!("No item found with the name '{}'.", item_name);
    } else {
        println!("Updated price for item '{}'.", item_name);
    }
    Ok(())
}

// Function to delete a grocery item by name
fn delete_item(conn: &Connection, item_name: &str) -> Result<()> {
    let rows_deleted = conn.execute(
        "DELETE FROM grocery_items WHERE name = ?1",
        params![item_name],
    )?;
    if rows_deleted == 0 {
        println!("No item found with the name '{}'.", item_name);
    } else {
        println!("Deleted item '{}'.", item_name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_connection() {
        // This test simply verifies if we can open a connection to the database.
        let conn = Connection::open_in_memory().expect("Failed to open database connection");
        assert!(conn.is_autocommit());
    }

    #[test]
    fn test_insert_and_read_data() {
        // Testing if data can be inserted and then read.
        let conn = Connection::open_in_memory().expect("Failed to open database connection");
        
        setup_database(&conn).expect("Failed to set up database");

        let mut grocery_items = HashMap::new();
        grocery_items.insert(String::from("Fruits"), vec![("Apple", 1.2)]);
        insert_data(&conn, &grocery_items).expect("Failed to insert data");

        let items = read_items(&conn).expect("Failed to read items");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Apple");
    }
}