use std::fs;
use std::collections::HashMap;
use chrono::{DateTime, Local};
use std::path::Path;

pub fn collect_images(dir: &str) -> Vec<String> {
    let mut images = vec![];
    visit_dirs(Path::new(dir), &mut images);
    images
}

fn visit_dirs(dir: &Path, images: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, images);
            } else if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_ascii_lowercase();
                if ["jpg", "jpeg", "png", "bmp", "gif"].contains(&ext.as_str()) {
                    images.push(path.display().to_string());
                }
            }
        }
    }
}

pub fn group_by_date(files: &[String]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for f in files {
        let meta = fs::metadata(f);
        if let Ok(meta) = meta {
            if let Ok(time) = meta.created() {
                let dt: DateTime<Local> = time.into();
                let date = dt.format("%Y-%m-%d").to_string();
                map.entry(date).or_insert_with(Vec::new).push(f.clone());
            }
        }
    }
    map
}