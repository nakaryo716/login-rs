use repository::{AppState, SessionDb, TodoDb, UserDb};

mod handler;
mod repository;
mod router;
#[tokio::main]
async fn main() {
    //各種データベース(今回はメモリ上)のコネクションpoolを作成
    let session_db = SessionDb::new();
    let user_db = UserDb::new();
    let todo_db = TodoDb::new();

    // Stateとして一つにまとめる
    let app_state = AppState {
        user_db,
        session_db,
        todo_db,
    };

    let app = router::service(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
