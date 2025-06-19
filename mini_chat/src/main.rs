mod server;
mod client;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: mini_chat <server|client>");
        return;
    }
    match args[1].as_str() {
        "server" => server::run(),
        "client" => client::run(),
        _ => println!("Unknown mode"),
    }
}