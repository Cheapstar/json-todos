mod storage;
mod task;
mod input;
mod todo;
use core::panic;
use std::{env, fs, io::Write, path::Path};
use chrono::{DateTime, Local, Utc};
use serde_json::{Value::Array};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use task::{Task,Status};

use crate::{input::Input, storage::{convert_to_json_vec, get_content}, todo::{add_todo, delete_todo, update_todo,list_todos}};

fn main() {
    let path = setup_env();
    let args= env::args();

    let input = Input::build(args).unwrap();

    // Check if the file exists or not 
    // if not then create it 
    execute(&input, path);
}


fn setup_env() -> &'static Path{
    let path = Path::new("todos.json");

    if path.exists() {
        println!("Path exists continuing with the function");
    }
    else {
        match fs::write(path,"[]") {
            Ok(_) => println!("File has been successfully created"),
            Err(_) => panic!("Could not create the todos.json")
        }
    }

    path
}

fn execute(input:&Input,path:&Path){
    match input.get_action() {
        "add" => {
            let id = Uuid::new_v4().to_string();
            add_todo(path, Task::build(String::from(input.get_todo()),Utc::now(),Status::Pending));
        },
        "delete" => {
            delete_todo(path, input.get_todo());
        },
        "update" => {
            update_todo(path,input.get_todo() );
        },
        "list" => {
            list_todos(path);
        },
        _ => panic!("Invalid action detected")
    }

}


