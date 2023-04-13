use axum::extract::Json;
use bson::oid::ObjectId;
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    errors::Error,
    models::word_model::Word,
    utils::{
        custom_response::{CustomResponse, CustomResponseBuilder},
        models::ModelExt,
        token::TokenUser,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WordRequest {
    word: String,
    definition: String,
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
