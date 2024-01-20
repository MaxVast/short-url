use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

// Function to establish a connection to the PostgreSQL database
pub fn establish_connection() -> PgConnection {
    // Load environment variables from the .env file
    dotenv().ok();

    // Retrieve the DATABASE_URL from the environment variables
    let uri = match env::var("DATABASE_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable DATABASE_URL"),
    };

    // Establish a connection to the PostgreSQL database
    PgConnection::establish(&uri)
        .expect(&format!("Error connecting to {}", uri))
}

// Async function to initialize the database
pub async fn init() {
    // Obtain a mutable reference to the database connection
    let connection = &mut establish_connection();

    // Execute a SQL query to create the short_urls table if it does not exist
    diesel::sql_query("CREATE TABLE IF NOT EXISTS short_urls (
        id TEXT NOT NULL PRIMARY KEY,
        url TEXT NOT NULL,
        short_url TEXT NOT NULL
    )")
    .execute(connection)
    .expect("Error creating short_urls table");

    // Print a message indicating successful connection to the database
    println!("✅ Connected to database and table created !");
}
