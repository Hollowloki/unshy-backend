pub mod books_model;
pub mod user_model;
pub mod word_log_model;
pub mod word_model;

use crate::errors::Error;
use crate::utils::models::ModelExt;

pub async fn sync_indexes() -> Result<(), Error> {
    user_model::User::sync_indexes().await?;
    books_model::Book::sync_indexes().await?;
    Ok(())
}
