use crate::types::*;
use reqwest::Client;

pub async fn get_address_info(query: &str) -> Result<AddressResult, Error> {
    let url = format!("https://data.geopf.fr/geocodage/search/?q={}", query);
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| Error::HttpError(e))?;
    let text = response
        .text()
        .await
        .map_err(|e| Error::GetTextError(e.to_string()))?;
    let parsed =
        serde_json::from_str(&text).map_err(|e| Error::UnmarshalJsonError(e.to_string()))?;
    Ok(parsed)
}

pub async fn get_reverse_info(lon: f64, lat: f64) -> Result<AddressResult, Error> {
    let url = format!(
        "https://data.geopf.fr/geocodage/reverse/?lon={}&lat={}",
        lon, lat
    );
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| Error::HttpError(e))?;
    let text = response
        .text()
        .await
        .map_err(|e| Error::GetTextError(e.to_string()))?;
    let parsed =
        serde_json::from_str(&text).map_err(|e| Error::UnmarshalJsonError(e.to_string()))?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use crate::async_api::{get_address_info, get_reverse_info};

    #[tokio::test]
    async fn test_get_address_info() {
        let result = get_address_info("200 Chemin de puy petit").await.unwrap();

        assert_eq!(result.features[0].properties.postcode, "26270");
        assert_eq!(result.features[0].properties.citycode, "26166");
    }

    #[tokio::test]
    async fn test_get_reverse_info() {
        let result = get_reverse_info(6.301054, 46.3123975).await.unwrap();

        assert_eq!(
            result.features[0].properties.label,
            "38 Avenue du Bas-Chablais 74140 Douvaine"
        );
    }
}
