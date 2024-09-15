use std::collections::HashMap;

use axum::{response::Html, routing::{get, post}, extract::Form, Router, debug_handler};
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
    // let slots = vec!["Weapon", "Shield", "Head", "Chest", "Hands", "Legs", "Feet"]
    let slots = vec!["Weapon", "Head"]
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

// #[derive(Deserialize, Debug)]
// struct EquipmentAffinities {
//     #[serde(rename = "job1[]")]
//     job1: Vec<String>,
//     #[serde(rename = "job2[]")]
//     job2: Vec<String>,
//     #[serde(rename = "strength[]")]
//     strength: Vec<u32>,
// }

// #[derive(Deserialize, Debug)]
// struct EquipmentAffinity {
//     #[serde(rename = "job1")]
//     job1: String,
//     #[serde(rename = "job2")]
//     job2: String,
//     #[serde(rename = "strength")]
//     strength: String,
// }

// This is absolutely dreadful... I wish I could get arrays working in POST form data
#[derive(Deserialize, Debug)]
struct FormData {
    Weapon_job1: String,
    Weapon_job2: String,
    Weapon_strength: u32,
    Head_job1: String,
    Head_job2: String,
    Head_strength: u32,
}

fn map_formdata_to_map(form_data: FormData) -> HashMap<String, u32> {
    let mut job_affinity_sums: HashMap<String, u32> = HashMap::new();
    job_affinity_sums.insert(form_data.Weapon_job1, form_data.Weapon_strength);
    job_affinity_sums.insert(form_data.Weapon_job2, form_data.Weapon_strength);
    job_affinity_sums.insert(form_data.Head_job1, form_data.Head_strength);
    job_affinity_sums.insert(form_data.Head_job2, form_data.Head_strength);

    job_affinity_sums
}

#[debug_handler]
async fn sum_affinities(Form(form): Form<FormData>) -> Html<String> {
    println!("Form: {:?}", form); 

    let job_affinity_sums = map_formdata_to_map(form);
    println!("HashMap: {:?}", job_affinity_sums); 

    let result = html! {
        @for a in job_affinity_sums {
            p {
                (a.0) ": " (a.1) "%"
            }
        }
    };

    Html(result.into())
}
