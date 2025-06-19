use std::collections::HashMap;
use std::fs;

pub fn find_duplicates(files: &[String]) {
    let mut size_map: HashMap<u64, Vec<String>> = HashMap::new();
    for file in files {
        if let Ok(meta) = fs::metadata(file) {
            size_map.entry(meta.len()).or_default().push(file.clone());
        }
    }
    println!("Duplicate files by size:");
    for (_size, group) in size_map.iter().filter(|(_, v)| v.len() > 1) {
        println!("{:?}", group);
    }
}