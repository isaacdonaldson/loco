use axum::Extension;
use loco_rs::prelude::*;
use serde_json::Value;
use mongodb::{bson::doc, Database};

pub async fn list(Extension(mongodb): Extension<Database>) -> Result<Response> {
    let user: Option<Value> = mongodb.collection("users").find_one(doc!{}, None).await.map_err(|_| Error::NotFound)?;
    format::json(user.ok_or_else(|| Error::NotFound)?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("mongo")
        .add("/", get(list))
}
