use actix_web::{get, post, web, HttpResponse};
use actix_files::Files;
use askama::Template;
use actix_multipart::Multipart;
use crate::{encode::encode::unique_code, shorturl::handler::{handle_csv_upload, handle_upload_asset}, frontend::template::{HomeTemplate, UploadCsvTemplate}};

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

#[post("/upload-asset")]
async fn upload_asset(mut payload: Multipart) -> HttpResponse {
    return handle_upload_asset(&mut payload).await;
}

#[get("/upload-csv")]
async fn upload_csv() -> HttpResponse {
    let template = UploadCsvTemplate {};
    let response_body = template.render().unwrap();
    HttpResponse::Ok().body(response_body)
}

#[post("/import-csv")]
async fn import_csv(mut payload: Multipart) -> HttpResponse {
    return handle_csv_upload(&mut payload).await;
}

// fallback route
async fn handler_404() -> HttpResponse {
    HttpResponse::NotFound().body("404 : Nothing here..")
}

//Config server
pub fn config(conf: &mut web::ServiceConfig) {
    let path = web::scope("")
        .service(root)
        .service(random)
        .service(upload_asset)
        .service(upload_csv)
        .service(import_csv)
        .service(Files::new("/uploads", "uploads").show_files_listing())
        .default_service(web::to(handler_404));

    conf.service(path);
}