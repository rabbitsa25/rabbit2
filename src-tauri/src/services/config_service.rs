use crate::database::SqliteDbService;
use crate::entities::ConfigEntity;
use rusqlite::{params, Result};

pub struct ConfigService;

impl ConfigService {
    /// Busca uma configuração por ID
    pub fn find_by_id(id: &str) -> Result<Option<ConfigEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, flowBaseUrl, codeUf, nserieSAT, nserieSATNao, nrNfSim, nrNfNao, 
                    signAC, regimeTributario, cnpj, name, shortName, zipcode, addressName, 
                    addressNumber, addressCity, addressCityCode, tipoAmbiente, addressCpl, 
                    addressNeiborhood, addressState, fone, createdAt, updatedAt, percentS, 
                    onlyMoney, errorAsSuccess, ie, pagamentos, ignoreCpf, numeroCaixa, 
                    emitirL, habilitarContador, habilitarContadorNao, controleEstoque, modelo
             FROM config WHERE id = ?1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let config = stmt.query_row(params![id], |row| {
            Ok(ConfigEntity {
                id: row.get(0)?,
                flow_base_url: row.get(1)?,
                code_uf: row.get(2)?,
                nserie_sat: row.get(3)?,
                nserie_sat_nao: row.get(4)?,
                nr_nf_sim: row.get(5)?,
                nr_nf_nao: row.get(6)?,
                sign_ac: row.get(7)?,
                regime_tributario: row.get(8)?,
                cnpj: row.get(9)?,
                name: row.get(10)?,
                short_name: row.get(11)?,
                zipcode: row.get(12)?,
                address_name: row.get(13)?,
                address_number: row.get(14)?,
                address_city: row.get(15)?,
                address_city_code: row.get(16)?,
                tipo_ambiente: row.get(17)?,
                address_cpl: row.get(18)?,
                address_neiborhood: row.get(19)?,
                address_state: row.get(20)?,
                fone: row.get(21)?,
                created_at: row.get(22)?,
                updated_at: row.get(23)?,
                percent_s: row.get(24)?,
                only_money: row.get(25)?,
                error_as_success: row.get(26)?,
                ie: row.get(27)?,
                pagamentos: row.get(28)?,
                ignore_cpf: row.get(29)?,
                numero_caixa: row.get(30)?,
                emitir_l: row.get(31)?,
                habilitar_contador: row.get(32)?,
                habilitar_contador_nao: row.get(33)?,
                controle_estoque: row.get(34)?,
                modelo: row.get(35)?,
            })
        });

        match config {
            Ok(c) => Ok(Some(c)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to query config: {}", e)),
        }
    }

    /// Salva ou atualiza uma configuração
    pub fn save(config: &ConfigEntity) -> Result<ConfigEntity, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        // Verifica se já existe
        let exists: bool = conn.query_row(
            "SELECT COUNT(*) FROM config WHERE id = ?1",
            params![config.id],
            |row| {
                let count: i32 = row.get(0)?;
                Ok(count > 0)
            }
        ).map_err(|e| format!("Failed to check config existence: {}", e))?;

        if exists {
            // Update
            conn.execute(
                "UPDATE config SET flowBaseUrl = ?1, codeUf = ?2, nserieSAT = ?3, nserieSATNao = ?4, 
                        nrNfSim = ?5, nrNfNao = ?6, signAC = ?7, regimeTributario = ?8, cnpj = ?9, 
                        name = ?10, shortName = ?11, zipcode = ?12, addressName = ?13, addressNumber = ?14, 
                        addressCity = ?15, addressCityCode = ?16, tipoAmbiente = ?17, addressCpl = ?18, 
                        addressNeiborhood = ?19, addressState = ?20, fone = ?21, updatedAt = ?22, 
                        percentS = ?23, onlyMoney = ?24, errorAsSuccess = ?25, ie = ?26, pagamentos = ?27, 
                        ignoreCpf = ?28, numeroCaixa = ?29, emitirL = ?30, habilitarContador = ?31, 
                        habilitarContadorNao = ?32, controleEstoque = ?33, modelo = ?34 
                 WHERE id = ?35",
                params![
                    config.flow_base_url, config.code_uf, config.nserie_sat, config.nserie_sat_nao,
                    config.nr_nf_sim, config.nr_nf_nao, config.sign_ac, config.regime_tributario,
                    config.cnpj, config.name, config.short_name, config.zipcode, config.address_name,
                    config.address_number, config.address_city, config.address_city_code, config.tipo_ambiente,
                    config.address_cpl, config.address_neiborhood, config.address_state, config.fone,
                    config.updated_at, config.percent_s, config.only_money, config.error_as_success,
                    config.ie, config.pagamentos, config.ignore_cpf, config.numero_caixa, config.emitir_l,
                    config.habilitar_contador, config.habilitar_contador_nao, config.controle_estoque,
                    config.modelo, config.id
                ],
            ).map_err(|e| format!("Failed to update config: {}", e))?;
        } else {
            // Insert
            conn.execute(
                "INSERT INTO config (id, flowBaseUrl, codeUf, nserieSAT, nserieSATNao, nrNfSim, nrNfNao, 
                        signAC, regimeTributario, cnpj, name, shortName, zipcode, addressName, addressNumber, 
                        addressCity, addressCityCode, tipoAmbiente, addressCpl, addressNeiborhood, addressState, 
                        fone, createdAt, updatedAt, percentS, onlyMoney, errorAsSuccess, ie, pagamentos, 
                        ignoreCpf, numeroCaixa, emitirL, habilitarContador, habilitarContadorNao, 
                        controleEstoque, modelo) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, 
                         ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36)",
                params![
                    config.id, config.flow_base_url, config.code_uf, config.nserie_sat, config.nserie_sat_nao,
                    config.nr_nf_sim, config.nr_nf_nao, config.sign_ac, config.regime_tributario,
                    config.cnpj, config.name, config.short_name, config.zipcode, config.address_name,
                    config.address_number, config.address_city, config.address_city_code, config.tipo_ambiente,
                    config.address_cpl, config.address_neiborhood, config.address_state, config.fone,
                    config.created_at, config.updated_at, config.percent_s, config.only_money,
                    config.error_as_success, config.ie, config.pagamentos, config.ignore_cpf,
                    config.numero_caixa, config.emitir_l, config.habilitar_contador,
                    config.habilitar_contador_nao, config.controle_estoque, config.modelo
                ],
            ).map_err(|e| format!("Failed to insert config: {}", e))?;
        }

        Ok(config.clone())
    }

    /// Lista todas as configurações
    pub fn find_all() -> Result<Vec<ConfigEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, flowBaseUrl, codeUf, nserieSAT, nserieSATNao, nrNfSim, nrNfNao, 
                    signAC, regimeTributario, cnpj, name, shortName, zipcode, addressName, 
                    addressNumber, addressCity, addressCityCode, tipoAmbiente, addressCpl, 
                    addressNeiborhood, addressState, fone, createdAt, updatedAt, percentS, 
                    onlyMoney, errorAsSuccess, ie, pagamentos, ignoreCpf, numeroCaixa, 
                    emitirL, habilitarContador, habilitarContadorNao, controleEstoque, modelo
             FROM config"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let configs = stmt.query_map([], |row| {
            Ok(ConfigEntity {
                id: row.get(0)?,
                flow_base_url: row.get(1)?,
                code_uf: row.get(2)?,
                nserie_sat: row.get(3)?,
                nserie_sat_nao: row.get(4)?,
                nr_nf_sim: row.get(5)?,
                nr_nf_nao: row.get(6)?,
                sign_ac: row.get(7)?,
                regime_tributario: row.get(8)?,
                cnpj: row.get(9)?,
                name: row.get(10)?,
                short_name: row.get(11)?,
                zipcode: row.get(12)?,
                address_name: row.get(13)?,
                address_number: row.get(14)?,
                address_city: row.get(15)?,
                address_city_code: row.get(16)?,
                tipo_ambiente: row.get(17)?,
                address_cpl: row.get(18)?,
                address_neiborhood: row.get(19)?,
                address_state: row.get(20)?,
                fone: row.get(21)?,
                created_at: row.get(22)?,
                updated_at: row.get(23)?,
                percent_s: row.get(24)?,
                only_money: row.get(25)?,
                error_as_success: row.get(26)?,
                ie: row.get(27)?,
                pagamentos: row.get(28)?,
                ignore_cpf: row.get(29)?,
                numero_caixa: row.get(30)?,
                emitir_l: row.get(31)?,
                habilitar_contador: row.get(32)?,
                habilitar_contador_nao: row.get(33)?,
                controle_estoque: row.get(34)?,
                modelo: row.get(35)?,
            })
        })
        .map_err(|e| format!("Failed to query configs: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect configs: {}", e))?;

        Ok(configs)
    }

    /// Deleta uma configuração por ID
    pub fn delete_by_id(id: &str) -> Result<(), String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        conn.execute("DELETE FROM config WHERE id = ?1", params![id])
            .map_err(|e| format!("Failed to delete config: {}", e))?;

        Ok(())
    }
}
