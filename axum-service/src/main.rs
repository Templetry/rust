//! TemplateApp service entry point.

// tpl:if environments
use std::path::Path;

use template_app::{app, config};
// tpl:endif
// tpl:if !environments
use template_app::app;
// tpl:endif

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{port}");

    // tpl:if environments
    // A broken profile stops the service here, before it accepts a request.
    let config = match config::load(Path::new("."), None) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };
    println!(
        "template-app starting in {} (log level {})",
        config.environment, config.log_level
    );
    let router = app(config);
    // tpl:endif
    // tpl:if !environments
    let router = app();
    // tpl:endif

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind");
    println!("template-app listening on {addr}");
    axum::serve(listener, router).await.expect("server error");
}
