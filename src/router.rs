use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{create_session_handle, create_user_handle, get_all_todo_handle, post_todo_handle},
    AppState,
};

pub fn service(app_state: AppState) -> Router {
    Router::new()
        .route("/sign/:id", get(create_session_handle))
        .route("/user/create", post(create_user_handle))
        .route("/todo", post(post_todo_handle).get(get_all_todo_handle))
        .with_state(app_state)
}
