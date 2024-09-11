use std::fs;
use serde_json::{Value};

const JOBS_FILENAME: &str = "jobs.json";

pub fn get_jobs() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    read_json(JOBS_FILENAME)
}

pub fn read_json(file_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = "data/".to_owned() + file_name;
    let file_content = fs::read_to_string(path)?;

    // let json_data: Value = serde_json::from_str(&file_content)?;
    let data: Vec<String> = serde_json::from_str(&file_content)?;

    // println!("{}\n{}", file_name, serde_json::to_string_pretty(&json_data)?);
    
    Ok(data)
}
