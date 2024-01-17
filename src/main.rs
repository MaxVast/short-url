use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use std::io::Result;
use short_url::routes::routes;

#[actix_web::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let addr = "127.0.0.1:3000";

    println!("✅ Server started successfully");
    println!("✅ http://{}", addr);

    // run our app with actix_web, listening globally on port 3000
    HttpServer::new(move || {
        App::new()
            .configure(routes::config)
            .wrap(Logger::default())
    })
    .bind(addr)?
    .run()
    .await
}