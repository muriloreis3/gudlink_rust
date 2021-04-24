use super::constants;
use anyhow::Result;
use async_trait::async_trait;
use futures_util::stream::StreamExt;
use mongodb::{
    bson::{self, doc, oid, Document},
    options::ClientOptions,
    Client, Collection, Database,
};
use serde::{Serialize, de::DeserializeOwned};
use std::env;
use std::io::{Error, ErrorKind};

#[async_trait]
pub trait Model: Sized + Sync + Serialize + DeserializeOwned {
    fn id(&self) -> Option<oid::ObjectId>;
    fn set_id(&mut self, id: Option<oid::ObjectId>);
    async fn collection() -> Result<Collection>;

    fn as_document(&self) -> Result<Document> {
        Ok(bson::to_bson(self)?
            .as_document()
            .ok_or_else(|| Error::new(ErrorKind::Other, "error parsing document"))?
            .to_owned())
    }

    async fn save(&mut self) -> Result<oid::ObjectId> {
        match self.id() {
            None => {
                let id = Some(self.insert().await?);
                self.set_id(id);
            }
            Some(_) => {
                self.update().await?;
            }
        }
        Ok(self.id().ok_or_else(|| Error::new(ErrorKind::Other, "no id after saving"))?)
    }

    async fn find(filter: Document) -> Result<Vec<Document>> {
        Ok(Self::collection()
            .await?
            .aggregate(vec![doc! {"$match": filter,}], None)
            .await?
            .map(|x| x.expect("error mapping document"))
            .collect::<Vec<Document>>()
            .await)
    }

    async fn find_by_id(id: oid::ObjectId) -> Result<Document> {
        Ok(bson::from_document(
            Self::collection()
                .await?
                .find_one(doc! {"_id": id}, None)
                .await?
                .expect("find error"),
        )?)
    }

    async fn delete(self) -> Result<Self> {
        Self::collection()
            .await?
            .delete_one(doc! {"_id": self.id().clone().unwrap()}, None)
            .await?;
        Ok(self)
    }

    async fn insert(&mut self) -> Result<oid::ObjectId> {
        Ok(Self::collection()
            .await?
            .insert_one(self.as_document()?.to_owned(), None)
            .await?
            .inserted_id
            .as_object_id()
            .expect("No id inserted")
            .to_owned())
    }

    async fn update(&mut self) -> Result<()> {
        Self::collection()
            .await?
            .update_one(
                doc! {"_id": self.id().clone().expect("No id inserted")},
                self.as_document()?,
                None,
            )
            .await?;
        Ok(())
    }
}

fn get_db_url() -> String {
    env::var("DB_URL")
        .ok()
        .unwrap_or_else(|| constants::DB_URL.to_string())
}

pub async fn get_connection() -> Result<Database> {
    let options = ClientOptions::parse(&get_db_url()).await?;

    let client = Client::with_options(options)?;

    Ok(client.database(constants::DB_NAME))
}
