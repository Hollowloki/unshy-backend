use bson::{
    oid::ObjectId,
    serde_helpers::{bson_datetime_as_rfc3339_string, serialize_object_id_as_hex_string},
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use wither::Model as WitherModel;

use crate::utils::{
    date::{self, Date},
    models::ModelExt,
};

#[derive(Serialize, Deserialize, Debug, WitherModel, Validate)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub title: String,
    pub description: String,
    pub updated_at: Date,
    pub created_at: Date,
}

impl ModelExt for Book {
    type T = Book;
}

impl Book {
    pub fn new<T, D, I>(title: T, description: D, user_id: I) -> Self
    where
        T: Into<String>,
        D: Into<String>,
        I: Into<ObjectId>,
    {
        let now = date::now();
        Self {
            id: None,
            user_id: user_id.into(),
            title: title.into(),
            description: description.into(),
            updated_at: now,
            created_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicBook {
    #[serde(alias = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub user_id: ObjectId,
    pub title: String,
    pub description: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: Date,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: Date,
}

impl From<Book> for PublicBook {
    fn from(book: Book) -> Self {
        Self {
            id: book.id.unwrap(),
            user_id: book.user_id,
            title: book.title,
            description: book.description,
            updated_at: book.updated_at,
            created_at: book.created_at,
        }
    }
}
