pub mod routes {
    use axum::{Router, routing::{post}, };
    use http::header;
    use tower_http::sensitive_headers::SetSensitiveHeadersLayer;

    use crate::controllers::{authentication::{register, authenticate_user}, books::create_book};
    
        pub async fn routes() -> Router
        {
            let authenticaiton = Router::new()
            .route("/register", post(register))
            .route("/login", post(authenticate_user))
            .route("/books", post(create_book))
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                header::AUTHORIZATION,
              )));
            return authenticaiton;
        }
}