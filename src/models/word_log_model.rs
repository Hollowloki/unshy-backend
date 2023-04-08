use crate::utils::{date::Date, models::ModelExt};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;
use wither::Model as WitherModel;

impl ModelExt for WordLog {
    type T = WordLog;
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, WitherModel)]
pub struct WordLog {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub word: String,
    pub user_id: ObjectId,
    pub book_id: ObjectId,
    pub date: Date,
}

impl WordLog {
    pub fn new(word: String, user_id: ObjectId, book_id: ObjectId, date: Date) -> Self {
        Self {
            id: None,
            word,
            user_id,
            book_id,
            date,
        }
    }
}
