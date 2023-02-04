use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Debug, Deserialize, Serialize)]
struct WeatherData {
    name: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Main {
    temp: f32,
    humidity: i32,
    feels_like: f32,
    pressure: i32,
    temp_min: f32,
    temp_max: f32,
}

async fn get_weather(city: &str) -> Result<WeatherData, reqwest::Error> {
    let client = Client::new();
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid=eb7c178f50772e2ac375adff2a06fdea", city);

    let resp = client
        .get(&url)
        .send()
        .await?
        .json::<WeatherData>()
        .await?;

    Ok(resp)
}

#[tokio::main]
async fn main() {
    println!("Enter a city name:");
    io::stdout().flush().unwrap();

    let mut city = String::new();
    io::stdin().read_line(&mut city).unwrap();
    city = city.trim().to_string();

    match get_weather(&city).await {
        Ok(weather) => {
            println!("We got the weather data for {}:", weather.name);
            println!("Temperature: {}°F", weather.main.temp * 9. / 5. - 459.67);
            println!("Feels like: {}°F", weather.main.feels_like * 9. / 5. - 459.67);
            println!("Conditions: {} ({})", weather.weather[0].description, weather.weather[0].icon);
        },
        Err(e) => println!("Error: {}", e),
    }
}
