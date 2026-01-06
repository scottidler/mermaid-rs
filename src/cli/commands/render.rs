use std::path::PathBuf;

use clap::Parser;
use tokio::io::AsyncReadExt;

use crate::cli::{GlobalOptions, OutputFormat, OutputHandler};
use crate::core::MermaidError;
use crate::render::{MermaidClient, RenderOptions};

#[derive(Parser, Debug)]
pub struct RenderArgs {
    /// Path to .mmd file (omit for stdin)
    #[arg()]
    pub file: Option<PathBuf>,

    /// Read raw mermaid from stdin
    #[arg(long)]
    pub stdin: bool,

    /// Raw mermaid string to render
    #[arg(short, long)]
    pub mermaid: Option<String>,
}

pub async fn run(args: RenderArgs, global: &GlobalOptions) -> Result<(), MermaidError> {
    // Get the mermaid script from one of the input sources
    let script = get_script(&args).await?;

    // Build render options from global options
    let render_options = RenderOptions {
        width: global.width,
        height: global.height,
        scale: global.scale,
        background_color: None,
    };

    // Create output handler
    let output_handler = OutputHandler::new(
        global.output.clone(),
        global.stdout,
        global.clipboard,
        global.open,
    );

    // Handle mermaid format specially (no rendering needed)
    if matches!(global.format, OutputFormat::Mermaid) {
        output_handler.write_mermaid(&script).await?;
        return Ok(());
    }

    // Create client and render
    let client = MermaidClient::new(Some(global.server.clone()));

    match global.format {
        OutputFormat::Svg => {
            let svg = client
                .render_svg_from_script(&script, &render_options)
                .await?;
            output_handler.write_svg(&svg).await?;
        }
        OutputFormat::Png => {
            let png = client
                .render_png_from_script(&script, &render_options)
                .await?;
            output_handler.write_png(&png).await?;
        }
        OutputFormat::Mermaid => unreachable!(),
    }

    Ok(())
}

async fn get_script(args: &RenderArgs) -> Result<String, MermaidError> {
    // Priority: --mermaid flag > file argument > --stdin flag
    if let Some(mermaid) = &args.mermaid {
        return Ok(mermaid.clone());
    }

    if let Some(file) = &args.file {
        let content = tokio::fs::read_to_string(file).await?;
        return Ok(content);
    }

    if args.stdin {
        let mut buffer = String::new();
        tokio::io::stdin().read_to_string(&mut buffer).await?;
        return Ok(buffer);
    }

    // If no explicit input, check if there's data on stdin
    let mut buffer = String::new();
    tokio::io::stdin().read_to_string(&mut buffer).await?;
    if !buffer.is_empty() {
        return Ok(buffer);
    }

    Err(MermaidError::InvalidInput(
        "No input provided. Use --mermaid, provide a file, or pipe to stdin.".to_string(),
    ))
}
