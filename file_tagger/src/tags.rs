use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::io::{Result, Error, ErrorKind};

const TAGS_FILE: &str = ".tags.json";

#[derive(Serialize, Deserialize, Default)]
pub struct TagStore {
    pub files: HashMap<String, Vec<String>>,
}

fn load_store(dir: &str) -> Result<TagStore> {
    let path = Path::new(dir).join(TAGS_FILE);
    if path.exists() {
        let data = fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&data)?)
    } else {
        Ok(TagStore::default())
    }
}

fn save_store(dir: &str, store: &TagStore) -> Result<()> {
    let path = Path::new(dir).join(TAGS_FILE);
    let data = serde_json::to_string_pretty(store)?;
    fs::write(path, data)
}

fn get_parent_dir(file: &str) -> Result<String> {
    let p = Path::new(file).parent()
        .ok_or_else(|| Error::new(ErrorKind::Other, "No parent directory"))?;
    Ok(p.to_string_lossy().to_string())
}

pub fn add_tag(file: &str, tag: &str) -> Result<()> {
    let dir = get_parent_dir(file)?;
    let mut store = load_store(&dir)?;
    let entry = store.files.entry(file.to_string()).or_default();
    if !entry.contains(&tag.to_string()) {
        entry.push(tag.to_string());
    }
    save_store(&dir, &store)
}

pub fn remove_tag(file: &str, tag: &str) -> Result<()> {
    let dir = get_parent_dir(file)?;
    let mut store = load_store(&dir)?;
    if let Some(tags) = store.files.get_mut(file) {
        tags.retain(|t| t != tag);
    }
    save_store(&dir, &store)
}

pub fn list_tags(file: &str) -> Result<Vec<String>> {
    let dir = get_parent_dir(file)?;
    let store = load_store(&dir)?;
    Ok(store.files.get(file).cloned().unwrap_or_default())
}