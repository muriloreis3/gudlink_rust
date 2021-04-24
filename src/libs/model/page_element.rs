use serde::{Deserialize, Serialize};
use super::folder::Folder;
use super::link::Link;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PageElement {
    Folder(Folder),
    Link(Link),
}