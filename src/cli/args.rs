use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;
use std::path::PathBuf;

use super::commands;
use crate::core::Mode;

#[derive(Parser)]
#[command(
    name = "mermaid",
    about = "A CLI tool for generating Mermaid diagrams programmatically",
    version = env!("GIT_DESCRIBE"),
    after_help = "For more information, visit: https://github.com/scottidler/mermaid-rs"
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOptions,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug, Clone)]
pub struct GlobalOptions {
    /// Mermaid.ink server URL
    #[arg(
        short,
        long,
        env = "MERMAID_INK_SERVER",
        default_value = "https://mermaid.ink",
        global = true
    )]
    pub server: String,

    /// Display mode (affects theme and background)
    #[arg(long, default_value = "dark", global = true)]
    pub mode: Mode,

    /// Diagram theme
    #[arg(short, long, default_value = "default", global = true)]
    pub theme: String,

    /// Output file path (extension determines format: .svg, .png, .mmd)
    #[arg(short, long, global = true)]
    pub output: Option<PathBuf>,

    /// Write to stdout instead of file
    #[arg(long, global = true)]
    pub stdout: bool,

    /// Copy result to clipboard
    #[arg(long, global = true)]
    pub clipboard: bool,

    /// Open result in default browser
    #[arg(long, global = true)]
    pub open: bool,

    /// Output format
    #[arg(short, long, default_value = "svg", global = true)]
    pub format: OutputFormat,

    /// Output width in pixels
    #[arg(long, global = true)]
    pub width: Option<u32>,

    /// Output height in pixels
    #[arg(long, global = true)]
    pub height: Option<u32>,

    /// Scale factor (0.1 to 3.0)
    #[arg(long, global = true)]
    pub scale: Option<f32>,

    /// Suppress non-error output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Svg,
    Png,
    Mermaid,
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Svg => "svg",
            Self::Png => "png",
            Self::Mermaid => "mmd",
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate shell completions
    Completions(CompletionsArgs),

    /// Generate an ER (entity-relationship) diagram
    #[command(name = "er")]
    ER(commands::er::ERArgs),

    /// Generate a flowchart diagram
    Flowchart(commands::flowchart::FlowchartArgs),

    /// Generate a user journey diagram
    Journey(commands::journey::JourneyArgs),

    /// Generate a mindmap diagram
    Mindmap(commands::mindmap::MindmapArgs),

    /// Generate a pie chart
    Pie(commands::pie::PieArgs),

    /// Render a raw .mmd file or mermaid string
    Render(commands::render::RenderArgs),

    /// Generate a requirement diagram
    Requirement(commands::requirement::RequirementArgs),

    /// Generate a sequence diagram
    Sequence(commands::sequence::SequenceArgs),

    /// Generate a state diagram
    State(commands::state::StateArgs),
}

#[derive(Parser, Debug)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: Shell,
}

/// Common input options for diagram subcommands
#[derive(Parser, Debug, Clone)]
pub struct InputOptions {
    /// Read diagram definition from JSON/YAML/TOML file
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Read diagram definition from stdin (JSON/YAML)
    #[arg(long)]
    pub stdin: bool,

    /// Raw mermaid syntax passthrough
    #[arg(long)]
    pub mermaid: Option<String>,
}

impl InputOptions {
    /// Check if any input source was specified
    pub fn has_input(&self) -> bool {
        self.input.is_some() || self.stdin || self.mermaid.is_some()
    }
}
