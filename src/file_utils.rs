use serde::Deserialize;
use serde_json::from_reader;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

const DATA_FILENAME: &str = "data.json";

#[derive(Deserialize, Debug)]
pub struct Job {
    pub name: String,
    pub tier: String,
    pub r#type: String,
    pub weapons: Vec<String>,
    // pub classes
    #[serde(rename(deserialize = "jobAffinityBonuses"))]
    pub affinities: Affinities,
}

#[derive(Deserialize, Debug)]
pub struct Affinities {
    #[serde(rename(deserialize = "250"))]
    _250: AffinityBonus,
    #[serde(rename(deserialize = "400"))]
    _400: AffinityBonus,
    #[serde(rename(deserialize = "600"))]
    _600: AffinityBonus,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AffinityBonus {
    pub name: String,
    pub description: String,
}

impl Affinities {
    pub fn get_affinity_bonuses(&self, affinity_strength: u32) -> Vec<AffinityBonus> {
        let mut result = vec![];

        if affinity_strength >= 250 {
            result.push(self._250.clone());
        }
        if affinity_strength >= 400 {
            result.push(self._400.clone());
        }
        if affinity_strength >= 600 {
            result.push(self._600.clone());
        }

        result
    }
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
