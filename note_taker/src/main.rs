mod notes;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: note_taker <add|list|search> [args]");
        return;
    }
    match args[1].as_str() {
        "add" => {
            if args.len() < 4 {
                println!("Usage: note_taker add <title> <content>");
                return;
            }
            notes::add_note(&args[2], &args[3]).unwrap();
        }
        "list" => notes::list_notes().unwrap(),
        "search" => {
            if args.len() < 3 {
                println!("Usage: note_taker search <keyword>");
                return;
            }
            notes::search_notes(&args[2]).unwrap();
        }
        _ => println!("Unknown command."),
    }
}