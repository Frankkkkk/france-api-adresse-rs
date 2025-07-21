//! A minimal wrapper for the Base Adresse Nationale "BAN" french geo API.
//!
//! This crate provides functions to query geographical address information
//! from the [Base Adresse Nationale](https://adresse.data.gouv.fr/) API.
//!
//! # Features
//! - Forward geocoding: convert address strings into coordinates
//! - Reverse geocoding: convert coordinates into address details
//!
//! # Example
//! ```
//! use france_api_adresse::{get_address_info, get_reverse_info};
//!
//! let address_result = get_address_info("38 Rue des Blancs Manteaux").unwrap();
//! let reverse_result = get_reverse_info(2.3522, 48.8566).unwrap();
//! ```
//!
//! # Errors
//! Errors are returned as a custom `Error` enum to distinguish between HTTP,
//! text extraction, and JSON deserialization failures.
//!
//!
use reqwest::blocking;

use std::error::Error as StdError;

use serde::{Deserialize, Serialize};
use std::fmt;

const API_URL_SEARCH: &str = "https://data.geopf.fr/geocodage/search/?q=";
const API_URL_REVERSE: &str = "https://data.geopf.fr/geocodage/reverse/?";

#[derive(Debug)]

pub enum Error {
    HttpError,
    GetTextError,
    UnmarshalJsonError,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HttpError => write!(f, "Can't access https://data.geopf.fr"),
            Error::GetTextError => write!(f, "Can't unmarshal data response to text"),
            Error::UnmarshalJsonError => write!(f, "Can't unmarshal text response to json"),
        }
    }
}

/// A list of features (addresses) returned by the API
#[derive(Serialize, Deserialize, Debug)]
pub struct AddressResult {
    pub r#type: String,
    pub features: Vec<Feature>,
}

/// A feature is basically a single address + its coordinates
#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    pub r#type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

/// Basically the point of the address
#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Coordinates,
}

/// Latitude and Longitude, WGS 84 format
#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinates {
    #[serde(rename = "0")]
    pub lat: f64,

    #[serde(rename = "1")]
    pub lon: f64,
}

/// An Address returned by the API
#[derive(Serialize, Deserialize, Debug)]
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

fn get_data(url: &str) -> Result<AddressResult, Error> {
    let response = match blocking::get(url) {
        Ok(value) => value,

        _ => return Err(Error::HttpError),
    };

    let value = match response.text() {
        Ok(value) => value,

        _ => return Err(Error::GetTextError),
    };

    let data: AddressResult = match serde_json::from_str(&value) {
        Ok(value) => value,

        Err(e) => {
            println!("Error unmarshalling Json: {}", e);
            return Err(Error::UnmarshalJsonError);
        }
    };

    Ok(data)
}

/// Returns the addresses that match the search query
pub fn get_address_info(search: &str) -> Result<AddressResult, Error> {
    let url = format!("{}{}", API_URL_SEARCH, search);

    get_data(&url)
}

/// Reverse geocoding: returns the address at the given coordinates
pub fn get_reverse_info(lon: f64, lat: f64) -> Result<AddressResult, Error> {
    let url = format!("{}lon={}&lat={}", API_URL_REVERSE, lon, lat);

    get_data(&url)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_get_address_info() {
        let result = get_address_info("200 Chemin de puy petit").unwrap();

        assert_eq!(result.features[0].properties.postcode, "26270");
        assert_eq!(result.features[0].properties.citycode, "26166");
    }

    #[test]
    fn test_get_reverse_info() {
        let result = get_reverse_info(6.301054, 46.3123975).unwrap();

        assert_eq!(
            result.features[0].properties.label,
            "38 Avenue du Bas-Chablais 74140 Douvaine"
        );
    }
}
