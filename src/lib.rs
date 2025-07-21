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
            Error::HttpError => write!(f, "Can't access to http://api-adresse.data.gouv.fr"),
            Error::GetTextError => write!(f, "Can't unmarshal data response to text"),
            Error::UnmarshalJsonError => write!(f, "Can't unmarshal text response to json"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressResult {
    pub r#type: String,
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    pub r#type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Coordinates,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinates {
    #[serde(rename = "0")]
    pub lat: f64,

    #[serde(rename = "1")]
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    pub r#type: String,
    pub id: String,

    pub score: f64,
    pub label: String,

    pub x: f64,
    pub y: f64,

    pub importance: f64,

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

pub fn get_address_info(search: &str) -> Result<AddressResult, Error> {
    let url = format!("{}{}", API_URL_SEARCH, search);

    get_data(&url)
}

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
