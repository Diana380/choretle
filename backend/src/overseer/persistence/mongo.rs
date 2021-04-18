use crate::overseer::{Task, TaskData};

use super::{Config, Handler};

use mongodb::bson::{doc, oid::ObjectId};

use async_std::stream::StreamExt;

pub struct Mongo {
    collection: mongodb::Collection<Task>,
}

impl Mongo {
    pub async fn new(config: &Config) -> tide::Result<Self> {
        let collection = mongodb::Client::with_uri_str(&config.conn)
            .await?
            .database(&config.db)
            .collection_with_type::<Task>("tasks");

        Ok(Self { collection })
    }
}

#[async_trait::async_trait]
impl Handler for Mongo {
    async fn get_all_tasks(&self) -> tide::Result<Vec<Task>> {
        let cursor = self.collection.find(doc! {}, None).await?;

        let v: Result<Vec<Task>, mongodb::error::Error> = cursor.collect().await;
        Ok(v?)
    }

    async fn get_task(&self, id: &str) -> tide::Result<Task> {
        let oid = id.parse::<mongodb::bson::oid::ObjectId>()?;
        let filter = doc! {
            "_id": oid.clone()
        };

        let task = self
            .collection
            .find_one(filter, None)
            .await?
            .ok_or_else(|| anyhow::format_err!("task {} not found", id))?;
        Ok(task)
    }

    async fn create_task(&self, data: TaskData) -> tide::Result<Task> {
        let task = Task {
            id: ObjectId::new(),
            data,
        };

        self.collection.insert_one(task.clone(), None).await?;

        Ok(task)
    }

    async fn update_task(&self, id: &str, data: TaskData) -> tide::Result<()> {
        let oid = id.parse::<mongodb::bson::oid::ObjectId>()?;
        let filter = doc! {
            "_id": oid.clone()
        };
        let update = doc! {
            "$set": mongodb::bson::to_document(&data)?,
        };

        self.collection.update_one(filter, update, None).await?;

        Ok(())
    }

    async fn delete_task(&self, id: &str) -> tide::Result<()> {
        let oid = id.parse::<mongodb::bson::oid::ObjectId>()?;
        let filter = doc! {
            "_id": oid.clone()
        };

        self.collection.delete_one(filter, None).await?;
        Ok(())
    }
}
