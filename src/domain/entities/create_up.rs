use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUp {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CreateUp {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_dates(created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> Self {
        Self {
            created_at,
            updated_at,
        }
    }

    pub fn with_created_at(created_at: DateTime<Utc>) -> Self {
        Self {
            created_at,
            updated_at: created_at,
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn set_dates(&mut self, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) {
        self.created_at = created_at;
        self.updated_at = updated_at;
    }
}
