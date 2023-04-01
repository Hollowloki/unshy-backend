use axum::{Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::time::Instant;
use crate::{models::article::{Article}, utils::models::ModelExt, errors::Error};

#[derive(Deserialize, Serialize)]
pub struct ArticleWriteRequest {
    title: String,
    abstracts: String,
    content: String,
}

pub async fn write_post( Json(article): Json<ArticleWriteRequest>) -> Result<Json<Value>, Error>
{
  let user = Article::new(article.title, article.abstracts, article.content);

  
  let now = Instant::now();

  let user = Article::create(user).await?;

  let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

  return Ok(Json(json!({"message": user})));

}
