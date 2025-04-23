use crate::weather_response::WeatherResponse;
use colored::Colorize;

pub fn display_weather_info(response: WeatherResponse) {
    // Extracting weather information from the response
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let feels_like = response.main.feels_like;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;
    let coord = response.coord;

    // Formatting weather information into a string
    let weather_text = format!(
        "Weather in {} ({:.4}, {:.4}): {} {}
> Temperature: {:.1} °C  ({:.1} °C)
> Humidity: {:.1}%
> Pressure: {:.1} hPa 
> Wind Speed: {:.1} m/s",
        response.name,
        coord.lat,
        coord.lon,
        description,
        get_temperature_emoji(temperature),
        temperature,
        feels_like,
        humidity,
        pressure,
        wind_speed,
    );

    // Coloring the weather text based on weather conditions
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" | "heavy intensity rain" => {
            weather_text.bright_cyan()
        }
        _ => weather_text.normal(),
    };

    // Printing the colored weather information
    println!("{}", weather_text_colored);
}

// Function to get emoji based on temperature
fn get_temperature_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}
