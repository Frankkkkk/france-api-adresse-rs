use crate::types::AddressResult;
use crate::types::Error;

fn get_data(url: &str) -> Result<AddressResult, Error> {
    let response = match reqwest::blocking::get(url) {
        Ok(value) => value,
        Err(e) => return Err(Error::HttpError(e)),
    };

    let value = match response.text() {
        Ok(value) => value,
        Err(e) => return Err(Error::GetTextError(e.to_string())),
    };

    let data: AddressResult = match serde_json::from_str(&value) {
        Ok(value) => value,
        Err(e) => return Err(Error::UnmarshalJsonError(e.to_string())),
    };

    Ok(data)
}

/// Returns the addresses that match the search query
pub fn get_address_info(search: &str) -> Result<AddressResult, Error> {
    let url = format!("{}{}", crate::API_URL_SEARCH, search);

    get_data(&url)
}

/// Reverse geocoding: returns the address at the given coordinates
pub fn get_reverse_info(lon: f64, lat: f64) -> Result<AddressResult, Error> {
    let url = format!("{}lon={}&lat={}", crate::API_URL_REVERSE, lon, lat);

    get_data(&url)
}

#[cfg(test)]
mod tests {
    use crate::blocking::{get_address_info, get_reverse_info};

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
