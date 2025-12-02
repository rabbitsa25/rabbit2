use axum::{
    extract::Query,
    routing::get,
    Router,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::services::VendaService;

#[derive(Debug, Deserialize)]
struct DateIntervalQuery {
    #[serde(rename = "dtInit")]
    dt_init: String,
    #[serde(rename = "dtFim")]
    dt_end: String,
}

/// GET /vendas/
async fn get_all() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "message": "Vendas por intervalo de datas" }))
    )
}

/// GET /vendas/get-vendas-by-interval?dtInit=2024-01-01&dtFim=2024-12-31
async fn get_vendas_by_interval(
    Query(params): Query<DateIntervalQuery>,
) -> impl IntoResponse {
    match VendaService::get_vendas_by_interval(&params.dt_init, &params.dt_end) {
        Ok(vendas) => (StatusCode::OK, Json(vendas)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e }))
        ).into_response(),
    }
}

/// GET /vendas/get-items-by-interval?dtInit=2024-01-01&dtFim=2024-12-31
async fn get_items_by_interval(
    Query(params): Query<DateIntervalQuery>,
) -> impl IntoResponse {
    match VendaService::get_items_by_interval(&params.dt_init, &params.dt_end) {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e }))
        ).into_response(),
    }
}

/// GET /vendas/get-payments-by-interval?dtInit=2024-01-01&dtFim=2024-12-31
async fn get_payments_by_interval(
    Query(params): Query<DateIntervalQuery>,
) -> impl IntoResponse {
    match VendaService::get_payments_by_interval(&params.dt_init, &params.dt_end) {
        Ok(payments) => (StatusCode::OK, Json(payments)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e }))
        ).into_response(),
    }
}

/// GET /vendas/resumo-by-interval?dtInit=2024-01-01&dtFim=2024-12-31
async fn get_resumo_by_interval(
    Query(params): Query<DateIntervalQuery>,
) -> impl IntoResponse {
    match VendaService::get_resumo_by_interval(&params.dt_init, &params.dt_end) {
        Ok(resumo) => (StatusCode::OK, Json(resumo)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e }))
        ).into_response(),
    }
}

/// Cria as rotas do controller de vendas
pub fn venda_routes() -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/get-vendas-by-interval", get(get_vendas_by_interval))
        .route("/get-items-by-interval", get(get_items_by_interval))
        .route("/get-payments-by-interval", get(get_payments_by_interval))
        .route("/resumo-by-interval", get(get_resumo_by_interval))
}
