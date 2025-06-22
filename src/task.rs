use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local, Utc};
use uuid::Uuid;


#[derive(Serialize,Deserialize)]
pub struct Task {
    id:String,
    contents:String,
    created_at:DateTime<Utc>,
    status:Status
}


impl Task {
    pub fn build(content:String,created_at:DateTime<Utc>,status:Status)->Task{
        let id = Uuid::new_v4().to_string();
        Task { id:id , contents: content, created_at:created_at, status: status}
    }
    pub fn get_id(&self)->&str{
        &self.id
    }

    pub fn upgrade_status(&mut self){
        self.status = Status::Completed;
    }

    pub fn get_content(&self)->&str{
        &self.contents
    }

    pub fn get_date(&self)->&DateTime<Utc>{
        &self.created_at
    }    

    pub fn get_status(&self)->&Status{
        &self.status
    }
}


#[derive(Serialize,Deserialize)]
pub enum Status {
    Pending,
    Completed
}