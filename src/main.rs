use mongo_data_exporter::{
    export::Export,
    operations::{MongoDBConnection, Operation},
};

#[tokio::main]
async fn main() {
    // Example
    let source_db =
        MongoDBConnection::new("mongodb://localhost/test", "test", "transactions").await;
    let target_db =
        MongoDBConnection::new("mongodb://localhost2/test", "test", "transactions").await;
    let mut export = Export::init(source_db, target_db, 1000, Some(1000)).await;
    let res = export.start_export().await;
    println!("Total data inserted: {:?}", res)
}
