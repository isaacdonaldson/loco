# This Initializer adds support for connection to a MongoDB database

It is recommended to be used with the base starter that does not come with a database connection (as those all use SQL), but it can be used with any other starter as well.

Steps to add this initializer to your project:
* Add the mongodb and mongodb initializer to your loco project:

```toml
[dependencies]
loco-extras = { version = "*", features = ["mongodb"] }
mongodb = { version = "2.8.0"}
```

* Add a mongodb connection section to you config.yaml file:

```yaml
initializers:
  mongodb:
    uri: {{get_env(name="MONGODB_URI", default="mongodb://localhost:27017/")}}
    db_name: "db_name"
    client_options:
      connectTimeout:
        secs: 2
        nanos: 0
      serverSelectionTimeout:
        secs: 2
        nanos: 0
```

Although untested, the client_options should be able to take any options that the mongodb driver supports. The options are passed as a serde_json::Value, so however that type is serialized/deserialized should work here. Example above shows how `Duration` is serialized.


* Add the initializer to your `src/app.rs` file:

```rust

pub struct App;
#[async_trait]
impl Hooks for App {
    // Other code...
    async fn initializers(ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        let mut initializers: Vec<Box<dyn Initializer>> = vec![
            Box::new(loco_extras::initializers::mongodb::MongoDbInitializer),
        ];

        Ok(initializers)
    }
    // Other code...
}
```

* Now you can use the connection in your handler code:

```rust
use axum::Extension;
use loco_rs::prelude::*;
use serde_json::Value;
use mongodb::{bson::doc, Database};

pub async fn fetch(Extension(mongodb): Extension<Database>) -> Result<Response> {
    let user: Option<Value> = mongodb.collection("users").find_one(doc!{}, None).await.map_err(|_| Error::NotFound)?;
    format::json(user.ok_or_else(|| Error::NotFound)?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("mongo")
        .add("/", get(fetch))
}
```
