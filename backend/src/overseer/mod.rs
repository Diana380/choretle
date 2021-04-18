pub mod persistence;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tide::{Request, Response};

use self::persistence::Handler;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Effort {
    Low,
    Medium,
    High,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskData {
    pub name: String,
    pub effort: Effort,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(flatten)]
    pub data: TaskData,
}
#[derive(Serialize, Deserialize)]
pub struct TaskJson {
    #[serde(skip_deserializing, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(flatten)]
    pub data: TaskData,
}

impl From<Task> for TaskJson {
    fn from(val: Task) -> Self {
        TaskJson {
            id: val.id.to_hex(),
            data: val.data,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub port: u16,
    pub db: self::persistence::Config,
}

pub struct Service {
    pub config: Config,
    db: Box<dyn Handler>,
}

impl Service {
    pub fn create(config: Config, db: Box<dyn Handler>) -> Self {
        Service { config, db }
    }

    pub async fn start(self) -> tide::Result<()> {
        tide::log::start();

        let port = self.config.port;

        let mut srv = tide::with_state(Arc::new(self));
        srv.at("/tasks")
            .post(Self::create_task)
            .get(Self::get_all_tasks);

        srv.at("/tasks/:id")
            .get(Self::get_task)
            .put(Self::update_task)
            .delete(Self::delete_task);

        srv.listen(format!("0.0.0.0:{}", port)).await?;
        Ok(())
    }

    async fn create_task(mut req: Request<Arc<Self>>) -> tide::Result {
        let task = req.body_json::<TaskData>().await?;
        let created = req.state().db.create_task(task).await?;

        Ok(Response::builder(201)
            .body(json!(TaskJson::from(created)))
            .build())
    }

    async fn get_all_tasks(req: Request<Arc<Self>>) -> tide::Result {
        let tasks: Vec<TaskJson> = req
            .state()
            .db
            .get_all_tasks()
            .await?
            .into_iter()
            .map(TaskJson::from)
            .collect();

        Ok(Response::builder(200).body(json!(tasks)).build())
    }

    async fn get_task(req: Request<Arc<Self>>) -> tide::Result {
        let id = req.param("id")?;
        let task = req.state().db.get_task(id).await?;
        Ok(Response::builder(200)
            .body(json!(TaskJson::from(task)))
            .build())
    }

    async fn update_task(mut req: Request<Arc<Self>>) -> tide::Result {
        let body = req.body_json().await?;
        let id = req.param("id")?;
        req.state().db.update_task(id, body).await?;
        Ok(Response::new(200))
    }

    async fn delete_task(req: Request<Arc<Self>>) -> tide::Result {
        let id = req.param("id")?;
        req.state().db.delete_task(id).await?;
        Ok(Response::new(200))
    }
}
