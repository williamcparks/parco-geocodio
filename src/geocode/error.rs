use thiserror::Error;

use crate::{AccuracyType, ApiError};

/// Errors that can occur while performing a geocoding request.
#[derive(Debug, Error)]
pub enum GeocodeError {
    /// HTTP or network error from [`reqwest`].
    #[error("Reqwest: {0}")]
    Reqwest(
        #[from]
        #[source]
        reqwest::Error,
    ),

    /// An Api Error From Geocodio
    #[error("Geocodio: {0}")]
    Api(
        #[from]
        #[source]
        ApiError,
    ),

    /// Failed to parse the JSON response.
    ///
    /// Contains the parse error and the raw response body.
    #[error("Json: {0} - {1}")]
    Json(#[source] serde_json::Error, String),

    /// The request succeeded but returned no results.
    #[error("No Results In Geocodio Response: {0}")]
    NoResults(String),

    /// The returned result did not meet the required accuracy threshold.
    ///
    /// Contains the accuracy score, accuracy type, and formatted address.
    #[error("The Address: {2} Was Not Accurate Enough: {0} / {1:?}")]
    NotAccurate(f64, AccuracyType, Box<str>),
}
