<div align="center">
  <h1>Mongo Data Exporter</h1>
  <p>
    <strong>This is a simple tool to export data from a MongoDB database to another MongoDB database instance. It is fully written in Rust.</strong>
  </p>
  <p>

<!-- prettier-ignore-start -->

[![crates.io](https://img.shields.io/crates/v/mongo_data_exporter?label=latest)](https://crates.io/crates/mongo_data_exporter)
[![Documentation](https://docs.rs/mongo_data_exporter/badge.svg?version=0.1.2)](https://docs.rs/mongo_data_exporter/0.1.2)
![MSRV](https://img.shields.io/badge/rustc-1.72+-ab6000.svg)
[![Dependency Status](https://deps.rs/crate/actix-web/4.7.0/status.svg)](https://deps.rs/crate/mongo_data_exporter/0.1.2)
<br />
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/mongo_data_exporter.svg)
[![CI](https://github.com/mahendrakevin/mongo_data_exporter/actions/workflows/build-binary.yml/badge.svg)](https://github.com/mahendrakevin/mongo_data_exporter/actions/workflows/build-binary.yml)
![downloads](https://img.shields.io/crates/d/mongo_data_exporter.svg)

<!-- prettier-ignore-end -->

  </p>
</div>

## Dependencies:

```toml
[dependencies]
async-trait = "0.1.80"
mongodb = { version = "2.8.2", features = ["bson-chrono-0_4"] }
tokio = { version = "1.38.0", features = ["rt", "rt-multi-thread", "macros"] }
chrono = "0.4.38"
futures = "0.3.30"
```

## Usage:

```rust
use mongo_data_exporter::export;
use mongo_data_exporter::operations::{MongoDBConnection, Operation};

#[tokio::main]
async fn main() {
    // Source database connection
    let source_db_dr = MongoDBConnection::new("mongodb://localhost:27017/test", "test", "test-collection").await;
    
    // Target database connection
    let target_db_dr = MongoDBConnection::new("mongodb://localhost2:27017/test", "test", "test-collection").await;

    // Create export operation, you can specify the batch size and the limit data to export
    let mut export_dr = export::Export::init(source_db_dr, target_db_dr, 10000, None).await;
    
    // Start the export operation
    export_dr.start_export().await;
}
```

## License

This project is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])
