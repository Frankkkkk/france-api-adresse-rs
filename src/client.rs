use crate::{geocode::SearchQuery, reverse::ReverseQuery, types::Error};

#[derive(Debug, Clone)]
pub struct BAN {
    pub base_url: String,
}

impl Default for BAN {
    fn default() -> Self {
        Self::new()
    }
}

impl BAN {
    /// Creates a new instance to the france api adresse client (Base d'Adresse Nationale).
    pub fn new() -> Self {
        Self {
            base_url: "https://data.geopf.fr/geocodage".to_string(),
        }
    }

    /// Creates a new instance with a custom base URL, if you self host the api instead
    pub fn with_base_url(base_url: String) -> Self {
        Self { base_url }
    }

    #[cfg(feature = "async")]
    pub(crate) async fn send_async_request<T>(&self, url: &str) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let resp = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .map_err(Error::HttpError)?;
        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| Error::GetTextError(e.to_string()))?;

        if !status.is_success() {
            use crate::types::ApiErrorResponse;

            let err = serde_json::from_str::<ApiErrorResponse>(&text)
                .map_err(|e| Error::UnmarshalJsonError(e.to_string()))?;
            return Err(Error::ApiError {
                code: err.code,
                message: err.message,
                detail: err.detail,
            });
        }

        serde_json::from_str(text.as_str()).map_err(|e| Error::UnmarshalJsonError(e.to_string()))
    }

    #[cfg(feature = "blocking")]
    pub(crate) fn send_blocking_request<T>(&self, url: &str) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let resp = reqwest::blocking::Client::new()
            .get(url)
            .send()
            .map_err(Error::HttpError)?;

        let status = resp.status();
        let text = resp
            .text()
            .map_err(|e| Error::GetTextError(e.to_string()))?;

        if !status.is_success() {
            use crate::types::ApiErrorResponse;

            let err = serde_json::from_str::<ApiErrorResponse>(&text)
                .map_err(|e| Error::UnmarshalJsonError(e.to_string()))?;
            return Err(Error::ApiError {
                code: err.code,
                message: err.message,
                detail: err.detail,
            });
        }

        serde_json::from_str(&text).map_err(|e| Error::UnmarshalJsonError(e.to_string()))
    }

    /// Creates a new geocode search query.
    pub fn geocode(&self, query: String) -> SearchQuery {
        SearchQuery::new(self.clone(), query)
    }

    /// Creates a new reverse geocode query
    pub fn reverse(&self, lat: f64, lon: f64) -> ReverseQuery {
        ReverseQuery::new(self.clone(), lat, lon)
    }
}
