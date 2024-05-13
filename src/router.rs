use axum::{routing::{get, post}, Router};

use crate::{handler::{create_session_handle, create_user_handle, post_todo_handle}, repository::{SessionDb, TodoDb, UserDb}};

pub fn service(
    user_db: UserDb,
    session_db: SessionDb,
    todo_db: TodoDb,
) -> Router {
    Router::new()
        .route("/sign/:id", get(create_session_handle))
        .route("/user/create", post(create_user_handle))
        // .route("/todo", post(post_todo_handle))
        .with_state(user_db)
        .with_state(todo_db)
        .with_state(session_db)
}
