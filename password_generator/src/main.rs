mod gen;

fn main() {
    let len = std::env::args().nth(1).and_then(|n| n.parse().ok()).unwrap_or(16);
    let pw = gen::generate(len, true, true, true, true);
    println!("{}", pw);
}