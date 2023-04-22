use axum::Json;
use bson::oid::ObjectId;
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    errors::Error,
    models::word_log_model::WordLog,
    utils::{
        custom_response::{CustomResponse, CustomResponseBuilder},
        date::Date,
        models::ModelExt,
        token::TokenUser,
    },
};
#[derive(Serialize, Deserialize)]
pub struct WordLogRequest {
    word: String,
    definition: String,
}

pub async fn save_word_log(
    user: TokenUser,
    Json(log): Json<WordLogRequest>,
) -> Result<CustomResponse<WordLog>, Error> {
    let user_id = user.id;
    let word_log = WordLog::new(log.word, user_id, ObjectId::new(), Date::now());
    let word_log = WordLog::create(word_log).await?;
    let res = CustomResponseBuilder::new()
        .body(word_log)
        .status(StatusCode::CREATED)
        .build();
    Ok(res)
}
