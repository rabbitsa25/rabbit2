use crate::entities::ConfigEntity;
use crate::services::ConfigService;
use crate::dtos::{CreateOrUpdateConfigDto, UpdatePercentConfigDto, CnpjResponseDto};

pub struct GetFirstConfigUseCase;

impl GetFirstConfigUseCase {
    /// Busca a primeira configuração (default)
    pub fn execute() -> Result<Option<ConfigEntity>, String> {
        // Busca pela configuração com id "default"
        ConfigService::find_by_id("default")
    }
}

pub struct CreateOrUpdateConfigUseCase;

impl CreateOrUpdateConfigUseCase {
    /// Cria ou atualiza uma configuração
    pub fn execute(dto: CreateOrUpdateConfigDto) -> Result<ConfigEntity, String> {
        let id = dto.id.unwrap_or_else(|| "default".to_string());
        
        // Busca a configuração existente ou cria uma nova com valores padrão
        let mut config = ConfigService::find_by_id(&id)?
            .unwrap_or_else(|| ConfigEntity::default());
        
        // Atualiza os campos fornecidos no DTO
        if let Some(flow_base_url) = dto.flow_base_url {
            config.flow_base_url = flow_base_url;
        }
        if let Some(code_uf) = dto.code_uf {
            config.code_uf = code_uf;
        }
        if let Some(nserie_sat) = dto.nserie_sat {
            config.nserie_sat = nserie_sat;
        }
        if let Some(nserie_sat_nao) = dto.nserie_sat_nao {
            config.nserie_sat_nao = Some(nserie_sat_nao);
        }
        if let Some(nr_nf_sim) = dto.nr_nf_sim {
            config.nr_nf_sim = nr_nf_sim;
        }
        if let Some(nr_nf_nao) = dto.nr_nf_nao {
            config.nr_nf_nao = nr_nf_nao;
        }
        if let Some(sign_ac) = dto.sign_ac {
            config.sign_ac = Some(sign_ac);
        }
        if let Some(regime_tributario) = dto.regime_tributario {
            config.regime_tributario = regime_tributario;
        }
        if let Some(cnpj) = dto.cnpj {
            config.cnpj = cnpj;
        }
        if let Some(name) = dto.name {
            config.name = name;
        }
        if let Some(short_name) = dto.short_name {
            config.short_name = Some(short_name);
        }
        if let Some(zipcode) = dto.zipcode {
            config.zipcode = zipcode;
        }
        if let Some(address_name) = dto.address_name {
            config.address_name = address_name;
        }
        if let Some(address_number) = dto.address_number {
            config.address_number = address_number;
        }
        if let Some(address_city) = dto.address_city {
            config.address_city = address_city;
        }
        if let Some(address_city_code) = dto.address_city_code {
            config.address_city_code = Some(address_city_code);
        }
        if let Some(tipo_ambiente) = dto.tipo_ambiente {
            config.tipo_ambiente = tipo_ambiente;
        }
        if let Some(address_cpl) = dto.address_cpl {
            config.address_cpl = Some(address_cpl);
        }
        if let Some(address_neiborhood) = dto.address_neiborhood {
            config.address_neiborhood = address_neiborhood;
        }
        if let Some(address_state) = dto.address_state {
            config.address_state = address_state;
        }
        if let Some(fone) = dto.fone {
            config.fone = Some(fone);
        }
        if let Some(percent_s) = dto.percent_s {
            config.percent_s = percent_s;
        }
        if let Some(only_money) = dto.only_money {
            config.only_money = only_money;
        }
        if let Some(error_as_success) = dto.error_as_success {
            config.error_as_success = error_as_success;
        }
        if let Some(ie) = dto.ie {
            config.ie = Some(ie);
        }
        if let Some(pagamentos) = dto.pagamentos {
            config.pagamentos = Some(pagamentos);
        }
        if let Some(ignore_cpf) = dto.ignore_cpf {
            config.ignore_cpf = ignore_cpf;
        }
        if let Some(numero_caixa) = dto.numero_caixa {
            config.numero_caixa = numero_caixa;
        }
        if let Some(emitir_l) = dto.emitir_l {
            config.emitir_l = emitir_l;
        }
        if let Some(habilitar_contador) = dto.habilitar_contador {
            config.habilitar_contador = habilitar_contador;
        }
        if let Some(habilitar_contador_nao) = dto.habilitar_contador_nao {
            config.habilitar_contador_nao = habilitar_contador_nao;
        }
        if let Some(controle_estoque) = dto.controle_estoque {
            config.controle_estoque = controle_estoque;
        }
        if let Some(modelo) = dto.modelo {
            config.modelo = modelo;
        }

        // Atualiza o timestamp
        config.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        // Se for uma nova configuração, define o created_at
        if config.id.is_empty() {
            config.id = id;
            config.created_at = config.updated_at;
        }

        ConfigService::save(&config)
    }
}

pub struct UpdatePercentUseCase;

impl UpdatePercentUseCase {
    /// Atualiza apenas o percentual de desconto da configuração
    pub fn execute(dto: UpdatePercentConfigDto) -> Result<ConfigEntity, String> {
        let mut config = ConfigService::find_by_id("default")?
            .ok_or_else(|| "Configuração não encontrada".to_string())?;
        
        config.percent_s = dto.percent_s;
        config.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        ConfigService::save(&config)
    }
}

pub struct GetCnpjUseCase;

impl GetCnpjUseCase {
    /// Consulta informações de um CNPJ em API externa
    pub async fn execute(cnpj: String) -> Result<CnpjResponseDto, String> {
        // Remove caracteres não numéricos do CNPJ
        let cnpj_clean = cnpj.chars().filter(|c| c.is_numeric()).collect::<String>();
        
        if cnpj_clean.len() != 14 {
            return Err("CNPJ inválido".to_string());
        }

        // Consulta API pública de CNPJ (exemplo: ReceitaWS)
        let url = format!("https://www.receitaws.com.br/v1/cnpj/{}", cnpj_clean);
        
        let response = reqwest::get(&url)
            .await
            .map_err(|e| format!("Erro ao consultar CNPJ: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Erro na consulta: status {}", response.status()));
        }

        let json: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Erro ao parsear resposta: {}", e))?;

        // Verifica se houve erro na API
        if let Some(status) = json.get("status").and_then(|s| s.as_str()) {
            if status == "ERROR" {
                let message = json.get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Erro desconhecido");
                return Err(message.to_string());
            }
        }

        // Mapeia a resposta para o DTO
        Ok(CnpjResponseDto {
            cnpj: json.get("cnpj")
                .and_then(|v| v.as_str())
                .unwrap_or(&cnpj_clean)
                .to_string(),
            razao_social: json.get("nome")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            nome_fantasia: json.get("fantasia")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            logradouro: json.get("logradouro")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            numero: json.get("numero")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            complemento: json.get("complemento")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            bairro: json.get("bairro")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            municipio: json.get("municipio")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            uf: json.get("uf")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            cep: json.get("cep")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            telefone: json.get("telefone")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            email: json.get("email")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        })
    }
}
