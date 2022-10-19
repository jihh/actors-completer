use crate::tmdb::models::{Configuration, Country, Job, Language, Timezone};
use crate::tmdb::Configuration as Config;
use crate::tmdb::Error;
use std::borrow::Borrow;
use std::rc::Rc;
pub trait ConfigurationClient {
    fn configuration(&self) -> Result<Configuration, Error>;
    fn countries(&self) -> Result<Vec<Country>, Error>;
    fn jobs(&self) -> Result<Vec<Job>, Error>;
    fn languages(&self) -> Result<Vec<Language>, Error>;
    fn primary_translations(&self) -> Result<Vec<String>, Error>;
    fn timezones(&self) -> Result<Vec<Timezone>, Error>;
}

pub struct ConfigurationApiClient {
    configuration: Rc<Config>,
}
impl ConfigurationApiClient {
    pub fn new(configuration: Rc<Config>) -> ConfigurationApiClient {
        ConfigurationApiClient { configuration }
    }
}

impl ConfigurationClient for ConfigurationApiClient {
    fn configuration(&self) -> Result<Configuration, Error> {
        let config: &Config = self.configuration.borrow();
        let client = &config.client;
        let url = format!("{}/configuration", config.base_path);
        let mut builder = client.get(url.as_str());
        builder = builder.query(&[("api_key", &config.api_key)]);
        let request = builder.build()?;
        Ok(client.execute(request)?.error_for_status()?.json()?)
    }

    fn countries(&self) -> Result<Vec<Country>, Error> {
        let config: &Config = self.configuration.borrow();
        let client = &config.client;
        let url = format!("{}/configuration/countries", config.base_path);
        let mut builder = client.get(url.as_str());
        builder = builder.query(&[("api_key", &config.api_key)]);
        let request = builder.build()?;
        Ok(client.execute(request)?.error_for_status()?.json()?)
    }

    fn jobs(&self) -> Result<Vec<Job>, Error> {
        let config: &Config = self.configuration.borrow();
        let client = &config.client;
        let url = format!("{}/configuration/jobs", config.base_path);
        let mut builder = client.get(url.as_str());
        builder = builder.query(&[("api_key", &config.api_key)]);
        let request = builder.build()?;
        Ok(client.execute(request)?.error_for_status()?.json()?)
    }

    fn languages(&self) -> Result<Vec<Language>, Error> {
        let config: &Config = self.configuration.borrow();
        let client = &config.client;
        let url = format!("{}/configuration/languages", config.base_path);
        let mut builder = client.get(url.as_str());
        builder = builder.query(&[("api_key", &config.api_key)]);
        let request = builder.build()?;
        Ok(client.execute(request)?.error_for_status()?.json()?)
    }

    fn primary_translations(&self) -> Result<Vec<String>, Error> {
        let config: &Config = self.configuration.borrow();
        let client = &config.client;
        let url = format!("{}/configuration/primary_translations", config.base_path);
        let mut builder = client.get(url.as_str());
        builder = builder.query(&[("api_key", &config.api_key)]);
        let request = builder.build()?;
        Ok(client.execute(request)?.error_for_status()?.json()?)
    }

    fn timezones(&self) -> Result<Vec<Timezone>, Error> {
        let config: &Config = self.configuration.borrow();
        let client = &config.client;
        let url = format!("{}/configuration/timezones", config.base_path);
        let mut builder = client.get(url.as_str());
        builder = builder.query(&[("api_key", &config.api_key)]);
        let request = builder.build()?;
        Ok(client.execute(request)?.error_for_status()?.json()?)
    }
}
