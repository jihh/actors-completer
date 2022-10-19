use super::super::Configuration;
use crate::tmdb::Error;
use std::{borrow::Borrow, rc::Rc};

pub trait SearchClient {
    fn search_people(
        &self,
        language: Option<&str>,
        query: &str,
        page: Option<u32>,
        include_adult: Option<bool>,
        region: Option<&str>,
    ) -> Result<crate::tmdb::models::PaginatedPeoples, Error>;
}
pub struct SearchApiClient {
    configuration: Rc<Configuration>,
}

impl SearchApiClient {
    pub fn new(configuration: Rc<Configuration>) -> Self {
        SearchApiClient { configuration }
    }
}

impl SearchClient for SearchApiClient {
    fn search_people(
        &self,
        language: Option<&str>,
        query: &str,
        page: Option<u32>,
        include_adult: Option<bool>,
        region: Option<&str>,
    ) -> Result<crate::tmdb::models::PaginatedPeoples, Error> {
        let configuration: &Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/search/person", configuration.base_path);
        let mut request_builder = client.get(uri_str.as_str());
        request_builder =
            request_builder.query(&[("api_key", (&configuration.api_key).to_string())]);
        if let Some(s) = language.or(Some("en-US")) {
            request_builder = request_builder.query(&[("language", &s.to_string())]);
        }
        request_builder = request_builder.query(&[("query", &query.to_string())]);
        if let Some(s) = page.or(Some(1)) {
            request_builder = request_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(s) = include_adult {
            request_builder = request_builder.query(&[("include_adult", &s.to_string())]);
        }
        if let Some(s) = region {
            request_builder = request_builder.query(&[("region", &s.to_string())]);
        }

        let request = request_builder.build()?;
        Ok(client.execute(request)?.error_for_status()?.json()?)
    }
}
