use std::fmt;

use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: unable to access the API")]
    HttpError(#[from] reqwest::Error),
    #[error("Error extracting response: {0}")]
    GetTextError(String),
    #[error("Error unmarshalling JSON response: {0}")]
    UnmarshalJsonError(String),
    #[error("API error: {code} - {message}")]
    ApiError {
        code: u16,
        message: String,
        detail: Option<Vec<String>>,
    },
}

/// A list of features (addresses) returned by the API
#[derive(Deserialize, Debug)]
pub struct AddressResult {
    pub r#type: String,
    pub features: Vec<FeatureResult>,
}

/// A feature is basically a single address + its coordinates
#[derive(Deserialize, Debug)]
pub struct FeatureResult {
    pub r#type: String,
    pub geometry: GeometryResult,
    pub properties: PropertiesResult,
}

/// Basically the point of the address
#[derive(Deserialize, Debug)]
pub struct GeometryResult {
    pub r#type: String,
    pub coordinates: Coordinates,
}

/// Latitude and Longitude, WGS 84 format
#[derive(Deserialize, Debug)]
pub struct Coordinates {
    #[serde(rename = "0")]
    pub lat: f64,

    #[serde(rename = "1")]
    pub lon: f64,
}

/// An Address returned by the API
#[derive(Deserialize, Debug)]
pub struct PropertiesResult {
    pub id: String,

    pub score: f64,
    /// Full address label
    pub label: String,

    /// X coord in the Lambert-93 projection
    pub x: f64,
    /// Y coord in the Lambert-93 projection
    pub y: f64,

    pub importance: f64,

    /// Type of the address, e.g. "housenumber", "street", …
    pub r#type: String,

    /// name of the address, I guess housenumber + street
    pub name: String,

    pub housenumber: Option<String>,
    pub street: Option<String>,
    pub postcode: String,
    pub citycode: String,

    pub context: String,
}

/// Filter types for geocoding and reverse geocoding queries
#[derive(Debug, Clone)]
pub enum FilterType {
    HouseNumber,
    Street,
    Locality,
    Municipality,
}
impl fmt::Display for FilterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilterType::HouseNumber => write!(f, "housenumber"),
            FilterType::Street => write!(f, "street"),
            FilterType::Locality => write!(f, "locality"),
            FilterType::Municipality => write!(f, "municipality"),
        }
    }
}
#[derive(Debug, serde::Deserialize)]
pub(crate) struct ApiErrorResponse {
    pub code: u16,
    pub message: String,
    pub detail: Option<Vec<String>>,
}
