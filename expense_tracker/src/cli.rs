use std::env;
use crate::tracker::{add_expense, list_expenses};

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: expense <add|list>");
        return;
    }
    match args[1].as_str() {
        "add" => {
            if args.len() != 5 {
                eprintln!("Usage: expense add <amount> <category> <desc>");
                return;
            }
            let amount: f64 = args[2].parse().expect("Invalid amount");
            let category = &args[3];
            let desc = &args[4];
            add_expense(amount, category, desc).unwrap();
            println!("Expense added.");
        }
        "list" => {
            let expenses = list_expenses().unwrap();
            for e in expenses {
                println!("{} | {} | {} | {}", e.date, e.amount, e.category, e.desc);
            }
        }
        _ => eprintln!("Unknown command."),
    }
}