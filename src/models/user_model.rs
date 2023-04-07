use bson::serde_helpers::{serialize_object_id_as_hex_string, bson_datetime_as_rfc3339_string};
use serde::{Serialize, Deserialize};
use tokio::task;
use validator::Validate;
use wither::Model as WitherModel;
use wither::bson::{doc, oid::ObjectId};



use crate::{utils::{date::Date, models::ModelExt}, errors::Error};


impl ModelExt for User {
    type T = User;
}

#[derive(Debug, Serialize, Deserialize, WitherModel, Validate,Clone)]
#[model(index(keys=r#"doc!{"email": 1}"#, options=r#"doc!{"unique": true}"#))]
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



impl User {
    pub fn new<N, E, P>(name: N, email: E, password: P) -> Self
    where
      N: Into<String>,
      E: Into<String>,
      P: Into<String>,
    {
      Self {
        id: None,
        name: name.into(),
        email: email.into(),
        password: password.into(),
        updated_at: Date::now(),
        created_at: Date::now(),
        locked_at: None,
      }
    }

    pub fn is_password_match(self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PublicUser {
    #[serde(alias = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: Date,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: Date,
  }
  
  impl From<User> for PublicUser {
    fn from(user: User) -> Self {
      Self {
        id: user.id.unwrap(),
        name: user.name.clone(),
        email: user.email.clone(),
        updated_at: user.updated_at,
        created_at: user.created_at,
      }
    }
  }

  pub async fn hash_password<P>(password: P) -> Result<String, Error>
where
  P: AsRef<str> + Send + 'static,
{
  // TODO: Hash password with salt.
  // https://docs.rs/bcrypt/latest/bcrypt/fn.hash_with_salt.html
  let cost = 4;
  task::spawn_blocking(move || bcrypt::hash(password.as_ref(), cost))
    .await
    .map_err(Error::RunSyncTask)?
    .map_err(Error::HashPassword)
}