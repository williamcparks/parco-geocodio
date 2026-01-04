use std::borrow::Cow;

use super::request::AddressRequest;

/// Builder for creating a geocode request.
///
/// Returned from [`crate::GeocodioClient`](GeocodioClient).
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

    /// Sets the address to be geocoded.
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
        }
    }
}

/// Builder for a geocode request with an address set.
pub struct GeocodeRequestBuilderWithAddress<'a, 'b, 'c, 'd> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    address: &'c str,
    country: Option<&'d str>,
    limit: u64,
}

impl<'a, 'b, 'c, 'd> GeocodeRequestBuilderWithAddress<'a, 'b, 'c, 'd> {
    /// Restricts results to a specific country.
    pub fn country(self, country: &'d str) -> Self {
        Self {
            country: Some(country),
            ..self
        }
    }

    /// Limits the number of results returned.
    pub fn limit(self, limit: u64) -> Self {
        Self { limit, ..self }
    }

    /// Finalizes the request and returns an executable geocode request.
    pub fn build(self) -> AddressRequest<'a, 'b, 'c, 'd> {
        AddressRequest::new(
            self.client,
            self.api_key,
            self.url,
            self.address,
            self.country,
            self.limit,
        )
    }
}
