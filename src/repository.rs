use std::{
    collections::HashMap, error::Error, sync::{atomic::AtomicI32, Arc, Mutex}
};

use axum::extract::FromRef;
use serde::{Deserialize, Serialize};

static USER_ID_PROVIDER: AtomicI32 = AtomicI32::new(0);

// ユーザー
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
}

// 新規作成
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub user_name: String,
}

// ユーザデータベース
#[derive(Debug, Clone)]
pub struct UserDb {
    pub pool: Arc<Mutex<HashMap<i32, User>>>,
}

impl UserDb {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }

    pub fn create_user(&self, payload: CreateUser) -> Result<User, Box<dyn Error>> {
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
        Ok(user)
    }

    pub fn get_user_by_id(&self, id: i32) -> Result<Option<User>, Box<dyn Error>> {
        let optional_user;
        {
            let db = self.pool.lock().unwrap();
            optional_user = db.get(&id).map(|e| e.clone());
        }

        Ok(optional_user)
    }
}

// セッション管理するためのDB
#[derive(Debug, Clone)]
pub struct SessionDb {
    pub pool: Arc<Mutex<HashMap<String, User>>>,
}

impl SessionDb {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }

    pub fn insert_session_info(&self, session_id: String, user_data: User) -> Result<(), Box<dyn Error>>{
        let mut db = self.pool.lock().unwrap();
        db.insert(session_id, user_data).unwrap();
        Ok(())
    }

    pub fn get_session(&self, session_id: String) -> Result<Option<User>, Box<dyn Error>> {
        let user;
        {
            let db = self.pool.lock().unwrap();
            user = db.get(&session_id).map(|user| user.clone());
        }
        Ok(user)
    }
}

impl FromRef<UserDb> for SessionDb {
    fn from_ref(input: &UserDb) -> Self {
        let pool = input.pool.clone();
        SessionDb {
           pool,
        }
    }
}

static TODO_ID_PROVIDER: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    todo_id: i32,
    user_id: i32,
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTodo {
    user_id: i32,
    text: String,
}

//　Todoを格納するためのDB
#[derive(Debug, Clone)]
pub struct TodoDb {
    pool: Arc<Mutex<HashMap<i32, Todo>>>,
}

impl TodoDb {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }

    pub fn create_todo(&self, payload: CreateTodo, user: User) -> Result<Todo, Box<dyn Error>> {
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
        Ok(todo)
    }

    pub fn read_todo(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let todo: Vec<Todo>;
        {
            let db  = self.pool.lock().unwrap();
            todo = db.iter().map(|(_id, todo)| todo.clone()).collect();
        }
        Ok(todo)
    }
}
