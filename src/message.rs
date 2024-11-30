use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, serde::ts_seconds};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub sender: String,
    #[serde(with = "ts_seconds")]
    pub send_time: DateTime<Utc>,
    pub target: MessageTarget,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageTarget {
    Broadcast,
    Unicast(String),
}