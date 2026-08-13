# Templetry parent: rust

Rust templates for [Templetry](https://github.com/Templetry). One **parent repo**, multiple **forms** — each form is a subdirectory that compiles on its own and carries its own `template.yml` ([ADR-0011](https://github.com/Templetry/wiki/blob/main/adr/0011-template-forms.md)).

| Form | What it is | Status |
|---|---|---|
| [`cli/`](cli/) | CLI — clap derive, lib + bin split, unit tests | ✅ ready |
| [`axum-service/`](axum-service/) | HTTP service — axum + tokio, handler tests, optional Dockerfile | ✅ ready |

## Usage

```sh
templetry init rust/axum-service --out ./my-svc --set "project_name=My Service"
```

Forms are **chosen**, not combined. Inside a form, the manifest's features are freely combinable.
