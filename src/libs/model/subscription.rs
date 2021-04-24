use serde::{Deserialize, Serialize};
use crate::libs::db::{self, Model};
use mongodb::{bson::oid, Collection};
use async_trait::async_trait;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubType {
    BASIC
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscription {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<oid::ObjectId>,
    s_type: SubType,
}

impl Subscription {
    pub fn new() -> Subscription {
        Subscription {
            id: None,
            s_type: SubType::BASIC
        }
    }
}

#[async_trait]
impl Model for Subscription {
    fn id(&self) -> Option<oid::ObjectId> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Option<oid::ObjectId>) {
        self.id = id;
    }

    async fn collection() -> Result<Collection> {
        Ok(db::get_connection().await?.collection("subscriptions"))
    }
}