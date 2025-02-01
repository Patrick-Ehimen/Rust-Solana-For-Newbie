fn main() {
    let api_key = "YOUR_API_KEY"; // Replace with your actual API key
    let city = "London"; // Replace with the desired city
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    let response = reqwest::blocking::get(&url)
        .expect("Failed to fetch weather data");

    if response.status().is_success() {
        let weather_data: serde_json::Value = response.json().expect("Failed to parse JSON");
        let temperature = weather_data["main"]["temp"].as_f64().expect("Failed to get temperature");
        let weather_description = weather_data["weather"][0]["description"].as_str().expect("Failed to get weather description");

        println!("The current temperature in {} is {:.1}°C with {}.", city, temperature, weather_description);
    } else {
        println!("Failed to fetch weather data: {}", response.status());
    }
}