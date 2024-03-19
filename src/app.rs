use std::{env, error::Error};

use axum::{routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/", get(hello))
}

async fn hello() -> String {
    "Hello".to_string()
}



pub struct App {
    db: sqlx::PgPool,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        
        let db = sqlx::PgPool::connect(&env::var("DATABASE_URL")?).await?;
        
        Ok(Self { db })
    }

}

// Todo that I shoul implment
// define Database Struct

// implment for Database Struct 
    // fn new()
    
    // fn serve(self)
    // create session-store by using database pool clone
    
    // define delete-sore task
    // define session-store (settings)
    
    // create backend struct(instance)
    // establish session layer 
    
    // routing app
    // top
    // ---route_layer (checking auth)---
    // merge
    // (message-layer)
    // (auth-layer)
    
    
// define async fn shut down signal()