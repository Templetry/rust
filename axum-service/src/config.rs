//! The active environment profile.
//!
//! Rust has no blessed mechanism for this, so the convention is one
//! `.env.<profile>` at the crate root, selected by `APP_ENV`, parsed into a
//! validated struct. Nothing else in the crate reads `std::env` for settings.

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;

/// The closed set of profiles (ADR-0018).
pub const ENVIRONMENTS: [&str; 3] = ["development", "staging", "production"];

/// What the application reads. `Clone` because axum stores it as router state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub environment: String,
    pub log_level: String,
    pub verbose_errors: bool,
    pub cache_seconds: u32,
}

/// A profile that is missing, unreadable or nonsensical.
#[derive(Debug, PartialEq, Eq)]
pub struct ConfigError(pub String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "config: {}", self.0)
    }
}

impl std::error::Error for ConfigError {}

/// Loads a profile by name, or the one `APP_ENV` selects.
///
/// `.env.local` layers on top when present and is gitignored; a real
/// environment variable beats both, which is what lets a container run with
/// no profile file in the image.
pub fn load(root: &Path, profile: Option<&str>) -> Result<Config, ConfigError> {
    let profile = profile
        .map(str::to_owned)
        .or_else(|| std::env::var("APP_ENV").ok())
        .unwrap_or_else(|| "development".to_owned());

    let mut values = HashMap::new();
    for name in [format!(".env.{profile}"), ".env.local".to_owned()] {
        read_into(&mut values, &root.join(name))?;
    }
    for key in ["ENVIRONMENT", "LOG_LEVEL", "VERBOSE_ERRORS", "CACHE_SECONDS"] {
        if let Ok(value) = std::env::var(key) {
            values.insert(key.to_owned(), value);
        }
    }

    let environment = required(&values, "ENVIRONMENT", &profile)?;
    if !ENVIRONMENTS.contains(&environment.as_str()) {
        return Err(ConfigError(format!(
            "unknown ENVIRONMENT {environment:?} (want one of {ENVIRONMENTS:?})"
        )));
    }
    let raw_cache = required(&values, "CACHE_SECONDS", &profile)?;
    let cache_seconds = raw_cache.parse::<u32>().map_err(|_| {
        ConfigError(format!("CACHE_SECONDS must be a non-negative integer, got {raw_cache:?}"))
    })?;

    Ok(Config {
        environment,
        log_level: required(&values, "LOG_LEVEL", &profile)?,
        verbose_errors: values.get("VERBOSE_ERRORS").map(String::as_str) == Some("true"),
        cache_seconds,
    })
}

fn required(values: &HashMap<String, String>, key: &str, profile: &str) -> Result<String, ConfigError> {
    match values.get(key) {
        Some(value) if !value.is_empty() => Ok(value.clone()),
        _ => Err(ConfigError(format!("{key} is missing from profile {profile:?}"))),
    }
}

/// Parses `KEY=VALUE` lines. A missing file is not an error: only the
/// selected profile has to exist, and `.env.local` usually does not.
fn read_into(values: &mut HashMap<String, String>, path: &Path) -> Result<(), ConfigError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(err) => return Err(ConfigError(format!("reading {}: {err}", path.display()))),
    };

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            values.insert(
                key.trim().to_owned(),
                value.trim().trim_matches(['"', '\'']).to_owned(),
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> &'static Path {
        // The profiles live at the crate root, next to Cargo.toml.
        Path::new(env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn each_profile_declares_its_own_name() {
        for name in ENVIRONMENTS {
            let config = load(root(), Some(name)).expect("profile should load");
            assert_eq!(config.environment, name);
        }
    }

    #[test]
    fn profiles_differ_where_it_matters() {
        // Staging exists to be production-like while still debuggable, so it
        // is the one profile whose values must not equal either neighbour's.
        for (profile, verbose, cache) in [
            ("development", true, 0),
            ("staging", true, 30),
            ("production", false, 300),
        ] {
            let config = load(root(), Some(profile)).expect("profile should load");
            assert_eq!(config.verbose_errors, verbose, "{profile} verbose_errors");
            assert_eq!(config.cache_seconds, cache, "{profile} cache_seconds");
        }
    }

    #[test]
    fn an_unknown_profile_fails_loudly() {
        assert!(load(root(), Some("qa")).is_err());
    }
}
