use serde::Deserialize;

/// Struct for the dezirialized openWeatherMap response
#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub wind: Wind,
    pub name: String,
    pub coord: Coord,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub description: String,
}

/// Struct to represent main weather information
#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: f64,
    pub humidity: f64,
}

/// Struct to represent wind data
#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
}

#[derive(Debug, Deserialize)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}
