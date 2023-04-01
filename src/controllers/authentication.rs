use std::time::Instant;

use axum::{http::{StatusCode}, Json};
use bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::debug;

use crate::{errors::{Error, AuthenticateError}, utils::{custom_response::{CustomResponse, CustomResponseBuilder}, models::ModelExt, token}, models::user_model::{User, hash_password}};

#[derive(Deserialize, Serialize, Debug)]
pub struct Register {
    username: String,
    email: String,
    password: String,
}



pub async fn register(Json(credentials): Json<Register>) -> Result<CustomResponse<User>, Error> {
    let password_hash = hash_password(credentials.password).await?;
    let user = User::new(credentials.username, credentials.email, password_hash);
    let res = User::create(user).await?;
    let res = CustomResponseBuilder::new().body(res).status(StatusCode::CREATED).build();
    Ok(res)
}

pub async fn authenticate_user(Json(body): Json<AuthorizeBody>) -> Result<Json<AuthenticateResponse>, Error> {
    let email = &body.email;
    let password = &body.password;

    if email.is_empty() {
        debug!("Missing email, returning 400 status code");
        return Err(Error::bad_request());
      }
    
      if password.is_empty() {
        debug!("Missing password, returning 400 status code");
        return Err(Error::bad_request());
      }
      
      let user = User::find_one(doc! { "email": email }, None).await?;

      let user = match user {
        Some(user) => user,
        None => {
          debug!("User not found, returning 401");
          return Err(Error::not_found());
        }
      };

    
      let secret = "secret";
      let token = token::create(user.clone(), secret)
        .map_err(|_| Error::Authenticate(AuthenticateError::TokenCreation))?;
    
      let res = AuthenticateResponse {
        access_token: token,
        user: user.clone()
      };
    
      Ok(Json(res))
}

#[derive(Debug, Deserialize)]
pub struct AuthorizeBody {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateResponse {
  pub access_token: String,
  pub user: User,
}