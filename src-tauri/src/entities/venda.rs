use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendaEntity {
    pub id: Option<i64>,
    pub tip: i32,
    pub mod_: i32,
    pub serie_origin: String,
    pub serie: String,
    pub nr_nf_origin: i32,
    pub nr_nf: i32,
    pub cnpj: String,
    pub doc_destinatario: Option<String>,
    pub dh_emi: String,
    pub dh_emi_canc: Option<String>,
    pub total: f64,
    pub addition: f64,
    pub discount: f64,
    pub chave: String,
    pub chave_canc: Option<String>,
    pub file_path: Option<String>,
    pub cancel_file_path: Option<String>,
    pub protocolo: Option<String>,
    pub cancelled: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VendaEntity {
    pub fn new(
        tip: i32,
        mod_: i32,
        serie: String,
        nr_nf: i32,
        cnpj: String,
        dh_emi: String,
        total: f64,
        chave: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            tip,
            mod_,
            serie_origin: "0".to_string(),
            serie,
            nr_nf_origin: 0,
            nr_nf,
            cnpj,
            doc_destinatario: None,
            dh_emi,
            dh_emi_canc: None,
            total,
            addition: 0.0,
            discount: 0.0,
            chave,
            chave_canc: None,
            file_path: None,
            cancel_file_path: None,
            protocolo: None,
            cancelled: 0,
            created_at: now,
            updated_at: now,
        }
    }
}
