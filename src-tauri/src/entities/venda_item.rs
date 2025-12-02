use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendaItemEntity {
    pub id: Option<i64>,
    pub venda_id: i64,
    pub produto_code: String,
    pub produto_description: String,
    pub produto_medida: String,
    pub quantidade: f64,
    pub preco_unitario: f64,
    pub desconto: f64,
    pub desconto_rat: f64,
    pub acrescimo: f64,
    pub acrescimo_rat: f64,
    pub preco_total: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VendaItemEntity {
    pub fn new(
        venda_id: i64,
        produto_code: String,
        produto_description: String,
        produto_medida: String,
        quantidade: f64,
        preco_unitario: f64,
    ) -> Self {
        let now = Utc::now();
        let preco_total = quantidade * preco_unitario;
        Self {
            id: None,
            venda_id,
            produto_code,
            produto_description,
            produto_medida,
            quantidade,
            preco_unitario,
            desconto: 0.0,
            desconto_rat: 0.0,
            acrescimo: 0.0,
            acrescimo_rat: 0.0,
            preco_total,
            created_at: now,
            updated_at: now,
        }
    }
}
