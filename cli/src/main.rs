//! TemplateApp CLI.

use clap::Parser;
use template_app::greeting;

#[derive(Parser)]
#[command(name = "template-app", version, about = "TemplateApp CLI")]
struct Cli {
    /// Who to greet
    #[arg(short, long, default_value = "world")]
    name: String,
}

fn main() {
    let cli = Cli::parse();
    println!("{}", greeting(&cli.name));
}
