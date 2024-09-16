use maud::{html, Markup};

fn capitalise_first_letter(s: &str) -> String {
    let mut char_iter = s.chars();
    match char_iter.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + char_iter.as_str(),
    }
}

pub fn render_form(slots: Vec<String>, jobs: &Vec<String>) -> Markup {
    let job_select_options = html! {
        @for job in jobs {
            option value=(job) { (job) }
        }
    };

    html! {
        @for slot in slots {
            div {
                label for=(slot) {(capitalise_first_letter(&slot))}
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
}
