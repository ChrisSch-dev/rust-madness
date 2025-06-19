use std::collections::HashMap;

pub fn print_todos(todos: &HashMap<String, Vec<String>>) {
    for (file, items) in todos {
        println!("File: {}", file);
        for todo in items {
            println!("  {}", todo);
        }
        println!();
    }
}