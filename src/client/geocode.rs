use std::borrow::Cow;

use crate::{GeocodioClient, geocode::request_builder::GeocodeRequestBuilder};

impl<'a, 'b> GeocodioClient<'a, 'b> {
    /// Construct a geocode request
    pub fn geocode(&self) -> GeocodeRequestBuilder<'a, 'b> {
        use super::urls::{DEFAULT_BASE_URL, DEFAULT_GEOCODE_URL, build_url};

        let url = match self.base_url == DEFAULT_BASE_URL {
            true => Cow::Borrowed(DEFAULT_GEOCODE_URL),
            false => Cow::Owned(build_url(self.base_url, "geocode")),
        };

        GeocodeRequestBuilder::new(self.client.clone(), self.api_key, url)
    }
}
