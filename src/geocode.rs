//use crate::client::{send_async_request, send_blocking_request};
use crate::{
    client::BAN,
    types::{AddressResult, Error, FilterType},
};

/// Represents a geocode search query.
#[derive(Debug, Clone)]
pub struct SearchQuery {
    client: BAN,
    query: String,

    /// Filter results by postal code
    postcode: Option<String>,
    /// Filter results by city
    city: Option<String>,

    /// Sorts results by distance to location
    lat: Option<f64>,
    lon: Option<f64>,

    ty: Option<FilterType>,

    /// Limit number of results
    limit: Option<usize>,
}

impl SearchQuery {
    pub(crate) fn new(client: BAN, query: String) -> Self {
        Self {
            client,
            query,

            postcode: None,
            city: None,

            lat: None,
            lon: None,

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

    /// Sorts results by distance to location
    pub fn around_lat_lon(mut self, lat: f64, lon: f64) -> Self {
        self.lat = Some(lat);
        self.lon = Some(lon);
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
            "{}/search/?q={}",
            self.client.base_url,
            urlencoding::encode(&self.query)
        );

        if let Some(ref pc) = self.postcode {
            url.push_str(&format!("&postcode={}", urlencoding::encode(pc)));
        }
        if let Some(ref city) = self.city {
            url.push_str(&format!("&city={}", urlencoding::encode(city)));
        }

        if let (Some(lat), Some(lon)) = (self.lat, self.lon) {
            url.push_str(&format!("&lat={}&lon={}", lat, lon));
        }

        if let Some(ty) = &self.ty {
            url.push_str(&format!("&type={}", ty));
        }

        if let Some(limit) = self.limit {
            url.push_str(&format!("&limit={}", limit));
        }
        url
    }

    /// Executes the search query asynchronously.
    #[cfg(feature = "async")]
    pub async fn execute_async(&self) -> Result<AddressResult, Error> {
        self.client.send_async_request(&self.to_url()).await
    }

    /// Executes the search query synchronously.
    #[cfg(feature = "blocking")]
    pub fn execute_blocking(&self) -> Result<AddressResult, Error> {
        self.client.send_blocking_request(&self.to_url())
    }
}
