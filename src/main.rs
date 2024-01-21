use actix_web::{App, HttpServer, web::Data, middleware::Logger};
use std::io::Result;
use short_url::{routes::routes, database::database};

#[actix_web::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let addr = "127.0.0.1:8080";

    println!("✅ Server started successfully");
    println!("✅ http://{}", addr);
    
    let db = database::init().await;
    let db_data = Data::new(db);

    // run our app with actix_web, listening globally on port 3000
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(routes::config)
            .wrap(Logger::default())
    })
    .bind(addr)?
    .run()
    .await
}