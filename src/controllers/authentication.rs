use std::time::Instant;

use axum::{http::{StatusCode}, Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{errors::Error, utils::{custom_response::{CustomResponse, CustomResponseBuilder}, models::ModelExt}, models::user_model::{User, hash_password}};

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
