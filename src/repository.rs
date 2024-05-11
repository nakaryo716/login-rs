use std::{
    collections::HashMap,
    sync::{atomic::AtomicI32, Arc, Mutex},
};

use serde::{Deserialize, Serialize};

static USER_ID_PROVIDER: AtomicI32 = AtomicI32::new(0);

// ユーザー
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    user_id: i32,
    user_name: String,
}

// 新規作成
#[derive(Debug, Clone, Deserialize)]
struct CreateUser {
    user_name: String,
}

// ユーザデータベース
#[derive(Debug, Clone)]
pub struct UserDb {
    pool: Arc<Mutex<HashMap<i32, User>>>,
}

impl UserDb {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }

    pub fn create_user(&self, payload: CreateUser) -> User {
        let created_id = USER_ID_PROVIDER.load(std::sync::atomic::Ordering::Relaxed);
        let user = User {
            user_id: created_id,
            user_name: payload.user_name,
        };

        {
            let mut db = self.pool.lock().unwrap();
            db.insert(user.user_id, user.clone()).unwrap();
        }

        USER_ID_PROVIDER.store(created_id + 1, std::sync::atomic::Ordering::Relaxed);
        user
    }
}

// セッション管理するためのDB
#[derive(Debug, Clone)]
struct SessionDb {
    pub pool: Arc<Mutex<HashMap<String, String>>>,
}

impl SessionDb {
    fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }
}

static TODO_ID_PROVIDER: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    todo_id: i32,
    user_id: i32,
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CreateTodo {
    user_id: i32,
    text: String,
}

//　Todoを格納するためのDB
struct TodoDb {
    pool: Arc<Mutex<HashMap<i32, Todo>>>,
}

impl TodoDb {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }

    pub fn create_todo(&self, payload: CreateTodo, user: User) -> Todo {
        let created_id = TODO_ID_PROVIDER.load(std::sync::atomic::Ordering::Relaxed);

        let todo = Todo {
            todo_id: created_id,
            user_id: user.user_id,
            text: payload.text,
        };

        {
            let mut db = self.pool.lock().unwrap();
            db.insert(todo.todo_id, todo.clone());
        }

        TODO_ID_PROVIDER.store(created_id + 1, std::sync::atomic::Ordering::Relaxed);
        todo
    }
}
