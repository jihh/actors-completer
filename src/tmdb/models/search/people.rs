use crate::tmdb::models::MediaType;
use serde::Deserialize;

// Api DOC: https://developers.themoviedb.org/3/search/search-people

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PaginatedPeoples {
    pub page: Option<u32>,
    pub total_page: Option<u32>,
    pub total_result: Option<u32>,
    pub results: Option<Vec<PersonObject>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PersonObject {
    pub profile_path: Option<String>,
    pub adult: Option<bool>,
    pub id: Option<u32>,
    pub known_for: Option<Vec<serde_json::Value>>,
    pub name: Option<String>,
    pub popularity: Option<f32>,
    pub media_type: Option<MediaType>,
}

impl Default for PersonObject {
    fn default() -> Self {
        PersonObject {
            profile_path: None,
            adult: None,
            id: None,
            known_for: None,
            name: None,
            popularity: None,
            media_type: Some(MediaType::Person),
        }
    }
}
