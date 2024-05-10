# This Initializer adds support for connection to a MongoDB database

It is recommended to be used with the base starter that does not come with a database connection (as those all use SQL).

Steps to be taken:
* Add the mongodb feature to you loco project:

```toml
[dependencies]
loco-extras = { version = "*", features = ["mongodb"] }
```

* Add a mongodb connection section to you config.yaml file:

```yaml
mongodb:
  uri: "mongodb://localhost:27017"
  db_name: "my_db"
```

* Add the initializer to your main.rs file:

```rust
use loco_extras::mongodb;

async fn initializers(ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
    let  initializers: Vec<Box<dyn Initializer>> = vec![
        Box::new(loco_extras::initializers::mongodb::MongoInitializer),
    ];

    Ok(initializers)
}
```

* Use the connection in your code:

```rust
use axum::{response::IntoResponse, Extension};
use loco_extras::mongodb::MongoDb;

pub async fn route_handler(State(ctx): State<AppContext>, Extension(mongo): Extension<MongoDb>) -> Result<impl IntoResponse> {
    // TODO: Use the mongo connection
}
```
