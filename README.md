# `france-api-adresse` : A rust wrapper around France's Base Adresse Nationale

[![Crates.io](https://img.shields.io/crates/v/france-api-adresse.svg)](https://crates.io/crates/france-api-adresse)
[![Docs.rs](https://docs.rs/france-api-adresse/badge.svg)](https://docs.rs/france-api-adresse)
[![License](https://img.shields.io/crates/l/france-api-adresse.svg)](#license)

A Rust client for the [Base Adresse Nationale (BAN) API](https://adresse.data.gouv.fr/) and compatible self-hosted services.
Supports both **async** and **blocking** requests, configurable base URLs, and multiple geocoding endpoints.

## Features

- Search addresses with advanced filtering
- Reverse geocoding from coordinates
- Configurable API base URL (use official or self-hosted instances)
- Async and blocking modes
- Builder-style query API
- Strongly typed filters

## Installation

`cargo add france-api-adresse`

Optionally, enable only the mode you need:

```
[dependencies]
france-api-adresse = { version = "0.1", default-features = false, features = ["async"] }
# or
france-api-adresse = { version = "0.1", default-features = false, features = ["blocking"] }
```

## Quick Start

### Async Example

```rust
use france_api_adresse::{BAN, GeocodeFilterType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BAN::default(); // Defaults to https://data.geopf.fr/geocodage

    let result = client
        .search("200 Chemin de puy petit")
        .postcode("26270")
        .filter(GeocodeFilterType::Street)
        .execute_async()
        .await?;

    println!("{:#?}", result);

    Ok(())
}
```

### Blocking Example

```rust
use france_api_adresse::{BAN, GeocodeFilterType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BAN::new();

    let result = client
        .reverse(46.3123975, 6.301054)
        .execute_blocking()?;

    println!("{:#?}", result);

    Ok(())
}
```

## Filtering

The `GeocodeFilterType` enum ensures only valid filter values are used:

```rust
use france_api_adresse::GeocodeFilterType;

let q = client
    .search("Route du lieu")
    .filter(GeocodeFilterType::HouseNumber);
```

This will produce:
`...&filter=housenumber`

## Self-hosted API

The BAN client works with any API compatible with the official GeoAPI BAN:

```rust
let client = BAN::with_base_url("https://selfhosted.example.com/geocodage".into());
```

### Contributing

Please send your patches either to my email: `frank@villaro-dixon.eu` or to the [github mirror](https://github.com/Frankkkkk/france-api-adresse-rs).


## Links

- Forge: https://forge.k3s.fr/frank/france-api-adresse-rs
- GitHub mirror: https://github.com/frankkkkk/france-api-adresse-rs
- BAN documentation: https://geoservices.ign.fr/documentation/services/services-geoplateforme/geocodage


## License

Copyright (c) 2025 Frank Villaro-Dixon

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
