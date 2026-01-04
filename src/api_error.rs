use std::fmt::Display;

use serde::Deserialize;
use thiserror::Error;

/// An Api Error Message From Geocodio
#[derive(Debug, Deserialize, Error)]
pub struct ApiError {
    pub error: Box<str>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.error.as_ref())
    }
}
