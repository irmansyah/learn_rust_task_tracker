use std::io;

use crate::apis::api_task::{self, input_user, ApiData, TaskInput, UserSelection};

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
	let task_data_json = api_task::get_apis("data.json", "inputs")?;

	let task_datas = if let Some(task_data_json) = task_data_json {
		serde_json::from_value::<Vec<TaskInput>>(task_data_json.clone())?
	} else {
		Vec::new()
	};

	let mut transformed_data = serde_json::Map::new();
	for (_index, task) in task_datas.iter().enumerate() {
		match input_user(task) {
			Ok(selection) => match selection {
				UserSelection::Text(value) => {
					transformed_data.insert(task.variable.clone(), serde_json::Value::String(value.clone()));
				}
				UserSelection::Date(date) => {
					transformed_data.insert(
						task.variable.clone(),
						serde_json::Value::String(date.to_string().clone()),
					);
				}
				UserSelection::MultiText(_values) => {}
			},
			Err(err) => {
				eprintln!("Error: {}", err);
			}
		}
	}

	let transformed_json = serde_json::to_string_pretty(&transformed_data)?;
	println!("{}", transformed_json);

	println!("Is this data correct? (yes/no)");
	let mut confirm_input = String::new();
	io::stdin().read_line(&mut confirm_input)?;

	if confirm_input.trim().to_lowercase() == "yes" {
		let api_source_datas = api_task::get_apis("data.json", "apis")?;
		let api_datas = if let Some(api_source_datas) = api_source_datas {
			serde_json::from_value::<Vec<ApiData>>(api_source_datas.clone())?
		} else {
			Vec::new()
		};

		let json_body = serde_json::to_string(&transformed_json)?;
		send_request(&api_datas[0].url, &json_body)?;
	} else {
		println!("Task creation cancelled.");
	}

	Ok(())
}

fn send_request(url: &str, json_body: &str) -> Result<(), Box<dyn std::error::Error>> {
	let client = reqwest::blocking::Client::new();

	let response = client
		.post(url)
		.header(reqwest::header::CONTENT_TYPE, "application/json")
		.body(json_body.to_owned())
		.send()?;

	// Check if the request was successful!
	if response.status().is_success() {
		let body = response.text()?;
		println!("Response body: {}", body);
	} else {
		handle_error(response.status());
	}

	Ok(())
}

fn handle_error(status: reqwest::StatusCode) {
	println!("POST request failed with status code: {}", status);
}
