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

#[tokio::main]
async fn main() {
    // Define routes
    let app = Router::new()
        .route("/", get(render_form))   // Serve the form page
        .route("/update", get(update_text)); // Serve dynamic content based on dropdown

    // Run the web server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn render_equipment_slot_row(slot: &str) -> Markup {
    html! {
        div {
            label for=(slot) {(slot)}
            (render_job_options())
            input id="affinity-strength" name="affinity-strength" type="number" min="0" max="999" {}
        }
    }
}

fn render_job_options() -> Markup {
    html! {
        select id="options" name="option" hx-get="/update" hx-target="#result" hx-trigger="change" {
            @for job in get_jobs() {
                option value=(job) { (job) }
            }
        }
    }
}

// TODO: Split into Basic, Advanced, and Expert?
fn get_jobs() -> Vec<&'static str> {
    vec![
        "Swordfighter",
        "Swordsman",
        "Duelist",
        "Pugilist",
        "Mage",
        "Lancer"
    ]
}

// Function to render the initial form page with dropdown
async fn render_form() -> Html<String> {
    // Generate HTML using Maud
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
                    label for="options" { "Weapon" }
                    (render_job_options())
                    input id="affinity-strength" name="affinity-strength" type="number" min="0" max="999" {}
                }
                h2 { "Result" }
                // This div will be updated by HTMX
                div id="result" {
                    p { "Please select an option to see the result." }
                }
            }
        }
    };

    // Return the rendered HTML
    Html(markup.into_string())
}

// Function to handle HTMX request and return updated text
async fn update_text(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    // Extract the selected option from the query parameters
    let default_string = &"unknown".to_string();
    let option = params.get("option").unwrap_or(default_string);

    // Generate the dynamic response based on the selected option
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

    // Return the updated HTML to be injected into the target div
    Html(response_markup.into_string())
}
