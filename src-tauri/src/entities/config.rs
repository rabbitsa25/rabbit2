use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEntity {
    pub id: String,
    pub flow_base_url: String,
    pub code_uf: i32,
    pub nserie_sat: String,
    pub nserie_sat_nao: Option<String>,
    pub nr_nf_sim: i32,
    pub nr_nf_nao: i32,
    pub sign_ac: Option<String>,
    pub regime_tributario: String, // '1', '2', or '3'
    pub cnpj: String,
    pub name: String,
    pub short_name: Option<String>,
    pub zipcode: String,
    pub address_name: String,
    pub address_number: String,
    pub address_city: String,
    pub address_city_code: Option<String>,
    pub tipo_ambiente: String, // '1' or '2'
    pub address_cpl: Option<String>,
    pub address_neiborhood: String,
    pub address_state: String,
    pub fone: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub percent_s: i32,
    pub only_money: i32, // 0 or 1
    pub error_as_success: i32, // 0 or 1
    pub ie: Option<String>,
    pub pagamentos: Option<String>, // JSON string of EPagamento[]
    pub ignore_cpf: i32,
    pub numero_caixa: i32,
    pub emitir_l: i32,
    pub habilitar_contador: i32,
    pub habilitar_contador_nao: i32,
    pub controle_estoque: i32,
    pub modelo: i32,
}

impl Default for ConfigEntity {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            flow_base_url: String::new(),
            code_uf: 35,
            nserie_sat: "123456789".to_string(),
            nserie_sat_nao: None,
            nr_nf_sim: 0,
            nr_nf_nao: 0,
            sign_ac: None,
            regime_tributario: "1".to_string(),
            cnpj: "28095955000199".to_string(),
            name: "EMPRESA TESTE".to_string(),
            short_name: None,
            zipcode: "00000000".to_string(),
            address_name: "AV Paulista".to_string(),
            address_number: "2000".to_string(),
            address_city: "São Paulo".to_string(),
            address_city_code: None,
            tipo_ambiente: "1".to_string(),
            address_cpl: None,
            address_neiborhood: "Consolação".to_string(),
            address_state: "SP".to_string(),
            fone: None,
            created_at: 0,
            updated_at: 0,
            percent_s: 50,
            only_money: 0,
            error_as_success: 0,
            ie: None,
            pagamentos: None,
            ignore_cpf: 0,
            numero_caixa: 0,
            emitir_l: 0,
            habilitar_contador: 0,
            habilitar_contador_nao: 0,
            controle_estoque: 0,
            modelo: 59,
        }
    }
}
