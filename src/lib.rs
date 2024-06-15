use mongodb::bson;
use mongodb::results::InsertManyResult;

pub struct BackupProperties<'a> {
    pub source_db: MongoDBConnection<'a>,
    pub target_db: MongoDBConnection<'a>,
    pub batch_size: i32,
}

#[derive(Debug)]
pub struct MongoDBConnection<'a> {
    pub connection: mongodb::Client,
    pub database: &'a str,
    pub collection: &'a str,
}

#[async_trait::async_trait]
pub trait Connect<'a> {
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