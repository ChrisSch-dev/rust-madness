mod tree;

fn main() {
    let dir = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    tree::print_tree(&dir, "".to_string(), true);
}