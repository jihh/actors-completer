use crate::tmdb::models::{Images, PeopleDetails};
use crate::tmdb::{Configuration, Error};
use std::borrow::Borrow;
use std::rc::Rc;

pub trait PeopleClient {
    fn details(
        &self,
        person_id: u32,
        language: Option<&str>,
        append_to_response: Option<&str>,
    ) -> Result<PeopleDetails, Error>;

    fn images(&self, person_id: u32) -> Result<Images, Error>;
}
pub struct PeopleApiClient {
    configuration: Rc<Configuration>,
}
impl PeopleApiClient {
    pub fn new(configuration: Rc<Configuration>) -> Self {
        PeopleApiClient { configuration }
    }
}

impl PeopleClient for PeopleApiClient {
    fn details(
        &self,
        person_id: u32,
        language: Option<&str>,
        append_to_response: Option<&str>,
    ) -> Result<PeopleDetails, Error> {
        let config: &Configuration = self.configuration.borrow();
        let client = &config.client;
        let url = format!(
            "{}/person/{person_id}",
            config.base_path,
            person_id = person_id
        );
        let mut builder = client.get(url.as_str());
        builder = builder.query(&[("api_key", &config.api_key)]);
        if let Some(ref s) = language {
            builder = builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = append_to_response {
            builder = builder.query(&[("append_to_response", &s.to_string())]);
        }
        let request = builder.build()?;
        Ok(client.execute(request)?.error_for_status()?.json()?)
    }

    fn images(&self, person_id: u32) -> Result<crate::tmdb::models::Images, Error> {
        let config: &Configuration = self.configuration.borrow();
        let client = &config.client;
        let url = format!(
            "{}/person/{person_id}/images",
            config.base_path,
            person_id = person_id
        );
        let mut builder = client.get(url.as_str());
        builder = builder.query(&[("api_key", &config.api_key)]);
        let request = builder.build()?;
        Ok(client.execute(request)?.error_for_status()?.json()?)
    }
}
