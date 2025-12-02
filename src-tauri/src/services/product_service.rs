use crate::database::SqliteDbService;
use crate::entities::ProductEntity;
use rusqlite::{params, Result};
use chrono::Utc;

pub struct ProductService;

impl ProductService {
    /// POST /products - Cria um novo produto
    pub fn create(code: String, name: String) -> Result<ProductEntity, String> {
        let product = ProductEntity::new(code, name);
        Self::save(&product)
    }

    /// GET /products/:id - Busca um produto por ID
    pub fn find_by_id(id: i64) -> Result<Option<ProductEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, code, name, active, balance, created_at, updated_at 
             FROM produtos WHERE id = ?1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let product = stmt.query_row(params![id], |row| {
            let created_at_str: String = row.get(5)?;
            let updated_at_str: String = row.get(6)?;
            
            Ok(ProductEntity {
                id: row.get(0)?,
                code: row.get(1)?,
                name: row.get(2)?,
                active: row.get(3)?,
                balance: row.get(4)?,
                created_at: created_at_str.parse().unwrap_or(Utc::now()),
                updated_at: updated_at_str.parse().unwrap_or(Utc::now()),
            })
        });

        match product {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to query product: {}", e)),
        }
    }

    /// GET /products/code/:code - Busca um produto por código
    pub fn find_by_code(code: &str) -> Result<Option<ProductEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, code, name, active, balance, created_at, updated_at 
             FROM produtos WHERE code = ?1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let product = stmt.query_row(params![code], |row| {
            let created_at_str: String = row.get(5)?;
            let updated_at_str: String = row.get(6)?;
            
            Ok(ProductEntity {
                id: row.get(0)?,
                code: row.get(1)?,
                name: row.get(2)?,
                active: row.get(3)?,
                balance: row.get(4)?,
                created_at: created_at_str.parse().unwrap_or(Utc::now()),
                updated_at: updated_at_str.parse().unwrap_or(Utc::now()),
            })
        });

        match product {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to query product: {}", e)),
        }
    }

    /// GET /products - Lista todos os produtos
    pub fn find_all() -> Result<Vec<ProductEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, code, name, active, balance, created_at, updated_at 
             FROM produtos ORDER BY name"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let products = stmt.query_map([], |row| {
            let created_at_str: String = row.get(5)?;
            let updated_at_str: String = row.get(6)?;
            
            Ok(ProductEntity {
                id: row.get(0)?,
                code: row.get(1)?,
                name: row.get(2)?,
                active: row.get(3)?,
                balance: row.get(4)?,
                created_at: created_at_str.parse().unwrap_or(Utc::now()),
                updated_at: updated_at_str.parse().unwrap_or(Utc::now()),
            })
        })
        .map_err(|e| format!("Failed to query products: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect products: {}", e))?;

        Ok(products)
    }

    /// Lista todos os produtos ativos
    pub fn find_all_active() -> Result<Vec<ProductEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, code, name, active, balance, created_at, updated_at 
             FROM produtos WHERE active = 1 ORDER BY name"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let products = stmt.query_map([], |row| {
            let created_at_str: String = row.get(5)?;
            let updated_at_str: String = row.get(6)?;
            
            Ok(ProductEntity {
                id: row.get(0)?,
                code: row.get(1)?,
                name: row.get(2)?,
                active: row.get(3)?,
                balance: row.get(4)?,
                created_at: created_at_str.parse().unwrap_or(Utc::now()),
                updated_at: updated_at_str.parse().unwrap_or(Utc::now()),
            })
        })
        .map_err(|e| format!("Failed to query products: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect products: {}", e))?;

        Ok(products)
    }

    /// PUT /products/:id - Atualiza um produto
    pub fn update(id: i64, code: Option<String>, name: Option<String>, active: Option<i32>, balance: Option<f64>) -> Result<ProductEntity, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        // Busca o produto existente
        let existing = Self::find_by_id(id)?
            .ok_or_else(|| format!("Product with id {} not found", id))?;

        // Atualiza apenas os campos fornecidos
        let updated_code = code.unwrap_or(existing.code);
        let updated_name = name.unwrap_or(existing.name);
        let updated_active = active.unwrap_or(existing.active);
        let updated_balance = balance.unwrap_or(existing.balance);

        conn.execute(
            "UPDATE produtos SET code = ?1, name = ?2, active = ?3, balance = ?4, updated_at = ?5 WHERE id = ?6",
            params![
                updated_code,
                updated_name,
                updated_active,
                updated_balance,
                Utc::now().to_rfc3339(),
                id
            ],
        ).map_err(|e| format!("Failed to update product: {}", e))?;

        Ok(ProductEntity {
            id: Some(id),
            code: updated_code,
            name: updated_name,
            active: updated_active,
            balance: updated_balance,
            created_at: existing.created_at,
            updated_at: Utc::now(),
        })
    }

    /// Salva ou atualiza um produto (interno)
    pub fn save(product: &ProductEntity) -> Result<ProductEntity, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        if let Some(id) = product.id {
            // Update
            conn.execute(
                "UPDATE produtos SET code = ?1, name = ?2, active = ?3, balance = ?4, updated_at = ?5 WHERE id = ?6",
                params![
                    product.code,
                    product.name,
                    product.active,
                    product.balance,
                    Utc::now().to_rfc3339(),
                    id
                ],
            ).map_err(|e| format!("Failed to update product: {}", e))?;

            Ok(ProductEntity { id: Some(id), updated_at: Utc::now(), ..product.clone() })
        } else {
            // Insert
            conn.execute(
                "INSERT INTO produtos (code, name, active, balance, created_at, updated_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    product.code,
                    product.name,
                    product.active,
                    product.balance,
                    product.created_at.to_rfc3339(),
                    product.updated_at.to_rfc3339()
                ],
            ).map_err(|e| format!("Failed to insert product: {}", e))?;

            let id = conn.last_insert_rowid();
            Ok(ProductEntity { id: Some(id), ..product.clone() })
        }
    }

    /// PATCH /products/:id/increment - Incrementa o saldo de um produto
    pub fn increment_balance(id: i64, amount: f64) -> Result<ProductEntity, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        conn.execute(
            "UPDATE produtos SET balance = balance + ?1, updated_at = ?2 WHERE id = ?3",
            params![amount, Utc::now().to_rfc3339(), id],
        ).map_err(|e| format!("Failed to increment balance: {}", e))?;

        Self::find_by_id(id)?
            .ok_or_else(|| format!("Product with id {} not found after update", id))
    }

    /// PATCH /products/:id/decrement - Decrementa o saldo de um produto
    pub fn decrement_balance(id: i64, amount: f64) -> Result<ProductEntity, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        conn.execute(
            "UPDATE produtos SET balance = balance - ?1, updated_at = ?2 WHERE id = ?3",
            params![amount, Utc::now().to_rfc3339(), id],
        ).map_err(|e| format!("Failed to decrement balance: {}", e))?;

        Self::find_by_id(id)?
            .ok_or_else(|| format!("Product with id {} not found after update", id))
    }

    /// Atualiza o saldo de um produto (interno - genérico)
    pub fn update_balance(id: i64, quantity: f64) -> Result<(), String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        conn.execute(
            "UPDATE produtos SET balance = balance + ?1, updated_at = ?2 WHERE id = ?3",
            params![quantity, Utc::now().to_rfc3339(), id],
        ).map_err(|e| format!("Failed to update balance: {}", e))?;

        Ok(())
    }

    /// DELETE /products/:id - Deleta um produto (soft delete - marca como inativo)
    pub fn delete(id: i64) -> Result<(), String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        conn.execute(
            "UPDATE produtos SET active = 0, updated_at = ?1 WHERE id = ?2",
            params![Utc::now().to_rfc3339(), id],
        ).map_err(|e| format!("Failed to delete product: {}", e))?;

        Ok(())
    }
}
