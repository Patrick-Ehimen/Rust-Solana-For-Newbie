// This file contains the core logic of the weather application.
// It defines functions for making API requests, parsing the response,
// and formatting the weather information for display.

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    name: String,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
    pressure: u32,
    humidity: u32,
}

pub async fn fetch_weather(api_key: &str, city: &str) -> Result<WeatherResponse, Error> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);
    let response = reqwest::get(&url).await?;
    let weather_data = response.json::<WeatherResponse>().await?;
    Ok(weather_data)
}

pub fn format_weather(weather: &WeatherResponse) -> String {
    format!(
        "Weather in {}:\nTemperature: {}°C\nPressure: {} hPa\nHumidity: {}%",
        weather.name, weather.main.temp, weather.main.pressure, weather.main.humidity
    )
}