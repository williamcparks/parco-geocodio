#![doc = include_str!("../README.md")]

mod api_error;
mod client;
mod geocode;

pub use api_error::ApiError;
pub use client::GeocodioClient;
pub use geocode::{AccuracyType, GeocodeError, Geocoded, Location};

#[cfg(test)]
mod test;
