mod scanner;
mod cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = args.get(1).map(String::as_str).unwrap_or(".");
    let todos = scanner::find_todos(dir);
    cli::print_todos(&todos);
}