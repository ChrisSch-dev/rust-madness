use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub name: String,
    pub weather: Vec<WeatherDesc>,
    pub main: WeatherMain,
}

#[derive(Debug, Deserialize)]
pub struct WeatherDesc {
    pub main: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct WeatherMain {
    pub temp: f64,
    pub humidity: u8,
}

pub async fn fetch_weather(city: &str) -> Result<Weather, reqwest::Error> {
    let api_key = std::env::var("OPENWEATHERMAP_KEY")
        .expect("Set OPENWEATHERMAP_KEY env variable");
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}&units=metric"
    );
    let resp = reqwest::get(&url).await?.json::<Weather>().await?;
    Ok(resp)
}