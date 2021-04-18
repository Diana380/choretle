use async_std::sync::Arc;
use tide::Response;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    name: String,
    port: u16,
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
        srv.at("/login").get(Self::login);
        srv.listen(format!("0.0.0.0:{}", port)).await?;
        Ok(())
    }

    async fn login(_: tide::Request<Arc<Self>>) -> tide::Result {
        Ok(Response::new(200))
    }
}
