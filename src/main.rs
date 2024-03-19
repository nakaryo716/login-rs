use axum_login::tracing::info;
use tracing_subscriber::filter::LevelFilter;

mod app;
mod users;
mod web;

#[tokio::main]
async fn main() {
    // setting log level
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();

    // routing
    let app = app::app();
    
    // listner bind
    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();
    info!("listening on {:?}", &listner);

    // remember to set shutdou signal 
    axum::serve(listner, app).await.unwrap();
}
