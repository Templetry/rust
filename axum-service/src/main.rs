//! TemplateApp service entry point.

use template_app::app;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind");
    println!("template-app listening on {addr}");
    axum::serve(listener, app()).await.expect("server error");
}
