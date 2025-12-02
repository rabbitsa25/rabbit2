use crate::database::SqliteDbService;
use crate::entities::{ResumeEntity, PaymentTypes};
use rusqlite::params;
use chrono::Utc;

pub struct ResumeService;

impl ResumeService {
    /// Busca todos os resumos do dia atual
    pub fn get_all_today() -> Result<Vec<ResumeEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        // Pega o timestamp do início do dia atual (00:00:00)
        let today_start = Utc::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis();

        let mut stmt = conn.prepare(
            "SELECT id, code, amount_s, amount_n, updated_at, created_at 
             FROM resumes 
             WHERE created_at >= ?1
             ORDER BY code"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let resumes = stmt.query_map(params![today_start], |row| {
            let code_str: String = row.get(1)?;
            let code = PaymentTypes::from_str(&code_str)
                .ok_or(rusqlite::Error::InvalidQuery)?;
            
            Ok(ResumeEntity {
                id: row.get(0)?,
                code,
                amount_s: row.get(2)?,
                amount_n: row.get(3)?,
                updated_at: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| format!("Failed to query resumes: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect resumes: {}", e))?;

        Ok(resumes)
    }

    /// Busca um resumo por ID
    pub fn find_by_id(id: &str) -> Result<Option<ResumeEntity>, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, code, amount_s, amount_n, updated_at, created_at 
             FROM resumes WHERE id = ?1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let resume = stmt.query_row(params![id], |row| {
            let code_str: String = row.get(1)?;
            let code = PaymentTypes::from_str(&code_str)
                .ok_or(rusqlite::Error::InvalidQuery)?;
            
            Ok(ResumeEntity {
                id: row.get(0)?,
                code,
                amount_s: row.get(2)?,
                amount_n: row.get(3)?,
                updated_at: row.get(4)?,
                created_at: row.get(5)?,
            })
        });

        match resume {
            Ok(r) => Ok(Some(r)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to query resume: {}", e)),
        }
    }

    /// Busca ou cria um resumo por código de pagamento (para o dia atual)
    pub fn find_or_create_by_code(code: PaymentTypes) -> Result<ResumeEntity, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        // Pega o timestamp do início do dia atual
        let today_start = Utc::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis();

        let code_str = code.as_str();

        // Tenta buscar um resumo existente para hoje
        let mut stmt = conn.prepare(
            "SELECT id, code, amount_s, amount_n, updated_at, created_at 
             FROM resumes 
             WHERE code = ?1 AND created_at >= ?2
             LIMIT 1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let existing = stmt.query_row(params![code_str, today_start], |row| {
            let code_str: String = row.get(1)?;
            let code = PaymentTypes::from_str(&code_str)
                .ok_or(rusqlite::Error::InvalidQuery)?;
            
            Ok(ResumeEntity {
                id: row.get(0)?,
                code,
                amount_s: row.get(2)?,
                amount_n: row.get(3)?,
                updated_at: row.get(4)?,
                created_at: row.get(5)?,
            })
        });

        match existing {
            Ok(resume) => Ok(resume),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // Cria um novo resumo
                let new_resume = ResumeEntity::new(code);
                Self::save(&new_resume)?;
                Ok(new_resume)
            }
            Err(e) => Err(format!("Failed to query resume: {}", e)),
        }
    }

    /// Salva ou atualiza um resumo
    pub fn save(resume: &ResumeEntity) -> Result<(), String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        conn.execute(
            "INSERT OR REPLACE INTO resumes (id, code, amount_s, amount_n, updated_at, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                resume.id,
                resume.code.as_str(),
                resume.amount_s,
                resume.amount_n,
                resume.updated_at,
                resume.created_at
            ],
        ).map_err(|e| format!("Failed to save resume: {}", e))?;

        Ok(())
    }

    /// Atualiza os valores de um resumo
    pub fn update_amounts(id: &str, amount_s: f64, amount_n: f64) -> Result<(), String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let updated_at = Utc::now().timestamp_millis();

        conn.execute(
            "UPDATE resumes SET amount_s = ?1, amount_n = ?2, updated_at = ?3 WHERE id = ?4",
            params![amount_s, amount_n, updated_at, id],
        ).map_err(|e| format!("Failed to update resume: {}", e))?;

        Ok(())
    }

    /// Incrementa os valores de um resumo
    pub fn increment_amounts(id: &str, amount_s_inc: f64, amount_n_inc: f64) -> Result<(), String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let updated_at = Utc::now().timestamp_millis();

        conn.execute(
            "UPDATE resumes 
             SET amount_s = amount_s + ?1, 
                 amount_n = amount_n + ?2, 
                 updated_at = ?3 
             WHERE id = ?4",
            params![amount_s_inc, amount_n_inc, updated_at, id],
        ).map_err(|e| format!("Failed to increment resume amounts: {}", e))?;

        Ok(())
    }

    /// Deleta resumos antigos (opcional - manutenção)
    pub fn delete_old_resumes(days_old: i64) -> Result<usize, String> {
        let db = SqliteDbService::get_instance()?;
        let conn = db.get_connection()?;

        let cutoff_timestamp = Utc::now()
            .timestamp_millis() - (days_old * 24 * 60 * 60 * 1000);

        let deleted = conn.execute(
            "DELETE FROM resumes WHERE created_at < ?1",
            params![cutoff_timestamp],
        ).map_err(|e| format!("Failed to delete old resumes: {}", e))?;

        Ok(deleted)
    }
}
