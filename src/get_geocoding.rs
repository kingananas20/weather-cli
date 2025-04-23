use reqwest::blocking::Client;
use serde::Serialize;

use crate::geocoding_response::GeocodingResponse;

#[derive(Debug, Serialize)]
struct Params {
    q: String,
    limit: u8,
    appid: String,
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    NoSuccess(u16),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Reqwest(value)
    }
}

pub fn get_geocoding(
    country_code: &str,
    city: &str,
    api_key: &str,
) -> Result<GeocodingResponse, Error> {
    let url = "http://api.openweathermap.org/geo/1.0/direct";
    let params = Params {
        q: format!("{},{}", city, country_code),
        limit: 1,
        appid: api_key.to_string(),
    };

    let client = Client::new();
    let response = client.get(url).query(&params).send()?;

    if !response.status().is_success() {
        return Err(Error::NoSuccess(response.status().as_u16()));
    }

    let response_json = response.json::<Vec<GeocodingResponse>>()?;

    Ok(response_json[0])
}
