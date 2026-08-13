# AGENTS

Operating contract for AI agents and automation helpers working in this project.

## Mission

- Keep the split honest: logic in `src/lib.rs` (testable), argument parsing in `src/main.rs`.

## Core Rules

- Errors surface as `Result`; `main` is the only place that may print and exit.
- Every public function in the library has unit tests in its module's `mod tests`.
- Prefer the standard library; add a crate only when it removes real complexity.
- Update docs in the same change when behavior or process changes.

## Required Checks Before Finishing

- `cargo build` and `cargo test` pass.
- `cargo fmt --check` and `cargo clippy` clean.

## Safe Change Workflow

1. Read the affected files fully before editing.
2. Make the smallest change that solves the task.
3. Build and test, then review the diff with git before committing.
