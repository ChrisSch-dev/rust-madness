use std::fs::{self, OpenOptions};
use std::io::{Write, BufRead, BufReader};
use std::path::Path;

pub fn add_note(title: &str, content: &str) -> std::io::Result<()> {
    let filename = format!("notes/{}.md", title.replace(" ", "_"));
    fs::create_dir_all("notes")?;
    let mut file = OpenOptions::new().create(true).append(true).open(filename)?;
    writeln!(file, "# {}\n{}", title, content)?;
    println!("Note added.");
    Ok(())
}

pub fn list_notes() -> std::io::Result<()> {
    if !Path::new("notes").exists() {
        println!("No notes found.");
        return Ok(());
    }
    for entry in fs::read_dir("notes")? {
        let entry = entry?;
        println!("{}", entry.file_name().to_string_lossy());
    }
    Ok(())
}

pub fn search_notes(keyword: &str) -> std::io::Result<()> {
    if !Path::new("notes").exists() {
        println!("No notes found.");
        return Ok(());
    }
    for entry in fs::read_dir("notes")? {
        let entry = entry?;
        let file = fs::File::open(entry.path())?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let l = line?;
            if l.contains(keyword) {
                println!("{}: {}", entry.file_name().to_string_lossy(), l);
            }
        }
    }
    Ok(())
}