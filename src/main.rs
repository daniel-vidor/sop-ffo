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
use view::{active_job_affinities_template, equipment_form_template, index_template};

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
        .route("/update", post(update_result_section))
        .route("/update-equipment-form", post(update_equipment_form))
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

    let is_two_handed = true;

    // View
    let markup = index_template(equipment_slot_names, &jobs, is_two_handed);

    Html(markup.into())
}

// TODO: Get arrays working in POST form data, or write a macro to generate this struct
#[derive(Deserialize, Debug, Serialize)]
struct FormData {
    active_job: String,
    active_job_strength: u32,
    weapon_type: String,
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
    // accessory_job: String,
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
async fn update_result_section(Form(form_data): Form<FormData>) -> Html<String> {
    // Model
    let job_affinity_sums = get_job_affinity_sums_from_form_data(&form_data);
    let active_affinity_bonuses_for_jobs = get_active_affinity_bonuses(job_affinity_sums.clone());

    // View
    let result =
        active_job_affinities_template(job_affinity_sums, active_affinity_bonuses_for_jobs);

    let ser = serialize_form_data_to_string(&form_data);
    //println!("Serialised: {}", ser);

    let deser = deserialize_string_to_form_data(&ser);
    //println!("Deserialised: {:?}", deser);

    // println!("update: {:?}", result);
    Html(result.into())
}

async fn test_load() -> Html<String> {
    let dummy_data = r#"{"active_job":"Berserker","active_job_strength":800,"weapon_job1":"Samurai","weapon_job2":"Marauder","weapon_strength":350,"shield_job1":"(None)","shield_job2":"(None)","shield_strength":0,"head_job1":"Samurai","head_job2":"Marauder","head_strength":250,"chest_job1":"Dragoon","chest_job2":"Warrior","chest_strength":250,"hands_job1":"Dragoon","hands_job2":"Dark Knight","hands_strength":250,"legs_job1":"Monk","legs_job2":"Dark Knight","legs_strength":250,"feet_job1":"Red Mage","feet_job2":"Sage","feet_strength":250}"#;
    let deser = deserialize_string_to_form_data(&dummy_data);
    update_result_section(Form(deser)).await
}

async fn update_equipment_form(form_data: axum::Form<FormData>) -> Html<String> {
    let is_two_handed = form_data.weapon_type == "2H";
    println!("is_two_handed: {is_two_handed}");

    let equipment_slot_names = model::get_equipment_slot_names();

    let jobs = match file_utils::get_jobs() {
        Ok(jobs) => jobs,
        Err(error) => panic!("Problem getting job data: {error:?}"),
    };

    let result = equipment_form_template(equipment_slot_names, &jobs, is_two_handed);
    //println!("update_equipment_form: {:?}", result);
    Html(result.into())
}
