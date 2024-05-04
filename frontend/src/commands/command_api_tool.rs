use crate::apis::api_api_tool;
use std::io::{self, Write};

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let apis = api_api_tool::get_apis()?;
    // Display available APIs
    println!("Available APIs:");
    for (index, api) in apis.iter().enumerate() {
        println!("{}. {}", index + 1, api.name);
    }

    // Get user input
    print!("Enter the number of the API you want to call: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let index: usize = input.trim().parse()?;
    if index < 1 || index > apis.len() {
        return Err("Invalid API number".into());
    }
    let selected_api = &apis[index - 1];

    // Make request to selected API
    let response = reqwest::blocking::get(&selected_api.url)?;
    let json: serde_json::Value = response.json()?;
    println!("Response from {}: {:#?}", selected_api.name, json);

    Ok(())
}

