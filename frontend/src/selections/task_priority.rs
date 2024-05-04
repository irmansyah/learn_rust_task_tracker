use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskPriority {
	#[serde(rename = "low")]
	Low,
	#[serde(rename = "medium")]
	Medium,
	#[serde(rename = "high")]
	High,
}

impl Default for TaskPriority {
	fn default() -> Self {
		TaskPriority::Low
	}
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskPriority::Low => write!(f, "Low"),
            TaskPriority::Medium => write!(f, "Medium"),
            TaskPriority::High => write!(f, "High"),
        }
    }
}

