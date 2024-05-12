use axum::{routing::get, Router};

use crate::{handler::create_session_handle, repository::{SessionDb, TodoDb, UserDb}};

pub fn service(
    user_db: UserDb,
    session_db: SessionDb,
    todo_db: TodoDb,
) -> Router {
    Router::new()
        .route("/sign", get(create_session_handle))
        .with_state(user_db)
        .with_state(todo_db)
        .with_state(session_db)
}
