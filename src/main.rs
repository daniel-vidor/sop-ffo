use axum::{
    extract::Query,
    response::Html,
    routing::get,
    Router,
};
use maud::{html, Markup};
use std::collections::HashMap;

mod items;
mod jobs;
mod file_utils;
mod render_utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let app = Router::new()
        .route("/", get(render_form))
        .route("/update", get(update_text));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn render_form() -> Html<String> {

    let jobs = match file_utils::get_jobs() {
        Ok(jobs) => jobs,
        Err(error) => panic!("Problem getting job data: {error:?}"),
    };

    let markup = html! {
        html {
            head {
                title { "Stranger of Paradise: Final Fantasy Origin | Build simulator" }
                script src="https://unpkg.com/htmx.org@1.9.2" {}
            }
            body {
                h1 { "Stranger of Paradise: Final Fantasy Origin | Build simulator" }
                h2 { "Equipment" }
                form {
                    (render_utils::render_equipment_slot_row("Weapon".to_string(), jobs))
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

async fn update_text(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let default_string = &"unknown".to_string();
    let option = params.get("option").unwrap_or(default_string);

    let response_markup = html! {
        p {
            @match option.as_str() {
                "swordfighter" => { "You selected Option 1" }
                "knight" => { "You selected Option 2" }
                "paladin" => { "You selected Option 3" }
                _ => { "Unknown option selected" }
            }
        }
    };

    Html(response_markup.into_string())
}
