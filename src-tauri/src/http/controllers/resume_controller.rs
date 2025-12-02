use axum::{
    routing::get,
    Router,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::services::ResumeService;

/// GET /resumes/
async fn get_all_resumes_today() -> impl IntoResponse {
    match ResumeService::get_all_today() {
        Ok(resumes) => (StatusCode::OK, Json(resumes)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e }))
        ).into_response(),
    }
}

/// Cria as rotas do controller de resumos
pub fn resume_routes() -> Router {
    Router::new()
        .route("/", get(get_all_resumes_today))
}
