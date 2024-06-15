use mongodb::bson;
use mongo_data_exporter::{Connect, MongoDBConnection};

#[tokio::main]
async fn main() {
    let mut connection = MongoDBConnection::new(
        "mongodb://192.168.180.125:27017",
        "log_check",
        "test_mongo_exporter",
    )
    .await;
    connection.change_collection("test_mongo_exporter2").await;
    let res = connection
        .insert_many(vec![bson::doc! { "name": "John Doe" }])
        .await;
    println!("{:?}", res);
}
