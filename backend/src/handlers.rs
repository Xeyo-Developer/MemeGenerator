use actix_web::{HttpResponse, Result, error, get, http::StatusCode, post, web};
use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemeError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Meme not found")]
    NotFound,
    #[error("Invalid request: {0}")]
    BadRequest(String),
    #[error("Internal server error")]
    InternalServerError,
}

impl error::ResponseError for MemeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MemeError::Io(_) | MemeError::Json(_) | MemeError::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal Server Error".to_string(),
                    details: self.to_string(),
                })
            }
            MemeError::NotFound => HttpResponse::NotFound().json(ErrorResponse {
                error: "Not Found".to_string(),
                details: self.to_string(),
            }),
            MemeError::BadRequest(details) => HttpResponse::BadRequest().json(ErrorResponse {
                error: "Bad Request".to_string(),
                details: details.to_string(),
            }),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MemeError::Io(_) | MemeError::Json(_) | MemeError::InternalServerError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            MemeError::NotFound => StatusCode::NOT_FOUND,
            MemeError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

const MAX_MEME_COUNT: u32 = 50;
const MEMES_DIR: &str = "../assets/memes";
const FAVORITES_FILE: &str = "../assets/favorites.json";
const ALLOWED_EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "gif"];

fn validate_meme_path(filename: &str) -> Result<PathBuf, MemeError> {
    let path = Path::new(MEMES_DIR).join(filename);

    if !path.starts_with(MEMES_DIR) {
        return Err(MemeError::BadRequest("Invalid file path".to_string()));
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .ok_or_else(|| MemeError::BadRequest("File has no extension".to_string()))?;

    if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
        return Err(MemeError::BadRequest(format!(
            "Invalid file extension: {}",
            ext
        )));
    }

    Ok(path)
}

#[get("/health")]
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HealthResponse {
        status: "OK".to_string(),
        message: "Meme server is running properly".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    }))
}

#[get("/list")]
pub async fn list_templates() -> Result<HttpResponse, MemeError> {
    let mut templates = Vec::new();

    let entries = fs::read_dir(MEMES_DIR)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if ALLOWED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                let metadata = fs::metadata(&path)?;
                let size = metadata.len();
                let modified = metadata.modified().ok().and_then(|t| {
                    DateTime::from_timestamp(
                        t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                        0,
                    )
                });

                templates.push(MemeTemplate {
                    name: path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.to_string())
                        .ok_or(MemeError::InternalServerError)?,
                    path: path.to_string_lossy().to_string(),
                    size_bytes: size,
                    file_type: ext.to_lowercase(),
                    last_modified: modified.map(|dt| dt.to_rfc3339()),
                });
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
pub async fn generate_random_meme() -> Result<HttpResponse, MemeError> {
    let mut template_files = Vec::new();

    let entries = fs::read_dir(MEMES_DIR)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if ALLOWED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    template_files.push(filename.to_string());
                }
            }
        }
    }

    if template_files.is_empty() {
        return Err(MemeError::NotFound);
    }

    let mut rng = rand::rng();
    let random_template = &template_files[rng.random_range(0..template_files.len())];
    let path = validate_meme_path(random_template)?;

    let image_data = fs::read(&path)?;
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("png");

    let content_type = match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        _ => "image/png",
    };

    let base64_image = general_purpose::STANDARD.encode(&image_data);
    let data_url = format!("data:{};base64,{}", content_type, base64_image);

    Ok(HttpResponse::Ok().json(RandomMemeResponse {
        template_name: random_template.to_string(),
        image_url: data_url,
        content_type: content_type.to_string(),
        size_bytes: image_data.len(),
        generated_at: Utc::now().to_rfc3339(),
    }))
}

#[get("/meme/{filename}")]
pub async fn get_specific_meme(path: web::Path<String>) -> Result<HttpResponse, MemeError> {
    let filename = path.into_inner();
    let path = validate_meme_path(&filename)?;

    if !path.exists() {
        return Err(MemeError::NotFound);
    }

    let image_data = fs::read(&path)?;
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("png");

    let content_type = match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        _ => "image/png",
    };

    let base64_image = general_purpose::STANDARD.encode(&image_data);
    let data_url = format!("data:{};base64,{}", content_type, base64_image);

    Ok(HttpResponse::Ok().json(SpecificMemeResponse {
        template_name: filename,
        image_url: data_url,
        content_type: content_type.to_string(),
        size_bytes: image_data.len(),
        requested_at: Utc::now().to_rfc3339(),
    }))
}

#[get("/random/{count}")]
pub async fn generate_multiple_memes(path: web::Path<u32>) -> Result<HttpResponse, MemeError> {
    let count = path.into_inner();

    if count == 0 || count > MAX_MEME_COUNT {
        return Err(MemeError::BadRequest(format!(
            "Count must be between 1 and {}",
            MAX_MEME_COUNT
        )));
    }

    let mut template_files = Vec::new();
    let entries = fs::read_dir(MEMES_DIR)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if ALLOWED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    template_files.push(filename.to_string());
                }
            }
        }
    }

    if template_files.is_empty() {
        return Err(MemeError::NotFound);
    }

    let mut rng = rand::rng();
    let mut memes = Vec::new();

    for _ in 0..count {
        let random_template = &template_files[rng.random_range(0..template_files.len())];
        let path = validate_meme_path(random_template)?;

        if let Ok(image_data) = fs::read(&path) {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("png");

            let content_type = match ext.to_lowercase().as_str() {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "gif" => "image/gif",
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
    }

    let count = memes.len();
    Ok(HttpResponse::Ok().json(MultipleMemeResponse {
        memes,
        count,
        generated_at: Utc::now().to_rfc3339(),
    }))
}

#[get("/stats")]
pub async fn get_meme_stats() -> Result<HttpResponse, MemeError> {
    let mut stats = MemeStats::default();
    let entries = fs::read_dir(MEMES_DIR)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if ALLOWED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                stats.total_memes += 1;

                let metadata = fs::metadata(&path)?;
                let size = metadata.len();
                stats.total_size_bytes += size;

                if size > stats.largest_file_size {
                    stats.largest_file_size = size;
                    stats.largest_file_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                }

                if stats.smallest_file_size == 0 || size < stats.smallest_file_size {
                    stats.smallest_file_size = size;
                    stats.smallest_file_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                }

                match ext.to_lowercase().as_str() {
                    "jpg" | "jpeg" => *stats.file_types.entry("jpeg".to_string()).or_insert(0) += 1,
                    "png" => *stats.file_types.entry("png".to_string()).or_insert(0) += 1,
                    "gif" => *stats.file_types.entry("gif".to_string()).or_insert(0) += 1,
                    _ => (),
                };
            }
        }
    }

    if stats.total_memes > 0 {
        stats.average_file_size = stats.total_size_bytes / stats.total_memes as u64;
    }

    Ok(HttpResponse::Ok().json(stats))
}

#[get("/search")]
pub async fn search_memes(query: web::Query<SearchQuery>) -> Result<HttpResponse, MemeError> {
    let search_term = query.q.to_lowercase();
    let mut matching_memes = Vec::new();
    let entries = fs::read_dir(MEMES_DIR)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if ALLOWED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if filename.to_lowercase().contains(&search_term) {
                        let metadata = fs::metadata(&path)?;
                        let size = metadata.len();
                        let modified = metadata.modified().ok().and_then(|t| {
                            DateTime::from_timestamp(
                                t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                                0,
                            )
                        });

                        matching_memes.push(MemeTemplate {
                            name: filename.to_string(),
                            path: path.to_string_lossy().to_string(),
                            size_bytes: size,
                            file_type: ext.to_lowercase(),
                            last_modified: modified.map(|dt| dt.to_rfc3339()),
                        });
                    }
                }
            }
        }
    }

    let count = matching_memes.len();
    Ok(HttpResponse::Ok().json(SearchResponse {
        query: search_term,
        results: matching_memes,
        count,
    }))
}

#[post("/favorite")]
pub async fn toggle_favorite(
    favorite_req: web::Json<FavoriteRequest>,
) -> Result<HttpResponse, MemeError> {
    validate_meme_path(&favorite_req.meme_name)?;

    let mut favorites: Vec<String> = if Path::new(FAVORITES_FILE).exists() {
        serde_json::from_str(&fs::read_to_string(FAVORITES_FILE)?)?
    } else {
        Vec::new()
    };

    let was_favorite = favorites.contains(&favorite_req.meme_name);

    if was_favorite {
        favorites.retain(|x| x != &favorite_req.meme_name);
    } else {
        favorites.push(favorite_req.meme_name.clone());
    }

    fs::write(FAVORITES_FILE, serde_json::to_string_pretty(&favorites)?)?;

    Ok(HttpResponse::Ok().json(FavoriteResponse {
        meme_name: favorite_req.meme_name.clone(),
        is_favorite: !was_favorite,
        message: if was_favorite {
            "Removed from favorites".to_string()
        } else {
            "Added to favorites".to_string()
        },
    }))
}

#[get("/favorites")]
pub async fn get_favorites() -> Result<HttpResponse, MemeError> {
    let favorites: Vec<String> = if Path::new(FAVORITES_FILE).exists() {
        serde_json::from_str(&fs::read_to_string(FAVORITES_FILE)?)?
    } else {
        Vec::new()
    };

    Ok(HttpResponse::Ok().json(FavoritesResponse {
        favorites: favorites.clone(),
        count: favorites.len(),
    }))
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
    pub timestamp: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: String,
}

#[derive(Serialize, Clone)]
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
