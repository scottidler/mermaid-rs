use clap::Parser;

use crate::cli::{GlobalOptions, InputOptions, OutputFormat, OutputHandler};
use crate::core::{Config, Diagram, Direction, MermaidError};
use crate::diagrams::state::{State, StateDiagram, Transition};
use crate::render::{MermaidClient, RenderOptions};

#[derive(Parser, Debug)]
pub struct StateArgs {
    #[command(flatten)]
    pub input: InputOptions,

    /// Add state: "id:description"
    #[arg(long, value_name = "SPEC")]
    pub state: Vec<String>,

    /// Add transition: "from->to:label"
    #[arg(long, value_name = "SPEC")]
    pub transition: Vec<String>,

    /// Direction [TB, BT, LR, RL]
    #[arg(long, default_value = "TB")]
    pub direction: String,

    /// Diagram title
    #[arg(long)]
    pub title: Option<String>,
}

pub async fn run(args: StateArgs, global: &GlobalOptions) -> Result<(), MermaidError> {
    let mut diagram = build_diagram(&args).await?;

    // Apply mode's theme to diagram config
    let config = diagram.config.get_or_insert_with(Config::default);
    config.theme = global.mode.theme();

    let render_options = RenderOptions {
        width: global.width,
        height: global.height,
        scale: global.scale,
        background_color: global
            .background_color
            .clone()
            .or_else(|| global.mode.background_color().map(String::from)),
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

async fn build_diagram(args: &StateArgs) -> Result<StateDiagram, MermaidError> {
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
        return Ok(StateDiagram::from_raw_mermaid(mermaid_str.to_string()));
    }

    // Build from CLI arguments
    let mut builder = StateDiagram::builder();

    // Set direction
    let direction = match args.direction.to_uppercase().as_str() {
        "TB" | "TD" => Direction::TopBottom,
        "BT" => Direction::BottomTop,
        "LR" => Direction::LeftRight,
        "RL" => Direction::RightLeft,
        _ => Direction::TopBottom,
    };
    builder = builder.direction(direction);

    if let Some(title) = &args.title {
        builder = builder.title(title);
    }

    // Parse states
    for state_spec in &args.state {
        let state = parse_state_spec(state_spec)?;
        builder = builder.state(state);
    }

    // Parse transitions
    for trans_spec in &args.transition {
        let transition = parse_transition_spec(trans_spec)?;
        builder = builder.transition(transition);
    }

    Ok(builder.build())
}

fn parse_diagram(content: &str, format: &str) -> Result<StateDiagram, MermaidError> {
    match format.to_lowercase().as_str() {
        "json" => StateDiagram::from_json(content),
        "yaml" | "yml" => StateDiagram::from_yaml(content),
        "toml" => StateDiagram::from_toml(content),
        _ => Err(MermaidError::InvalidInput(format!(
            "Unsupported format: {}",
            format
        ))),
    }
}

fn parse_state_spec(spec: &str) -> Result<State, MermaidError> {
    // Format: "id:description"
    let parts: Vec<&str> = spec.splitn(2, ':').collect();
    if parts.is_empty() {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid state spec '{}'. Expected format: 'id:description'",
            spec
        )));
    }

    let id = parts[0].trim().to_string();

    // Handle special states
    if id == "[*]" {
        return Ok(State::start());
    }

    let state = State::new(&id);
    if parts.len() > 1 && !parts[1].is_empty() {
        Ok(state.with_description(parts[1].trim()))
    } else {
        Ok(state)
    }
}

fn parse_transition_spec(spec: &str) -> Result<Transition, MermaidError> {
    // Format: "from->to:label"
    let arrow_pos = spec.find("->").ok_or_else(|| {
        MermaidError::InvalidInput(format!(
            "Invalid transition spec '{}'. Expected '->' between states",
            spec
        ))
    })?;

    let from = spec[..arrow_pos].trim().to_string();
    let rest = &spec[arrow_pos + 2..];

    let parts: Vec<&str> = rest.splitn(2, ':').collect();
    if parts.is_empty() {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid transition spec '{}'. Expected format: 'from->to:label'",
            spec
        )));
    }

    let to = parts[0].trim().to_string();
    let transition = Transition::new(from, to);

    if parts.len() > 1 && !parts[1].is_empty() {
        Ok(transition.with_label(parts[1].trim()))
    } else {
        Ok(transition)
    }
}
