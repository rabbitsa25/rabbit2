use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendaPagamentoEntity {
    pub id: Option<i64>,
    pub venda_id: i64,
    pub code: String,
    pub name: String,
    pub total_pagamento: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VendaPagamentoEntity {
    pub fn new(venda_id: i64, code: String, name: String, total_pagamento: f64) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            venda_id,
            code,
            name,
            total_pagamento,
            created_at: now,
            updated_at: now,
        }
    }
}
