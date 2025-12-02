use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductEntity {
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub active: i32,
    pub balance: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ProductEntity {
    pub fn new(code: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            code,
            name,
            active: 1,
            balance: 0.0,
            created_at: now,
            updated_at: now,
        }
    }
}
