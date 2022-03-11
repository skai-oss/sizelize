

use glob::glob;
use crate::config_struct::FileConfig;

pub fn scan_files(configs: &Vec<FileConfig>, test: fn(&str, &String) -> bool) -> bool {
    let mut res = true;
    for config in configs {
    
        for entry in glob(&config.path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let path_str = path.into_os_string().into_string().unwrap();
                    if !test(&path_str, &config.max_size) {
                        res = false;
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }
    return res;
}