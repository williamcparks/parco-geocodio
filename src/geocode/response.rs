use serde::Deserialize;

use crate::GeocodeError;

/// Raw response returned by the Geocodio geocode API.
#[derive(Clone, Debug, Deserialize)]
pub struct GeocodeResponse {
    /// List of geocoded results.
    pub results: Vec<Geocoded>,
}

/// A single geocoded address result.
#[derive(Clone, Debug, Deserialize)]
pub struct Geocoded {
    /// Parsed address components keyed by component name.
    pub address_components: serde_json::Value,

    /// Individual address lines returned by Geocodio.
    pub address_lines: Vec<Box<str>>,

    /// Fully formatted address.
    pub formatted_address: Box<str>,

    /// Geographic coordinates of the result.
    pub location: Location,

    /// Accuracy score between 0.0 and 1.0.
    pub accuracy: f64,

    /// Type describing how the location was determined.
    pub accuracy_type: AccuracyType,
}

/// Geographic coordinates for a geocoded result.
#[derive(Clone, Debug, Deserialize)]
pub struct Location {
    /// Longitude.
    pub lng: f64,

    /// Latitude.
    pub lat: f64,
}

/// Describes how precise a geocoded location is.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccuracyType {
    /// Exact rooftop-level match.
    Rooftop,

    /// Single point location.
    Point,

    /// Interpolated range along a street.
    RangeInterpolation,

    /// Nearest known rooftop match.
    NearestRooftopMatch,

    /// Street intersection.
    Intersection,

    /// Center of a street.
    StreetCenter,

    /// Named place or landmark.
    Place,

    /// County-level location.
    County,

    /// State-level location.
    State,
}

impl Geocoded {
    /// Ensures the result meets a minimum accuracy threshold.
    ///
    /// Returns an error if the accuracy is not greater than `0.8`.
    pub fn try_accurate(self) -> Result<Self, GeocodeError> {
        match self.accuracy > 0.8 {
            true => Ok(self),
            false => Err(GeocodeError::NotAccurate(
                self.accuracy,
                self.accuracy_type,
                self.formatted_address,
            )),
        }
    }
}
