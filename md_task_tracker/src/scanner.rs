use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn find_todos(dir: &str) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();
    visit_dirs(Path::new(dir), &mut result);
    result
}

fn visit_dirs(dir: &Path, result: &mut HashMap<String, Vec<String>>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, result);
            } else if let Some(ext) = path.extension() {
                if ext == "md" {
                    let path_str = path.display().to_string();
                    let content = fs::read_to_string(&path).unwrap_or_default();
                    let todos: Vec<String> = content
                        .lines()
                        .filter(|l| l.trim_start().starts_with("- [ ]"))
                        .map(|l| l.trim().to_string())
                        .collect();
                    if !todos.is_empty() {
                        result.insert(path_str, todos);
                    }
                }
            }
        }
    }
}