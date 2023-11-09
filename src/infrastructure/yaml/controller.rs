use std::{io::Read, process::exit};

use configs::config::Config;

use crate::configs;

pub fn load_config(file_path: &str) -> Config {
    let mut f = std::fs::File::open(file_path).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    return match serde_yaml::from_str(&data).ok() {
        Some(fc) => fc,
        _ => {
            println!("Invalid YAML or cannot be converted to Config.");
            exit(0);
        }
    };
}