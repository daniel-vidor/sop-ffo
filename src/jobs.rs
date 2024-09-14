use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Job {
    name: String,
    tier: String,
    r#type: String,
    // weapons: Vec<String>
}

pub struct Affinities {
    _50: String,
    _120: String,
    _250: String,
    _400: String,
    _600: String,
}
