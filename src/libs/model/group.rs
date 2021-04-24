use super::page_element::PageElement;
use crate::libs::db::{self, Model};
use anyhow::Result;
use async_trait::async_trait;
use mongodb::{
    bson::{self, doc, oid, Document},
    Collection,
};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<oid::ObjectId>,
    pub title: String,
    pub elements: Vec<PageElement>,
}

impl Group {
    pub fn new(title: String) -> Group {
        Group {
            id: None,
            title,
            elements: vec![],
        }
    }

    pub fn add_element(&mut self, element: PageElement) {
        self.elements.push(element);
    }

    pub async fn save(&mut self) -> Result<oid::ObjectId> {
        for e in &mut self.elements {
            match e {
                PageElement::Folder(f) => {
                    f.save().await?;
                }
                PageElement::Link(l) => {
                    l.save().await?;
                }
            }
        }
        db::save(self).await
    }

    pub async fn find(filter: Document) -> Result<Vec<Group>> {
        let mut docs = db::find::<Group>(filter).await?;
        for d in &mut docs {
            *d.get_mut("elements").unwrap() =
                PageElement::get_elements(d.get("elements").unwrap()).await?;
        }
        Ok(docs
            .into_iter()
            .map(|d| bson::from_document(d).unwrap())
            .collect())
    }

    pub async fn find_by_id(id: oid::ObjectId) -> Result<Group> {
        let mut doc = db::find_by_id::<Group>(id).await?;
        *doc.get_mut("elements").unwrap() =
            PageElement::get_elements(doc.get("elements").unwrap()).await?;
        Ok(bson::from_document(doc)?)
    }

    pub async fn delete(self) -> Result<Group> {
        db::delete::<Group>(self).await
    }
}

#[async_trait]
impl Model for Group {
    fn id(&self) -> Option<oid::ObjectId> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Option<oid::ObjectId>) {
        self.id = id;
    }

    async fn collection() -> Result<Collection> {
        Ok(db::get_connection().await?.collection("groups"))
    }

    fn as_document(&mut self) -> Result<bson::Document> {
        let mut doc = bson::to_bson(self)?
            .as_document()
            .ok_or_else(|| Error::new(ErrorKind::Other, "error parsing document"))?
            .to_owned();
        *doc.get_mut("elements").unwrap() = bson::to_bson(
            &self
                .elements
                .iter()
                .map(|e| match e {
                    PageElement::Folder(f) => doc! {"Folder": bson::to_bson(&f.id()).expect("error serializing folder id")},
                    PageElement::Link(l) => doc! {"Link": bson::to_bson(&l.id()).expect("error serializing link id")},
                })
                .collect::<Vec<Document>>(),
        )?;
        Ok(doc)
    }
}
