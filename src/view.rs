use std::collections::HashMap;

use maud::{html, Markup};

use crate::model::AffinityBonus;


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

pub fn display_active_job_affinities(
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
