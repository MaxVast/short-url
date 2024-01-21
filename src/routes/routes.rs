use actix_web::{get, post, web, HttpResponse};
use actix_files::Files;
use askama::Template;
use actix_multipart::Multipart;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::{
    encode::encode::unique_code, 
    shorturl::handler::{handle_csv_upload, handle_upload_asset}, 
    frontend::template::{HomeTemplate, UploadCsvTemplate}, 
    database::{
        database::DbConnection, 
        schema::short_urls::{url, short_url, dsl::short_urls}
    }
};

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

async fn redirect_short_url(info: web::Path<String>) -> HttpResponse {
    let token = info.into_inner();
    let connection = &mut DbConnection::establish_connection().connection;
    let url_value = match short_urls.filter(short_url.eq(format!("https://dps.re/{}", token)))
        .select(url)
        .first::<String>(connection)
    {
        Ok(url_value) => url_value,
        Err(_) => {
            return HttpResponse::NotFound().body("404 : Nothing here..")
        }
    };
    HttpResponse::Found().append_header(("Location", url_value)).finish()
}

//Config server
pub fn config(conf: &mut web::ServiceConfig) {
    conf
        .service(root)
        .service(random)
        .service(upload_asset)
        .service(upload_csv)
        .service(import_csv)
        .service(Files::new("/uploads", "uploads").show_files_listing())
        .default_service(web::to(handler_404))
        .route("/{token}", web::get().to(redirect_short_url))
    ;
}