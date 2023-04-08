use axum::extract::Json;

use crate::{
    errors::Error,
    models::books_model::{Book, PublicBook},
    utils::{
        custom_response::{CustomResponse, CustomResponseBuilder},
        models::ModelExt,
        token::TokenUser,
    },
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BookRequest {
    pub title: String,
    pub description: String,
}

pub async fn create_book(
    user: TokenUser,
    Json(book): Json<BookRequest>,
) -> Result<CustomResponse<PublicBook>, Error> {
    let user_id = user.id;
    let book = Book::new(book.title, book.description, user_id);
    let res = Book::create(book).await?;
    let res = PublicBook::from(res);
    let res = CustomResponseBuilder::new()
        .body(res)
        .status(StatusCode::CREATED)
        .build();
    Ok(res)
}
