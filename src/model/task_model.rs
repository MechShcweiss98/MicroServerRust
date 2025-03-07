use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct  TaskModel{
    pub id: i32,
    pub name: String, 
    pub description:String, 
    pub created_at: NaiveDateTime
}

#[derive(Serialize, Deserialize)]
pub struct CreateTaskReq{
    pub name: String, 
    pub description:String, 
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTaskReq{
    pub name: Option<String>, 
    pub description:Option<String>, 
}