use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};

use model::{get_active_affinity_bonuses, get_job_affinity_sums_from_form_data};
use maud::html;
use serde::Deserialize;
use view::display_active_job_affinities;

mod file_utils;
mod items;
mod model;
mod view;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(render_page))
        .route("/update", post(update));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn render_page() -> Html<String> {
    let slots = vec!["weapon", "shield", "head", "chest", "hands", "legs", "feet"]
        .iter()
        .map(|s| s.to_string())
        .collect();

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
                form hx-post="/update" hx-trigger="change" hx-target="#result" enctype="json" {
                    (view::render_form(slots, &job_names))
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

/**
 * Updates the Result section with the currently active job affinity bonuses.
 */
async fn update(Form(form_data): Form<FormData>) -> Html<String> {
    // Model
    let job_affinity_sums = get_job_affinity_sums_from_form_data(form_data);

    let active_affinity_bonuses_for_jobs =
        get_active_affinity_bonuses(job_affinity_sums.clone());

    // View
    let result = display_active_job_affinities(job_affinity_sums, active_affinity_bonuses_for_jobs);

    Html(result.into())
}
