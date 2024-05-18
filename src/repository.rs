use std::{
    collections::HashMap,
    error::Error,
    sync::{atomic::AtomicI32, Arc, Mutex},
};

use axum::extract::FromRef;
use serde::{Deserialize, Serialize};

static USER_ID_PROVIDER: AtomicI32 = AtomicI32::new(0);

// ユーザー型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
}

// 新規作成をする際の型
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub user_name: String,
}

// ユーザ情報を保存するDb
#[derive(Debug, Clone)]
pub struct UserDb {
    pub pool: Arc<Mutex<HashMap<String, User>>>,
}

impl UserDb {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }

    pub fn create_user(&self, payload: CreateUser) -> Result<User, Box<dyn Error>> {
        let created_id = USER_ID_PROVIDER.load(std::sync::atomic::Ordering::Relaxed);
        let created_s_id = created_id.to_string();

        let user = User {
            user_id: created_s_id.clone(),
            user_name: payload.user_name,
        };

        println!("{:?}", user);

        {
            let mut db = self.pool.lock().unwrap();
            db.insert(created_s_id, user.clone());
        }

        USER_ID_PROVIDER.store(created_id + 1, std::sync::atomic::Ordering::Relaxed);
        Ok(user)
    }

    pub fn get_user_by_id(&self, id: String) -> Result<Option<User>, Box<dyn Error>> {
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

    pub fn insert_session_info(
        &self,
        session_id: String,
        user_data: User,
    ) -> Result<(), Box<dyn Error>> {
        let mut db = self.pool.lock().unwrap();
        db.insert(session_id, user_data);
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

static TODO_ID_PROVIDER: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    todo_id: i32,
    user_id: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTodo {
    text: String,
}

//　Todoを格納するためのDB
#[derive(Debug, Clone)]
pub struct TodoDb {
    pub pool: Arc<Mutex<HashMap<i32, Todo>>>,
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
            let db = self.pool.lock().unwrap();
            todo = db.iter().map(|(_id, todo)| todo.clone()).collect();
        }
        Ok(todo)
    }
}


#[derive(Debug, Clone)]
pub struct AppState {
    pub user_db: UserDb,
    pub session_db: SessionDb,
    pub todo_db: TodoDb,
}

// RouterでStateを付与する際にAppStateを渡すのではなく、
// State<UserDb>のようにするためにFromRefトレイトを実装
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
