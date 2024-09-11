use maud::{html, Markup};

pub fn render_equipment_slot_row(slot: String, jobs: Vec<String>) -> Markup {
    html! {
        label for=(slot) {(slot)}
        (render_job_options(jobs))
        input id="affinity-strength" name="affinity-strength" type="number" min="0" max="999" {}
    }
}

fn render_job_options(jobs: Vec<String>) -> Markup {
    let job_select = html! {
        select id="options" name="option" hx-get="/update" hx-target="#result" hx-trigger="change" {
            @for job in jobs {
                option value=(job) { (job) }
            }
        }
    };

    html! {
        (job_select)
        (job_select)
    }
}
