use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Configuration {
    pub images: Option<ConfigurationImages>,
    pub change_keys: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ConfigurationImages {
    pub base_url: Option<String>,
    pub secure_base_url: Option<String>,
    pub backdrop_sizes: Option<Vec<String>>,
    pub log_sizes: Option<Vec<String>>,
    pub poster_sizes: Option<Vec<String>>,
    pub prifile_sizes: Option<Vec<String>>,
    pub still_sizes: Option<Vec<String>>,
}
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Country {
    pub iso_3166_1: Option<String>,
    pub english_name: Option<String>,
}
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Language {
    pub iso_639_1: Option<String>,
    pub english_name: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Job {
    pub department: Option<String>,
    pub jobs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Timezone {
    pub iso_3166_1: Option<String>,
    pub zones: Option<Vec<String>>,
}
