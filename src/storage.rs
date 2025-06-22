use std::{env, fs, io::Write, path::Path};

use crate::task::Task;


pub fn get_content(path:&Path) -> Result<String, Box<dyn std::error::Error>>{
    let content = fs::read_to_string(path)?;
    Ok(content)
}

pub fn convert_to_json_vec(content: &str) -> Result<Vec<Task>, serde_json::Error> {
    serde_json::from_str(content)
}



pub fn write_to_file(jvec:Vec<Task>,path:&Path){
    fs::write(path, serde_json::to_vec(&jvec).unwrap());
}



