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
//! use france_api_adresse::blocking::{get_address_info, get_reverse_info};
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

pub mod blocking;
pub mod types;

const API_URL_SEARCH: &str = "https://data.geopf.fr/geocodage/search/?q=";
const API_URL_REVERSE: &str = "https://data.geopf.fr/geocodage/reverse/?";
