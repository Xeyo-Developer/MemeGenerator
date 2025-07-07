use actix_web::{HttpResponse, Result, get};
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;
use serde::Serialize;
use std::fs;

#[get("/health")]
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HealthResponse {
        status: "OK".to_string(),
        message: "Meme server is running properly".to_string(),
    }))
}

#[get("/list")]
pub async fn list_templates() -> Result<HttpResponse> {
    let templates_dir = "static/memes";
    let mut templates = Vec::new();

    if let Ok(entries) = fs::read_dir(templates_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif") {
                        if let Some(filename) = path.file_name() {
                            templates.push(MemeTemplate {
                                name: filename.to_string_lossy().to_string(),
                                path: path.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(HttpResponse::Ok().json(TemplatesResponse { templates }))
}

#[get("/generate")]
pub async fn generate_random_meme() -> HttpResponse {
    let templates_dir = "static/memes";
    let mut template_files = Vec::new();

    if let Ok(entries) = fs::read_dir(templates_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif") {
                        if let Some(filename) = path.file_name() {
                            template_files.push(filename.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }

    if template_files.is_empty() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: "No memes found".to_string(),
            details: "No image memes found in static/memes directory".to_string(),
        });
    }

    let mut rng = rand::rng();
    let random_template = &template_files[rng.random_range(0..template_files.len())];
    let full_path = format!("static/memes/{}", random_template);

    match fs::read(&full_path) {
        Ok(image_data) => {
            let content_type = match full_path.split('.').last() {
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("png") => "image/png",
                Some("gif") => "image/gif",
                _ => "image/png",
            };

            let base64_image = general_purpose::STANDARD.encode(&image_data);
            let data_url = format!("data:{};base64,{}", content_type, base64_image);

            HttpResponse::Ok().json(RandomMemeResponse {
                template_name: random_template.to_string(),
                image_url: data_url,
                content_type: content_type.to_string(),
            })
        }
        Err(e) => {
            eprintln!("❌ Error loading template: {}: {}", full_path, e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Cannot load template".to_string(),
                details: format!("Cannot load image from {}: {}", full_path, e),
            })
        }
    }
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: String,
}

#[derive(Serialize)]
pub struct MemeTemplate {
    pub name: String,
    pub path: String,
}

#[derive(Serialize)]
pub struct RandomMemeResponse {
    pub template_name: String,
    pub image_url: String,
    pub content_type: String,
}

#[derive(Serialize)]
pub struct TemplatesResponse {
    pub templates: Vec<MemeTemplate>,
}
