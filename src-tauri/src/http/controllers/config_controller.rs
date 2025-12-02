use axum::{
    extract::{Path, Json},
    routing::{get, post, patch},
    Router,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::dtos::{CreateOrUpdateConfigDto, UpdatePercentConfigDto};
use crate::usecases::{
    CreateOrUpdateConfigUseCase,
    GetFirstConfigUseCase,
    UpdatePercentUseCase,
    GetCnpjUseCase,
};

/// GET /config/cnpj/:cnpj
async fn get_cnpj(Path(cnpj): Path<String>) -> impl IntoResponse {
    match GetCnpjUseCase::execute(cnpj).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e }))
        ).into_response(),
    }
}

/// GET /config/
async fn get_first_config() -> impl IntoResponse {
    match GetFirstConfigUseCase::execute() {
        Ok(Some(config)) => (StatusCode::OK, Json(config)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({ "error": "Configuração não encontrada" }))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e }))
        ).into_response(),
    }
}

/// POST /config/
async fn create_or_update_config(Json(body): Json<CreateOrUpdateConfigDto>) -> impl IntoResponse {
    match CreateOrUpdateConfigUseCase::execute(body) {
        Ok(config) => (StatusCode::OK, Json(config)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e }))
        ).into_response(),
    }
}

/// PATCH /config/percent
async fn update_percent(Json(body): Json<UpdatePercentConfigDto>) -> impl IntoResponse {
    match UpdatePercentUseCase::execute(body) {
        Ok(config) => (StatusCode::OK, Json(config)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e }))
        ).into_response(),
    }
}

/// Cria as rotas do controller de configuração
pub fn config_routes() -> Router {
    Router::new()
        .route("/cnpj/:cnpj", get(get_cnpj))
        .route("/", get(get_first_config))
        .route("/", post(create_or_update_config))
        .route("/percent", patch(update_percent))
}
