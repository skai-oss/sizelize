use std::fs;
use crate::config_struct::{FileConfig, PackageJson};
use serde_json::Result;


pub fn read_config(path: String) -> Result<Vec<FileConfig>> {
    let data = fs::read_to_string(path).expect("Unable to read file");

    let package_json: PackageJson = serde_json::from_str(&data).expect("Unable to parse json");

    Ok(package_json.sizeable)
}