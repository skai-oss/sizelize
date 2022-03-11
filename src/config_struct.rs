use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileConfig {
    pub path: String,
    pub max_size: String,
}

#[derive(Serialize, Deserialize)]
pub struct PackageJson {
    pub sizelize: Vec<FileConfig>,
}
