use chrono::{DateTime, Utc};

pub struct Event {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub start_datetime: DateTime<Utc>,
    pub end_datetime: DateTime<Utc>,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
