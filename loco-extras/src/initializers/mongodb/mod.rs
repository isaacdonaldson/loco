use std::time::Duration;

use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use loco_rs::prelude::*;

use mongodb::{bson::doc, options::ClientOptions, Client, Database};

#[allow(clippy::module_name_repetitions)]
pub struct MongoDbInitializer;

#[async_trait]
impl Initializer for MongoDbInitializer {
    fn name(&self) -> String {
        "mongodb".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let mongo_db_config = ctx
            .config
            .initializers
            .clone()
            .ok_or_else(|| Error::Message("initializers config not configured".to_string()))?;

        let mongo_db_value = mongo_db_config
            .get("mongodb")
            .ok_or_else(|| Error::Message("mongo not configured as initializer".to_string()))?;

        let mongo_db: MongoDbConfig = serde_json::from_value(mongo_db_value.clone())
            .map_err(|e| Error::Message(e.to_string()))?;

        let db = connect_to_db(mongo_db).await?;

        Ok(router.layer(Extension(db)))
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct MongoDbConfig {
    uri: String,
    db_name: String,
    connect_timeout: Option<u64>,
}

async fn connect_to_db(config: MongoDbConfig) -> Result<Database> {
    let mut client_options = ClientOptions::parse(&config.uri)
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

    let timeout = Duration::from_millis(
        config.connect_timeout.unwrap_or(2000), // default to 2 seconds
    );

    client_options.connect_timeout = Some(timeout);
    client_options.server_selection_timeout = Some(timeout);

    let client = Client::with_options(client_options).map_err(|e| Error::Message(e.to_string()))?;

    let db = client.database(config.db_name.as_ref());
    db.run_command(doc! { "ping": 1 }, None)
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

    Ok(db)
}
