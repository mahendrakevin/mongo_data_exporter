use crate::operations::{MongoDBConnection, Operation};
use futures::StreamExt;
use mongodb::bson::{doc, Document};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Export<'a> {
    pub source_db: MongoDBConnection<'a>,
    pub target_db: MongoDBConnection<'a>,
    pub batch_size: i32,
    pub limit_backup: Option<i64>,
}

impl<'a> Export<'a> {
    pub async fn init(
        source_db: MongoDBConnection<'a>,
        target_db: MongoDBConnection<'a>,
        batch_size: i32,
        limit_backup: Option<i64>,
    ) -> Self {
        Self {
            source_db,
            target_db,
            batch_size,
            limit_backup,
        }
    }
    pub async fn start_export(&mut self) -> i64 {
        println!("Start exporting data from source to target database");
        let last_id = self.target_db.get_last_id().await;
        self.target_db.create_index(doc! { "collection": 1 }).await;
        let cursor = self.source_db.get_data(last_id, self.limit_backup).await;
        match cursor {
            Some(mut cursor) => {
                let docs: Rc<RefCell<Vec<Document>>> = Rc::new(RefCell::new(Vec::new()));
                let mut counter: i64 = 0;
                while let Some(document) = cursor.next().await {
                    match document {
                        Ok(document) => {
                            docs.borrow_mut().push(document);
                            if docs.borrow_mut().len() == self.batch_size as usize {
                                let res = self.target_db.insert_many(docs.borrow().clone()).await;
                                counter += res.inserted_ids.len() as i64;
                                self.target_db
                                    .update_last_id(&Some(
                                        docs.borrow().last().unwrap().get_object_id("_id").unwrap(),
                                    ))
                                    .await;
                                println!("total data inserted: {:?} data", counter);
                                docs.borrow_mut().clear();
                            }
                        }
                        Err(e) => {
                            println!("Error reading the document: {:?}", e);
                        }
                    }
                }
                if !docs.borrow().is_empty() {
                    let res = self.target_db.insert_many(docs.borrow().clone()).await;
                    counter += res.inserted_ids.len() as i64;
                    self.target_db
                        .update_last_id(&Some(
                            docs.borrow().last().unwrap().get_object_id("_id").unwrap(),
                        ))
                        .await;
                    println!("total data inserted: {:?} data", counter);
                }
                counter
            }
            None => 0,
        }
    }
}
