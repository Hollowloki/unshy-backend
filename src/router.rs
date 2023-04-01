pub mod routes {
    use axum::{Router, routing::{post}, };

    use crate::controllers::{article::write_post, authentication::{register, authenticate_user}};
    
        pub async fn routes() -> Router
        {
            let authenticaiton = Router::new()
            .route("/article", post(write_post))
            .route("/register", post(register))
            .route("/login", post(authenticate_user));
            return authenticaiton;
        }
}