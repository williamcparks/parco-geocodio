use crate::GeocodioClient;

/// Builder for constructing a [`GeocodioClient`].
///
/// Enforces providing a [`reqwest::Client`] and API key before use.
pub struct GeocodioClientBuilder;

impl GeocodioClientBuilder {
    /// Sets the underlying [`reqwest::Client`].
    pub fn client(self, client: reqwest::Client) -> GeocodioClientBuilderWithClient {
        GeocodioClientBuilderWithClient { client }
    }
}

/// Intermediate builder state after a [`reqwest::Client`] is provided.
pub struct GeocodioClientBuilderWithClient {
    client: reqwest::Client,
}

impl GeocodioClientBuilderWithClient {
    /// Sets the Geocodio API key.
    pub fn api_key<'a>(self, api_key: &'a str) -> GeocodioClientBuilderWithApiKey<'a> {
        GeocodioClientBuilderWithApiKey {
            client: self.client,
            api_key,
        }
    }
}

/// Final builder state with an API key set.
pub struct GeocodioClientBuilderWithApiKey<'a> {
    client: reqwest::Client,
    api_key: &'a str,
}

impl<'a> GeocodioClientBuilderWithApiKey<'a> {
    /// Builds a client using the default Geocodio base URL.
    pub fn build(self) -> GeocodioClient<'a, 'static> {
        use super::urls::DEFAULT_BASE_URL;
        GeocodioClient {
            client: self.client,
            api_key: self.api_key,
            base_url: DEFAULT_BASE_URL,
        }
    }

    /// Builds a client using a custom base URL.
    pub fn build_with_base_url<'b>(self, base_url: &'b str) -> GeocodioClient<'a, 'b> {
        GeocodioClient {
            client: self.client,
            api_key: self.api_key,
            base_url,
        }
    }
}

impl<'a, 'b> GeocodioClient<'a, 'b> {
    /// Create a client builder
    pub fn builder() -> GeocodioClientBuilder {
        GeocodioClientBuilder
    }
}
