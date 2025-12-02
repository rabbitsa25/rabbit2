use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EPagamento {
    pub id: String,
    pub nome: String,
    pub ativo: bool,
}
