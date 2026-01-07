use clap::Parser;

use crate::cli::{GlobalOptions, InputOptions, OutputFormat, OutputHandler};
use crate::core::{Config, Diagram, MermaidError};
use crate::diagrams::requirement::{
    Element, ReqRelationship, Requirement, RequirementDiagram, Risk, VerifyMethod,
};
use crate::render::{MermaidClient, RenderOptions};

#[derive(Parser, Debug)]
pub struct RequirementArgs {
    #[command(flatten)]
    pub input: InputOptions,

    /// Add requirement: "id:name:text:risk:verify"
    #[arg(long, value_name = "SPEC")]
    pub requirement: Vec<String>,

    /// Add element: "id:name"
    #[arg(long, value_name = "SPEC")]
    pub element: Vec<String>,

    /// Add relationship: "from->to:type" (types: satisfies, verifies, derives, etc.)
    #[arg(long, value_name = "SPEC")]
    pub relationship: Vec<String>,

    /// Diagram title
    #[arg(long)]
    pub title: Option<String>,
}

pub async fn run(args: RequirementArgs, global: &GlobalOptions) -> Result<(), MermaidError> {
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

async fn build_diagram(args: &RequirementArgs) -> Result<RequirementDiagram, MermaidError> {
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
        return Ok(RequirementDiagram::from_raw_mermaid(
            mermaid_str.to_string(),
        ));
    }

    // Build from CLI arguments
    let mut builder = RequirementDiagram::builder();

    if let Some(title) = &args.title {
        builder = builder.title(title);
    }

    // Parse requirements
    for req_spec in &args.requirement {
        let req = parse_requirement_spec(req_spec)?;
        builder = builder.requirement(req);
    }

    // Parse elements
    for elem_spec in &args.element {
        let elem = parse_element_spec(elem_spec)?;
        builder = builder.element(elem);
    }

    // Parse relationships
    for rel_spec in &args.relationship {
        let rel = parse_relationship_spec(rel_spec)?;
        builder = builder.relationship(rel);
    }

    Ok(builder.build())
}

fn parse_diagram(content: &str, format: &str) -> Result<RequirementDiagram, MermaidError> {
    match format.to_lowercase().as_str() {
        "json" => RequirementDiagram::from_json(content),
        "yaml" | "yml" => RequirementDiagram::from_yaml(content),
        "toml" => RequirementDiagram::from_toml(content),
        _ => Err(MermaidError::InvalidInput(format!(
            "Unsupported format: {}",
            format
        ))),
    }
}

fn parse_requirement_spec(spec: &str) -> Result<Requirement, MermaidError> {
    // Format: "id:name:text:risk:verify"
    let parts: Vec<&str> = spec.splitn(5, ':').collect();
    if parts.len() < 2 {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid requirement spec '{}'. Expected format: 'id:name' or 'id:name:text:risk:verify'",
            spec
        )));
    }

    let id = parts[0].trim().to_string();
    let name = parts[1].trim().to_string();

    let mut req = Requirement::new(&id, &name);

    if parts.len() > 2 && !parts[2].is_empty() {
        req = req.with_text(parts[2].trim());
    }

    if parts.len() > 3 && !parts[3].is_empty() {
        let risk = Risk::parse(parts[3].trim()).unwrap_or_default();
        req = req.with_risk(risk);
    }

    if parts.len() > 4 && !parts[4].is_empty() {
        let verify = VerifyMethod::parse(parts[4].trim()).unwrap_or_default();
        req = req.with_verify_method(verify);
    }

    Ok(req)
}

fn parse_element_spec(spec: &str) -> Result<Element, MermaidError> {
    // Format: "id:name"
    let parts: Vec<&str> = spec.splitn(2, ':').collect();
    if parts.len() < 2 {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid element spec '{}'. Expected format: 'id:name'",
            spec
        )));
    }

    Ok(Element::new(parts[0].trim(), parts[1].trim()))
}

fn parse_relationship_spec(spec: &str) -> Result<ReqRelationship, MermaidError> {
    // Format: "from->to:type"
    let arrow_pos = spec.find("->").ok_or_else(|| {
        MermaidError::InvalidInput(format!(
            "Invalid relationship spec '{}'. Expected '->' between elements",
            spec
        ))
    })?;

    let from = spec[..arrow_pos].trim().to_string();
    let rest = &spec[arrow_pos + 2..];

    let parts: Vec<&str> = rest.splitn(2, ':').collect();
    if parts.is_empty() {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid relationship spec '{}'. Expected format: 'from->to:type'",
            spec
        )));
    }

    let to = parts[0].trim().to_string();
    let rel_type = if parts.len() > 1 {
        parts[1].trim()
    } else {
        "satisfies"
    };

    let rel = match rel_type.to_lowercase().as_str() {
        "satisfies" => ReqRelationship::satisfies(from, to),
        "verifies" => ReqRelationship::verifies(from, to),
        "derives" => ReqRelationship::derives(from, to),
        "contains" => ReqRelationship::contains(from, to),
        "copies" => ReqRelationship::copies(from, to),
        "refines" => ReqRelationship::refines(from, to),
        "traces" => ReqRelationship::traces(from, to),
        _ => ReqRelationship::satisfies(from, to),
    };

    Ok(rel)
}
