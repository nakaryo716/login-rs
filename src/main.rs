use repository::{SessionDb, TodoDb, UserDb};

mod handler;
mod repository;
mod router;
#[tokio::main]
async fn main() {
    let session_store = SessionDb::new();
    let user_db = UserDb::new();
    let todo_db = TodoDb::new();
    
    let app = router::service(user_db, session_store, todo_db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
