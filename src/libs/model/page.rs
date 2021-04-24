use super::{group::Group, media::Media, subscription::Subscription};
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
pub struct Page {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<oid::ObjectId>,
    pub title: String,
    pub handle: String,
    pub logo: String,
    pub medias: Vec<Media>,
    pub groups: Vec<Group>,
    pub subscription: Subscription,
}

impl Page {
    pub fn new(title: String, handle: String, logo: String) -> Page {
        Page {
            id: None,
            title,
            handle,
            logo,
            medias: vec![],
            groups: vec![],
            subscription: Subscription::new(),
        }
    }

    pub fn set_medias(&mut self, medias: Vec<Media>) {
        self.medias = medias;
    }

    pub fn set_groups(&mut self, groups: Vec<Group>) {
        self.groups = groups;
    }

    pub fn add_group(&mut self, group: Group) {
        self.groups.push(group);
    }

    pub fn add_media(&mut self, media: Media) {
        self.medias.push(media);
    }

    pub async fn save(&mut self) -> Result<oid::ObjectId> {
        for g in &mut self.groups {
            g.save().await?;
        }
        db::save(self).await
    }

    pub async fn find(filter: Document) -> Result<Vec<Page>> {
        let mut docs = db::find::<Page>(filter).await?;
        for d in &mut docs {
            *d.get_mut("groups").unwrap() = bson::to_bson(&Group::find(doc! {}).await?)?;
        }
        Ok(docs
            .into_iter()
            .map(|d| bson::from_document(d).unwrap())
            .collect())
    }

    pub async fn find_by_id(id: oid::ObjectId) -> Result<Page> {
        let mut doc = db::find_by_id::<Page>(id).await?;
        *doc.get_mut("groups").unwrap() = bson::to_bson(&Group::find(doc! {}).await?)?;

        Ok(bson::from_document(doc)?)
    }
}

#[async_trait]
impl Model for Page {
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
                .groups
                .iter()
                .map(|m| m.id())
                .collect::<Vec<Option<oid::ObjectId>>>(),
        )?;
        Ok(doc)
    }
}
