pub mod routes {
    use axum::{
        routing::{get, post},
        Router,
    };
    use http::header;
    use tower_http::sensitive_headers::SetSensitiveHeadersLayer;

    use crate::controllers::{
        authentication::{authenticate_user, register},
        books::create_book,
        words::save_word,
    };

    pub async fn routes() -> Router {
        let authenticaiton = Router::new()
            .route("/register", post(register))
            .route("/login", post(authenticate_user))
            .route("/words", post(save_word))
            .route("/books", post(create_book))
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                header::AUTHORIZATION,
            )));
        return authenticaiton;
    }
}
