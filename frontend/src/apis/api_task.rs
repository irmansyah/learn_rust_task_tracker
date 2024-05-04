use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{self, Read};
use chrono::{NaiveDate, Weekday};
use inquire::{DateSelect, Select, Text};
use inquire::{InquireError, MultiSelect};

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskInput {
	pub title: String,
	pub variable: String,
	pub question: String,
	pub input_type: String,
	pub data: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiData {
	pub name: String,
	pub typ: String,
	pub url: String,
}

#[derive(Debug)]
pub enum UserSelection {
	Text(String),
	Date(NaiveDate),
	MultiText(Vec<String>),
}


pub fn get_apis<'a>(source: &'a str, data: &'a str) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    let mut file = File::open(source)?;
    
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let parsed_data: Value = serde_json::from_str(&json_data)?;

    Ok(parsed_data.get(data).cloned())
}

pub fn input_user(input: &TaskInput) -> Result<UserSelection, InquireError> {
	match input.input_type.as_str() {
		"text" => Text::new(&input.question)
			.prompt()
			.map(|selected_value| UserSelection::Text(selected_value)),
		"select" => Select::new(&input.question, input.data.clone())
			.prompt()
			.map(|selected_value| UserSelection::Text(selected_value)),
		"date" => {
			// Use DateSelect::new if the title is "date"
			DateSelect::new(&input.question)
				// .with_starting_date(NaiveDate::from_ymd_opt(2021, 8, 1))
				// .with_min_date(NaiveDate::from_ymd_opt(2021, 8, 1))
				// .with_max_date(NaiveDate::from_ymd_opt(2021, 12, 31))
				.with_week_start(Weekday::Mon)
				.with_help_message("Possible flights will be displayed according to the selected date")
				.prompt()
				.map(|date| UserSelection::Date(date))
		}
		"multi_select" => MultiSelect::new(&input.question, input.data.clone())
			.prompt()
			.map(|selected_values| UserSelection::MultiText(selected_values)),
		_ => Select::new(&input.question, input.data.clone())
			.prompt()
			.map(|selected_value| UserSelection::Text(selected_value)),
	}
}

