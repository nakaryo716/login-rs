use axum::{response::IntoResponse, routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/", get(protected))
}

async fn protected() -> impl IntoResponse {
    todo!()
}
