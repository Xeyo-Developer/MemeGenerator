use actix_web::{HttpRequest, HttpResponse, Result, get, post, web};
use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[get("/health")]
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HealthResponse {
        status: "OK".to_string(),
        message: "Meme server is running properly".to_string(),
    }))
}

#[get("/list")]
pub async fn list_templates() -> Result<HttpResponse> {
    let templates_dir = "../assets/memes";
    let mut templates = Vec::new();

    if let Ok(entries) = fs::read_dir(templates_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif") {
                        if let Some(filename) = path.file_name() {
                            let metadata = fs::metadata(&path).ok();
                            let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                            let modified = metadata.and_then(|m| m.modified().ok()).and_then(|t| {
                                DateTime::from_timestamp(
                                    t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                                    0,
                                )
                            });

                            templates.push(MemeTemplate {
                                name: filename.to_string_lossy().to_string(),
                                path: path.to_string_lossy().to_string(),
                                size_bytes: size,
                                file_type: ext.clone(),
                                last_modified: modified.map(|dt| dt.to_rfc3339()),
                            });
                        }
                    }
                }
            }
        }
    }

    templates.sort_by(|a, b| a.name.cmp(&b.name));

    let total_count = templates.len();
    Ok(HttpResponse::Ok().json(TemplatesResponse {
        templates,
        total_count,
    }))
}

#[get("/generate")]
pub async fn generate_random_meme() -> HttpResponse {
    let templates_dir = "../assets/memes";
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
            details: "No image memes found in memes directory".to_string(),
        });
    }

    let mut rng = rand::rng();
    let random_template = &template_files[rng.random_range(0..template_files.len())];
    let full_path = format!("../assets/memes/{}", random_template);

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
                size_bytes: image_data.len(),
                generated_at: Utc::now().to_rfc3339(),
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

#[get("/meme/{filename}")]
pub async fn get_specific_meme(path: web::Path<String>) -> HttpResponse {
    let filename = path.into_inner();
    let full_path = format!("../assets/memes/{}", filename);

    if !Path::new(&full_path).exists() {
        return HttpResponse::NotFound().json(ErrorResponse {
            error: "Meme not found".to_string(),
            details: format!("Meme '{}' does not exist", filename),
        });
    }

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

            HttpResponse::Ok().json(SpecificMemeResponse {
                template_name: filename,
                image_url: data_url,
                content_type: content_type.to_string(),
                size_bytes: image_data.len(),
                requested_at: Utc::now().to_rfc3339(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Cannot load meme".to_string(),
            details: format!("Cannot load image from {}: {}", full_path, e),
        }),
    }
}

#[get("/random/{count}")]
pub async fn generate_multiple_memes(path: web::Path<u32>) -> HttpResponse {
    let count = path.into_inner();

    if count == 0 || count > 50 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid count".to_string(),
            details: "Count must be between 1 and 50".to_string(),
        });
    }

    let templates_dir = "../assets/memes";
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
            details: "No image memes found in memes directory".to_string(),
        });
    }

    let mut rng = rand::rng();
    let mut memes = Vec::new();

    for _ in 0..count {
        let random_template = &template_files[rng.random_range(0..template_files.len())];
        let full_path = format!("../assets/memes/{}", random_template);

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

                memes.push(RandomMemeResponse {
                    template_name: random_template.to_string(),
                    image_url: data_url,
                    content_type: content_type.to_string(),
                    size_bytes: image_data.len(),
                    generated_at: Utc::now().to_rfc3339(),
                });
            }
            Err(e) => {
                eprintln!("❌ Error loading template: {}: {}", full_path, e);
            }
        }
    }

    let count = memes.len();
    HttpResponse::Ok().json(MultipleMemeResponse {
        memes,
        count,
        generated_at: Utc::now().to_rfc3339(),
    })
}

#[get("/stats")]
pub async fn get_meme_stats() -> HttpResponse {
    let templates_dir = "../assets/memes";
    let mut stats = MemeStats::default();

    if let Ok(entries) = fs::read_dir(templates_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif") {
                        stats.total_memes += 1;

                        if let Ok(metadata) = fs::metadata(&path) {
                            let size = metadata.len();
                            stats.total_size_bytes += size;

                            if size > stats.largest_file_size {
                                stats.largest_file_size = size;
                                stats.largest_file_name = path
                                    .file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_default();
                            }

                            if stats.smallest_file_size == 0 || size < stats.smallest_file_size {
                                stats.smallest_file_size = size;
                                stats.smallest_file_name = path
                                    .file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_default();
                            }
                        }

                        match ext.as_str() {
                            "jpg" | "jpeg" => stats.file_types.insert(
                                "jpeg".to_string(),
                                stats.file_types.get("jpeg").unwrap_or(&0) + 1,
                            ),
                            "png" => stats.file_types.insert(
                                "png".to_string(),
                                stats.file_types.get("png").unwrap_or(&0) + 1,
                            ),
                            "gif" => stats.file_types.insert(
                                "gif".to_string(),
                                stats.file_types.get("gif").unwrap_or(&0) + 1,
                            ),
                            _ => None,
                        };
                    }
                }
            }
        }
    }

    if stats.total_memes > 0 {
        stats.average_file_size = stats.total_size_bytes / stats.total_memes as u64;
    }

    HttpResponse::Ok().json(stats)
}

#[get("/search")]
pub async fn search_memes(query: web::Query<SearchQuery>) -> HttpResponse {
    let search_term = query.q.to_lowercase();
    let templates_dir = "../assets/memes";
    let mut matching_memes = Vec::new();

    if let Ok(entries) = fs::read_dir(templates_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif") {
                        if let Some(filename) = path.file_name() {
                            let filename_str = filename.to_string_lossy().to_string();
                            if filename_str.to_lowercase().contains(&search_term) {
                                let metadata = fs::metadata(&path).ok();
                                let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);

                                matching_memes.push(MemeTemplate {
                                    name: filename_str,
                                    path: path.to_string_lossy().to_string(),
                                    size_bytes: size,
                                    file_type: ext.clone(),
                                    last_modified: metadata
                                        .and_then(|m| m.modified().ok())
                                        .and_then(|t| {
                                            DateTime::from_timestamp(
                                                t.duration_since(std::time::UNIX_EPOCH)
                                                    .ok()?
                                                    .as_secs()
                                                    as i64,
                                                0,
                                            )
                                        })
                                        .map(|dt| dt.to_rfc3339()),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    let count = matching_memes.len();
    HttpResponse::Ok().json(SearchResponse {
        query: search_term,
        results: matching_memes,
        count,
    })
}

#[post("/favorite")]
pub async fn toggle_favorite(
    _req: HttpRequest,
    favorite_req: web::Json<FavoriteRequest>,
) -> HttpResponse {
    let favorites_file = "../assets/favorites.json";

    let mut favorites: Vec<String> = if Path::new(favorites_file).exists() {
        match fs::read_to_string(favorites_file) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    };

    let meme_name = &favorite_req.meme_name;
    let was_favorite = favorites.contains(meme_name);

    if was_favorite {
        favorites.retain(|x| x != meme_name);
    } else {
        favorites.push(meme_name.clone());
    }

    if let Ok(json) = serde_json::to_string_pretty(&favorites) {
        let _ = fs::write(favorites_file, json);
    }

    HttpResponse::Ok().json(FavoriteResponse {
        meme_name: meme_name.clone(),
        is_favorite: !was_favorite,
        message: if was_favorite {
            "Removed from favorites".to_string()
        } else {
            "Added to favorites".to_string()
        },
    })
}

#[get("/favorites")]
pub async fn get_favorites() -> HttpResponse {
    let favorites_file = "../assets/favorites.json";

    let favorites: Vec<String> = if Path::new(favorites_file).exists() {
        match fs::read_to_string(favorites_file) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    };

    let count = favorites.len();
    HttpResponse::Ok().json(FavoritesResponse { favorites, count })
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
    pub size_bytes: u64,
    pub file_type: String,
    pub last_modified: Option<String>,
}

#[derive(Serialize)]
pub struct RandomMemeResponse {
    pub template_name: String,
    pub image_url: String,
    pub content_type: String,
    pub size_bytes: usize,
    pub generated_at: String,
}

#[derive(Serialize)]
pub struct SpecificMemeResponse {
    pub template_name: String,
    pub image_url: String,
    pub content_type: String,
    pub size_bytes: usize,
    pub requested_at: String,
}

#[derive(Serialize)]
pub struct TemplatesResponse {
    pub templates: Vec<MemeTemplate>,
    pub total_count: usize,
}

#[derive(Serialize)]
pub struct MultipleMemeResponse {
    pub memes: Vec<RandomMemeResponse>,
    pub count: usize,
    pub generated_at: String,
}

#[derive(Serialize, Default)]
pub struct MemeStats {
    pub total_memes: u32,
    pub total_size_bytes: u64,
    pub average_file_size: u64,
    pub largest_file_name: String,
    pub largest_file_size: u64,
    pub smallest_file_name: String,
    pub smallest_file_size: u64,
    pub file_types: HashMap<String, u32>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub query: String,
    pub results: Vec<MemeTemplate>,
    pub count: usize,
}

#[derive(Deserialize)]
pub struct FavoriteRequest {
    pub meme_name: String,
}

#[derive(Serialize)]
pub struct FavoriteResponse {
    pub meme_name: String,
    pub is_favorite: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct FavoritesResponse {
    pub favorites: Vec<String>,
    pub count: usize,
}
