mod handlers;

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
    println!("  ");

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::JsonConfig::default()
                    .limit(10 * 1024 * 1024)
                    .error_handler(|err, _req| {
                        let err_msg = format!("JSON parsing failed: {}", err);
                        println!("❌ JSON Error: {}", err_msg);
                        actix_web::error::InternalError::from_response(
                            err,
                            actix_web::HttpResponse::BadRequest().json(serde_json::json!({
                                "error": "Invalid JSON",
                                "details": err_msg
                            })),
                        )
                        .into()
                    }),
            )
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
