mod api;
mod ui;

#[tokio::main]
async fn main() {
    let city = std::env::args().nth(1).unwrap_or_else(|| "London".to_string());
    println!("Fetching weather for: {}", city);

    match api::fetch_weather(&city).await {
        Ok(weather) => ui::print_weather(&weather),
        Err(e) => eprintln!("Error: {}", e),
    }
}