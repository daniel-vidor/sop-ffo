use std::collections::{BTreeMap, HashMap};

use maud::{html, Markup, DOCTYPE};

use crate::{
    model::{AffinityBonus, Job, JobTier},
    FormData,
};

pub fn head_template() -> Markup {
    html! {
        head {
            title { "Stranger of Paradise: Final Fantasy Origin | Build simulator" }
            link rel="icon" href="/static/favicon.png" {}
            link rel="stylesheet" href="/static/styles.css" {}
            link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;700&display=swap" rel="stylesheet" {}
            script src="https://unpkg.com/htmx.org@1.9.2" {}
        }
    }
}

pub fn index_template(
    equipment_slot_names: Vec<String>,
    jobs: &[Job],
    job_affinity_sums: HashMap<String, u32>,
    active_affinity_bonuses_for_jobs: HashMap<String, BTreeMap<u32, AffinityBonus>>,
) -> Markup {
    html! {
        (DOCTYPE)
        html {
            (head_template())
            body {
                img class="sop-ffo-logo" src="/static/logo.png" alt="Stranger of Paradise: Final Fantasy Origin" {}
                div class="panel" {
                    h2 { "Save & Load" }
                    div class="panel-contents" {
                        button hx-post="/test-load" hx-target="#page-content" { "Load sample build" }
                    }
                }

                div id="page-content" {
                    (page_content_template(equipment_slot_names, jobs, job_affinity_sums, active_affinity_bonuses_for_jobs, &FormData::new()))
                }
            }
        }
    }
}

pub fn page_content_template(
    equipment_slot_names: Vec<String>,
    jobs: &[Job],
    job_affinity_sums: HashMap<String, u32>,
    active_affinity_bonuses_for_jobs: HashMap<String, BTreeMap<u32, AffinityBonus>>,
    form_data: &FormData,
) -> Markup {
    html! {
        form hx-post="/update" hx-trigger="change" hx-target="#page-content" enctype="json" {
            div class="panel" {
                h2 { "Job" }
                div class="panel-contents" {
                    div class="job-form" {
                        (active_job_template(&jobs, form_data.active_job.clone()))
                    }
                    div class="weapon-type-form" {
                        label {"Class (WIP)"}
                        div {
                            input type="radio" name="job-class" value="(None)" checked {}
                            label { "(None)" }
                        }
                        div {
                            input type="radio" name="job-class" value="evocation" {}
                            label { "Evocation" }
                        }
                        div {
                            input type="radio" name="job-class" value="ultima" {}
                            label { "Ultima" }
                        }
                    }
                }
            }

            div class="panel" {
                h2 { "Equipment" }
                div id="equipment-panel" class="panel-contents" {
                    (equipment_form_template(equipment_slot_names, &jobs, form_data))
                }
            }
        }

        div class="panel" {
            h2 { "Job Affinity Bonus" }
            div class="panel-contents" id="result" {
                (active_job_affinities_template(job_affinity_sums, active_affinity_bonuses_for_jobs))
            }
        }
    }
}

pub fn active_job_template(jobs: &[Job], active_job: String) -> Markup {
    html! {
        label for="active_job" {"Active job"}
        select name="active_job" {
            (get_job_options(jobs, active_job))
        }
        input name="active_job_strength"
            type="number" min="0" max="999" value="800" {} "%"
    }
}

// To do: do not use FormData directly; map FormData to a better model
pub fn equipment_form_template(
    slot_names: Vec<String>,
    jobs: &[Job],
    form_data: &FormData,
) -> Markup {
    html! {
        div class="weapon-type-form" {
            label { "Weapon Type" }
            div {
                input type="radio" name="weapon_type" value="1H"
                checked?[form_data.weapon_type == "1H"] {}
                label { "One-handed" }
            }
            div {
                input type="radio" name="weapon_type" value="2H"
                checked?[form_data.weapon_type == "2H"] {}
                label { "Two-handed" }
            }
        }

        div class="weapon-type-form" {
            label { "Body Equipment Type" }
            @let value_names = ["body-only", "body-head", "body-leg"];
            @for value_name in value_names {
                div {
                    @let label_text = match value_name {
                        "body-only" => "Body",
                        "body-head" => "Body + Head",
                        "body-leg" => "Body + Legs",
                        _ => ""
                    };
                    input type="radio" name="body_type" value=(value_name) checked?[form_data.body_type == value_name] {}
                    label { (label_text) }
                }
            }
        }

        div id="equipment-form" class="equipment-form" {
            @for slot_name in slot_names {
                @let is_slot_disabled =
                    slot_name == "shield" && form_data.weapon_type == "2H" ||
                    slot_name == "head" && form_data.body_type == "body-head" ||
                    slot_name == "leg" && form_data.body_type == "body-leg";

                @let selected_jobs_and_strength = match slot_name.as_str() {
                    "weapon" => [form_data.weapon_job1.clone(), form_data.weapon_job2.clone(), form_data.weapon_strength.to_string()],
                    "shield" => [form_data.shield_job1.clone(), form_data.shield_job2.clone(), form_data.shield_strength.to_string()],
                    "head" => [form_data.head_job1.clone(), form_data.head_job2.clone(), form_data.head_strength.to_string()],
                    "body" => [form_data.body_job1.clone(), form_data.body_job2.clone(), form_data.body_strength.to_string()],
                    "hand" => [form_data.hand_job1.clone(), form_data.hand_job2.clone(), form_data.hand_strength.to_string()],
                    "leg" => [form_data.leg_job1.clone(), form_data.leg_job2.clone(), form_data.leg_strength.to_string()],
                    "foot" => [form_data.foot_job1.clone(), form_data.foot_job2.clone(), form_data.foot_strength.to_string()],
                    _ => panic!(),
                };

                label for=(slot_name) {(capitalise_first_letter(&slot_name))}

                @for n in 1..3 {
                    @let slot_job_name = format!("{slot_name}_job{n}");
                    select id=(slot_job_name) name=(slot_job_name) disabled?[is_slot_disabled] {
                        (get_job_options(jobs, selected_jobs_and_strength[n - 1].clone()))
                    }
                    // If a form element is disabled, then its form_data is not sent in the HTML request.
                    // This breaks the deserialisation into FormData, so create a hidden field to be used in its stead
                    // https://stackoverflow.com/questions/7357256/disabled-form-inputs-do-not-appear-in-the-request
                    @if is_slot_disabled {
                        select name=(slot_job_name) hidden value="(None)" {}
                    }
                }

                @let slot_strength_name = format!("{slot_name}_strength");

                input id=(slot_strength_name) name=(slot_strength_name) disabled?[is_slot_disabled]
                    type="number" min="0" max="999" value=(selected_jobs_and_strength[2]) {} "%"

                @if is_slot_disabled {
                    input name=(slot_strength_name) hidden
                        type="number" min="0" max="999" value=(0) {}
                }
            }
        }
    }
}

fn get_job_options(jobs: &[Job], selected_job: String) -> Markup {
    let job_tiers = vec![JobTier::Basic, JobTier::Advanced, JobTier::Expert];

    html! {
        option { "(None)" }
        @for job_tier in job_tiers {
            // A disabled option is to create a "header" of sorts in the dropdown
            option disabled { (job_tier.to_string()) }

            @for job in jobs.iter().filter(|job| job.tier == job_tier) {
                option value=(job.name) selected?[job.name == selected_job] { (job.name) }
            }
        }
    }
}

pub fn active_job_affinities_template(
    job_affinity_sums: HashMap<String, u32>,
    active_affinity_bonuses_for_jobs: HashMap<String, BTreeMap<u32, AffinityBonus>>,
) -> Markup {
    let mut job_names: Vec<&String> = job_affinity_sums.keys().collect();
    job_names.sort();

    let mut job_affinities: Vec<(&String, &u32)> = job_affinity_sums.iter().collect();
    job_affinities.sort_by(|a, b| b.1.cmp(a.1));
    println!("{:?}", job_affinities);

    let no_active_bonuses_text = "No job affinity bonuses are active.";
    let empty_map: BTreeMap<u32, AffinityBonus> = BTreeMap::new();

    html! {
        @if job_affinities.is_empty() {
            p { (no_active_bonuses_text) }
        }
        @else {
            @for job_affinity_pair in job_affinities {
                div {
                    h3 {
                        (job_affinity_pair.0)
                        // (job_affinity_pair.0) ": " (job_affinity_sums.get(job_affinity_pair.0).unwrap_or(&0)) "%"
                    }

                    @let active_affinity_bonuses_for_job = active_affinity_bonuses_for_jobs.get(job_affinity_pair.0).unwrap_or(&empty_map);
                    @if active_affinity_bonuses_for_job.is_empty() {
                        p {(no_active_bonuses_text)}
                    } @else {
                        @for active_affinity_bonus in active_affinity_bonuses_for_job {
                            div class="active_affinity_bonus" {
                                div {
                                    span class="active_affinity_bonus__strength" {
                                        (active_affinity_bonus.0) "%"
                                    }
                                    span class="active_affinity_bonus__name" {
                                        (active_affinity_bonus.1.name)
                                    }
                                }
                                div class="active_affinity_bonus__description" {
                                    span  {
                                        (active_affinity_bonus.1.description)
                                    }
                                }
                            }
                        }
                    }
                }
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
