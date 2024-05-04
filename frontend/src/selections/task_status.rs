// use serde::{Deserialize, Serialize};
// use std::fmt;

// #[derive(Debug, Serialize, Deserialize)]
// pub enum TaskStatus {
// 	#[serde(rename = "todo")]
// 	Todo,
// 	#[serde(rename = "bug")]
// 	Bug,
// 	#[serde(rename = "doing")]
// 	Doing,
// 	#[serde(rename = "testing")]
// 	Testing,
// 	#[serde(rename = "done")]
// 	Done,
// }

// impl Default for TaskStatus {
// 	fn default() -> Self {
// 		TaskStatus::Todo
// 	}
// }

// impl fmt::Display for TaskStatus {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		match self {
// 			TaskStatus::Todo => write!(f, "Todo"),
// 			TaskStatus::Bug => write!(f, "Bug"),
// 			TaskStatus::Doing => write!(f, "Doing"),
// 			TaskStatus::Testing => write!(f, "Testing"),
// 			TaskStatus::Done => write!(f, "Done"),
// 		}
// 	}
// }

// impl TaskStatus {
// 	// pub fn get_datas() -> Vec<&'static str> {
// 	// 	let datas = vec![
// 	// 		&TaskStatus::Todo.to_string(),
// 	// 		&TaskStatus::Bug.to_string(),
// 	// 		&TaskStatus::Doing.to_string(),
// 	// 		&TaskStatus::Testing.to_string(),
// 	// 		&TaskStatus::Done.to_string(),
// 	// 	];
// 	// }
// }

// pub fn get_input_selection_status(prompt: &str) -> Result<TaskStatus, Box<dyn std::error::Error>> {
// 	print!("{}", prompt);

// 	// let items: Vec<&str> = TaskStatus::variants().iter().map(|&v| v).collect();
// 	let items: Vec<String> = vec![
//         TaskStatus::Todo.to_string(),
//         TaskStatus::Bug.to_string(),
//         TaskStatus::Doing.to_string(),
//         TaskStatus::Testing.to_string(),
//         TaskStatus::Done.to_string(),
//     ];

// 	let selection = Select::with_theme(&ColorfulTheme::default())
// 		.items(&items)
// 		.default(0) // Default selection
// 		.interact()
// 		.unwrap_or(0); // Default to 0 if there's an error

// 	// Match the input to the corresponding enum variant
// 	let terminal_selection = match selection {
// 		1 => TaskStatus::Bug,
// 		2 => TaskStatus::Doing,
// 		3 => TaskStatus::Testing,
// 		4 => TaskStatus::Done,
// 		_ => TaskStatus::Todo,
// 	};

// 	match terminal_selection {
// 		TaskStatus::Todo => return Ok(TaskStatus::Todo),
// 		TaskStatus::Bug => return Ok(TaskStatus::Bug),
// 		TaskStatus::Doing => return Ok(TaskStatus::Doing),
// 		TaskStatus::Testing => return Ok(TaskStatus::Testing),
// 		TaskStatus::Done => return Ok(TaskStatus::Done),
// 	}
// }
