use std::path::Path;

use chrono::Local;

use crate::{storage, task::{Status, Task}};



pub fn add_todo(path:&Path,task:Task)-> Result<(),Box<dyn std::error::Error>>{
    let content = storage::get_content(path)?;
    let mut json_vec = storage::convert_to_json_vec(&content)?;
    
    json_vec.push(task);

    storage::write_to_file(json_vec, path);

    Ok(())
}

pub fn update_todo(path:&Path,id:&str)-> Result<(),Box<dyn std::error::Error>>{
    let content = storage::get_content(path)?;
    let json_vec = storage::convert_to_json_vec(&content)?;
    let updated_vec:Vec<Task> = json_vec.into_iter().map(|mut task| {
        if task.get_id() == id {
            task.upgrade_status();
        }
        task
    }).collect();

    storage::write_to_file(updated_vec, path);

    Ok(())
}

pub fn delete_todo(path:&Path,id:&str)->Result<(), Box<dyn std::error::Error>>{
    let content = storage::get_content(path)?;
    let json_vec = storage::convert_to_json_vec(&content)?;
    let updated_vec:Vec<Task> = json_vec.into_iter().filter(|task| {
        task.get_id() != id
    }).collect();

    storage::write_to_file(updated_vec, path);

    Ok(())
}


pub fn list_todos(path:&Path)-> Result<(),Box<dyn std::error::Error>>{
    let content = storage::get_content(path)?;
    let tasks: Vec<Task> = storage::convert_to_json_vec(&content)?;

    for task in tasks {
        println!(
            "[{}] {} - {} ({})",
            match task.get_status() {
                Status::Pending => "❌",
                Status::Completed => "✅",
            },
            task.get_id(),
            task.get_content(),
            task.get_date().with_timezone(&Local).format("%Y-%m-%d %H:%M")
        );
    }

    Ok(())
}