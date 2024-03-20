use axum::Router;
use axum_login::{login_required, tower_sessions::ExpiredDeletion, AuthManagerLayerBuilder};
use cookie::Key;
use std::{env, error::Error};
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

use crate::{users::Backend, web::protected};

pub struct App {
    db: sqlx::PgPool,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let db = sqlx::PgPool::connect(&env::var("DATABASE_URL")?).await?;

        Ok(Self { db })
    }

    pub async fn serve(self) -> Result<Router, Box<dyn Error>> {
        let session_store = PostgresStore::new(self.db.clone());

        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        let key = Key::generate();

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(true)
            .with_expiry(Expiry::OnInactivity(time::Duration::days(1)))
            .with_signed(key);

        let backend = Backend::new(self.db);
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let app = protected::router()
            .route_layer(login_required!(Backend, login_url = "/login"))
            .merge(other)
            .layer(auth_layer);

        deletion_task.await??;
        Ok(app)
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
