use std::collections::HashMap;

use axum::{
    extract::Form,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router,
};

use model::{get_active_affinity_bonuses, get_job_affinity_sums_from_form_data};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use view::{index_template, page_content_template};

mod file_utils;
mod items;
mod model;
mod view;

#[tokio::main]
async fn main() {
    let static_files_service = axum::routing::get_service(ServeDir::new("./static")).handle_error(
        |error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error serving file: {}", error),
            )
        },
    );

    let app = Router::new()
        .route("/", get(index))
        .route("/update", post(update_page_content))
        .route("/test-load", post(test_load))
        .nest_service("/static", static_files_service);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<String> {
    // Model
    let equipment_slot_names = model::get_equipment_slot_names();

    let jobs = match file_utils::get_jobs() {
        Ok(jobs) => jobs,
        Err(error) => panic!("Problem getting job data: {error:?}"),
    };

    let job_affinity_sums: HashMap<String, u32> = HashMap::new();
    let active_affinity_bonuses_for_jobs = get_active_affinity_bonuses(job_affinity_sums.clone());

    // View
    let markup = index_template(
        equipment_slot_names,
        &jobs,
        job_affinity_sums,
        active_affinity_bonuses_for_jobs,
    );

    Html(markup.into())
}

// TODO: Get arrays working in POST form data, or write a macro to generate this struct
#[derive(Default, Deserialize, Debug, Serialize)]
struct FormData {
    active_job: String,
    active_job_strength: u32,
    weapon_type: String,
    body_type: String,
    weapon_job1: String,
    weapon_job2: String,
    weapon_strength: u32,
    shield_job1: String,
    shield_job2: String,
    shield_strength: u32,
    head_job1: String,
    head_job2: String,
    head_strength: u32,
    body_job1: String,
    body_job2: String,
    body_strength: u32,
    hand_job1: String,
    hand_job2: String,
    hand_strength: u32,
    leg_job1: String,
    leg_job2: String,
    leg_strength: u32,
    foot_job1: String,
    foot_job2: String,
    foot_strength: u32,
    // accessory_job: String,
}

impl FormData {
    fn new() -> Self {
        FormData {
            weapon_type: "2H".to_string(),
            body_type: "body-only".to_string(),
            weapon_strength: 350,
            shield_strength: 75,
            head_strength: 250,
            body_strength: 250,
            hand_strength: 250,
            leg_strength: 250,
            foot_strength: 250,
            ..Default::default()
        }
    }
}

fn serialize_form_data_to_string(form_data: &FormData) -> String {
    serde_json::to_string(form_data).unwrap()
}

fn deserialize_string_to_form_data(string: &str) -> FormData {
    serde_json::from_str(string).unwrap()
}

/**
 * Updates the Result section with the currently active job affinity bonuses.
 */
async fn update_page_content(Form(form_data): Form<FormData>) -> Html<String> {
    // Model
    let equipment_slot_names = model::get_equipment_slot_names();
    let jobs = match file_utils::get_jobs() {
        Ok(jobs) => jobs,
        Err(error) => panic!("Problem getting job data: {error:?}"),
    };
    let job_affinity_sums = get_job_affinity_sums_from_form_data(&form_data);
    let active_affinity_bonuses_for_jobs = get_active_affinity_bonuses(job_affinity_sums.clone());

    // View
    // TODO: Pass in form data
    let result = page_content_template(
        equipment_slot_names,
        &jobs,
        job_affinity_sums,
        active_affinity_bonuses_for_jobs,
        &form_data,
    );

    Html(result.into())
}

async fn test_load() -> Html<String> {
    let dummy_data = r#"{"active_job":"Berserker","active_job_strength":800,"weapon_type":"2H","body_type":"body-only","weapon_job1":"Samurai","weapon_job2":"Marauder","weapon_strength":350,"shield_job1":"(None)","shield_job2":"(None)","shield_strength":0,"head_job1":"Samurai","head_job2":"Marauder","head_strength":250,"body_job1":"Dragoon","body_job2":"Warrior","body_strength":250,"hand_job1":"Dragoon","hand_job2":"Dark Knight","hand_strength":250,"leg_job1":"Monk","leg_job2":"Dark Knight","leg_strength":250,"foot_job1":"Red Mage","foot_job2":"Sage","foot_strength":250}"#;
    let deser = deserialize_string_to_form_data(&dummy_data);
    update_page_content(Form(deser)).await
}
