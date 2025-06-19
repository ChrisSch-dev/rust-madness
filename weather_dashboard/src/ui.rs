use crate::api::Weather;

pub fn print_weather(data: &Weather) {
    println!("City: {}", data.name);
    if let Some(desc) = data.weather.get(0) {
        println!("Weather: {} ({})", desc.main, desc.description);
    }
    println!("Temperature: {}°C", data.main.temp);
    println!("Humidity: {}%", data.main.humidity);
}