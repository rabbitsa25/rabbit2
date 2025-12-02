use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentTypes {
    #[serde(rename = "01")]
    Dinheiro,
    #[serde(rename = "02")]
    Cheque,
    #[serde(rename = "03")]
    CartaoDeCredito,
    #[serde(rename = "04")]
    CartaoDeDebito,
    #[serde(rename = "05")]
    CreditoLoja,
    #[serde(rename = "10")]
    ValeAlimentacao,
    #[serde(rename = "11")]
    ValeRefeicao,
    #[serde(rename = "12")]
    ValePresente,
    #[serde(rename = "13")]
    ValeCombustivel,
    #[serde(rename = "14")]
    DuplicataMercantil,
    #[serde(rename = "15")]
    BoletoBancario,
    #[serde(rename = "90")]
    SemPagamento,
    #[serde(rename = "99")]
    Outros,
}

impl PaymentTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentTypes::Dinheiro => "01",
            PaymentTypes::Cheque => "02",
            PaymentTypes::CartaoDeCredito => "03",
            PaymentTypes::CartaoDeDebito => "04",
            PaymentTypes::CreditoLoja => "05",
            PaymentTypes::ValeAlimentacao => "10",
            PaymentTypes::ValeRefeicao => "11",
            PaymentTypes::ValePresente => "12",
            PaymentTypes::ValeCombustivel => "13",
            PaymentTypes::DuplicataMercantil => "14",
            PaymentTypes::BoletoBancario => "15",
            PaymentTypes::SemPagamento => "90",
            PaymentTypes::Outros => "99",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "01" => Some(PaymentTypes::Dinheiro),
            "02" => Some(PaymentTypes::Cheque),
            "03" => Some(PaymentTypes::CartaoDeCredito),
            "04" => Some(PaymentTypes::CartaoDeDebito),
            "05" => Some(PaymentTypes::CreditoLoja),
            "10" => Some(PaymentTypes::ValeAlimentacao),
            "11" => Some(PaymentTypes::ValeRefeicao),
            "12" => Some(PaymentTypes::ValePresente),
            "13" => Some(PaymentTypes::ValeCombustivel),
            "14" => Some(PaymentTypes::DuplicataMercantil),
            "15" => Some(PaymentTypes::BoletoBancario),
            "90" => Some(PaymentTypes::SemPagamento),
            "99" => Some(PaymentTypes::Outros),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeEntity {
    pub id: String,
    pub code: PaymentTypes,
    pub amount_s: f64,
    pub amount_n: f64,
    pub updated_at: i64,
    pub created_at: i64,
}

impl ResumeEntity {
    pub fn new(code: PaymentTypes) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            id: Uuid::new_v4().to_string(),
            code,
            amount_s: 0.0,
            amount_n: 0.0,
            updated_at: now,
            created_at: now,
        }
    }
}
