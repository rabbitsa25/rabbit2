use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::path::PathBuf;
use directories::ProjectDirs;

lazy_static! {
    static ref DB_INSTANCE: Arc<Mutex<Option<SqliteDbService>>> = Arc::new(Mutex::new(None));
}

#[derive(Clone)]
pub struct SqliteDbService {
    db_path: PathBuf,
}

impl SqliteDbService {
    /// Obtém a instância singleton do serviço de banco de dados
    pub fn get_instance() -> Result<Self, String> {
        let mut instance = DB_INSTANCE.lock().map_err(|e| format!("Failed to lock DB_INSTANCE: {}", e))?;
        
        if instance.is_none() {
            let service = Self::new()?;
            service.initialize()?;
            *instance = Some(service.clone());
        }
        
        Ok(instance.as_ref().unwrap().clone())
    }

    /// Cria uma nova instância do serviço
    fn new() -> Result<Self, String> {
        let db_path = Self::get_database_path()?;
        Ok(Self { db_path })
    }

    /// Determina o caminho do banco de dados baseado no ambiente
    fn get_database_path() -> Result<PathBuf, String> {
        // Verifica se existe variável de ambiente SQLITE_PATH (para produção/Docker)
        if let Ok(custom_path) = std::env::var("SQLITE_PATH") {
            let path = PathBuf::from(custom_path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create database directory: {}", e))?;
            }
            return Ok(path);
        }

        // Para Tauri/Desktop: usa diretório de dados da aplicação
        if let Some(proj_dirs) = ProjectDirs::from("com", "rabbit", "rabbit-api") {
            let data_dir = proj_dirs.data_dir();
            let sqlite_dir = data_dir.join("sqlite");
            
            std::fs::create_dir_all(&sqlite_dir)
                .map_err(|e| format!("Failed to create sqlite directory: {}", e))?;
            
            return Ok(sqlite_dir.join("db.sqlite"));
        }

        // Fallback: diretório atual
        Ok(PathBuf::from("db.sqlite"))
    }

    /// Obtém uma conexão com o banco de dados
    pub fn get_connection(&self) -> Result<Connection, String> {
        Connection::open(&self.db_path)
            .map_err(|e| format!("Failed to open database: {}", e))
    }

    /// Inicializa o banco de dados criando as tabelas
    fn initialize(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        
        // Cria todas as tabelas automaticamente (similar ao synchronize do TypeORM)
        self.create_tables(&conn)?;
        
        Ok(())
    }

    /// Cria todas as tabelas do banco de dados
    fn create_tables(&self, conn: &Connection) -> Result<(), String> {
        // Tabela de configurações
        conn.execute(
            "CREATE TABLE IF NOT EXISTS config (
                id TEXT PRIMARY KEY,
                flowBaseUrl TEXT NOT NULL,
                codeUf INTEGER NOT NULL DEFAULT 35,
                nserieSAT TEXT NOT NULL DEFAULT '123456789',
                nserieSATNao TEXT,
                nrNfSim INTEGER NOT NULL DEFAULT 0,
                nrNfNao INTEGER NOT NULL DEFAULT 0,
                signAC TEXT,
                regimeTributario TEXT NOT NULL DEFAULT '1',
                cnpj TEXT NOT NULL DEFAULT '28095955000199',
                name TEXT NOT NULL DEFAULT 'EMPRESA TESTE',
                shortName TEXT,
                zipcode TEXT NOT NULL DEFAULT '00000000',
                addressName TEXT NOT NULL DEFAULT 'AV Paulista',
                addressNumber TEXT NOT NULL DEFAULT '2000',
                addressCity TEXT NOT NULL DEFAULT 'São Paulo',
                addressCityCode TEXT,
                tipoAmbiente TEXT NOT NULL DEFAULT '1',
                addressCpl TEXT,
                addressNeiborhood TEXT NOT NULL DEFAULT 'Consolação',
                addressState TEXT NOT NULL DEFAULT 'SP',
                fone TEXT,
                createdAt INTEGER NOT NULL,
                updatedAt INTEGER NOT NULL,
                percentS INTEGER NOT NULL DEFAULT 50,
                onlyMoney INTEGER NOT NULL DEFAULT 0,
                errorAsSuccess INTEGER NOT NULL DEFAULT 0,
                ie TEXT,
                pagamentos TEXT,
                ignoreCpf INTEGER NOT NULL DEFAULT 0,
                numeroCaixa INTEGER NOT NULL DEFAULT 0,
                emitirL INTEGER NOT NULL DEFAULT 0,
                habilitarContador INTEGER NOT NULL DEFAULT 0,
                habilitarContadorNao INTEGER NOT NULL DEFAULT 0,
                controleEstoque INTEGER NOT NULL DEFAULT 0,
                modelo INTEGER NOT NULL DEFAULT 59
            )",
            [],
        ).map_err(|e| format!("Failed to create config table: {}", e))?;

        // Tabela de resumos
        conn.execute(
            "CREATE TABLE IF NOT EXISTS resume (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        ).map_err(|e| format!("Failed to create resume table: {}", e))?;

        // Tabela de histórico
        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                action TEXT NOT NULL,
                entity_type TEXT NOT NULL,
                entity_id INTEGER,
                details TEXT,
                created_at TEXT NOT NULL
            )",
            [],
        ).map_err(|e| format!("Failed to create history table: {}", e))?;

        // Tabela de produtos
        conn.execute(
            "CREATE TABLE IF NOT EXISTS product (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                price REAL NOT NULL,
                stock INTEGER NOT NULL,
                barcode TEXT,
                active INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        ).map_err(|e| format!("Failed to create product table: {}", e))?;

        // Tabela de vendas
        conn.execute(
            "CREATE TABLE IF NOT EXISTS venda (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                total REAL NOT NULL,
                discount REAL NOT NULL DEFAULT 0,
                final_total REAL NOT NULL,
                status TEXT NOT NULL,
                customer_name TEXT,
                customer_document TEXT,
                notes TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        ).map_err(|e| format!("Failed to create venda table: {}", e))?;

        // Tabela de itens de venda
        conn.execute(
            "CREATE TABLE IF NOT EXISTS venda_item (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                venda_id INTEGER NOT NULL,
                product_id INTEGER NOT NULL,
                product_name TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                unit_price REAL NOT NULL,
                total_price REAL NOT NULL,
                discount REAL NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                FOREIGN KEY (venda_id) REFERENCES venda(id),
                FOREIGN KEY (product_id) REFERENCES product(id)
            )",
            [],
        ).map_err(|e| format!("Failed to create venda_item table: {}", e))?;

        // Tabela de pagamentos
        conn.execute(
            "CREATE TABLE IF NOT EXISTS venda_pagamento (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                venda_id INTEGER NOT NULL,
                payment_method TEXT NOT NULL,
                amount REAL NOT NULL,
                status TEXT NOT NULL,
                transaction_id TEXT,
                notes TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (venda_id) REFERENCES venda(id)
            )",
            [],
        ).map_err(|e| format!("Failed to create venda_pagamento table: {}", e))?;

        // Criar índices para melhor performance
        conn.execute("CREATE INDEX IF NOT EXISTS idx_product_barcode ON product(barcode)", [])
            .map_err(|e| format!("Failed to create index: {}", e))?;
        
        conn.execute("CREATE INDEX IF NOT EXISTS idx_venda_status ON venda(status)", [])
            .map_err(|e| format!("Failed to create index: {}", e))?;
        
        conn.execute("CREATE INDEX IF NOT EXISTS idx_venda_item_venda_id ON venda_item(venda_id)", [])
            .map_err(|e| format!("Failed to create index: {}", e))?;
        
        conn.execute("CREATE INDEX IF NOT EXISTS idx_venda_pagamento_venda_id ON venda_pagamento(venda_id)", [])
            .map_err(|e| format!("Failed to create index: {}", e))?;

        Ok(())
    }

    /// Retorna o caminho do banco de dados
    pub fn get_db_path(&self) -> &PathBuf {
        &self.db_path
    }

    /// Cria uma instância para testes (em memória)
    #[cfg(test)]
    pub fn new_in_memory() -> Result<Self, String> {
        let service = Self {
            db_path: PathBuf::from(":memory:"),
        };
        service.initialize()?;
        Ok(service)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_database() {
        let db_service = SqliteDbService::new_in_memory().expect("Failed to create in-memory database");
        let conn = db_service.get_connection().expect("Failed to get connection");
        
        // Testa se as tabelas foram criadas
        let table_count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
            [],
            |row| row.get(0)
        ).expect("Failed to count tables");
        
        assert_eq!(table_count, 7, "Should have 7 tables");
    }

    #[test]
    fn test_insert_config() {
        let db_service = SqliteDbService::new_in_memory().expect("Failed to create in-memory database");
        let conn = db_service.get_connection().expect("Failed to get connection");
        
        conn.execute(
            "INSERT INTO config (key, value, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            ["test_key", "test_value", "2024-01-01T00:00:00Z", "2024-01-01T00:00:00Z"],
        ).expect("Failed to insert config");
        
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM config",
            [],
            |row| row.get(0)
        ).expect("Failed to count configs");
        
        assert_eq!(count, 1);
    }
}
