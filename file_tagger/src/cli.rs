use std::env;
use crate::tags::{add_tag, remove_tag, list_tags};

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: tag <add|remove|list> <file> [tag]");
        return;
    }
    match args[1].as_str() {
        "add" => {
            if args.len() < 4 {
                eprintln!("Usage: tag add <file> <tag>");
                return;
            }
            add_tag(&args[2], &args[3]).unwrap();
            println!("Added tag '{}' to '{}'", &args[3], &args[2]);
        }
        "remove" => {
            if args.len() < 4 {
                eprintln!("Usage: tag remove <file> <tag>");
                return;
            }
            remove_tag(&args[2], &args[3]).unwrap();
            println!("Removed tag '{}' from '{}'", &args[3], &args[2]);
        }
        "list" => {
            if args.len() < 3 {
                eprintln!("Usage: tag list <file>");
                return;
            }
            let tags = list_tags(&args[2]).unwrap();
            println!("Tags for '{}': {:?}", &args[2], tags);
        }
        _ => eprintln!("Unknown command."),
    }
}