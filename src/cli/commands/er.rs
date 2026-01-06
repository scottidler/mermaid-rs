use clap::Parser;

use crate::cli::{GlobalOptions, InputOptions, OutputFormat, OutputHandler};
use crate::core::{Diagram, MermaidError};
use crate::diagrams::er::{Attribute, AttributeKey, AttributeType, Cardinality, ERDiagram, Entity, Relationship};
use crate::render::{MermaidClient, RenderOptions};

#[derive(Parser, Debug)]
pub struct ERArgs {
    #[command(flatten)]
    pub input: InputOptions,

    /// Add entity: "name" or "name:attr1:type,attr2:type:PK"
    #[arg(long, value_name = "SPEC")]
    pub entity: Vec<String>,

    /// Add relationship: "from->to:type:label" (type: one, many, optional)
    #[arg(long, value_name = "SPEC")]
    pub relationship: Vec<String>,

    /// Diagram title
    #[arg(long)]
    pub title: Option<String>,
}

pub async fn run(args: ERArgs, global: &GlobalOptions) -> Result<(), MermaidError> {
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

async fn build_diagram(args: &ERArgs) -> Result<ERDiagram, MermaidError> {
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
        return Ok(ERDiagram::from_raw_mermaid(mermaid_str.to_string()));
    }

    // Build from CLI arguments
    let mut builder = ERDiagram::builder();

    if let Some(title) = &args.title {
        builder = builder.title(title);
    }

    // Parse entities
    for entity_spec in &args.entity {
        let entity = parse_entity_spec(entity_spec)?;
        builder = builder.entity(entity);
    }

    // Parse relationships
    for rel_spec in &args.relationship {
        let rel = parse_relationship_spec(rel_spec)?;
        builder = builder.relationship(rel);
    }

    Ok(builder.build())
}

fn parse_diagram(content: &str, format: &str) -> Result<ERDiagram, MermaidError> {
    match format.to_lowercase().as_str() {
        "json" => ERDiagram::from_json(content),
        "yaml" | "yml" => ERDiagram::from_yaml(content),
        "toml" => ERDiagram::from_toml(content),
        _ => Err(MermaidError::InvalidInput(format!("Unsupported format: {}", format))),
    }
}

fn parse_entity_spec(spec: &str) -> Result<Entity, MermaidError> {
    // Format: "name" or "name:attr1:type,attr2:type:PK"
    let parts: Vec<&str> = spec.splitn(2, ':').collect();
    let name = parts[0].trim().to_string();

    let mut entity = Entity::new(&name);

    if parts.len() > 1 && !parts[1].is_empty() {
        // Parse attributes
        for attr_spec in parts[1].split(',') {
            let attr_parts: Vec<&str> = attr_spec.split(':').collect();
            if attr_parts.is_empty() {
                continue;
            }

            let attr_name = attr_parts[0].trim();
            let attr_type = if attr_parts.len() > 1 {
                AttributeType::parse(attr_parts[1].trim()).unwrap_or_default()
            } else {
                AttributeType::default()
            };

            let mut attr = Attribute::new(attr_type, attr_name);

            if attr_parts.len() > 2 {
                let key = match attr_parts[2].trim().to_uppercase().as_str() {
                    "PK" => AttributeKey::PrimaryKey,
                    "FK" => AttributeKey::ForeignKey,
                    "UK" => AttributeKey::UniqueKey,
                    _ => AttributeKey::None,
                };
                attr = attr.with_key(key);
            }

            entity = entity.with_attribute(attr);
        }
    }

    Ok(entity)
}

fn parse_relationship_spec(spec: &str) -> Result<Relationship, MermaidError> {
    // Format: "from->to:type:label"
    let arrow_pos = spec.find("->").ok_or_else(|| {
        MermaidError::InvalidInput(format!(
            "Invalid relationship spec '{}'. Expected '->' between entities",
            spec
        ))
    })?;

    let from = spec[..arrow_pos].trim().to_string();
    let rest = &spec[arrow_pos + 2..];

    let parts: Vec<&str> = rest.splitn(3, ':').collect();
    if parts.is_empty() {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid relationship spec '{}'. Expected format: 'from->to:type:label'",
            spec
        )));
    }

    let to = parts[0].trim().to_string();

    // Default to one-to-many
    let mut rel = Relationship::one_to_many(&from, &to);

    if parts.len() > 1 && !parts[1].is_empty() {
        let rel_type = parts[1].trim().to_lowercase();
        let (from_card, to_card) = match rel_type.as_str() {
            "one-to-one" | "1:1" => (Cardinality::ExactlyOne, Cardinality::ExactlyOne),
            "one-to-many" | "1:n" | "1:m" => (Cardinality::ExactlyOne, Cardinality::ZeroOrMore),
            "many-to-one" | "n:1" | "m:1" => (Cardinality::ZeroOrMore, Cardinality::ExactlyOne),
            "many-to-many" | "n:n" | "m:m" => (Cardinality::ZeroOrMore, Cardinality::ZeroOrMore),
            _ => (Cardinality::ExactlyOne, Cardinality::ZeroOrMore),
        };
        rel = rel.with_cardinality(from_card, to_card);
    }

    if parts.len() > 2 && !parts[2].is_empty() {
        rel = rel.with_label(parts[2].trim());
    }

    Ok(rel)
}
