use std::collections::HashMap;

use serde::Deserialize;

use crate::GeocodeError;

#[derive(Clone, Debug, Deserialize)]
pub struct GeocodeResponse {
    pub results: Vec<Geocoded>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Geocoded {
    pub address_components: HashMap<Box<str>, Box<str>>,
    pub address_lines: Vec<Box<str>>,
    pub formatted_address: Box<str>,
    pub location: Location,
    pub accuracy: f64,
    pub accuracy_type: AccuracyType,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Location {
    pub lng: f64,
    pub lat: f64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccuracyType {
    Rooftop,
    Point,
    RangeInterpolation,
    NearestRooftopMatch,
    Intersection,
    StreetCenter,
    Place,
    County,
    State,
}

impl Geocoded {
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
