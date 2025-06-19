use std::fs;
use std::path::Path;

pub fn print_tree(path: &str, prefix: String, is_last: bool) {
    let path = Path::new(path);
    let name = path.file_name().map(|s| s.to_string_lossy()).unwrap_or_else(|| "".into());
    if prefix.is_empty() {
        println!("{}", path.display());
    } else {
        println!("{}{}{}", prefix, if is_last { "└── " } else { "├── " }, name);
    }
    let mut entries: Vec<_> = match fs::read_dir(path) {
        Ok(e) => e.flatten().collect(),
        Err(_) => return,
    };
    entries.sort_by_key(|e| e.file_name());
    let len = entries.len();
    for (i, entry) in entries.into_iter().enumerate() {
        let is_last = i == len - 1;
        let new_prefix = if prefix.is_empty() {
            String::new()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };
        print_tree(&entry.path().to_string_lossy(), new_prefix, is_last);
    }
}