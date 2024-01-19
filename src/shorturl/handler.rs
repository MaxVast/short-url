use actix_web::HttpResponse;
use askama::Template;
use csv::ReaderBuilder;
use csv::WriterBuilder;
use std::fs;
use std::{env, fs::{File}, io::{Write}};
use futures::StreamExt;
use actix_multipart::Multipart;
use uuid::Uuid;
use std::result::Result;
use crate::encode::encode::unique_code;
use crate::frontend::template::UploadTemplate;

pub async fn handle_csv_upload(payload: &mut Multipart) -> HttpResponse {
    // Get the upload folder
    let upload_dir = env::current_dir().unwrap().join("uploads/csv");

    // Create the upload folder if doesnt exist
    if !upload_dir.exists() {
        std::fs::create_dir_all(&upload_dir).unwrap();
    }

    // Generate an Uuid before each loop
    let id = Uuid::new_v4();

    // Declare the variable before entering the loop
    let mut filename_with_id = String::new();
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
                filename_with_id = format!("{}_{}", id, filename);

                file_path = format!("uploads/csv/{}", filename_with_id);
                //Upload the file into folder uploads/csv/
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
    // Open file CSV on Read only
    let file_csv = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error opening CSV file : {:?}", e));
        }
    };
    // Cloning file CSV 
    let cloned_file = match file_csv.try_clone(){
        Ok(file) => file,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error cloning CSV file : {:?}", e));
        }
    };

    let mut rdr = ReaderBuilder::new().from_reader(cloned_file);
    let mut records = rdr.records().collect::<Result<Vec<_>, _>>().unwrap();

    // Add a new value in the second column
    for record in records.iter_mut() {
        // Generate unique code
        let code = unique_code(7);
        let short_url = format!("https://dps.re/{}",code);
        // Add short url 
        record.push_field(&short_url);
    }

    // Write the modified records back to the CSV file
    let mut wtr = WriterBuilder::new().from_path(file_path).unwrap();
    for record in records {
        wtr.write_record(&record).unwrap();
    }
    wtr.flush().unwrap();
    
    file_path = format!("uploads/csv/{}", filename_with_id);

    // Convert file content to bytes
    let file_content = match fs::read(file_path) {
        Ok(content) => content,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error content to bytes CSV file : {:?}", e));
        }
    };
    file_path = format!("uploads/csv/{}", filename_with_id);
    
    if let Err(_) = fs::remove_file(&file_path) {
        return HttpResponse::InternalServerError().body(format!("Error delete CSV file"));
    }

    return HttpResponse::Ok()
        .header("Content-Disposition",format!("attachment; filename=\"{}\"", filename_with_id))
        .content_type("text/csv")
        .body(actix_web::web::Bytes::from(file_content))
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