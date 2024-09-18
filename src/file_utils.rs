use serde::Deserialize;
use serde_json::from_reader;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::model::Job;

const DATA_FILENAME: &str = "data.json";

pub fn get_jobs() -> Result<Vec<Job>, Box<dyn std::error::Error>> {
    read_json(DATA_FILENAME)
}

// TODO: Decouple from `Job` type. Can I use a generic type here?
fn read_json(file_name: &str) -> Result<Vec<Job>, Box<dyn Error>> {
    let path = format!("data/{}", file_name);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let jobs: Vec<Job> = from_reader(reader)?;

    Ok(jobs)
}
