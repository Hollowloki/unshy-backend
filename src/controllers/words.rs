use axum::extract::Json;
use bson::oid::ObjectId;

use crate::{
    errors::Error,
    models::word_model::Word,
    utils::{models::ModelExt, token::TokenUser},
};

pub struct WordRequest {
    word: String,
    definition: String,
}

pub async fn save_word(user: TokenUser, Json(context): Json<WordRequest>) -> Result<Word, Error> {
    let user_id = user.id;
    let word = Word::new(context.word, context.definition, ObjectId::new(), user_id);
    let word = Word::create(word).await?;
    Ok(word)
}
