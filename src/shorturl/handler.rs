use actix_web::{HttpResponse, web::Bytes};
use askama::Template;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use std::{env, fs, fs::File, io::Write, thread, path::Path, result::Result};
use futures::StreamExt;
use actix_multipart::Multipart;
use uuid::Uuid;
use crate::{encode::encode::unique_code, frontend::template::UploadTemplate, database::insert};

pub async fn handle_csv_upload(payload: &mut Multipart) -> HttpResponse {
    // Get the upload folder
    let upload_dir = env::current_dir().unwrap().join("uploads/csv");

    // Create the upload folder if it doesn't exist
    if !upload_dir.exists() {
        std::fs::create_dir_all(&upload_dir).unwrap();
    }

    // Generate a unique identifier (UUID) before each loop
    let id = Uuid::new_v4();
    // Declare variables before entering the loop
    let mut filename_with_id = String::new();
    let mut file_path = String::new();
    // Iterate over each part of the multipart payload
    while let Some(item) = payload.next().await {
        match item {
            Ok(mut field) => {
                // Extract filename from the content disposition header
                let filename = field.content_disposition().get_filename().unwrap_or("unknown");

                // Check if a file is uploaded
                if filename.len() <= 0 {
                    return HttpResponse::BadRequest().body("No file uploaded");
                }

                // Construct the filename with UUID
                filename_with_id = format!("{}_{}", id, filename);
                file_path = format!("uploads/csv/{}", filename_with_id);

                // Upload the file to the "uploads/csv/" folder
                let mut file = File::create(file_path.clone()).unwrap();

                // Copy the content of the field to the file
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    file.write_all(&data).unwrap();
                }
            }
            // If an error occurs, return Internal Server Error (500)
            Err(e) => {
                return HttpResponse::InternalServerError().body(format!("Error: {:?}", e));
            }
        }
    }

    // Open the CSV file in read-only mode
    let file_csv = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error opening CSV file: {:?}", e));
        }
    };

    // Clone the CSV file for reading and modifying separately
    let cloned_file = match file_csv.try_clone() {
        Ok(file) => file,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error cloning CSV file: {:?}", e));
        }
    };

    // Use the CSV crate to read the records from the cloned CSV file
    let mut rdr = ReaderBuilder::new().from_reader(cloned_file);
    let mut records = rdr.records().collect::<Result<Vec<_>, _>>().unwrap();

    // Add a new value in the "short url" column of each record
    for record in records.iter_mut() {
        // Generate a unique code
        let code = unique_code(7);
        let short_url_value = format!("https://dps.re/{}", code);

        // Add the short URL to the specified column
        record.push_field(&short_url_value);
    }

    // Add a new line at the beginning of the CSV file
    let mut header = StringRecord::new();
    header.push_field("Link");
    header.push_field("Short-Url");

    records.insert(0, header);

    // Write the modified records back to the CSV file
    let mut wtr = WriterBuilder::new().from_path(&file_path).unwrap();
    for record in records {
        wtr.write_record(&record).unwrap();
    }
    wtr.flush().unwrap();

    // Clone the file path for use in the thread
    let load_csv_data_path = file_path.clone();

    // Read the content of the modified CSV file into bytes
    let file_content = match fs::read::<&Path>(file_path.as_ref()) {
        Ok(content) => content,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error converting CSV file content to bytes: {:?}", e));
        }
    };

    // Launch the load_csv_data function in a new thread
    let _load_csv_data_handle = thread::spawn(move || {
        let _ = insert::load_csv_data(Path::new(&load_csv_data_path));
    });

    // Return an OK response with the modified CSV file as a downloadable attachment
    return HttpResponse::Ok()
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename_with_id)))
        .content_type("text/csv")
        .body(Bytes::from(file_content))
}

pub async fn handle_upload_asset(payload: &mut Multipart) -> HttpResponse {
    // Get the upload folder
    let upload_dir = env::current_dir().unwrap().join("uploads/img");

    // Create the upload folder if doesnt exist
    if !upload_dir.exists() {
        std::fs::create_dir(&upload_dir).unwrap();
    }

    // Generate an Uuid before each loop
    let id = Uuid::new_v4();

    // Declare the variable before entering the loop
    let mut file_path = String::new();

    while let Some(item) = payload.next().await {
        match item {
            Ok(mut field) => {
                let content_disposition = field.content_disposition();
                let filename = content_disposition.get_filename().unwrap_or("unknown");

                //Check if file is uploaded
                if filename.len() <= 0 {
                    return HttpResponse::BadRequest().body("No file uploaded");
                }

                 // Add id with the file name original
                let filename_with_id = format!("{}_{}", id, filename);

                file_path = format!("uploads/img/{}", filename_with_id);
                //Upload the file into folder uploads/
                let mut file = File::create(file_path.clone()).unwrap();

                // Copy the content of the field to the file
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    file.write_all(&data).unwrap();
                }
            }
            //If a error return error 500
            Err(e) => {
                return HttpResponse::InternalServerError().body(format!("Error: {:?}", e));
            }
        }
    }
    // Use the template for render
    let template = UploadTemplate { image: file_path };
    let response_body = template.render().unwrap();
    return HttpResponse::Ok().body(response_body)
}