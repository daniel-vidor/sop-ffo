use std::collections::HashMap;

use maud::{html, Markup, DOCTYPE};

use crate::model::{AffinityBonus, Job, JobTier};

pub fn head_template() -> Markup {
    html! {
        head {
            title { "Stranger of Paradise: Final Fantasy Origin | Build simulator" }
            link rel="icon" href="/static/favicon.png" {}
            link rel="stylesheet" href="/static/styles.css";
            script src="https://unpkg.com/htmx.org@1.9.2" {}
        }
    }
}

pub fn index_template(equipment_slot_names: Vec<String>, jobs: &[Job]) -> Markup {
    html! {
        (DOCTYPE)
        html {
            (head_template())
            body {
                h1 { "Stranger of Paradise: Final Fantasy Origin | Build simulator" }

                form hx-post="/update" hx-trigger="change" hx-target="#result" enctype="json" {
                    h2 { "Job" }
                    (active_job_template(&jobs))

                    h2 { "Equipment" }
                    (equipment_form_template(equipment_slot_names, &jobs))
                }

                h2 { "Job Affinity Bonus" }
                div id="result" {
                    p { "Please select an option to see the result." }
                }
            }
        }
    }
}

pub fn active_job_template(jobs: &[Job]) -> Markup {
    html! {
        select name="active_job" {
            (get_job_options(jobs))
        }
        input name="active_job_strength"
            type="number" min="0" max="999" value="50" {} "%"
    }
}

pub fn equipment_form_template(slot_names: Vec<String>, jobs: &[Job]) -> Markup {
    html! {
        @for slot_name in slot_names {
            div {
                label for=(slot_name) {(capitalise_first_letter(&slot_name))}
                select name=(format!("{slot_name}_job1")) {
                    (get_job_options(jobs))
                }
                select name=(format!("{slot_name}_job2")) {
                    (get_job_options(jobs))
                }
                input name=(format!("{slot_name}_strength"))
                    type="number" min="0" max="999" value="250" {} "%"
            }
        }
    }
}

fn get_job_options(jobs: &[Job]) -> Markup {
    let job_tiers = vec![JobTier::Basic, JobTier::Advanced, JobTier::Expert];

    html! {
        @for job_tier in job_tiers {
            // A disabled option is to create a "header" of sorts in the dropdown
            option disabled { (job_tier.to_string()) }

            @for job in jobs.iter().filter(|job| job.tier == job_tier) {
                option value=(job.name) { (job.name) }
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
