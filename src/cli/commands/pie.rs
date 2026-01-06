use clap::Parser;

use crate::cli::{GlobalOptions, InputOptions, OutputFormat, OutputHandler};
use crate::core::{Diagram, MermaidError};
use crate::diagrams::pie::PieChart;
use crate::render::{MermaidClient, RenderOptions};

#[derive(Parser, Debug)]
pub struct PieArgs {
    #[command(flatten)]
    pub input: InputOptions,

    /// Add data: "label:value" (can repeat)
    #[arg(short, long, value_name = "SPEC")]
    pub data: Vec<String>,

    /// Chart title
    #[arg(long)]
    pub title: Option<String>,

    /// Show percentage values
    #[arg(long)]
    pub show_data: bool,
}

pub async fn run(args: PieArgs, global: &GlobalOptions) -> Result<(), MermaidError> {
    // Build the pie chart from args or input file
    let chart = build_chart(&args).await?;

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
        let script = chart.build_script();
        output_handler.write_mermaid(&script).await?;
        return Ok(());
    }

    // Create client and render
    let client = MermaidClient::new(Some(global.server.clone()));

    match global.format {
        OutputFormat::Svg => {
            let svg = client.render_svg(&chart, &render_options).await?;
            output_handler.write_svg(&svg).await?;
        }
        OutputFormat::Png => {
            let png = client.render_png(&chart, &render_options).await?;
            output_handler.write_png(&png).await?;
        }
        OutputFormat::Mermaid => unreachable!(),
    }

    Ok(())
}

async fn build_chart(args: &PieArgs) -> Result<PieChart, MermaidError> {
    // If input file or stdin specified, load from there
    if let Some(path) = &args.input.input {
        let content = tokio::fs::read_to_string(path).await?;
        let ext = path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("yaml");
        return parse_chart(&content, ext);
    }

    if args.input.stdin {
        use tokio::io::AsyncReadExt;
        let mut buffer = String::new();
        tokio::io::stdin().read_to_string(&mut buffer).await?;
        // Try to detect format from content
        let ext = if buffer.trim_start().starts_with('{') {
            "json"
        } else {
            "yaml"
        };
        return parse_chart(&buffer, ext);
    }

    if let Some(mermaid_str) = &args.input.mermaid {
        // For passthrough, we create a minimal chart that outputs the raw mermaid
        return Ok(PieChart::from_raw_mermaid(mermaid_str.to_string()));
    }

    // Build from CLI arguments
    let mut builder = PieChart::builder();

    if let Some(title) = &args.title {
        builder = builder.title(title);
    }

    if args.show_data {
        builder = builder.show_data(true);
    }

    for data_spec in &args.data {
        let (label, value) = parse_data_spec(data_spec)?;
        builder = builder.data(label, value);
    }

    Ok(builder.build())
}

fn parse_chart(content: &str, format: &str) -> Result<PieChart, MermaidError> {
    match format.to_lowercase().as_str() {
        "json" => PieChart::from_json(content),
        "yaml" | "yml" => PieChart::from_yaml(content),
        "toml" => PieChart::from_toml(content),
        _ => Err(MermaidError::InvalidInput(format!(
            "Unsupported format: {}",
            format
        ))),
    }
}

fn parse_data_spec(spec: &str) -> Result<(String, f64), MermaidError> {
    let parts: Vec<&str> = spec.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid data spec '{}'. Expected format: 'label:value'",
            spec
        )));
    }

    let label = parts[0].trim().to_string();
    let value: f64 = parts[1].trim().parse().map_err(|_| {
        MermaidError::InvalidInput(format!("Invalid numeric value '{}' in data spec", parts[1]))
    })?;

    Ok((label, value))
}
