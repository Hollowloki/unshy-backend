use crate::{
    errors::Error,
    models::word_model::Word,
    utils::{
        custom_response::{CustomResponse, CustomResponseBuilder},
        models::ModelExt,
        token::TokenUser,
    },
};
use axum::extract::Json;
use bson::{doc, oid::ObjectId};
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WordRequest {
    word: String,
    definition: String,
    book_id: i32,
}

pub async fn save_word(
    user: TokenUser,
    Json(context): Json<WordRequest>,
) -> Result<CustomResponse<Word>, Error> {
    let user_id = user.id;
    let word = Word::new(context.word, context.definition, ObjectId::new(), user_id);
    let word = Word::create(word).await?;
    let res = CustomResponseBuilder::new()
        .body(word)
        .status(StatusCode::CREATED)
        .build();
    Ok(res)
}

pub async fn index(user: TokenUser) -> Result<CustomResponse<Vec<Word>>, Error> {
    let words = Word::find(doc! { "user_id": user.id }, None).await?;
    let res = CustomResponseBuilder::new()
        .body(words)
        .status(StatusCode::OK)
        .build();
    return Ok(res);
}
