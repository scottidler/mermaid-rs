use clap::Parser;

use crate::cli::{GlobalOptions, InputOptions, OutputFormat, OutputHandler};
use crate::core::{Config, Diagram, MermaidError};
use crate::diagrams::journey::Journey;
use crate::render::{MermaidClient, RenderOptions};

#[derive(Parser, Debug)]
pub struct JourneyArgs {
    #[command(flatten)]
    pub input: InputOptions,

    /// Start a new section
    #[arg(long, value_name = "NAME")]
    pub section: Vec<String>,

    /// Add task: "name:score" or "name:score:actor1,actor2"
    #[arg(long, value_name = "SPEC")]
    pub task: Vec<String>,

    /// Diagram title
    #[arg(long)]
    pub title: Option<String>,
}

pub async fn run(args: JourneyArgs, global: &GlobalOptions) -> Result<(), MermaidError> {
    let mut diagram = build_diagram(&args).await?;

    // Apply mode's theme to diagram config
    let config = diagram.config.get_or_insert_with(Config::default);
    config.theme = global.mode.theme();

    let render_options = RenderOptions {
        width: global.width,
        height: global.height,
        scale: global.scale,
        background_color: global.mode.background_color().map(String::from),
    };

    let output_handler = OutputHandler::new(
        global.output.clone(),
        global.stdout,
        global.clipboard,
        global.open,
    );

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

async fn build_diagram(args: &JourneyArgs) -> Result<Journey, MermaidError> {
    if let Some(path) = &args.input.input {
        let content = tokio::fs::read_to_string(path).await?;
        let ext = path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("yaml");
        return parse_diagram(&content, ext);
    }

    if args.input.stdin {
        use tokio::io::AsyncReadExt;
        let mut buffer = String::new();
        tokio::io::stdin().read_to_string(&mut buffer).await?;
        let ext = if buffer.trim_start().starts_with('{') {
            "json"
        } else {
            "yaml"
        };
        return parse_diagram(&buffer, ext);
    }

    if let Some(mermaid_str) = &args.input.mermaid {
        return Ok(Journey::from_raw_mermaid(mermaid_str.to_string()));
    }

    // Build from CLI arguments
    let mut builder = Journey::builder();

    if let Some(title) = &args.title {
        builder = builder.title(title);
    }

    // Process sections and tasks
    // Simple approach: each --section starts a new section, tasks go into current section
    let mut section_iter = args.section.iter();

    // Start first section if any
    if let Some(section_name) = section_iter.next() {
        builder = builder.section(section_name);
    }

    // Add tasks
    for task_spec in &args.task {
        let (name, score, actors) = parse_task_spec(task_spec)?;
        if actors.is_empty() {
            builder = builder.task(name, score);
        } else {
            builder = builder.task_with_actors(name, score, actors);
        }
    }

    // Continue with remaining sections
    for section_name in section_iter {
        builder = builder.section(section_name);
    }

    Ok(builder.build())
}

fn parse_diagram(content: &str, format: &str) -> Result<Journey, MermaidError> {
    match format.to_lowercase().as_str() {
        "json" => Journey::from_json(content),
        "yaml" | "yml" => Journey::from_yaml(content),
        "toml" => Journey::from_toml(content),
        _ => Err(MermaidError::InvalidInput(format!(
            "Unsupported format: {}",
            format
        ))),
    }
}

fn parse_task_spec(spec: &str) -> Result<(String, u8, Vec<String>), MermaidError> {
    // Format: "name:score" or "name:score:actor1,actor2"
    let parts: Vec<&str> = spec.splitn(3, ':').collect();
    if parts.len() < 2 {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid task spec '{}'. Expected format: 'name:score' or 'name:score:actors'",
            spec
        )));
    }

    let name = parts[0].trim().to_string();
    let score: u8 = parts[1]
        .trim()
        .parse()
        .map_err(|_| MermaidError::InvalidInput(format!("Invalid score in task spec: {}", spec)))?;

    let actors = if parts.len() > 2 && !parts[2].is_empty() {
        parts[2].split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };

    Ok((name, score, actors))
}
