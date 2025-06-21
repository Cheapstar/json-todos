use core::panic;
use std::{env, fs, io::Write, path::Path};
use chrono::{DateTime, Local, Utc};
use serde_json::{Value::Array};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

struct Input {
    action:String,
    todo:String,
}

impl Input {
    pub fn build(mut args:impl Iterator<Item = String>)->Result<Input, &'static str>{
        args.next();

        let action = match args.next() {
            Some(a) => a,
            None => return Err("Action is not provided")
        };

        let todo = match args.next() {
            Some(a) => a,
            None => {
                if(action == "list"){
                     String::from("")
                }
                else{
                    return Err("Todo Not provided");
                }
            }
        };

        Ok(Input {
            action,
            todo
        })
    }
}

#[derive(Serialize,Deserialize)]
struct Task {
    id:String,
    contents:String,
    created_at:DateTime<Utc>,
    status:Status
}

#[derive(Serialize,Deserialize)]
pub enum Status {
    Pending,
    Completed
}

fn execute(input:&Input,path:&Path){
    let Input {action,todo} = input;

    match action.as_ref() {
        "add" => {
            let id = Uuid::new_v4().to_string();
            add_the_todo(path, Task { id:id , contents: String::from(todo), created_at: Utc::now(), status: Status::Pending });
        },
        "delete" => {
            delete_the_todo(String::from(todo), path);
        },
        "update" => {
            update_status(String::from(todo), path);
        },
        "list" => {
            list_tasks(path);
        },
        _ => panic!("Invalid action detected")
    }

}


fn add_the_todo(path:&Path,task:Task){
    let content = fs::read_to_string(path).unwrap();
    let mut json_vec:Vec<Task>= serde_json::from_str(&content).unwrap();
    json_vec.push(task);

    fs::write(path, serde_json::to_vec(&json_vec).unwrap());
}

fn delete_the_todo(id:String,path:&Path){
    let content = fs::read_to_string(path).unwrap();

    let mut json_vec:Vec<Task>= serde_json::from_str(&content).unwrap();

    let updated_vec:Vec<Task> = json_vec.into_iter().filter(|task| {
        task.id != id
    }).collect();

    fs::write(path, serde_json::to_vec(&updated_vec).unwrap());
}

fn update_status(id:String,path:&Path){
    let content:String = fs::read_to_string(path).unwrap();
    let mut json_vec:Vec<Task>= serde_json::from_str(&content).unwrap();

    let updated_vec:Vec<Task> = json_vec.into_iter().map(|mut task| {
        if(task.id == id){
            task.status = Status::Completed;
        }
        task
    }).collect();

    fs::write(path, serde_json::to_vec(&updated_vec).unwrap());
}

fn list_tasks(path:&Path){
    let content:String = fs::read_to_string(path).unwrap();
     let tasks: Vec<Task> = serde_json::from_str(&content).unwrap();

    for task in tasks {
        println!(
            "[{}] {} - {} ({})",
            match task.status {
                Status::Pending => "❌",
                Status::Completed => "✅",
            },
            task.id,
            task.contents,
            task.created_at.with_timezone(&Local).format("%Y-%m-%d %H:%M")
        );
    }
}