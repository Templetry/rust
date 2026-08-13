# TemplateApp

Rust HTTP service generated with [Templetry](https://github.com/Templetry): axum + tokio, health endpoint, handler tests, optional Dockerfile.

```sh
cargo run                  # listens on :8080 (PORT overrides)
cargo test
docker build -t template-app .   # docker feature
```

Routes: `GET /healthz` · `GET /api/hello/{name}`.
