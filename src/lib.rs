mod client;
mod geocode;

pub use client::GeocodioClient;
pub use geocode::{AccuracyType, GeocodeError, Geocoded, Location};

#[cfg(test)]
mod test;
