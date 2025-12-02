use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntity {
    pub id: Option<i64>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<i64>,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl HistoryEntity {
    pub fn new(action: String, entity_type: String, entity_id: Option<i64>, details: Option<String>) -> Self {
        Self {
            id: None,
            action,
            entity_type,
            entity_id,
            details,
            created_at: Utc::now(),
        }
    }
}
