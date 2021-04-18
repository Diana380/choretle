use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct ServiceData {
    pub name: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub port: u16,
    pub services: Vec<ServiceData>,
}

pub struct Service {
    pub config: Config,
}

impl Service {
    pub fn create(config: Config) -> Self {
        Service { config }
    }

    pub async fn start(self) -> tide::Result<()> {
        tide::log::start();

        let port = self.config.port;

        let mut srv = tide::with_state(Arc::new(self));
        srv.at("/services").get(Self::services);
        srv.listen(format!("0.0.0.0:{}", port)).await?;
        Ok(())
    }

    async fn services(req: tide::Request<Arc<Self>>) -> tide::Result {
        Ok(tide::Response::builder(200)
            .body(json!(req.state().config.services))
            .build())
    }
}
