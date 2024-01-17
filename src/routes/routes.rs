use actix_web::{get, post, web, HttpResponse};
use actix_files::Files;
use askama::Template;
use std::env;
use futures::StreamExt;
use std::fs::File;
use std::io::Write;
use actix_multipart::Multipart;
use uuid::Uuid;
use crate::encode::encode::unique_code;

#[get("/")]
async fn root() -> HttpResponse {
    let template = HomeTemplate {};
    let response_body = template.render().unwrap();
    HttpResponse::Ok().body(response_body)
}

#[get("/rand")]
async fn random() -> HttpResponse {
    let code = unique_code(7);
    HttpResponse::Ok().body(format!("code random : {}", code))
}

#[post("/import")]
async fn import(mut payload: Multipart) -> HttpResponse {
    // Get the upload folder
    let upload_dir = env::current_dir().unwrap().join("uploads");

    // Create the upload folder if doesnt exist
    if !upload_dir.exists() {
        std::fs::create_dir(&upload_dir).unwrap();
    }

    // Generate an Uuid before each loop
    let id = Uuid::new_v4();

    // Declare the variable before entering the loop
    let mut filename_with_id = String::new();

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

                let file_path = format!("uploads/{}", filename_with_id);
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
                HttpResponse::InternalServerError().body(format!("Error: {:?}", e));
            }
        }
    }
    // Render the full path of the file
    let image_path = format!("uploads/{}", filename_with_id);

    // Use the template for render
    let template = UploadTemplate { image: image_path };
    let response_body = template.render().unwrap();
    HttpResponse::Ok().body(response_body)
}

// fallback route
async fn handler_404() -> HttpResponse {
    HttpResponse::NotFound().body("404 : Nothing here..")
}


// Structure for context template
#[derive(Template)]
#[template(path = "upload_template.html")]
struct UploadTemplate {
    image: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {}


//Config server
pub fn config(conf: &mut web::ServiceConfig) {
    let path = web::scope("")
        .service(root)
        .service(random)
        .service(import)
        .service(Files::new("/uploads", "uploads").show_files_listing())
        .default_service(web::to(handler_404));

    conf.service(path);
}