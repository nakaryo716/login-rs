use axum::extract::FromRef;
use repository::{SessionDb, TodoDb, UserDb};

mod handler;
mod repository;
mod router;
#[tokio::main]
async fn main() {
    let session_db = SessionDb::new();
    let user_db = UserDb::new();
    let todo_db = TodoDb::new();

    let app_state = AppState {
        user_db,
        session_db,
        todo_db,
    };

    let app = router::service(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub user_db: UserDb,
    pub session_db: SessionDb,
    pub todo_db: TodoDb,
}

impl FromRef<AppState> for UserDb {
    fn from_ref(input: &AppState) -> Self {
        UserDb {
            pool: input.user_db.pool.clone(),
        }
    }
}

impl FromRef<AppState> for SessionDb {
    fn from_ref(input: &AppState) -> Self {
        SessionDb {
            pool: input.session_db.pool.clone(),
        }
    }
}

impl FromRef<AppState> for TodoDb {
    fn from_ref(input: &AppState) -> Self {
        TodoDb {
            pool: input.todo_db.pool.clone(),
        }
    }
}
