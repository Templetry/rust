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

```sh templetry:checks
cargo build
cargo test
```

## Safe Change Workflow

1. Read the affected files fully before editing.
2. Make the smallest change that solves the task.
3. Build and test, then review the diff with git before committing.

## This project came from a template

Four facts you cannot infer from the code in front of you:

- **Never hand-edit `.templetry-answers.yml`.** It records what generated this project. Editing it makes the next update merge against a state that never existed.
- **Before writing a capability by hand, run `templetry pieces`.** Auth, RBAC, audit trails, API keys and whole CRUD resources may already exist as pieces for this template. Adopting one is `templetry add <name>`, and it brings its own tests.
- **`templetry update` pulls improvements from the template** through a three-way merge that keeps your edits. Use it instead of copying files from the template by hand.
- **Directives like `tpl:if` belong to the template, not here.** If you find one in this project, it is a rendering bug worth reporting — do not try to interpret it.
