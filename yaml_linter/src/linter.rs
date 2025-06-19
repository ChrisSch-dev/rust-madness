use serde_yaml::Value;
use std::fs;

pub fn lint_yaml(filename: &str) -> Result<(), String> {
    let content = fs::read_to_string(filename).map_err(|e| e.to_string())?;
    match serde_yaml::from_str::<Value>(&content) {
        Ok(val) => {
            // Example: check for required key "name"
            if let Value::Mapping(map) = &val {
                if !map.contains_key(&Value::String("name".to_string())) {
                    return Err("Missing required key: name".to_string());
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("YAML parse error: {}", e)),
    }
}