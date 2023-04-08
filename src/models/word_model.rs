use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;
use wither::Model as WitherModel;

use crate::utils::models::ModelExt;

impl ModelExt for Word {
    type T = Word;
}

#[derive(Clone, Debug, Deserialize, Serialize, WitherModel, Validate)]

pub struct Word {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub word: String,
    pub definition: String,
    pub book_id: ObjectId,
    pub user_id: ObjectId,
    pub count: Option<i32>,
}

impl Word {
    pub fn new(word: String, definition: String, book_id: ObjectId, user_id: ObjectId) -> Self {
        Self {
            id: None,
            word,
            definition,
            book_id,
            user_id,
            count: None,
        }
    }
}
