use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use serde_json;

pub const CONFIG_PATH: &'static str = "config.json";

#[derive(Serialize, Deserialize)]
pub struct FileSpec {
    pub path: String,
    pub depth: u32,
}

impl FileSpec {
    pub fn new(path: String, depth: u32) -> Self {
        FileSpec {
            path: path,
            depth: depth
        }
    }

    pub fn serialize(&self) -> Result<String, String> {
        match serde_json::to_string(self) {
            Ok(s) => Ok(s),
            Err(_) => Err("Failed to Serialize FileSpec".to_string())
        }
    }

    pub fn write_to_file(&self) -> Result<(), String>{
        let path = CONFIG_PATH.to_string();
        let mut file:File = match File::create(path) {
            Ok(f) => f,
            Err(_) => return Err("Failed to open config".to_string()),
        };
        match file.write_all(self.serialize()?.as_bytes()){
            Ok(f) => f,
            Err(_) => return Err("Failed to write config to file".to_string()),
        };
        Ok(())
    }
}

pub fn deserialize_to_file_spec(file_string: String) -> Result<FileSpec, String> {
    match serde_json::from_str(&file_string) {
        Ok(fs) => Ok(fs),
        Err(_) => Err("Failed to Deserialize FileSpec".to_string())
    }
}

pub fn load_file_spec() -> Result<FileSpec, String> {
    let path = CONFIG_PATH.to_string();
    let mut file = match File::open(path){
        Ok(f) => f,
        Err(_) => return Err("Failed to load config file".to_string()),
    };
    
    let mut file_string = String::new();
    match file.read_to_string(&mut file_string){
        Ok(r) => r,
        Err(_) => return Err("Failed to read config to string".to_string()),
    };
    Ok(deserialize_to_file_spec(file_string)?)
}
