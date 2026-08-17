//! TemplateApp service: routes and handlers.

// tpl:if environments
pub mod config;

use axum::extract::State;
use config::Config;
// tpl:endif
use axum::{extract::Path, routing::get, Json, Router};
use serde_json::{json, Value};

// tpl:if environments
/// Builds the service router around the active profile.
pub fn app(config: Config) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/hello/{name}", get(hello))
        .with_state(config)
}

pub async fn healthz(State(config): State<Config>) -> Json<Value> {
    Json(json!({ "status": "ok", "environment": config.environment }))
}
// tpl:endif
// tpl:if !environments
/// Builds the service router.
pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/hello/{name}", get(hello))
}

pub async fn healthz() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
// tpl:endif

pub async fn hello(Path(name): Path<String>) -> Json<Value> {
    Json(json!({ "message": format!("Hello, {name}!") }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // tpl:if environments
    #[tokio::test]
    async fn healthz_reports_ok_and_the_active_profile() {
        let config = Config {
            environment: "staging".to_owned(),
            log_level: "INFO".to_owned(),
            verbose_errors: true,
            cache_seconds: 30,
        };

        let Json(body) = healthz(State(config)).await;

        assert_eq!(body["status"], "ok");
        assert_eq!(body["environment"], "staging");
    }
    // tpl:endif
    // tpl:if !environments
    #[tokio::test]
    async fn healthz_reports_ok() {
        let Json(body) = healthz().await;
        assert_eq!(body["status"], "ok");
    }
    // tpl:endif

    #[tokio::test]
    async fn hello_greets_by_name() {
        let Json(body) = hello(Path("Rust".to_string())).await;
        assert_eq!(body["message"], "Hello, Rust!");
    }
}
