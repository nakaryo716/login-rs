mod app;
mod users;
mod web;

#[tokio::main]
async fn main() {
    let app = app::app();

    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listner, app).await.unwrap();
}
