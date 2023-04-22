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
        wordlog::save_word_log,
        words::{index, save_word},
    };

    pub async fn routes() -> Router {
        let authenticaiton = Router::new()
            .route("/register", post(register))
            .route("/login", post(authenticate_user))
            .route("/words", post(save_word))
            .route("/books", post(create_book))
            .route("/words", get(index))
            .route("/wordlog", post(save_word_log))
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                header::AUTHORIZATION,
            )));
        return authenticaiton;
    }
}
