use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utils::models::ModelExt;
use wither::{Model as WitherModel, bson::oid::ObjectId};




#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Category {
    name: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Comment {
    user_id: usize,
    post_id: usize,
    text: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pallete {
    color: Colors
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Colors {
    Blue,
    Black,
    Pink
}



#[derive(Debug, Clone, Serialize, Deserialize, WitherModel, Validate)]
pub struct Article {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub abstracts: String,
    pub content: String,
}
impl ModelExt for Article {
    type T = Article;
}



impl Article {
    pub fn new<A, B, C>(title: A, abstracts: B, content: C) -> Self
    where
      A: Into<String>,
      B: Into<String>,
      C: Into<String>,
    {
      Self {
        id: None,
        title: title.into(),
        abstracts: abstracts.into(),
        content: content.into(),
      }
    }
  }