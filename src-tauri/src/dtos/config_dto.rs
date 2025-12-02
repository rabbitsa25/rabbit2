use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrUpdateConfigDto {
    pub id: Option<String>,
    pub flow_base_url: Option<String>,
    pub code_uf: Option<i32>,
    pub nserie_sat: Option<String>,
    pub nserie_sat_nao: Option<String>,
    pub nr_nf_sim: Option<i32>,
    pub nr_nf_nao: Option<i32>,
    pub sign_ac: Option<String>,
    pub regime_tributario: Option<String>,
    pub cnpj: Option<String>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub zipcode: Option<String>,
    pub address_name: Option<String>,
    pub address_number: Option<String>,
    pub address_city: Option<String>,
    pub address_city_code: Option<String>,
    pub tipo_ambiente: Option<String>,
    pub address_cpl: Option<String>,
    pub address_neiborhood: Option<String>,
    pub address_state: Option<String>,
    pub fone: Option<String>,
    pub percent_s: Option<i32>,
    pub only_money: Option<i32>,
    pub error_as_success: Option<i32>,
    pub ie: Option<String>,
    pub pagamentos: Option<String>,
    pub ignore_cpf: Option<i32>,
    pub numero_caixa: Option<i32>,
    pub emitir_l: Option<i32>,
    pub habilitar_contador: Option<i32>,
    pub habilitar_contador_nao: Option<i32>,
    pub controle_estoque: Option<i32>,
    pub modelo: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePercentConfigDto {
    pub percent_s: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CnpjResponseDto {
    pub cnpj: String,
    pub razao_social: Option<String>,
    pub nome_fantasia: Option<String>,
    pub logradouro: Option<String>,
    pub numero: Option<String>,
    pub complemento: Option<String>,
    pub bairro: Option<String>,
    pub municipio: Option<String>,
    pub uf: Option<String>,
    pub cep: Option<String>,
    pub telefone: Option<String>,
    pub email: Option<String>,
}
