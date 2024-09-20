use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    file_utils::{self, get_jobs},
    FormData,
};

enum JobTier {
    Basic,
    Advanced,
    Expert,
}

enum JobType {
    Red,
    Orange,
    Green,
    Blue,
}

#[derive(Deserialize, Debug)]
pub struct Job {
    pub name: String,
    pub tier: String,
    pub r#type: String,
    // pub weapons: Vec<String>,
    // pub classes
    #[serde(rename(deserialize = "affinityBonuses"))]
    pub affinity_bonuses: AffinityBonuses,
}

#[derive(Deserialize, Debug)]
pub struct AffinityBonuses {
    #[serde(rename(deserialize = "250"))]
    _250: AffinityBonus,
    #[serde(rename(deserialize = "400"))]
    _400: AffinityBonus,
    #[serde(rename(deserialize = "600"))]
    _600: AffinityBonus,
}

impl AffinityBonuses {
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

#[derive(Clone, Deserialize, Debug)]
pub struct AffinityBonus {
    pub name: String,
    pub description: String,
}

pub struct EquipmentAffinity {
    pub slot: String,
    pub job_names: Vec<String>,
    pub strength: u32,
}

impl EquipmentAffinity {
    pub fn get_affinity_strengths(&self) -> HashMap<String, u32> {
        let mut affinity_strengths: HashMap<String, u32> = HashMap::new();

        for job_name in &self.job_names {
            // Ensures that if there are duplicate job names, only one will exist in the hash map
            affinity_strengths.insert(job_name.to_string(), self.strength);
        }

        affinity_strengths
    }
}

pub fn get_active_affinity_bonuses(
    job_affinity_sums: HashMap<String, u32>,
) -> HashMap<String, Vec<AffinityBonus>> {
    // TODO: Cache and don't panic on failure
    let jobs_data = match get_jobs() {
        Ok(data) => data,
        Err(error) => panic!("Problem getting job data: {error:?}"),
    };

    let mut active_affinity_bonuses_for_jobs: HashMap<String, Vec<AffinityBonus>> = HashMap::new();

    for job in jobs_data {
        let job_affinity_strength = *job_affinity_sums.get(&job.name).unwrap_or(&0);
        if job_affinity_strength == 0 {
            continue;
        }

        let active_affinity_bonuses = job
            .affinity_bonuses
            .get_affinity_bonuses(job_affinity_strength)
            .clone();
        active_affinity_bonuses_for_jobs.insert(job.name, active_affinity_bonuses);
    }

    active_affinity_bonuses_for_jobs
}

pub fn get_job_affinity_sums_from_form_data(form_data: FormData) -> HashMap<String, u32> {
    let equipment_affinities = map_formdata_to_equipment_affinities(form_data);
    get_job_affinity_sums(equipment_affinities)
}

pub fn get_job_affinity_sums(equipment_affinities: Vec<EquipmentAffinity>) -> HashMap<String, u32> {
    let mut job_affinity_sums: HashMap<String, u32> = HashMap::new();

    for equipment in equipment_affinities {
        for (job, strength) in equipment.get_affinity_strengths() {
            *job_affinity_sums.entry(job.to_string()).or_insert(0) += strength;
        }
    }

    job_affinity_sums
}

pub fn get_job_names() -> Vec<String> {
    let jobs = match file_utils::get_jobs() {
        Ok(jobs) => jobs,
        Err(error) => panic!("Problem getting job data: {error:?}"),
    };

    let job_names = jobs.iter().map(|job| job.name.clone()).collect();
    job_names
}

pub fn get_equipment_slot_names() -> Vec<String> {
    ["weapon", "shield", "head", "chest", "hands", "legs", "feet"]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn map_formdata_to_equipment_affinities(form_data: FormData) -> Vec<EquipmentAffinity> {
    let equipment_data = vec![
        (
            "weapon",
            vec![form_data.weapon_job1, form_data.weapon_job2],
            form_data.weapon_strength,
        ),
        (
            "shield",
            vec![form_data.shield_job1, form_data.shield_job2],
            form_data.shield_strength,
        ),
        (
            "head",
            vec![form_data.head_job1, form_data.head_job2],
            form_data.head_strength,
        ),
        (
            "chest",
            vec![form_data.chest_job1, form_data.chest_job2],
            form_data.chest_strength,
        ),
        (
            "hands",
            vec![form_data.hands_job1, form_data.hands_job2],
            form_data.hands_strength,
        ),
        (
            "legs",
            vec![form_data.legs_job1, form_data.legs_job2],
            form_data.legs_strength,
        ),
        (
            "feet",
            vec![form_data.feet_job1, form_data.feet_job2],
            form_data.feet_strength,
        ),
    ];

    equipment_data
        .into_iter()
        .map(|(slot, job_names, strength)| EquipmentAffinity {
            slot: slot.to_string(),
            job_names,
            strength,
        })
        .collect()
}
