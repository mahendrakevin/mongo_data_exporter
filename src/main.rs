use mongodb::bson;
use mongodb::results::InsertManyResult;

struct BackupProperties<'a> {
    source_db: MongoDBConnection<'a>,
    target_db: MongoDBConnection<'a>,
    batch_size: i32,
}

#[derive(Debug)]
struct MongoDBConnection<'a> {
    connection: mongodb::Client,
    database: &'a str,
    collection: &'a str,
}

#[async_trait::async_trait]
trait Connect<'a> {
    async fn new(uri: &str, database: &'a str, collection: &'a str) -> Self;
    async fn change_collection(&mut self, collection: &'a str);
    async fn insert_many(&self, document: Vec<bson::Document>) -> InsertManyResult;
}

#[async_trait::async_trait]
impl<'a> Connect<'a> for MongoDBConnection<'a> {
    async fn new(uri: &str, database: &'a str, collection: &'a str) -> Self {
        let connection = mongodb::Client::with_uri_str(uri).await.unwrap();
        Self {
            connection,
            database,
            collection,
        }
    }

    async fn change_collection(&mut self, collection: &'a str) {
        self.collection = collection;
    }

    async fn insert_many(&self, document: Vec<bson::Document>) -> InsertManyResult {
        let collection = self
            .connection
            .database(self.database)
            .collection::<bson::Document>(self.collection);
        collection.insert_many(document, None).await.unwrap()
    }
}

#[tokio::main]
async fn main() {
    // let mut connection = MongoDBConnection::new(
    //     "mongodb://192.168.180.125:27017",
    //     "log_check",
    //     "test_mongo_exporter",
    // )
    // .await;
    // connection.change_collection("test_mongo_exporter2").await;
    // let res = connection
    //     .insert_many(vec![bson::doc! { "name": "John Doe" }])
    //     .await;
    // println!("{:?}", res);
}
