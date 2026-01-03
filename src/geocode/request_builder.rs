use std::borrow::Cow;

use super::request::{AddressRequest, Format};

pub struct GeocodeRequestBuilder<'a, 'b> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
}

impl<'a, 'b> GeocodeRequestBuilder<'a, 'b> {
    pub(crate) fn new(client: reqwest::Client, api_key: &'a str, url: Cow<'b, str>) -> Self {
        Self {
            client,
            api_key,
            url,
        }
    }

    pub fn address<'c, 'd>(
        self,
        address: &'c str,
    ) -> GeocodeRequestBuilderWithAddress<'a, 'b, 'c, 'd> {
        GeocodeRequestBuilderWithAddress {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            address,
            country: None,
            limit: 0,
            format: None,
        }
    }
}

pub struct GeocodeRequestBuilderWithAddress<'a, 'b, 'c, 'd> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    address: &'c str,
    country: Option<&'d str>,
    limit: u64,
    format: Option<Format>,
}

impl<'a, 'b, 'c, 'd> GeocodeRequestBuilderWithAddress<'a, 'b, 'c, 'd> {
    pub fn country(self, country: &'d str) -> Self {
        Self {
            country: Some(country),
            ..self
        }
    }

    pub fn limit(self, limit: u64) -> Self {
        Self { limit, ..self }
    }

    pub fn format(self, format: Format) -> Self {
        Self {
            format: Some(format),
            ..self
        }
    }

    pub fn build(self) -> AddressRequest<'a, 'b, 'c, 'd> {
        AddressRequest::new(
            self.client,
            self.api_key,
            self.url,
            self.address,
            self.country,
            self.limit,
            self.format,
        )
    }
}
