use sqlx::{sqlite::SqlitePool, Error};
use std::fs;
use std::path::{Path, PathBuf};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    // Specify the absolute path to the SQLite database
    let db_file_path = "C:/Users/musta/RustroverProjects/rest-api-rs/rest_api_rust.db";

    // Ensure the directory exists
    let dir = Path::new(db_file_path).parent().unwrap();
    if !dir.exists() {
        println!("Directory does not exist. Creating it now...");
        fs::create_dir_all(dir).unwrap();
    }

    // Check if the database file exists
    if !Path::new(db_file_path).exists() {
        println!("Database file does not exist. SQLite will create it.");
    }

    // Construct the correct URL format for SQLite connection
    let db_url = format!("sqlite://{}", db_file_path);  // Corrected URL format
    println!("Connecting to database at: {}", db_url);

    // Try connecting to the database
    let pool = SqlitePool::connect(&db_url).await.map_err(|e| {
        eprintln!("Failed to connect to the database at {}: {}", db_url, e);
        e
    })?;

    // Create the table if it doesn't exist
    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);")
        .execute(&pool)
        .await?;

    println!("Database and table created (if not already existing).");

    Ok(())
}
