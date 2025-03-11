use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct  TaskModel{
    pub id: i32,
    pub name: String, 
    pub description:String, 
    pub created_at: NaiveDateTime
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateTaskReq{
    pub name: String, 
    pub description:String, 
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateTaskReq{
    pub name: Option<String>, 
    pub description:Option<String>, 
}