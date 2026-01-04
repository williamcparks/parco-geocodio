/// A reusable client for interacting with the Geocodio API.
///
/// Created via the [`GeocodioClient::builder`] Method
#[derive(Clone, Debug)]
pub struct GeocodioClient<'a, 'b> {
    client: reqwest::Client,
    api_key: &'a str,
    base_url: &'b str,
}

mod builder;
mod geocode;
mod urls;
