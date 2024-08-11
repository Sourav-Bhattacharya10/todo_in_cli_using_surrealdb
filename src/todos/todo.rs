use serde;
use serde::{ Serialize, Deserialize };
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    #[serde(rename = "id")]
    pub task_id: Option<Thing>,
    #[serde(rename = "taskName")]
    pub task_name: String,
    #[serde(rename = "doneStatus")]
    pub done_status: bool,
}