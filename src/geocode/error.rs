use thiserror::Error;

use crate::AccuracyType;

#[derive(Debug, Error)]
pub enum GeocodeError {
    #[error("Reqwest: {0}")]
    Reqwest(
        #[from]
        #[source]
        reqwest::Error,
    ),

    #[error("Json: {0} - {1}")]
    Json(#[source] serde_json::Error, String),

    #[error("No Results In Geocodio Response: {0}")]
    NoResults(String),

    #[error("The Address: {2} Was Not Accurate Enough: {0} / {1:?}")]
    NotAccurate(f64, AccuracyType, Box<str>),
}
