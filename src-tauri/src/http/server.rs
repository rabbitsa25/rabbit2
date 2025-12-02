use axum::{Router, http::Method};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

use crate::http::controllers::{config_routes, venda_routes, resume_routes};

pub async fn start_http_server() -> Result<(), Box<dyn std::error::Error>> {
    // Configura CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE, Method::PUT])
        .allow_headers(Any);

    // Configura as rotas
    let app = Router::new()
        .nest("/config", config_routes())
        .nest("/vendas", venda_routes())
        .nest("/resumes", resume_routes())
        .layer(cors);

    // Inicia o servidor na porta 8088
    let addr = SocketAddr::from(([127, 0, 0, 1], 8088));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    println!("🚀 Servidor HTTP rodando em http://{}", addr);
    println!("📡 Pronto para receber integrações externas");
    println!("   - GET  http://localhost:8088/vendas/get-vendas-by-interval?dtInit=2024-01-01&dtFim=2024-12-31");
    println!("   - GET  http://localhost:8088/vendas/get-items-by-interval?dtInit=2024-01-01&dtFim=2024-12-31");
    println!("   - GET  http://localhost:8088/vendas/get-payments-by-interval?dtInit=2024-01-01&dtFim=2024-12-31");
    println!("   - GET  http://localhost:8088/vendas/resumo-by-interval?dtInit=2024-01-01&dtFim=2024-12-31");
    println!("   - GET  http://localhost:8088/resumes/");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
