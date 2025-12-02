use crate::database::SqliteDbService;
use crate::entities::{VendaEntity, VendaItemEntity, VendaPagamentoEntity};
use rusqlite::{params, Result, Transaction};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendaWithRelations {
    #[serde(flatten)]
    pub venda: VendaEntity,
    pub itens: Vec<VendaItemEntity>,
    pub pagamentos: Vec<VendaPagamentoEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendaResumo {
    pub total_vendas: i64,
    pub total_valor: f64,
    pub total_desconto: f64,
    pub total_acrescimo: f64,
    pub total_canceladas: i64,
}

pub struct VendaService;

impl VendaService {
    /// Busca uma venda por ID
    pub fn find_by_id(id: i64) -> Result<Option<VendaEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, tip, mod, serie_origin, serie, nr_nf_origin, nr_nf, cnpj, doc_destinatario, 
             dh_emi, dh_emi_canc, total, addition, discount, chave, chave_canc, file_path, 
             cancel_file_path, protocolo, cancelled, created_at, updated_at 
             FROM vendas WHERE id = ?1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let venda = stmt.query_row(params![id], |row| {
            let created_at_str: String = row.get(20)?;
            let updated_at_str: String = row.get(21)?;
            
            Ok(VendaEntity {
                id: row.get(0)?,
                tip: row.get(1)?,
                mod_: row.get(2)?,
                serie_origin: row.get(3)?,
                serie: row.get(4)?,
                nr_nf_origin: row.get(5)?,
                nr_nf: row.get(6)?,
                cnpj: row.get(7)?,
                doc_destinatario: row.get(8)?,
                dh_emi: row.get(9)?,
                dh_emi_canc: row.get(10)?,
                total: row.get(11)?,
                addition: row.get(12)?,
                discount: row.get(13)?,
                chave: row.get(14)?,
                chave_canc: row.get(15)?,
                file_path: row.get(16)?,
                cancel_file_path: row.get(17)?,
                protocolo: row.get(18)?,
                cancelled: row.get(19)?,
                created_at: created_at_str.parse().unwrap_or(Utc::now()),
                updated_at: updated_at_str.parse().unwrap_or(Utc::now()),
            })
        });

        match venda {
            Ok(v) => Ok(Some(v)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to query venda: {}", e)),
        }
    }

    /// Cria uma nova venda com itens e pagamentos
    pub fn create_venda(
        venda: &VendaEntity,
        items: Vec<VendaItemEntity>,
        payments: Vec<VendaPagamentoEntity>,
    ) -> Result<i64, String> {
        let db = SqliteDbService::get_instance()?;
        let mut conn = db.get_connection()?;

        let tx = conn.transaction()
            .map_err(|e| format!("Failed to start transaction: {}", e))?;

        // Insere a venda
        tx.execute(
            "INSERT INTO vendas (tip, mod, serie_origin, serie, nr_nf_origin, nr_nf, cnpj, 
             doc_destinatario, dh_emi, dh_emi_canc, total, addition, discount, chave, chave_canc, 
             file_path, cancel_file_path, protocolo, cancelled, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)",
            params![
                venda.tip,
                venda.mod_,
                venda.serie_origin,
                venda.serie,
                venda.nr_nf_origin,
                venda.nr_nf,
                venda.cnpj,
                venda.doc_destinatario,
                venda.dh_emi,
                venda.dh_emi_canc,
                venda.total,
                venda.addition,
                venda.discount,
                venda.chave,
                venda.chave_canc,
                venda.file_path,
                venda.cancel_file_path,
                venda.protocolo,
                venda.cancelled,
                venda.created_at.to_rfc3339(),
                venda.updated_at.to_rfc3339()
            ],
        ).map_err(|e| format!("Failed to insert venda: {}", e))?;

        let venda_id = tx.last_insert_rowid();

        // Insere os itens
        for item in items {
            Self::insert_item_in_transaction(&tx, venda_id, &item)?;
        }

        // Insere os pagamentos
        for payment in payments {
            Self::insert_payment_in_transaction(&tx, venda_id, &payment)?;
        }

        tx.commit()
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;

        Ok(venda_id)
    }

    /// Insere um item de venda dentro de uma transação
    fn insert_item_in_transaction(
        tx: &Transaction,
        venda_id: i64,
        item: &VendaItemEntity,
    ) -> Result<(), String> {
        tx.execute(
            "INSERT INTO venda_itens (venda_id, produto_code, produto_description, produto_medida, 
             quantidade, preco_unitario, desconto, desconto_rat, acrescimo, acrescimo_rat, 
             preco_total, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                venda_id,
                item.produto_code,
                item.produto_description,
                item.produto_medida,
                item.quantidade,
                item.preco_unitario,
                item.desconto,
                item.desconto_rat,
                item.acrescimo,
                item.acrescimo_rat,
                item.preco_total,
                item.created_at.to_rfc3339(),
                item.updated_at.to_rfc3339()
            ],
        ).map_err(|e| format!("Failed to insert venda_item: {}", e))?;

        Ok(())
    }

    /// Insere um pagamento dentro de uma transação
    fn insert_payment_in_transaction(
        tx: &Transaction,
        venda_id: i64,
        payment: &VendaPagamentoEntity,
    ) -> Result<(), String> {
        tx.execute(
            "INSERT INTO venda_pagamentos (venda_id, code, name, total_pagamento, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                venda_id,
                payment.code,
                payment.name,
                payment.total_pagamento,
                payment.created_at.to_rfc3339(),
                payment.updated_at.to_rfc3339()
            ],
        ).map_err(|e| format!("Failed to insert venda_pagamento: {}", e))?;

        Ok(())
    }

    /// Busca itens de uma venda
    pub fn find_items_by_venda_id(venda_id: i64) -> Result<Vec<VendaItemEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, produto_code, produto_description, produto_medida, quantidade, preco_unitario, 
             desconto, desconto_rat, acrescimo, acrescimo_rat, preco_total, created_at, updated_at 
             FROM venda_itens WHERE venda_id = ?1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let items = stmt.query_map(params![venda_id], |row| {
            let created_at_str: String = row.get(11)?;
            let updated_at_str: String = row.get(12)?;
            
            Ok(VendaItemEntity {
                id: row.get(0)?,
                venda_id,
                produto_code: row.get(1)?,
                produto_description: row.get(2)?,
                produto_medida: row.get(3)?,
                quantidade: row.get(4)?,
                preco_unitario: row.get(5)?,
                desconto: row.get(6)?,
                desconto_rat: row.get(7)?,
                acrescimo: row.get(8)?,
                acrescimo_rat: row.get(9)?,
                preco_total: row.get(10)?,
                created_at: created_at_str.parse().unwrap_or(Utc::now()),
                updated_at: updated_at_str.parse().unwrap_or(Utc::now()),
            })
        })
        .map_err(|e| format!("Failed to query venda_items: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect venda_items: {}", e))?;

        Ok(items)
    }

    /// Busca pagamentos de uma venda
    pub fn find_payments_by_venda_id(venda_id: i64) -> Result<Vec<VendaPagamentoEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, code, name, total_pagamento, created_at, updated_at 
             FROM venda_pagamentos WHERE venda_id = ?1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let payments = stmt.query_map(params![venda_id], |row| {
            let created_at_str: String = row.get(4)?;
            let updated_at_str: String = row.get(5)?;
            
            Ok(VendaPagamentoEntity {
                id: row.get(0)?,
                venda_id,
                code: row.get(1)?,
                name: row.get(2)?,
                total_pagamento: row.get(3)?,
                created_at: created_at_str.parse().unwrap_or(Utc::now()),
                updated_at: updated_at_str.parse().unwrap_or(Utc::now()),
            })
        })
        .map_err(|e| format!("Failed to query venda_pagamentos: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect venda_pagamentos: {}", e))?;

        Ok(payments)
    }

    /// Busca vendas por intervalo de datas
    pub fn get_vendas_by_interval(dt_init: &str, dt_end: &str) -> Result<Vec<VendaWithRelations>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, tip, mod, serie_origin, serie, nr_nf_origin, nr_nf, cnpj, doc_destinatario, 
             dh_emi, dh_emi_canc, total, addition, discount, chave, chave_canc, file_path, 
             cancel_file_path, protocolo, cancelled, created_at, updated_at 
             FROM vendas 
             WHERE DATE(dh_emi) BETWEEN ?1 AND ?2 
             ORDER BY dh_emi DESC"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let vendas = stmt.query_map(params![dt_init, dt_end], |row| {
            let created_at_str: String = row.get(20)?;
            let updated_at_str: String = row.get(21)?;
            
            Ok(VendaEntity {
                id: row.get(0)?,
                tip: row.get(1)?,
                mod_: row.get(2)?,
                serie_origin: row.get(3)?,
                serie: row.get(4)?,
                nr_nf_origin: row.get(5)?,
                nr_nf: row.get(6)?,
                cnpj: row.get(7)?,
                doc_destinatario: row.get(8)?,
                dh_emi: row.get(9)?,
                dh_emi_canc: row.get(10)?,
                total: row.get(11)?,
                addition: row.get(12)?,
                discount: row.get(13)?,
                chave: row.get(14)?,
                chave_canc: row.get(15)?,
                file_path: row.get(16)?,
                cancel_file_path: row.get(17)?,
                protocolo: row.get(18)?,
                cancelled: row.get(19)?,
                created_at: created_at_str.parse().unwrap_or(Utc::now()),
                updated_at: updated_at_str.parse().unwrap_or(Utc::now()),
            })
        })
        .map_err(|e| format!("Failed to query vendas: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect vendas: {}", e))?;

        let mut result = Vec::new();
        for venda in vendas {
            let venda_id = venda.id.unwrap_or(0);
            let itens = Self::find_items_by_venda_id(venda_id)?;
            let pagamentos = Self::find_payments_by_venda_id(venda_id)?;
            
            result.push(VendaWithRelations {
                venda,
                itens,
                pagamentos,
            });
        }

        Ok(result)
    }

    /// Busca itens de vendas por intervalo de datas
    pub fn get_items_by_interval(dt_init: &str, dt_end: &str) -> Result<Vec<VendaItemEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT vi.id, vi.venda_id, vi.produto_code, vi.produto_description, vi.produto_medida, 
             vi.quantidade, vi.preco_unitario, vi.desconto, vi.desconto_rat, vi.acrescimo, 
             vi.acrescimo_rat, vi.preco_total, vi.created_at, vi.updated_at 
             FROM venda_itens vi
             INNER JOIN vendas v ON vi.venda_id = v.id
             WHERE DATE(v.dh_emi) BETWEEN ?1 AND ?2
             ORDER BY v.dh_emi DESC, vi.id"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let items = stmt.query_map(params![dt_init, dt_end], |row| {
            let created_at_str: String = row.get(12)?;
            let updated_at_str: String = row.get(13)?;
            
            Ok(VendaItemEntity {
                id: row.get(0)?,
                venda_id: row.get(1)?,
                produto_code: row.get(2)?,
                produto_description: row.get(3)?,
                produto_medida: row.get(4)?,
                quantidade: row.get(5)?,
                preco_unitario: row.get(6)?,
                desconto: row.get(7)?,
                desconto_rat: row.get(8)?,
                acrescimo: row.get(9)?,
                acrescimo_rat: row.get(10)?,
                preco_total: row.get(11)?,
                created_at: created_at_str.parse().unwrap_or(Utc::now()),
                updated_at: updated_at_str.parse().unwrap_or(Utc::now()),
            })
        })
        .map_err(|e| format!("Failed to query items: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect items: {}", e))?;

        Ok(items)
    }

    /// Busca pagamentos de vendas por intervalo de datas
    pub fn get_payments_by_interval(dt_init: &str, dt_end: &str) -> Result<Vec<VendaPagamentoEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT vp.id, vp.venda_id, vp.code, vp.name, vp.total_pagamento, vp.created_at, vp.updated_at 
             FROM venda_pagamentos vp
             INNER JOIN vendas v ON vp.venda_id = v.id
             WHERE DATE(v.dh_emi) BETWEEN ?1 AND ?2
             ORDER BY v.dh_emi DESC, vp.id"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let payments = stmt.query_map(params![dt_init, dt_end], |row| {
            let created_at_str: String = row.get(5)?;
            let updated_at_str: String = row.get(6)?;
            
            Ok(VendaPagamentoEntity {
                id: row.get(0)?,
                venda_id: row.get(1)?,
                code: row.get(2)?,
                name: row.get(3)?,
                total_pagamento: row.get(4)?,
                created_at: created_at_str.parse().unwrap_or(Utc::now()),
                updated_at: updated_at_str.parse().unwrap_or(Utc::now()),
            })
        })
        .map_err(|e| format!("Failed to query payments: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect payments: {}", e))?;

        Ok(payments)
    }

    /// Busca resumo de vendas por intervalo de datas
    pub fn get_resumo_by_interval(dt_init: &str, dt_end: &str) -> Result<VendaResumo, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT 
                COUNT(*) as total_vendas,
                COALESCE(SUM(total), 0) as total_valor,
                COALESCE(SUM(discount), 0) as total_desconto,
                COALESCE(SUM(addition), 0) as total_acrescimo,
                COALESCE(SUM(CASE WHEN cancelled = 1 THEN 1 ELSE 0 END), 0) as total_canceladas
             FROM vendas
             WHERE DATE(dh_emi) BETWEEN ?1 AND ?2"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let resumo = stmt.query_row(params![dt_init, dt_end], |row| {
            Ok(VendaResumo {
                total_vendas: row.get(0)?,
                total_valor: row.get(1)?,
                total_desconto: row.get(2)?,
                total_acrescimo: row.get(3)?,
                total_canceladas: row.get(4)?,
            })
        })
        .map_err(|e| format!("Failed to query resumo: {}", e))?;

        Ok(resumo)
    }

    /// Atualiza o status de cancelamento de uma venda
    pub fn cancel_venda(venda_id: i64, chave_canc: String, dh_emi_canc: String, cancel_file_path: Option<String>) -> Result<(), String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        conn.execute(
            "UPDATE vendas SET cancelled = 1, chave_canc = ?1, dh_emi_canc = ?2, cancel_file_path = ?3, updated_at = ?4 WHERE id = ?5",
            params![chave_canc, dh_emi_canc, cancel_file_path, Utc::now().to_rfc3339(), venda_id],
        ).map_err(|e| format!("Failed to cancel venda: {}", e))?;

        Ok(())
    }
}
