// src/stdlib/database/mongodb.rs
// MongoDB Driver

use super::{ConnectionConfig, DatabaseConnection, DatabaseType, Row, Value};
use std::collections::HashMap;

/// مستند MongoDB
#[derive(Debug, Clone)]
pub struct Document {
    pub fields: HashMap<String, Value>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, key: &str, value: Value) {
        self.fields.insert(key.to_string(), value);
    }
    
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.fields.get(key)
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

/// اتصال MongoDB
pub struct MongoConnection {
    connection: DatabaseConnection,
    collections: HashMap<String, Vec<Document>>,
}

impl MongoConnection {
    /// إنشاء اتصال جديد
    pub fn new(config: ConnectionConfig) -> Self {
        let mut config = config;
        config.port = if config.port == 3306 { 27017 } else { config.port };
        
        Self {
            connection: DatabaseConnection::new(DatabaseType::MongoDb, config),
            collections: HashMap::new(),
        }
    }
    
    /// الاتصال
    pub fn connect(&mut self) -> Result<(), String> {
        self.connection.connect()
    }
    
    /// إنشاء مجموعة
    pub fn create_collection(&mut self, name: &str) {
        self.collections.insert(name.to_string(), Vec::new());
    }
    
    /// إدراج مستند
    pub fn insert_one(&mut self, collection: &str, doc: Document) -> Result<(), String> {
        if let Some(docs) = self.collections.get_mut(collection) {
            docs.push(doc);
            Ok(())
        } else {
            Err(format!("المجموعة {} غير موجودة", collection))
        }
    }
    
    /// البحث
    pub fn find(&self, collection: &str, filter: Option<&HashMap<String, Value>>) -> Vec<&Document> {
        if let Some(docs) = self.collections.get(collection) {
            if let Some(f) = filter {
                docs.iter()
                    .filter(|doc| {
                        f.iter().all(|(k, v)| doc.get(k) == Some(v))
                    })
                    .collect()
            } else {
                docs.iter().collect()
            }
        } else {
            Vec::new()
        }
    }
    
    /// حذف
    pub fn delete_many(&mut self, collection: &str, filter: &HashMap<String, Value>) -> usize {
        if let Some(docs) = self.collections.get_mut(collection) {
            let original_len = docs.len();
            docs.retain(|doc| {
                !filter.iter().all(|(k, v)| doc.get(k) == Some(v))
            });
            original_len - docs.len()
        } else {
            0
        }
    }
}

// ===== دوال عربية =====

/// اتصال MongoDB جديد
pub fn اتصال_mongodb(config: ConnectionConfig) -> MongoConnection {
    MongoConnection::new(config)
}
