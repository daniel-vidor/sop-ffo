use serde::Deserialize;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::{collections::HashMap, fmt};

use crate::{
    file_utils::{self, get_jobs},
    FormData,
};

#[derive(Debug, Deserialize, PartialEq)]
pub enum JobTier {
    Basic,
    Advanced,
    Expert,
}

impl fmt::Display for JobTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tier_name = match self {
            JobTier::Basic => "Basic",
            JobTier::Advanced => "Advanced",
            JobTier::Expert => "Expert",
        };

        write!(f, "{}", tier_name)
    }
}

#[derive(Debug, Deserialize)]
pub enum JobType {
    Red,
    Orange,
    Green,
    Blue,
}

#[derive(Deserialize, Debug)]
pub struct Job {
    pub name: String,
    pub tier: JobTier,
    pub r#type: JobType,
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
    /// Returns a BTreeMap (i.e. a map sorted by its keys)
    pub fn get_active_affinity_bonuses(
        &self,
        affinity_strength: u32,
    ) -> BTreeMap<u32, AffinityBonus> {
        let mut result: BTreeMap<u32, AffinityBonus> = BTreeMap::new();

        if affinity_strength >= 250 {
            result.insert(250, self._250.clone());
        }
        if affinity_strength >= 400 {
            result.insert(400, self._400.clone());
        }
        if affinity_strength >= 600 {
            result.insert(600, self._600.clone());
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
) -> HashMap<String, BTreeMap<u32, AffinityBonus>> {
    // TODO: Cache and don't panic on failure
    let jobs_data = match get_jobs() {
        Ok(data) => data,
        Err(error) => panic!("Problem getting job data: {error:?}"),
    };

    let mut active_affinity_bonuses_for_jobs: HashMap<String, BTreeMap<u32, AffinityBonus>> =
        HashMap::new();

    for job in jobs_data {
        let job_affinity_strength = *job_affinity_sums.get(&job.name).unwrap_or(&0);
        if job_affinity_strength == 0 {
            continue;
        }

        let active_affinity_bonuses = job
            .affinity_bonuses
            .get_active_affinity_bonuses(job_affinity_strength)
            .clone();

        active_affinity_bonuses_for_jobs.insert(job.name, active_affinity_bonuses);
    }

    active_affinity_bonuses_for_jobs
}

pub fn get_job_affinity_sums_from_form_data(form_data: &FormData) -> HashMap<String, u32> {
    let mut job_affinity_sums: HashMap<String, u32> = HashMap::new();

    // Equipment
    map_formdata_to_equipment_affinities(form_data)
        .iter()
        .flat_map(|equipment| equipment.get_affinity_strengths())
        .filter(|(job, _strength)| !job.is_empty())
        .for_each(|(job, strength)| {
            accumulate_or_insert_into_hashmap(&mut job_affinity_sums, job, strength)
        });

    // Active job
    // TODO: Extract "(None)" to a constant
    if form_data.active_job != "(None)" {
        accumulate_or_insert_into_hashmap(
            &mut job_affinity_sums,
            form_data.active_job.clone(),
            form_data.active_job_strength,
        );
    }

    job_affinity_sums
}

// TODO: Turn into a method (associated function)?
fn accumulate_or_insert_into_hashmap<T>(hashmap: &mut HashMap<T, u32>, key: T, value: u32)
where
    T: Eq,
    T: Hash,
{
    *hashmap.entry(key).or_insert(0) += value;
}

// pub fn get_job_names() -> Vec<String> {
//     let jobs = match file_utils::get_jobs() {
//         Ok(jobs) => jobs,
//         Err(error) => panic!("Problem getting job data: {error:?}"),
//     };

//     let job_names = jobs.iter().map(|job| job.name.clone()).collect();
//     job_names
// }

pub fn get_equipment_slot_names() -> Vec<String> {
    ["weapon", "shield", "head", "chest", "hands", "legs", "feet"]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn map_formdata_to_equipment_affinities(form_data: &FormData) -> Vec<EquipmentAffinity> {
    let equipment_data = vec![
        (
            "weapon",
            vec![form_data.weapon_job1.clone(), form_data.weapon_job2.clone()],
            form_data.weapon_strength,
        ),
        (
            "shield",
            vec![form_data.shield_job1.clone(), form_data.shield_job2.clone()],
            form_data.shield_strength,
        ),
        (
            "head",
            vec![form_data.head_job1.clone(), form_data.head_job2.clone()],
            form_data.head_strength,
        ),
        (
            "chest",
            vec![form_data.chest_job1.clone(), form_data.chest_job2.clone()],
            form_data.chest_strength,
        ),
        (
            "hands",
            vec![form_data.hands_job1.clone(), form_data.hands_job2.clone()],
            form_data.hands_strength,
        ),
        (
            "legs",
            vec![form_data.legs_job1.clone(), form_data.legs_job2.clone()],
            form_data.legs_strength,
        ),
        (
            "feet",
            vec![form_data.feet_job1.clone(), form_data.feet_job2.clone()],
            form_data.feet_strength,
        ),
        // (
        //     "accessory",
        //     form_data.accessory_job.clone(),
        //     400,
        // )
    ];

    equipment_data
        .into_iter()
        .map(|(slot, job_names, strength)| EquipmentAffinity {
            slot: slot.to_string(),
            job_names: job_names
                .into_iter()
                .filter(|name| name != "(None)") // TODO: Map to None
                .collect(),
            strength,
        })
        .collect()
}
