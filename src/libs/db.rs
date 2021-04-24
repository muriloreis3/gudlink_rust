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
pub trait Model: Serialize + DeserializeOwned {
    fn id(&self) -> Option<oid::ObjectId>;
    fn set_id(&mut self, id: Option<oid::ObjectId>);
    async fn collection() -> Result<Collection>;

    fn as_document(&mut self) -> Result<Document> {
        Ok(bson::to_bson(self)?
            .as_document()
            .ok_or_else(|| Error::new(ErrorKind::Other, "error parsing document"))?
            .to_owned())
    }
}

pub async fn save<T>(obj: &mut T) -> Result<oid::ObjectId> where T: Model{
    match obj.id() {
        None => {
            let id = Some(insert(obj).await?);
            obj.set_id(id);
        }
        Some(_) => {
            update(obj).await?;
        }
    }
    Ok(obj.id().ok_or_else(|| Error::new(ErrorKind::Other, "no id after saving"))?)
}

pub async fn find<T>(filter: Document) -> Result<Vec<Document>> where T: Model {
    Ok(T::collection()
        .await?
        .aggregate(vec![doc! {"$match": filter,}], None)
        .await?
        .map(|x| x.expect("error mapping document"))
        .collect::<Vec<Document>>()
        .await)
}

pub async fn find_by_id<T>(id: oid::ObjectId) -> Result<Document> where T: Model{
    Ok(bson::from_document(
        T::collection()
            .await?
            .find_one(doc! {"_id": id}, None)
            .await?
            .expect("find error"),
    )?)
}

pub async fn delete<T>(obj: T) -> Result<T> where T: Model{
    T::collection()
        .await?
        .delete_one(doc! {"_id": obj.id().clone().unwrap()}, None)
        .await?;
    Ok(obj)
}

async fn insert<T>(obj: &mut T) -> Result<oid::ObjectId> where T: Model{
    Ok(T::collection()
        .await?
        .insert_one(obj.as_document()?.to_owned(), None)
        .await?
        .inserted_id
        .as_object_id()
        .expect("No id inserted")
        .to_owned())
}

async fn update<T>(obj: &mut T) -> Result<()> where T: Model {
    T::collection()
        .await?
        .update_one(
            doc! {"_id": obj.id().clone().expect("No id inserted")},
            obj.as_document()?,
            None,
        )
        .await?;
    Ok(())
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
