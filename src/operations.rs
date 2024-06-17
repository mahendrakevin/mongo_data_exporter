use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson, DateTime, Document};
use mongodb::options::{CreateIndexOptions, FindOptions, UpdateOptions};
use mongodb::results::{CreateIndexResult, InsertManyResult};
use mongodb::{Cursor, IndexModel};

#[derive(Debug)]
pub struct MongoDBConnection<'a> {
    pub connection: mongodb::Client,
    pub database: &'a str,
    pub collection: &'a str,
}

#[async_trait::async_trait]
pub trait Operation<'a> {
    async fn new(uri: &str, database: &'a str, collection: &'a str) -> Self;
    async fn change_collection(&mut self, collection: &'a str);
    async fn create_index(&self, index: Document) -> Option<CreateIndexResult>;
    async fn get_data(
        &self,
        last_id: Option<ObjectId>,
        limit: Option<i64>,
    ) -> Option<Cursor<Document>>;
    async fn get_last_id(&self) -> Option<ObjectId>;
    async fn insert_many(&self, document: Vec<Document>) -> InsertManyResult;
    async fn update_last_id(&self, last_id: &Option<ObjectId>) -> String;
}

#[async_trait::async_trait]
impl<'a> Operation<'a> for MongoDBConnection<'a> {
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

    async fn create_index(&self, index: Document) -> Option<CreateIndexResult> {
        let collection = self
            .connection
            .database(self.database)
            .collection::<Document>("export_last_ids");
        let index_model = IndexModel::builder().keys(index).build();
        let options = CreateIndexOptions::builder().build();
        collection.create_index(index_model, options).await.ok()
    }

    async fn get_data(
        &self,
        last_id: Option<ObjectId>,
        limit: Option<i64>,
    ) -> Option<Cursor<Document>> {
        let collection = self
            .connection
            .database(self.database)
            .collection::<Document>(self.collection);
        let options = match limit {
            Some(limit) => FindOptions::builder().limit(limit).build(),
            None => FindOptions::builder().build(),
        };
        let find_filter = doc! { "_id": { "$gt": last_id } };
        collection.find(find_filter, options).await.ok()
    }

    async fn get_last_id(&self) -> Option<ObjectId> {
        let collection = self
            .connection
            .database(self.database)
            .collection::<Document>("export_last_ids");
        let filter = doc! { "collection": self.collection };
        let result = collection.find_one(filter, None).await.unwrap();
        println!("{:?}", result);
        match result {
            Some(document) => {
                let last_id = document.get_object_id("last_id").unwrap();
                Some(last_id)
            }
            None => {
                let bytes = [0; 12];
                Some(ObjectId::from_bytes(bytes))
            }
        }
    }

    async fn insert_many(&self, document: Vec<Document>) -> InsertManyResult {
        let collection = self
            .connection
            .database(self.database)
            .collection::<Document>(self.collection);
        collection.insert_many(document, None).await.unwrap()
    }

    async fn update_last_id(&self, last_id: &Option<ObjectId>) -> String {
        let last_id = if last_id.is_none() {
            let bytes = [0; 12];
            ObjectId::from_bytes(bytes)
        } else {
            last_id.unwrap()
        };

        let now = Utc::now();
        let bson_datetime: Bson = Bson::DateTime(DateTime::from(now));

        let exported_last = doc! {
            "collection": self.collection,
            "last_id": last_id,
            "updated_at": bson_datetime,
        };

        let collection_name = self
            .connection
            .database(self.database)
            .collection::<Document>("export_last_ids");
        let options = UpdateOptions::builder().upsert(true).build();
        let update = collection_name
            .update_one(
                doc! { "collection": self.collection },
                doc! { "$set": &exported_last },
                options,
            )
            .await;

        match update {
            Ok(t) => {
                if t.upserted_id.is_some() {
                    String::from("A new last_id was inserted")
                } else {
                    String::from("The last_id was updated")
                }
            }
            Err(e) => String::from(format!("Error updating the last_id, {e}")),
        }
    }
}
