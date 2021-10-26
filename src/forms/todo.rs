use std::time::SystemTime;

use chrono::NaiveDateTime;



#[derive(serde::Deserialize)]
pub struct JTodo {
    pub title:String,
    pub descript:String,
    pub tags:Vec<String>,
    pub ddl:NaiveDateTime
}