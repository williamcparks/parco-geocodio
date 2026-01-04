# Geocodio (Rust)

A small, async Rust client for the [Geocodio](https://www.geocod.io/) API, built on top of `reqwest` with a type-safe builder API.

This library is intentionally minimal and focuses on:

- Explicit client construction
- A fluent geocode request builder
- Strongly typed error handling

## Constructing A Client

```rust,ignore
use geocodio::GeocodioClient;

let http_client = reqwest::Client::new();

let geocodio = GeocodioClient::builder()
    .client(http_client)
    .api_key("YOUR_API_KEY")
    .build();
```

## Geocoding An Address

```rust,ignore
let result = geocodio
    .geocode()
    .address("1600 Pennsylvania Ave NW, Washington, DC")
    .build()
    .send()
    .await?;
```
