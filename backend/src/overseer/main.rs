use choretle::overseer::{persistence::Mongo, Config, Service};

async fn load_config() -> anyhow::Result<Config> {
    if let Ok(config) = std::env::var("config") {
        Ok(serde_json::from_str(config.as_str())?)
    } else if let Ok(config) =
        async_std::fs::read_to_string("src/overseer/default.config.json").await
    {
        Ok(serde_json::from_str(config.as_str())?)
    } else {
        Err(anyhow::format_err!(
            "Failed to get config for overseer service"
        ))
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let config = load_config().await?;

    let mongo = Mongo::new(&config.db).await?;

    let overseer = Service::create(config, Box::new(mongo));
    overseer.start().await
}
