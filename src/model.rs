use serde::Deserialize;
use std::collections::HashMap;

use crate::{file_utils::get_jobs, FormData};

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

#[derive(Clone, Deserialize, Debug)]
pub struct AffinityBonus {
    pub name: String,
    pub description: String,
}

pub struct EquipmentAffinity {
    pub slot: String,
    pub jobs: Vec<String>,
    pub strength: u32,
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
        let job_affinity_strength = job_affinity_sums.get(&job.name).unwrap_or(&0).clone();
        if job_affinity_strength == 0 {
            continue;
        }

        let active_affinity_bonuses = job
            .affinities
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
        for job in equipment.jobs {
            upsert_into_hashmap(&mut job_affinity_sums, job, equipment.strength);
        }
    }

    job_affinity_sums
}

// Takes view model data, returns model data. Where to put this...?
fn map_formdata_to_equipment_affinities(form_data: FormData) -> Vec<EquipmentAffinity> {
    let weapon = EquipmentAffinity {
        slot: "weapon".to_string(),
        jobs: vec![form_data.weapon_job1, form_data.weapon_job2],
        strength: form_data.weapon_strength,
    };
    let shield = EquipmentAffinity {
        slot: "shield".to_string(),
        jobs: vec![form_data.shield_job1, form_data.shield_job2],
        strength: form_data.shield_strength,
    };
    let head = EquipmentAffinity {
        slot: "head".to_string(),
        jobs: vec![form_data.head_job1, form_data.head_job2],
        strength: form_data.head_strength,
    };
    let chest = EquipmentAffinity {
        slot: "chest".to_string(),
        jobs: vec![form_data.chest_job1, form_data.chest_job2],
        strength: form_data.chest_strength,
    };
    let hands = EquipmentAffinity {
        slot: "hands".to_string(),
        jobs: vec![form_data.hands_job1, form_data.hands_job2],
        strength: form_data.hands_strength,
    };
    let legs = EquipmentAffinity {
        slot: "legs".to_string(),
        jobs: vec![form_data.legs_job1, form_data.legs_job2],
        strength: form_data.legs_strength,
    };
    let feet = EquipmentAffinity {
        slot: "feet".to_string(),
        jobs: vec![form_data.feet_job1, form_data.feet_job2],
        strength: form_data.feet_strength,
    };

    vec![weapon, shield, head, chest, hands, legs, feet]
}

fn upsert_into_hashmap(hashmap: &mut HashMap<String, u32>, key: String, value: u32) {
    hashmap
        .entry(key)
        .and_modify(|e| *e += value)
        .or_insert(value);
}
