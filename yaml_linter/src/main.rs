mod linter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: yaml_linter <file.yaml>");
        return;
    }
    match linter::lint_yaml(&args[1]) {
        Ok(()) => println!("No errors found."),
        Err(e) => println!("YAML Lint Error: {}", e),
    }
}