use maud::{html, Markup};

pub fn render_form(slots: Vec<String>, jobs: &Vec<String>) -> Markup {
    let job_select_options = html! {
        @for job in jobs {
            option value=(job) { (job) }
        }
    };

    html! {
        @for slot in slots {
            label for=(slot) {(slot)}
            select name=(format!("{slot}_job1")) {
                (job_select_options)
            }
            select name=(format!("{slot}_job2")) {
                (job_select_options)
            }
            input name=(format!("{slot}_strength"))
                type="number" min="0" max="999" value="250" {}
        }
    }
}
