mod handlers;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use handlers::{generate_random_meme, health_check, list_templates};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("  ");
    println!("🌐 Starting server at http://localhost:8080");
    println!("  ");
    println!("📍 Available endpoints:");
    println!("  - GET  /health - server status check");
    println!("  - GET  /list - list of available memes");
    println!("  - GET  /generate - generate random meme");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::JsonConfig::default().limit(10 * 1024 * 1024))
            .app_data(web::PayloadConfig::default().limit(10 * 1024 * 1024))
            .wrap(Logger::default())
            .service(health_check)
            .service(list_templates)
            .service(generate_random_meme)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
