use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use serde::Deserialize;
use serde_json::from_reader;

const DATA_FILENAME: &str = "data.json";

#[derive(Deserialize, Debug)]
pub struct Job {
    pub name: String,
    pub tier: String,
    pub r#type: String,
}

pub fn get_jobs() -> Result<Vec<Job>, Box<dyn std::error::Error>> {
    read_json(DATA_FILENAME)
}

pub fn read_json(file_name: &str) -> Result<Vec<Job>, Box<dyn Error>> {
    let path = format!("data/{}", file_name);
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let jobs: Vec<Job> = from_reader(reader)?;

    Ok(jobs)
}
