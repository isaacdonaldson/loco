[package]
name = "migration"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
name = "migration"
path = "src/lib.rs"

[dependencies]
async-std = { version = "1", features = ["attributes", "tokio1"] }
loco-rs = { workspace = true }


[dependencies.sea-orm-migration]
version = "1.1.0"
features = [
  # Enable at least one `ASYNC_RUNTIME` and `DATABASE_DRIVER` feature if you want to run migration via CLI.
  # View the list of supported features at https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime.
  # e.g.
  "runtime-tokio-rustls", # `ASYNC_RUNTIME` feature
]
