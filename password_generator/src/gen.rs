use rand::Rng;

const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?";

pub fn generate(len: usize, use_lower: bool, use_upper: bool, use_digits: bool, use_symbols: bool) -> String {
    let mut charset = String::new();
    if use_lower { charset.push_str(LOWER); }
    if use_upper { charset.push_str(UPPER); }
    if use_digits { charset.push_str(DIGITS); }
    if use_symbols { charset.push_str(SYMBOLS); }
    let charset: Vec<char> = charset.chars().collect();
    let mut rng = rand::thread_rng();
    (0..len).map(|_| charset[rng.gen_range(0..charset.len())]).collect()
}