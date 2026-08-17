//! TemplateApp CLI.

use clap::Parser;
// tpl:if environments
use std::path::Path;

use template_app::{config, greeting};
// tpl:endif
// tpl:if !environments
use template_app::greeting;
// tpl:endif

#[derive(Parser)]
#[command(name = "template-app", version, about = "TemplateApp CLI")]
struct Cli {
    /// Who to greet
    #[arg(short, long, default_value = "world")]
    name: String,
}

fn main() {
    let cli = Cli::parse();

    // tpl:if environments
    // No `env` subcommand: adding one would change the CLI's shape, which a
    // feature must not do. The profile changes what the existing run does —
    // diagnostics go to stderr, so piping stdout stays clean.
    if let Ok(config) = config::load(Path::new("."), None) {
        if config.verbose_errors {
            eprintln!("[{}] log level {}", config.environment, config.log_level);
        }
    }
    // tpl:endif

    println!("{}", greeting(&cli.name));
}
