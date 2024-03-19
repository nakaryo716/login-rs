use axum::{routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/", get(hello))
}

async fn hello() -> String {
    "Hello".to_string()
}
