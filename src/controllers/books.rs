use axum::{Router};
use axum::extract::Json;
use bson::oid::ObjectId;
use http::{StatusCode, HeaderMap};
use serde::{Deserialize, Serialize};
use crate::{utils::{custom_response::{CustomResponseBuilder, CustomResponse}, models::ModelExt, token::TokenUser}, models::{books_model::{Book, PublicBook}, user_model::User}, errors::Error};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BookRequest {
    pub title: String,
    pub description: String,
}

pub async fn create_book(user: TokenUser, Json(book): Json<BookRequest>) -> Result<CustomResponse<PublicBook>, Error> {
    let user_id = user.id;
    let book = Book::new(book.title, book.description, user_id);
    let res = Book::create(book).await?;
    let res = PublicBook::from(res);
    let res = CustomResponseBuilder::new().body(res).status(StatusCode::CREATED).build();
    Ok(res)
}