pub mod clients;
pub mod models;

use self::clients::*;
use std::rc::Rc;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

pub struct ClientBuilder {
    // configuration: Rc<Configuration>,
    configuration_client: Box<dyn ConfigurationClient>,
    people_client: Box<dyn PeopleClient>,
    search_client: Box<dyn SearchClient>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::new_with_configuration(Configuration::default())
    }
    pub fn new_with_api_key<T: Into<String>>(api_key: T) -> Self {
        Self::new_with_configuration(Configuration::new(api_key))
    }
    pub fn new_with_configuration(configuration: Configuration) -> Self {
        let rc = Rc::new(configuration);
        ClientBuilder {
            configuration_client: Box::new(ConfigurationApiClient::new(rc.clone())),
            people_client: Box::new(PeopleApiClient::new(rc.clone())),
            search_client: Box::new(SearchApiClient::new(rc.clone())),
        }
    }

    pub fn search_client(&self) -> &dyn SearchClient {
        self.search_client.as_ref()
    }
    pub fn people_client(&self) -> &dyn PeopleClient {
        self.people_client.as_ref()
    }
    pub fn configuration_client(&self) -> &dyn ConfigurationClient {
        self.configuration_client.as_ref()
    }
}

pub struct Configuration {
    pub base_path: String,
    pub api_key: String,
    pub client: reqwest::blocking::Client,
}
impl Configuration {
    pub fn new<T: Into<String>>(api_key: T) -> Self {
        let mut config = Configuration::default();
        config.api_key = api_key.into();
        config
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "https://api.themoviedb.org/3".to_owned(),
            api_key: "13b64e3ea449d218a421a7f5d2c8176e".to_owned(),
            client: reqwest::blocking::Client::new(),
        }
    }
}
