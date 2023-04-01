pub mod article_model;
pub mod user_model;

use crate::utils::models::ModelExt;
use crate::errors::Error;

pub async fn sync_indexes() -> Result<(), Error> {
  user_model::User::sync_indexes().await?;
  article_model::Article::sync_indexes().await?;
  Ok(())
}
