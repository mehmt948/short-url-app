use std::sync::Arc;
use async_trait::async_trait;
use mongodb::{Client, Collection, bson::{doc, DateTime}};
use crate::structs;

#[async_trait]
pub trait IRepository {
    async fn create_short_link(&self, short_id: String, url: String);
    async fn find_short_link(&self, short_id: String) -> Result<structs::ShortUrl, &str>;
}

#[derive(Clone)]
pub struct Repository {
    pub(crate) client: Client
}

#[async_trait]
impl IRepository for Repository {
    async fn create_short_link(&self, short_id: String, url: String) {
        let short_links_collection: Collection<structs::ShortUrl> = self.client.database("short_link").collection::<structs::ShortUrl>("links");

        let short_url_item = structs::ShortUrl {
            short_id,
            url,
            timestamp: DateTime::now(),
        };

        let _insert_result = short_links_collection.insert_one(short_url_item.clone(), None).await;
    }
    async fn find_short_link(&self, short_id: String) -> Result<structs::ShortUrl, &str> {
        let short_links_collection: Collection<structs::ShortUrl> = self.client.database("short_link").collection::<structs::ShortUrl>("links");

        let res = short_links_collection.find_one(
            doc! {
                "short_id": short_id,
            },
            None
        ).await;

        match res {
            Ok(value) => {
                match value {
                    Some(short_url_item) => {
                        Ok(short_url_item)
                    },
                    None => {
                        Err("Url not found")
                    }
                }
            },
            Err(_) => {
                Err("System error. Try again later")
            }
        }
    }
}

pub type DynRepository = Arc<dyn IRepository + Send + Sync>;