use clap::Parser;

use crate::cli::{GlobalOptions, InputOptions, OutputFormat, OutputHandler};
use crate::core::{Config, Diagram, Direction, MermaidError};
use crate::diagrams::flowchart::{FlowChart, Link, LinkStyle, Node, NodeShape, Subgraph};
use crate::render::{MermaidClient, RenderOptions};

#[derive(Parser, Debug)]
pub struct FlowchartArgs {
    #[command(flatten)]
    pub input: InputOptions,

    /// Add node: "id:label:shape" (shape optional)
    #[arg(short, long, value_name = "SPEC")]
    pub node: Vec<String>,

    /// Add link: "from->to:style:label" (style/label optional)
    #[arg(short, long, value_name = "SPEC")]
    pub link: Vec<String>,

    /// Add subgraph: "id:title:node1,node2,..."
    #[arg(long, value_name = "SPEC")]
    pub subgraph: Vec<String>,

    /// Flow direction [TB, BT, LR, RL]
    #[arg(short, long, default_value = "TB")]
    pub direction: String,

    /// Diagram title
    #[arg(long)]
    pub title: Option<String>,
}

pub async fn run(args: FlowchartArgs, global: &GlobalOptions) -> Result<(), MermaidError> {
    let mut chart = build_chart(&args).await?;

    // Apply mode's theme to diagram config
    let config = chart.config.get_or_insert_with(Config::default);
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
        let script = chart.build_script();
        output_handler.write_mermaid(&script).await?;
        return Ok(());
    }

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

async fn build_chart(args: &FlowchartArgs) -> Result<FlowChart, MermaidError> {
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
        let ext = if buffer.trim_start().starts_with('{') {
            "json"
        } else {
            "yaml"
        };
        return parse_chart(&buffer, ext);
    }

    if let Some(mermaid_str) = &args.input.mermaid {
        return Ok(FlowChart::from_raw_mermaid(mermaid_str.to_string()));
    }

    // Build from CLI arguments
    let mut builder = FlowChart::builder();

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

    // Parse nodes
    for node_spec in &args.node {
        let node = parse_node_spec(node_spec)?;
        builder = builder.node(node);
    }

    // Parse links
    for link_spec in &args.link {
        let link = parse_link_spec(link_spec)?;
        builder = builder.link(link);
    }

    // Parse subgraphs
    for sg_spec in &args.subgraph {
        let sg = parse_subgraph_spec(sg_spec)?;
        builder = builder.subgraph(sg);
    }

    Ok(builder.build())
}

fn parse_chart(content: &str, format: &str) -> Result<FlowChart, MermaidError> {
    match format.to_lowercase().as_str() {
        "json" => FlowChart::from_json(content),
        "yaml" | "yml" => FlowChart::from_yaml(content),
        "toml" => FlowChart::from_toml(content),
        _ => Err(MermaidError::InvalidInput(format!(
            "Unsupported format: {}",
            format
        ))),
    }
}

fn parse_node_spec(spec: &str) -> Result<Node, MermaidError> {
    let parts: Vec<&str> = spec.splitn(3, ':').collect();
    if parts.is_empty() {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid node spec '{}'. Expected format: 'id:label:shape'",
            spec
        )));
    }

    let id = parts[0].trim().to_string();
    let label = if parts.len() > 1 {
        parts[1].trim().to_string()
    } else {
        id.clone()
    };
    let shape = if parts.len() > 2 {
        NodeShape::parse(parts[2].trim()).unwrap_or_default()
    } else {
        NodeShape::default()
    };

    Ok(Node::new(id, label, shape))
}

fn parse_link_spec(spec: &str) -> Result<Link, MermaidError> {
    // Format: "from->to:style:label"
    let arrow_pos = spec.find("->").ok_or_else(|| {
        MermaidError::InvalidInput(format!(
            "Invalid link spec '{}'. Expected '->' between nodes",
            spec
        ))
    })?;

    let from = spec[..arrow_pos].trim().to_string();
    let rest = &spec[arrow_pos + 2..];

    let parts: Vec<&str> = rest.splitn(3, ':').collect();
    if parts.is_empty() {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid link spec '{}'. Expected format: 'from->to:style:label'",
            spec
        )));
    }

    let to = parts[0].trim().to_string();
    let mut link = Link::new(from, to);

    if parts.len() > 1 && !parts[1].is_empty() {
        if let Some(style) = LinkStyle::parse(parts[1].trim()) {
            link = link.with_style(style);
        }
    }

    if parts.len() > 2 && !parts[2].is_empty() {
        link = link.with_label(parts[2].trim());
    }

    Ok(link)
}

fn parse_subgraph_spec(spec: &str) -> Result<Subgraph, MermaidError> {
    // Format: "id:title:node1,node2,..."
    let parts: Vec<&str> = spec.splitn(3, ':').collect();
    if parts.is_empty() {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid subgraph spec '{}'. Expected format: 'id:title:nodes'",
            spec
        )));
    }

    let id = parts[0].trim().to_string();
    let mut sg = Subgraph::new(&id);

    if parts.len() > 1 && !parts[1].is_empty() {
        sg = sg.with_title(parts[1].trim());
    }

    if parts.len() > 2 && !parts[2].is_empty() {
        let nodes: Vec<String> = parts[2].split(',').map(|s| s.trim().to_string()).collect();
        sg = sg.with_nodes(nodes);
    }

    Ok(sg)
}
