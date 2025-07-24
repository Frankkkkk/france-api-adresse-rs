use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: unable to access the API")]
    HttpError(#[from] reqwest::Error),
    #[error("Error extracting response: {0}")]
    GetTextError(String),
    #[error("Error unmarshalling JSON response: {0}")]
    UnmarshalJsonError(String),
}

/// A list of features (addresses) returned by the API
#[derive(Deserialize, Debug)]
pub struct AddressResult {
    pub r#type: String,
    pub features: Vec<Feature>,
}

/// A feature is basically a single address + its coordinates
#[derive(Deserialize, Debug)]
pub struct Feature {
    pub r#type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

/// Basically the point of the address
#[derive(Deserialize, Debug)]
pub struct Geometry {
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
pub struct Properties {
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
