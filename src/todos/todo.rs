use serde;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    // #[serde(rename = "taskId")]
    // pub task_id: u8,
    #[serde(rename = "taskName")]
    pub task_name: String,
    #[serde(rename = "doneStatus")]
    pub done_status: bool,
}