# AGENTS

Operating contract for AI agents and automation helpers working in this project.

## Mission

- Keep the service focused: axum + tokio; reach for tower middleware or a database crate only when the service actually needs it.

## Core Rules

- Routes are declared in `app()` in `src/lib.rs`; handlers stay small and return `Json<Value>` (or typed structs with serde).
- `src/main.rs` is a thin shell: bind the port from env, serve `app()`.
- Every handler gets a `#[tokio::test]` in the library's `mod tests`.
- Update docs in the same change when behavior or process changes.

## Required Checks Before Finishing

- `cargo build` and `cargo test` pass.
- `cargo fmt --check` and `cargo clippy` clean.

## Safe Change Workflow

1. Read the affected files fully before editing.
2. Make the smallest change that solves the task.
3. Build and test, then review the diff with git before committing.
