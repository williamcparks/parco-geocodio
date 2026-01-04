use std::borrow::Cow;

use super::{
    error::GeocodeError,
    response::{GeocodeResponse, Geocoded},
};

pub struct AddressRequest<'a, 'b, 'c, 'd> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,

    q: &'c str,
    country: Option<&'d str>,
    limit: u64,
}

impl<'a, 'b, 'c, 'd> AddressRequest<'a, 'b, 'c, 'd> {
    pub(crate) fn new(
        client: reqwest::Client,
        api_key: &'a str,
        url: Cow<'b, str>,
        q: &'c str,
        country: Option<&'d str>,
        limit: u64,
    ) -> Self {
        Self {
            client,
            api_key,
            url,

            q,
            country,
            limit,
        }
    }

    pub async fn send(self) -> Result<Geocoded, GeocodeError> {
        let req = self
            .client
            .get(self.url.as_ref())
            .query(&[("api_key", self.api_key), ("q", self.q)]);

        let req = match self.country {
            Some(country) => req.query(&[("country", country)]),
            None => req,
        };

        let req = match self.limit {
            limit @ 1.. => req.query(&[("limit", limit)]),
            0 => req,
        };

        let src = req.send().await?.text().await?;

        let response: GeocodeResponse = match serde_json::from_str(src.as_str()) {
            Ok(ok) => ok,
            Err(err) => return Err(GeocodeError::Json(err, src)),
        };

        match response.results.into_iter().next() {
            Some(some) => Ok(some),
            None => Err(GeocodeError::NoResults(src)),
        }
    }
}
