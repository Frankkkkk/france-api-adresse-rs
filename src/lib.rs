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
//! ```rust
//! use france_api_adresse;
//! let api = france_api_adresse::client::BAN::default();
//! let search = api.geocode("Route du marais".to_string());
//!
//! // Narrow the results to post code 74380
//! let search = search.postcode("74380");
//!
//! // Narrow the results to city "Cranves-Sales"
//! let search = search.city("Cranves-Sales".to_string());
//!
//! // Get the results
//! let result = search.execute_blocking().unwrap();
//! for result in result.features {
//!     println!("Address: {}", result.properties.label);
//!     println!("Coords: {:?}", result.geometry.coordinates);
//! }
//! ```
//!
//! You can use async mode by enabling the `async` feature in your `Cargo.toml`. Example:
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     let api = france_api_adresse::client::BAN::default();
//!     let search = api.geocode("Route du marais".to_string());
//!     let results = search.execute_async().await.unwrap();
//! }
//! ```

//!
//! # Errors
//! Errors are returned as a custom `Error` enum to distinguish between HTTP,
//! text extraction, and JSON deserialization failures.
//!
//!

pub mod client;
pub mod geocode;
pub mod reverse;
pub mod types;
