//! TemplateApp library: the logic the CLI is a shell over.

/// Builds the message the CLI prints.
pub fn greeting(name: &str) -> String {
    let name = if name.is_empty() { "world" } else { name };
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        assert_eq!(greeting("Rust"), "Hello, Rust!");
    }

    #[test]
    fn empty_name_falls_back_to_world() {
        assert_eq!(greeting(""), "Hello, world!");
    }
}
