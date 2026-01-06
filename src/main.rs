use clap::Parser;
use mermaid_rs::cli::{Cli, Commands};
use mermaid_rs::core::MermaidError;

#[tokio::main]
async fn main() -> Result<(), MermaidError> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Flowchart(args) => mermaid_rs::cli::commands::flowchart::run(args, &cli.global).await,
        Commands::Pie(args) => mermaid_rs::cli::commands::pie::run(args, &cli.global).await,
        Commands::Render(args) => mermaid_rs::cli::commands::render::run(args, &cli.global).await,
        Commands::Sequence(args) => mermaid_rs::cli::commands::sequence::run(args, &cli.global).await,
        Commands::State(args) => mermaid_rs::cli::commands::state::run(args, &cli.global).await,
    }
}
