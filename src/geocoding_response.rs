use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct GeocodingResponse {
    pub lat: f64,
    pub lon: f64,
}
