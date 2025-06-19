mod server;

fn main() {
    let port = 7878;
    let dir = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    println!("Serving '{}' on http://localhost:{}", dir, port);
    server::run(dir, port);
}