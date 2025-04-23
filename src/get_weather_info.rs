use crate::weather_response::WeatherResponse;
use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Params {
    lat: f64,
    lon: f64,
    units: String,
    appid: String,
}

pub enum Error {
    Reqwest(reqwest::Error),
    NoSuccess(u16),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Reqwest(value)
    }
}

pub fn get_weather_info(lat: f64, lon: f64, api_key: &str) -> Result<WeatherResponse, Error> {
    let url = "https://api.openweathermap.org/data/2.5/weather";
    let params = Params {
        lat,
        lon,
        units: "metric".to_string(),
        appid: api_key.to_string(),
    };

    let client = Client::new();
    let response = client.get(url).query(&params).send()?;

    if !response.status().is_success() {
        return Err(Error::NoSuccess(response.status().as_u16()));
    }

    let response_json = response.json::<WeatherResponse>()?;

    Ok(response_json)
}
