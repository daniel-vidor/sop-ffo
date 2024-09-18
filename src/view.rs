use std::collections::HashMap;

use maud::{html, Markup, DOCTYPE};

use crate::model::AffinityBonus;

pub fn index_template(equipment_slot_names: Vec<String>, job_names: Vec<String>) -> Markup {
    html! {
        (DOCTYPE)
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
                    (equipment_form_template(equipment_slot_names, &job_names))
                }

                h2 { "Result" }
                div id="result" {
                    p { "Please select an option to see the result." }
                }
            }
        }
    }
}

pub fn equipment_form_template(slots: Vec<String>, jobs: &Vec<String>) -> Markup {
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

pub fn active_job_affinities_template(
    job_affinity_sums: HashMap<String, u32>,
    active_affinity_bonuses_for_jobs: HashMap<String, Vec<AffinityBonus>>,
) -> Markup {
    let mut job_names: Vec<&String> = job_affinity_sums.keys().collect();
    job_names.sort();

    html! {
        @for job_name in job_names {
            h3 {
                (job_name) ": " (job_affinity_sums.get(job_name).unwrap_or(&0)) "%"
            }
            @for affinity_bonus in active_affinity_bonuses_for_jobs.get(job_name).unwrap_or(&vec![]) {
                p { b { (affinity_bonus.name) } ": " (affinity_bonus.description)}
            }
        }
    }
}

fn capitalise_first_letter(s: &str) -> String {
    let mut char_iter = s.chars();
    match char_iter.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + char_iter.as_str(),
    }
}
