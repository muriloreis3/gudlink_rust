use serde::{Deserialize, Serialize};
use crate::libs::db::{self, Model};
use mongodb::{bson::oid, Collection};
use async_trait::async_trait;
use anyhow::Result;
use super::page_element::PageElement;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folder {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<oid::ObjectId>,
    pub title: String,
    pub sub_title: String,
    pub elements: Vec<PageElement>,
}

impl Folder {
    pub fn new(title: String, sub_title: String) -> Folder {
        Folder {
            id: None,
            title,
            sub_title,
            elements: vec![],
        }
    }

    pub fn add_element(&mut self, element: PageElement) {
        self.elements.push(element);
    }
}

#[async_trait]
impl Model for Folder {
    fn id(&self) -> Option<oid::ObjectId> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Option<oid::ObjectId>) {
        self.id = id;
    }

    async fn collection() -> Result<Collection> {
        Ok(db::get_connection().await?.collection("folders"))
    }
}