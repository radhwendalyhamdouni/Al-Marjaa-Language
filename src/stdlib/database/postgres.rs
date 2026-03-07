// src/stdlib/database/postgres.rs
// PostgreSQL Driver

use super::{ConnectionConfig, DatabaseConnection, DatabaseType, QueryResult};

/// اتصال PostgreSQL
pub struct PostgresConnection {
    connection: DatabaseConnection,
}

impl PostgresConnection {
    /// إنشاء اتصال جديد
    pub fn new(config: ConnectionConfig) -> Self {
        let mut config = config;
        config.port = if config.port == 3306 { 5432 } else { config.port };
        
        Self {
            connection: DatabaseConnection::new(DatabaseType::Postgres, config),
        }
    }
    
    /// الاتصال
    pub fn connect(&mut self) -> Result<(), String> {
        self.connection.connect()
    }
    
    /// تنفيذ استعلام
    pub fn execute(&self, query: &str) -> Result<QueryResult, String> {
        self.connection.execute(query)
    }
}

// ===== دوال عربية =====

/// اتصال PostgreSQL جديد
pub fn اتصال_postgres(config: ConnectionConfig) -> PostgresConnection {
    PostgresConnection::new(config)
}
