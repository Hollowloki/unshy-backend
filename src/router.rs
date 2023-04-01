pub mod routes {
    use axum::{Router, routing::{post}, };

    use crate::controllers::{article::write_post, authentication::register};
    
        pub async fn routes() -> Router
        {
            let authenticaiton = Router::new()
            .route("/register", post(write_post))
            .route("/hello", post(register));
            return authenticaiton;
        }
}