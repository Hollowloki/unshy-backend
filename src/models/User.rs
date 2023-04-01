use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use validator::Validate;
use wither::Model as WitherModel;

use crate::utils::date::Date;

#[derive(Debug, Serialize, Deserialize, WitherModel, Validate)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub updated_at: Date,
    pub created_at: Date,
    pub locked_at: Option<Date>,

}