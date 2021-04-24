use crate::libs::db::{self, Model};
use anyhow::Result;
use async_trait::async_trait;
use mongodb::{
    bson::{self, oid, Document},
    Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LinkType {
    FILE,
    IMAGE,
    PAGE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
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
    pub async fn save(&mut self) -> Result<oid::ObjectId> {
        db::save(self).await
    }

    pub async fn find(filter: Document) -> Result<Vec<Link>> {
        Ok(db::find::<Link>(filter)
            .await?
            .iter()
            .map(|l| bson::from_document(l.to_owned()).expect("error parsing document"))
            .collect())
    }

    pub async fn find_by_id(id: oid::ObjectId) -> Result<Link> {
        Ok(bson::from_document(db::find_by_id::<Link>(id).await?)?)
    }

    pub async fn delete(self) -> Result<Link> {
        db::delete(self).await
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
