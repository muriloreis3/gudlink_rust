use serde::{Deserialize, Serialize};
use crate::libs::db::{self, Model};
use mongodb::{Collection, bson::{self, Document, oid}};
use async_trait::async_trait;
use anyhow::Result;
use super::page_element::PageElement;
use async_recursion::async_recursion;

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

    pub async fn save(&mut self) -> Result<oid::ObjectId> {
        db::save(self).await
    }

    pub async fn find(filter: Document) -> Result<Vec<Folder>> {
        let mut docs = db::find::<Folder>(filter).await?;
        for d in &mut docs {
            *d.get_mut("elements").unwrap() =
                PageElement::get_elements(d.get("elements").unwrap()).await?;
        }
        Ok(docs
            .into_iter()
            .map(|d| bson::from_document(d).unwrap())
            .collect())
    }

    #[async_recursion(?Send)]
    pub async fn find_by_id(id: oid::ObjectId) -> Result<Folder> {
        let mut doc = db::find_by_id::<Folder>(id).await?;
        *doc.get_mut("elements").unwrap() = PageElement::get_elements(doc.get("elements").unwrap()).await?;
        Ok(bson::from_document(doc)?)
    }

    pub async fn delete(self) -> Result<Folder> {
        db::delete(self).await
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