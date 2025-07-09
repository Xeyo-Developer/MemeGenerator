mod handlers;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use handlers::{
    generate_multiple_memes, generate_random_meme, get_favorites, get_meme_stats,
    get_specific_meme, health_check, list_templates, search_memes, toggle_favorite,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("  ");
    println!("🌐 Server running at: http://localhost:8080");
    println!("  ");
    println!("📍 Available endpoints:");
    println!("  ");
    println!("  - GET  /health           - server status check");
    println!("  - GET  /list             - list all available memes");
    println!("  - GET  /generate         - generate single random meme");
    println!("  ");
    println!("  - GET  /meme/{{filename}}  - get specific meme by filename");
    println!("  - GET  /random/{{count}}   - generate multiple random memes (1-50)");
    println!("  - GET  /stats            - get meme collection statistics");
    println!("  - GET  /search?q={{term}}  - search memes by filename");
    println!("  ");
    println!("  - POST /favorite         - toggle meme as favorite");
    println!("  - GET  /favorites        - get all favorite memes");
    println!("  ");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::ORIGIN,
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
            .service(get_specific_meme)
            .service(generate_multiple_memes)
            .service(get_meme_stats)
            .service(search_memes)
            .service(toggle_favorite)
            .service(get_favorites)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
