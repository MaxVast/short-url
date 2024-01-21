use diesel::prelude::*;
use diesel::dsl::insert_into;
use std::fs::remove_file;
use std::error::Error;
use std::path::Path;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use csv::ReaderBuilder;
use uuid::Uuid;
use indicatif::{ProgressBar, ProgressStyle};
use crate::database::schema::short_urls;
use crate::database::database;

const BATCH_SIZE: usize = 1500;

// Function to load CSV data into the database
pub fn load_csv_data(file_path: &Path) -> Result<(), Box<dyn Error>> {
    // Read CSV records from the file
    let records = ReaderBuilder::new().from_path(file_path)?.records().collect::<Result<Vec<_>, _>>()?;

    // Set up progress bar
    let total_records = records.len() as f64;
    let progress = Arc::new(Mutex::new(ProgressBar::new(total_records as u64)));
    progress.lock().unwrap().set_style(ProgressStyle::default_bar().template("{wide_bar} {percent}%")?);

    // Establish a database connection
    let connection = &mut database::establish_connection();
    println!("Data CSV insert in database");

    // Database transaction
    connection.transaction::<_, Box<dyn Error>, _>(|connection| {
        // Record start time for measuring elapsed time
        let start_time = Instant::now();

        // Process records in batches
        for (index, chunk) in records.chunks(BATCH_SIZE).enumerate() {
            // Map CSV records to database values
            let values: Vec<_> = chunk.iter().map(|record| (
                short_urls::id.eq(Uuid::new_v4().to_string()),
                short_urls::url.eq(record[0].to_string()),
                short_urls::short_url.eq(record[1].to_string()),
            )).collect();

            // Insert values into the database
            insert_into(short_urls::table)
                .values(&values)
                .execute(connection)?;

            // Update global progress for the progress bar
            let global_progress = ((index + 1) as f64 / (total_records / BATCH_SIZE as f64)) * 100.0;
            progress.lock().unwrap().set_position(global_progress as u64);
        }

        // Calculate and print elapsed time
        let elapsed_time = start_time.elapsed();
        progress.lock().unwrap().finish();
        println!("✅ Successful insertion of CSV data into the database in: {:?} ✅", elapsed_time);

        

        Ok(()) // Return Ok if the transaction is successful
    })?;
    // Remove the CSV file after successful insertion
    remove_file(file_path)?;
    println!("✅ File CSV Delete");
    
    Ok(()) // Return Ok if the overall function execution is successful
}
