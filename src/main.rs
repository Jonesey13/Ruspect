#[macro_use]
extern crate serde_derive;

extern crate serde_json;

mod spection_config;

use std::fs;

fn main() {
    let config = match spection_config::load_file_spec() {
        Ok(c) => c,
        Err(_) => spection_config::FileSpec::new("C:/".to_string(), 1),
    };
    
    let paths: fs::ReadDir = fs::read_dir(&config.path).unwrap();

    for path in paths {
        let pathname = path.unwrap().path();
        println!("Name: {:?}, Size: {:?}KB", &pathname, fs::metadata(&pathname).unwrap().len() / 1024);
    }

    config.write_to_file().unwrap_or_else(|e| { panic!(e) });
}
