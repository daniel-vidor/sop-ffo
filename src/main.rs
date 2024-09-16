use std::collections::HashMap;

use axum::{response::Html, routing::{get, post}, extract::Form, Router, debug_handler};
use file_utils::{get_jobs, AffinityBonus, Job};
use maud::html;
use serde::Deserialize;

mod file_utils;
mod items;
mod jobs;
mod render_utils;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(render_page))
        .route("/sum", post(sum_affinities));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn render_page() -> Html<String> {
    let slots = vec!["weapon", "shield", "head", "chest", "hands", "legs", "feet"]
        .iter().map(|s| s.to_string()).collect();

    let jobs = match file_utils::get_jobs() {
        Ok(jobs) => jobs,
        Err(error) => panic!("Problem getting job data: {error:?}"),
    };

    let job_names: Vec<String> = jobs.iter().map(|job| job.name.clone()).collect();

    let markup = html! {
        html {
            head {
                title { "Stranger of Paradise: Final Fantasy Origin | Build simulator" }
                script src="https://unpkg.com/htmx.org@1.9.2" {}
            }
            body {
                h1 { "Stranger of Paradise: Final Fantasy Origin | Build simulator" }

                h2 { "Job" }
                select {
                    @for job_name in &job_names {
                        option value=(job_name) { (job_name) }
                    }
                }

                h2 { "Equipment" }
                form hx-post="/sum" hx-trigger="change" hx-target="#result" enctype="json" {
                    (render_utils::render_form(slots, &job_names))
                }

                h2 { "Result" }
                div id="result" {
                    p { "Please select an option to see the result." }
                }
            }
        }
    };

    Html(markup.into_string())
}

// This is absolutely dreadful... I wish I could get arrays working in POST form data
// TODO: Consider writing a macro to generate this struct
#[derive(Deserialize, Debug)]
struct FormData {
    weapon_job1: String,
    weapon_job2: String,
    weapon_strength: u32,
    shield_job1: String,
    shield_job2: String,
    shield_strength: u32,
    head_job1: String,
    head_job2: String,
    head_strength: u32,
    chest_job1: String,
    chest_job2: String,
    chest_strength: u32,
    hands_job1: String,
    hands_job2: String,
    hands_strength: u32,
    legs_job1: String,
    legs_job2: String,
    legs_strength: u32,
    feet_job1: String,
    feet_job2: String,
    feet_strength: u32,
}

pub struct EquipmentAffinity {
    pub slot: String,
    pub jobs: Vec<String>,
    pub strength: u32,
}

fn map_formdata_to_equipment_affinities(form_data: FormData) -> Vec<EquipmentAffinity> {
    let weapon = EquipmentAffinity {
        slot: "weapon".to_string(),
        jobs: vec![form_data.weapon_job1, form_data.weapon_job2],
        strength: form_data.weapon_strength
    };
    let shield = EquipmentAffinity {
        slot: "shield".to_string(),
        jobs: vec![form_data.shield_job1, form_data.shield_job2],
        strength: form_data.shield_strength
    };
    let head = EquipmentAffinity {
        slot: "head".to_string(),
        jobs: vec![form_data.head_job1, form_data.head_job2],
        strength: form_data.head_strength
    };
    let chest = EquipmentAffinity {
        slot: "chest".to_string(),
        jobs: vec![form_data.chest_job1, form_data.chest_job2],
        strength: form_data.chest_strength
    };
    let hands = EquipmentAffinity {
        slot: "hands".to_string(),
        jobs: vec![form_data.hands_job1, form_data.hands_job2],
        strength: form_data.hands_strength
    };
    let legs = EquipmentAffinity {
        slot: "legs".to_string(),
        jobs: vec![form_data.legs_job1, form_data.legs_job2],
        strength: form_data.legs_strength
    };
    let feet = EquipmentAffinity {
        slot: "feet".to_string(),
        jobs: vec![form_data.feet_job1, form_data.feet_job2],
        strength: form_data.feet_strength
    };

    vec![weapon, shield, head, chest, hands, legs, feet]
}

fn get_job_affinity_sums(equipment_affinities: Vec<EquipmentAffinity>) -> HashMap<String, u32> {
    let mut job_affinity_sums: HashMap<String, u32> = HashMap::new();

    for equipment in equipment_affinities {
        for job in equipment.jobs {
            upsert_into_hashmap(&mut job_affinity_sums, job, equipment.strength);
        }
    }

    job_affinity_sums
}

fn upsert_into_hashmap(hashmap: &mut HashMap<String, u32>, key: String, value: u32) {
    hashmap.entry(key)
        .and_modify(|e| *e += value)
        .or_insert(value);
}

fn get_active_affinity_bonuses(jobs_data: Vec<Job>, job_affinity_sums: HashMap<String, u32>) -> HashMap<String, Vec<AffinityBonus>> {
    let mut active_affinity_bonuses_for_jobs: HashMap<String, Vec<AffinityBonus>> = HashMap::new();

    for job in jobs_data {
        let job_affinity_strength = job_affinity_sums.get(&job.name).unwrap_or(&0).clone();
        if job_affinity_strength == 0 { continue }

        let active_affinity_bonuses = job.affinities.get_affinity_bonuses(job_affinity_strength).clone();
        active_affinity_bonuses_for_jobs.insert(job.name, active_affinity_bonuses);
    }

    active_affinity_bonuses_for_jobs
}

#[debug_handler]
async fn sum_affinities(Form(form): Form<FormData>) -> Html<String> {
    let equipment_affinities = map_formdata_to_equipment_affinities(form);
    let job_affinity_sums = get_job_affinity_sums(equipment_affinities);
    let mut job_names: Vec<&String> = job_affinity_sums.keys().collect();
    job_names.sort();

    // TODO: Cache
    let jobs_data = match get_jobs() {
        Ok(data) => data,
        Err(error) => panic!("Problem getting job data: {error:?}"),
    };
        
    let active_affinity_bonuses_for_jobs = get_active_affinity_bonuses(jobs_data, job_affinity_sums.clone());

    let result = html! {
        @for job_name in job_names {
            h3 {
                (job_name) ": " (job_affinity_sums.get(job_name).unwrap()) "%"
            }
            @for a in active_affinity_bonuses_for_jobs.get(job_name).unwrap() {
                p { b { (a.name) } ": " (a.description)}
            }
        }
    };

    Html(result.into())
}
