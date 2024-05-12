use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use cookie::Cookie;
use password_auth::generate_hash;
use time::Duration;

use crate::repository::{CreateTodo, CreateUser, SessionDb, TodoDb, UserDb};

// 新規ユーザー作成
pub async fn create_user_handle(
    Json(payload): Json<CreateUser>,
    State(user_db): State<UserDb>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = user_db
        .create_user(payload)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(user)))
}

const SESSION_ID_KEY: &str = "text_session_id";

// セッション作成(ログイン)
pub async fn create_session_handle(
    Path(user_id): Path<i32>,
    State(user_db): State<UserDb>,
    State(session_db): State<SessionDb>,
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    // dbからユーザがあるかをidで認証
    let unchecked_user = user_db
        .get_user_by_id(user_id)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = match unchecked_user {
        Some(user) => user,
        None => {
            return Err(StatusCode::NOT_FOUND);
        },
    };

    // session_idの作成
    let random_id = cookie::Key::generate().signing().to_owned();
    let session_id = generate_hash(&random_id);

    // sessionを保存
    session_db
        .insert_session_info(session_id.clone(), user)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie = Cookie::build((SESSION_ID_KEY, session_id))
        .max_age(Duration::seconds(120))
        .secure(false)
        .http_only(false);

    Ok((jar.add(cookie), StatusCode::OK))
}

// Todoの作成
pub async fn post_todo_handle(
    Json(payload): Json<CreateTodo>,
    State(todo_db): State<TodoDb>,
    State(session_db): State<SessionDb>,
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> { 
    // jarからのcookie取り出し
    let cookie_value = match jar.get(SESSION_ID_KEY) {
        Some(session_id) => session_id.value(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    
    // 取り出したcookie valueと保存されているsession_idがあるかを見る
    // なかったらガード
    let uncheked_auth_user = session_db
    .get_session(cookie_value.to_string())
    .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = match uncheked_auth_user {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // TODOデータベースに保存
    let todo = todo_db
    .create_todo(payload, user)
    .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn get_all_todo_handle(
    State(todo_db): State<TodoDb>,
) -> Result<impl IntoResponse, StatusCode> {
    let todos = todo_db
        .read_todo()
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(todos)))
}


