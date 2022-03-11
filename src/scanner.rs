use crate::compressor::compress;
use crate::config_struct::FileConfig;
use byte_unit::Byte;
use glob::glob;

pub struct Result {
    pub path: String,
    pub compressed_size: Byte,
    pub max_size: Byte,
}

pub fn scan_files(configs: &Vec<FileConfig>) -> Vec<Result> {
    let mut results: Vec<Result> = Vec::new();
    for config in configs {
        for entry in glob(&config.path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let path_str = path.into_os_string().into_string().unwrap();
                    let compressed_size = compress(&path_str);
                    let max_size = Byte::from_str(&config.max_size).unwrap();
                    results.push(Result {
                        path: path_str,
                        compressed_size: compressed_size,
                        max_size: max_size,
                    });
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
    return results;
}
