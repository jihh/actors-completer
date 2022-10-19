use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Details {
    pub birthday: Option<String>,
    pub known_for_department: Option<String>,
    pub deathday: Option<String>,
    pub id: Option<u32>,
    pub name: Option<String>,
    pub also_known_as: Option<Vec<String>>,
    pub gender: Option<crate::tmdb::models::Gender>,
    pub biography: Option<String>,
    pub popularity: Option<f32>,
    pub place_of_birth: Option<String>,
    pub profile_path: Option<String>,
    pub adult: Option<bool>,
    pub imdb_id: Option<String>,
    pub homepage: Option<String>,
}