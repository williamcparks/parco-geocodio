#[derive(Clone, Debug)]
pub struct GeocodioClient<'a, 'b> {
    client: reqwest::Client,
    api_key: &'a str,
    base_url: &'b str,
}

mod builder;
mod geocode;
mod urls;
