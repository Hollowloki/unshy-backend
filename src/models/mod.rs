pub mod user_model;
pub mod books_model;

use crate::utils::models::ModelExt;
use crate::errors::Error;

pub async fn sync_indexes() -> Result<(), Error> {
  user_model::User::sync_indexes().await?;
  books_model::Book::sync_indexes().await?;
  Ok(())
}
