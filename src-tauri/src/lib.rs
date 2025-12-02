// Módulos
pub mod entities;
pub mod database;
pub mod services;
pub mod dtos;
pub mod usecases;
pub mod http;

use database::SqliteDbService;
use services::{ConfigService, ProductService, VendaService};
use entities::{ConfigEntity, ProductEntity, VendaEntity, VendaItemEntity, VendaPagamentoEntity};
use dtos::{CreateOrUpdateConfigDto, UpdatePercentConfigDto, CnpjResponseDto};
use usecases::{
    CreateOrUpdateConfigUseCase, 
    GetFirstConfigUseCase, 
    UpdatePercentUseCase,
    GetCnpjUseCase,
};
use http::start_http_server;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Comando para obter o caminho do banco de dados
#[tauri::command]
fn get_db_path() -> Result<String, String> {
    let db = SqliteDbService::get_instance()?;
    Ok(db.get_db_path().to_string_lossy().to_string())
}

// Comandos de Configuração (seguindo o controller NestJS)

/// GET /config/cnpj/:cnpj
#[tauri::command]
async fn get_cnpj(cnpj: String) -> Result<CnpjResponseDto, String> {
    GetCnpjUseCase::execute(cnpj).await
}

/// GET /config/
#[tauri::command]
fn get_first_config() -> Result<Option<ConfigEntity>, String> {
    GetFirstConfigUseCase::execute()
}

/// POST /config/
#[tauri::command]
fn create_or_update_config(body: CreateOrUpdateConfigDto) -> Result<ConfigEntity, String> {
    CreateOrUpdateConfigUseCase::execute(body)
}

/// PATCH /config/percent
#[tauri::command]
fn update_percent_config(body: UpdatePercentConfigDto) -> Result<ConfigEntity, String> {
    UpdatePercentUseCase::execute(body)
}

// Comandos legados (manter compatibilidade)
#[tauri::command]
fn get_config(id: String) -> Result<Option<ConfigEntity>, String> {
    ConfigService::find_by_id(&id)
}

#[tauri::command]
fn save_config(config: ConfigEntity) -> Result<ConfigEntity, String> {
    ConfigService::save(&config)
}

#[tauri::command]
fn list_configs() -> Result<Vec<ConfigEntity>, String> {
    ConfigService::find_all()
}

// Comandos de Produto (correspondentes ao ProdutosController do NestJS)

/// POST /products - Cria um novo produto
#[tauri::command]
fn create_product(code: String, name: String) -> Result<ProductEntity, String> {
    ProductService::create(code, name)
}

/// GET /products/:id - Busca produto por ID
#[tauri::command]
fn get_product(id: i64) -> Result<Option<ProductEntity>, String> {
    ProductService::find_by_id(id)
}

/// GET /products/code/:code - Busca produto por código
#[tauri::command]
fn get_product_by_code(code: String) -> Result<Option<ProductEntity>, String> {
    ProductService::find_by_code(&code)
}

/// GET /products - Lista todos os produtos
#[tauri::command]
fn get_all_products() -> Result<Vec<ProductEntity>, String> {
    ProductService::find_all()
}

/// Lista apenas produtos ativos (método auxiliar)
#[tauri::command]
fn list_active_products() -> Result<Vec<ProductEntity>, String> {
    ProductService::find_all_active()
}

/// PUT /products/:id - Atualiza um produto
#[tauri::command]
fn update_product(
    id: i64,
    code: Option<String>,
    name: Option<String>,
    active: Option<i32>,
    balance: Option<f64>
) -> Result<ProductEntity, String> {
    ProductService::update(id, code, name, active, balance)
}

/// DELETE /products/:id - Deleta um produto (soft delete)
#[tauri::command]
fn delete_product(id: i64) -> Result<(), String> {
    ProductService::delete(id)
}

/// PATCH /products/:id/increment - Incrementa saldo
#[tauri::command]
fn increment_product_balance(id: i64, amount: f64) -> Result<ProductEntity, String> {
    ProductService::increment_balance(id, amount)
}

/// PATCH /products/:id/decrement - Decrementa saldo
#[tauri::command]
fn decrement_product_balance(id: i64, amount: f64) -> Result<ProductEntity, String> {
    ProductService::decrement_balance(id, amount)
}

// Comandos de Venda
#[tauri::command]
fn get_venda(id: i64) -> Result<Option<VendaEntity>, String> {
    VendaService::find_by_id(id)
}

#[tauri::command]
fn create_venda(
    venda: VendaEntity,
    items: Vec<VendaItemEntity>,
    payments: Vec<VendaPagamentoEntity>,
) -> Result<i64, String> {
    VendaService::create_venda(&venda, items, payments)
}

#[tauri::command]
fn get_venda_items(venda_id: i64) -> Result<Vec<VendaItemEntity>, String> {
    VendaService::find_items_by_venda_id(venda_id)
}

#[tauri::command]
fn get_venda_payments(venda_id: i64) -> Result<Vec<VendaPagamentoEntity>, String> {
    VendaService::find_payments_by_venda_id(venda_id)
}

// Removidos: update_venda_status e list_vendas_by_status
// A nova estrutura usa campos específicos de NF-e (cancelled, etc)

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Inicializa o banco de dados
    if let Err(e) = SqliteDbService::get_instance() {
        eprintln!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }

    // Inicia o servidor HTTP em background para integrações externas
    tokio::spawn(async {
        if let Err(e) = start_http_server().await {
            eprintln!("Erro ao iniciar servidor HTTP: {}", e);
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_db_path,
            // Config commands (Controller-like)
            get_cnpj,
            get_first_config,
            create_or_update_config,
            update_percent_config,
            // Config commands (legacy)
            get_config,
            save_config,
            list_configs,
            // Product commands (Controller-like)
            create_product,
            get_product,
            get_product_by_code,
            get_all_products,
            update_product,
            delete_product,
            increment_product_balance,
            decrement_product_balance,
            list_active_products,
            // Venda commands
            get_venda,
            create_venda,
            get_venda_items,
            get_venda_payments,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


