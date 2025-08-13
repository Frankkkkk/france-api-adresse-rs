#[cfg(feature = "blocking")]
use crate::types::{AddressResult, Error};
use crate::{client::BAN, types::FilterType};

/// Represents a reverse geocode query.
#[derive(Debug, Clone)]
pub struct ReverseQuery {
    client: BAN,

    lat: f64,
    lon: f64,

    postcode: Option<String>,
    city: Option<String>,
    ty: Option<FilterType>,

    limit: Option<usize>,
}

impl ReverseQuery {
    pub(crate) fn new(client: BAN, lat: f64, lon: f64) -> Self {
        Self {
            client,
            lat,
            lon,

            postcode: None,
            city: None,

            ty: None,

            limit: None,
        }
    }

    /// Filter results by postal code
    pub fn postcode(mut self, code: impl Into<String>) -> Self {
        self.postcode = Some(code.into());
        self
    }
    /// Filter results by city
    pub fn city(mut self, name: impl Into<String>) -> Self {
        self.city = Some(name.into());
        self
    }

    /// Filter results by type, e.g. "housenumber", "street", etc.
    pub fn filter_type(mut self, ty: FilterType) -> Self {
        self.ty = Some(ty);
        self
    }

    /// Limit number of results returned
    pub fn limit_results(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    fn to_url(&self) -> String {
        let mut url = format!(
            "{}/reverse/?lat={}&lon={}",
            self.client.base_url, self.lat, self.lon
        );

        if let Some(postcode) = &self.postcode {
            url.push_str(&format!("&postcode={}", urlencoding::encode(postcode)));
        }
        if let Some(city) = &self.city {
            url.push_str(&format!("&city={}", urlencoding::encode(city)));
        }

        if let Some(ty) = &self.ty {
            url.push_str(&format!("&type={}", ty));
        }

        if let Some(limit) = self.limit {
            url.push_str(&format!("&limit={}", limit));
        }
        url
    }

    /// Executes the reverse geocode query asynchronously.
    #[cfg(feature = "async")]
    pub async fn execute_async(&self) -> Result<AddressResult, Error> {
        self.client.send_async_request(&self.to_url()).await
    }

    /// Executes the reverse geocode query synchronously.
    #[cfg(feature = "blocking")]
    pub fn execute_blocking(&self) -> Result<AddressResult, Error> {
        self.client.send_blocking_request(&self.to_url())
    }
}
