use clap::{CommandFactory, Parser};
use clap_complete::generate;
use mermaid_rs::cli::{Cli, Commands};
use mermaid_rs::core::MermaidError;

#[tokio::main]
async fn main() -> Result<(), MermaidError> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Completions(args) => {
            let mut cmd = Cli::command();
            generate(args.shell, &mut cmd, "mermaid", &mut std::io::stdout());
            Ok(())
        }
        Commands::ER(args) => mermaid_rs::cli::commands::er::run(args, &cli.global).await,
        Commands::Flowchart(args) => {
            mermaid_rs::cli::commands::flowchart::run(args, &cli.global).await
        }
        Commands::Journey(args) => mermaid_rs::cli::commands::journey::run(args, &cli.global).await,
        Commands::Mindmap(args) => mermaid_rs::cli::commands::mindmap::run(args, &cli.global).await,
        Commands::Pie(args) => mermaid_rs::cli::commands::pie::run(args, &cli.global).await,
        Commands::Render(args) => mermaid_rs::cli::commands::render::run(args, &cli.global).await,
        Commands::Requirement(args) => {
            mermaid_rs::cli::commands::requirement::run(args, &cli.global).await
        }
        Commands::Sequence(args) => {
            mermaid_rs::cli::commands::sequence::run(args, &cli.global).await
        }
        Commands::State(args) => mermaid_rs::cli::commands::state::run(args, &cli.global).await,
    }
}
