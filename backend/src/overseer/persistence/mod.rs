pub mod mongo;
pub use mongo::*;

use serde::{Deserialize, Serialize};

use super::{Task, TaskData};

#[derive(Serialize, Deserialize)]
pub struct Config {
    conn: String,
    db: String,
}

#[async_trait::async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn get_all_tasks(&self) -> tide::Result<Vec<Task>>;
    async fn get_task(&self, id: &str) -> tide::Result<Task>;
    async fn create_task(&self, task: TaskData) -> tide::Result<Task>;
    async fn update_task(&self, id: &str, data: TaskData) -> tide::Result<()>;
    async fn delete_task(&self, id: &str) -> tide::Result<()>;
}
