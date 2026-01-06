use clap::Parser;

use crate::cli::{GlobalOptions, InputOptions, OutputFormat, OutputHandler};
use crate::core::{Diagram, MermaidError};
use crate::diagrams::mindmap::{Mindmap, MindmapNodeShape};
use crate::render::{MermaidClient, RenderOptions};

#[derive(Parser, Debug)]
pub struct MindmapArgs {
    #[command(flatten)]
    pub input: InputOptions,

    /// Root node text
    #[arg(long)]
    pub root: Option<String>,

    /// Add child node (can be repeated)
    #[arg(long, value_name = "TEXT")]
    pub child: Vec<String>,

    /// Root node shape [default, square, rounded, circle, bang, cloud, hexagon]
    #[arg(long, default_value = "default")]
    pub shape: String,

    /// Diagram title
    #[arg(long)]
    pub title: Option<String>,
}

pub async fn run(args: MindmapArgs, global: &GlobalOptions) -> Result<(), MermaidError> {
    let diagram = build_diagram(&args).await?;

    let render_options = RenderOptions {
        width: global.width,
        height: global.height,
        scale: global.scale,
        background_color: None,
    };

    let output_handler = OutputHandler::new(global.output.clone(), global.stdout, global.clipboard, global.open);

    if matches!(global.format, OutputFormat::Mermaid) {
        let script = diagram.build_script();
        output_handler.write_mermaid(&script).await?;
        return Ok(());
    }

    let client = MermaidClient::new(Some(global.server.clone()));

    match global.format {
        OutputFormat::Svg => {
            let svg = client.render_svg(&diagram, &render_options).await?;
            output_handler.write_svg(&svg).await?;
        }
        OutputFormat::Png => {
            let png = client.render_png(&diagram, &render_options).await?;
            output_handler.write_png(&png).await?;
        }
        OutputFormat::Mermaid => unreachable!(),
    }

    Ok(())
}

async fn build_diagram(args: &MindmapArgs) -> Result<Mindmap, MermaidError> {
    if let Some(path) = &args.input.input {
        let content = tokio::fs::read_to_string(path).await?;
        let ext = path.extension().and_then(std::ffi::OsStr::to_str).unwrap_or("yaml");
        return parse_diagram(&content, ext);
    }

    if args.input.stdin {
        use tokio::io::AsyncReadExt;
        let mut buffer = String::new();
        tokio::io::stdin().read_to_string(&mut buffer).await?;
        let ext = if buffer.trim_start().starts_with('{') { "json" } else { "yaml" };
        return parse_diagram(&buffer, ext);
    }

    if let Some(mermaid_str) = &args.input.mermaid {
        return Ok(Mindmap::from_raw_mermaid(mermaid_str.to_string()));
    }

    // Build from CLI arguments
    let root_text = args.root.as_deref().unwrap_or("Root");
    let mut builder = Mindmap::builder(root_text);

    // Set root shape
    let shape = MindmapNodeShape::parse(&args.shape).unwrap_or_default();
    builder = builder.root_shape(shape);

    if let Some(title) = &args.title {
        builder = builder.title(title);
    }

    // Add children
    for child_text in &args.child {
        builder = builder.child(child_text);
    }

    Ok(builder.build())
}

fn parse_diagram(content: &str, format: &str) -> Result<Mindmap, MermaidError> {
    match format.to_lowercase().as_str() {
        "json" => Mindmap::from_json(content),
        "yaml" | "yml" => Mindmap::from_yaml(content),
        "toml" => Mindmap::from_toml(content),
        _ => Err(MermaidError::InvalidInput(format!("Unsupported format: {}", format))),
    }
}
