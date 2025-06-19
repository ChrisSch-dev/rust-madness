mod organizer;
mod dedupe;

fn main() {
    let dir = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let files = organizer::collect_images(&dir);
    let groups = organizer::group_by_date(&files);
    dedupe::find_duplicates(&files);

    println!("Images grouped by creation date:");
    for (date, imgs) in groups {
        println!("Date: {}", date);
        for img in imgs {
            println!("  {}", img);
        }
    }
}