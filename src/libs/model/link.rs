use serde::{Deserialize, Serialize};
use crate::libs::db::{self, Model};
use mongodb::{bson::oid, Collection};
use async_trait::async_trait;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LinkType {
    FILE,
    IMAGE,
    PAGE
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<oid::ObjectId>,
    pub title: String,
    pub link: String,
    pub private: bool,
    pub l_type: LinkType,
}

impl Link {
    pub fn new(title: String, link: String, l_type: LinkType) -> Link {
        Link {
            id: None,
            title,
            link,
            private: false,
            l_type,
        }
    }
}

#[async_trait]
impl Model for Link {
    fn id(&self) -> Option<oid::ObjectId> {
        self.id.clone()
    }
    
    fn set_id(&mut self, id: Option<oid::ObjectId>) {
        self.id = id;
    }

    async fn collection() -> Result<Collection> {
        Ok(db::get_connection().await?.collection("links"))
    }
}