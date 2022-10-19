mod configuration;
pub use self::configuration::*;

mod people;
pub use self::people::Details as PeopleDetails;

mod search;
pub use self::search::people::PaginatedPeoples;
pub use self::search::people::PersonObject;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum MediaType {
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "person")]
    Person,
    #[serde(rename = "tv")]
    Tv,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Gender {
    Unknown = 0,
    Male = 1,
    Female = 2,
    Nonbinary = 3,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Images {
    pub id: Option<u32>,
    pub profiles: Option<Vec<Image>>,
}
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Image {
    pub aspect_ratio: Option<f32>,
    pub file_path: Option<String>,
    pub height: Option<u32>,
    pub iso_639_1: Option<String>,
    pub vote_average: Option<f32>,
    pub vote_count: Option<u32>,
    pub width: Option<u32>,
}
