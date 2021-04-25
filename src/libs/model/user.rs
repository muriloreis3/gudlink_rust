use super::{group::Group, page::Page};
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
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<oid::ObjectId>,
    name: String,
    username: String,
    password: String,
    email: String,
    pages: Vec<Page>,
}

impl User {
    fn new(name: String, username: String, password: String, email: String) -> User {
        User {
            id: None,
            name,
            username,
            password,
            email,
            pages: vec![],
        }
    }

    pub async fn save(&mut self) -> Result<oid::ObjectId> {
        db::save(self).await
    }

    pub async fn find(filter: Document) -> Result<Vec<User>> {
        let mut docs = db::find::<User>(filter).await?;
        for d in &mut docs {
            *d.get_mut("pages").unwrap() = bson::to_bson(&Group::find(doc! {}).await?)?;
        }
        Ok(docs
            .into_iter()
            .map(|d| bson::from_document(d).unwrap())
            .collect())
    }

    pub async fn find_by_id(id: oid::ObjectId) -> Result<User> {
        let mut doc = db::find_by_id::<Page>(id).await?;
        *doc.get_mut("pages").unwrap() = bson::to_bson(&Group::find(doc! {}).await?)?;

        Ok(bson::from_document(doc)?)
    }

    pub async fn delete(self) -> Result<User> {
        db::delete(self).await
    }
}

#[async_trait]
impl Model for User {
    fn id(&self) -> Option<oid::ObjectId> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Option<oid::ObjectId>) {
        self.id = id;
    }

    async fn collection() -> Result<Collection> {
        Ok(db::get_connection().await?.collection("pages"))
    }

    fn as_document(&mut self) -> Result<Document> {
        let mut doc = bson::to_bson(self)?
            .as_document()
            .ok_or_else(|| Error::new(ErrorKind::Other, "error parsing document"))?
            .to_owned();
        *doc.get_mut("groups").unwrap() = bson::to_bson(
            &self
                .pages
                .iter()
                .map(|m| m.id())
                .collect::<Vec<Option<oid::ObjectId>>>(),
        )?;
        Ok(doc)
    }
}
