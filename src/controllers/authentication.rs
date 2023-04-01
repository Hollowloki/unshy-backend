use axum::{http::{StatusCode}, Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    username_or_email: String,
    password: String,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    username: String,
    email: String,
    password: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    user: User,
    exp: usize,
}



pub async fn register(_credentials: Option<Json<User>>) -> Result<Json<Value>, StatusCode> {
    return Ok(Json(json!({"message": "Hello World"})));
}
