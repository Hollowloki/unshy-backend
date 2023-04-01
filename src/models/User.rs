use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use tokio::task;
use validator::Validate;
use wither::Model as WitherModel;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::{utils::date::Date, errors::Error};

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

    pub fn is_password_match(self, password: String) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }
  }


  pub async fn hash_password<P>(password: P) -> Result<String, Error>
where
  P: AsRef<str> + Send + 'static,
{
  // TODO: Hash password with salt.
  // https://docs.rs/bcrypt/latest/bcrypt/fn.hash_with_salt.html
  #[cfg(not(test))]
  let cost = bcrypt::DEFAULT_COST;
  #[cfg(test)]
  let cost = 4;
  task::spawn_blocking(move || bcrypt::hash(password.as_ref(), cost))
    .await
    .map_err(Error::RunSyncTask)?
    .map_err(Error::HashPassword)
}