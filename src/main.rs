mod display_weather_info;
mod geocoding_response;
mod get_geocoding;
mod get_weather_info;
mod weather_response;

use clap::Parser;
use display_weather_info::display_weather_info;
use get_geocoding::get_geocoding;
use get_weather_info::get_weather_info;

/// simple program that gets current weatherdata of a city
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// country code of the country the city is in
    country_code: String,

    /// city name
    city: String,
}

fn main() {
    let cli = Cli::parse();
    let api_key = std::env::var("WEATHER_API_KEY")
        .expect("expected WEATHER_API_KEY in environment variables");

    let geoloaction = match get_geocoding(&cli.country_code, &cli.city, &api_key) {
        Ok(res) => res,
        Err(get_geocoding::Error::NoSuccess(status)) => {
            return eprintln!("request status is not success: {}", status);
        }
        Err(get_geocoding::Error::Reqwest(e)) => return eprintln!("reqwest error\n{:?}", e),
    };
    let weather = match get_weather_info(geoloaction.lat, geoloaction.lon, &api_key) {
        Ok(res) => res,
        Err(get_weather_info::Error::NoSuccess(status)) => {
            return eprintln!("request status is not success: {}", status);
        }
        Err(get_weather_info::Error::Reqwest(e)) => return eprintln!("reqwest error\n{:?}", e),
    };

    display_weather_info(weather);
}
