use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize, Serialize)]
pub struct Api {
    pub name: String,
    pub url: String,
}

pub fn get_apis() -> Result<Vec<Api>, Box<dyn std::error::Error>> {
    let file = File::open("api_api_tool.json")?;
    let reader = BufReader::new(file);
    let apis: Vec<Api> = serde_json::from_reader(reader)?;
    Ok(apis)
}

