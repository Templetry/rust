//! TemplateApp service: routes and handlers.

use axum::{extract::Path, routing::get, Json, Router};
use serde_json::{json, Value};

/// Builds the service router.
pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/hello/{name}", get(hello))
}

pub async fn healthz() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

pub async fn hello(Path(name): Path<String>) -> Json<Value> {
    Json(json!({ "message": format!("Hello, {name}!") }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn healthz_reports_ok() {
        let Json(body) = healthz().await;
        assert_eq!(body["status"], "ok");
    }

    #[tokio::test]
    async fn hello_greets_by_name() {
        let Json(body) = hello(Path("Rust".to_string())).await;
        assert_eq!(body["message"], "Hello, Rust!");
    }
}
