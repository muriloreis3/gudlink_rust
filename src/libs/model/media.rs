use serde::{Deserialize, Serialize};
use crate::libs::db::{self, Model};
use mongodb::{bson::oid, Collection};
use async_trait::async_trait;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum MediaType {
    INSTAGRAM,
    FACEBOOK,
    WHATSAPP,
    LINKEDIN,
    BEHANCE,
    PINTEREST,
    TWITTER,
    MEDIUM,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<oid::ObjectId>,
    pub media_type: MediaType,
    pub link: String,
}

impl Media {
    pub fn new(media_type: MediaType, link: String) -> Media {
        Media {
            id: None,
            media_type,
            link,
        }
    }
}

#[async_trait]
impl Model for Media {
    fn id(&self) -> Option<oid::ObjectId> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Option<oid::ObjectId>) {
        self.id = id;
    }

    async fn collection() -> Result<Collection> {
        Ok(db::get_connection().await?.collection("medias"))
    }
}