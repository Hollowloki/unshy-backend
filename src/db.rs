use async_once::AsyncOnce;
use lazy_static::lazy_static;
use self::mongodb::Database;
use wither::mongodb;

lazy_static! {
    pub static ref CONNECTION: AsyncOnce<Database> = AsyncOnce::new(async {
      let db_uri = "mongodb://localhost:27017";
      let db_name = "r_blog";
      mongodb::Client::with_uri_str(db_uri)
        .await
        .expect("Failed to initialize MongoDB connection")
        .database(db_name)
    });
  }