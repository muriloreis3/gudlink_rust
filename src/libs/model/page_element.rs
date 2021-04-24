use super::folder::Folder;
use super::link::Link;
use mongodb::bson::{self, doc, oid};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use async_recursion::async_recursion;

#[derive(Serialize, Deserialize, Debug)]
enum IntermediateElement {
    Folder(oid::ObjectId),
    Link(oid::ObjectId),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PageElement {
    Folder(Folder),
    Link(Link),
}

impl PageElement {
    #[async_recursion(?Send)]
    pub async fn get_elements(document: &bson::Bson) -> Result<bson::Bson> {
        let int_elements: Vec<IntermediateElement> = bson::from_bson(document.to_owned()).unwrap();
        let mut elements = vec![];
        for el in int_elements {
            elements.push(match el {
                IntermediateElement::Folder(f) => PageElement::Folder(Folder::find_by_id(f).await?),
                IntermediateElement::Link(l) => PageElement::Link(Link::find_by_id(l).await?),
            });
        }
        Ok(bson::to_bson(&elements).unwrap())
    }
}
