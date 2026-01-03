use crate::GeocodioClient;

pub struct GeocodioClientBuilder;

impl GeocodioClientBuilder {
    pub fn client(self, client: reqwest::Client) -> GeocodioClientBuilderWithClient {
        GeocodioClientBuilderWithClient { client }
    }
}

pub struct GeocodioClientBuilderWithClient {
    client: reqwest::Client,
}

impl GeocodioClientBuilderWithClient {
    pub fn api_key<'a>(self, api_key: &'a str) -> GeocodioClientBuilderWithApiKey<'a> {
        GeocodioClientBuilderWithApiKey {
            client: self.client,
            api_key,
        }
    }
}

pub struct GeocodioClientBuilderWithApiKey<'a> {
    client: reqwest::Client,
    api_key: &'a str,
}

impl<'a> GeocodioClientBuilderWithApiKey<'a> {
    pub fn build(self) -> GeocodioClient<'a, 'static> {
        use super::urls::DEFAULT_BASE_URL;
        GeocodioClient {
            client: self.client,
            api_key: self.api_key,
            base_url: DEFAULT_BASE_URL,
        }
    }

    pub fn base_url<'b>(self, base_url: &'b str) -> GeocodioClient<'a, 'b> {
        GeocodioClient {
            client: self.client,
            api_key: self.api_key,
            base_url,
        }
    }
}

impl<'a, 'b> GeocodioClient<'a, 'b> {
    pub fn builder() -> GeocodioClientBuilder {
        GeocodioClientBuilder
    }
}
