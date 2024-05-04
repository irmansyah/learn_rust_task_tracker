use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskType {
	#[serde(rename = "work")]
	Work,
	#[serde(rename = "personal")]
	Personal,
}

impl Default for TaskType {
	fn default() -> Self {
		TaskType::Work
	}
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskType::Work => write!(f, "Work"),
            TaskType::Personal => write!(f, "Personal"),
        }
    }
}
