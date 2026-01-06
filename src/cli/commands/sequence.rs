use clap::Parser;

use crate::cli::{GlobalOptions, InputOptions, OutputFormat, OutputHandler};
use crate::core::{Diagram, MermaidError};
use crate::diagrams::sequence::{Message, MessageType, Note, NotePosition, Participant, SequenceDiagram};
use crate::render::{MermaidClient, RenderOptions};

#[derive(Parser, Debug)]
pub struct SequenceArgs {
    #[command(flatten)]
    pub input: InputOptions,

    /// Add actor: "id:label"
    #[arg(short, long, value_name = "SPEC")]
    pub actor: Vec<String>,

    /// Add participant: "id:label"
    #[arg(short, long, value_name = "SPEC")]
    pub participant: Vec<String>,

    /// Add message: "from->to:type:text"
    #[arg(short, long, value_name = "SPEC")]
    pub message: Vec<String>,

    /// Add note: "position:over:text"
    #[arg(long, value_name = "SPEC")]
    pub note: Vec<String>,

    /// Enable message autonumbering
    #[arg(long)]
    pub autonumber: bool,

    /// Diagram title
    #[arg(long)]
    pub title: Option<String>,
}

pub async fn run(args: SequenceArgs, global: &GlobalOptions) -> Result<(), MermaidError> {
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

async fn build_diagram(args: &SequenceArgs) -> Result<SequenceDiagram, MermaidError> {
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
        return Ok(SequenceDiagram::from_raw_mermaid(mermaid_str.to_string()));
    }

    // Build from CLI arguments
    let mut builder = SequenceDiagram::builder();

    if let Some(title) = &args.title {
        builder = builder.title(title);
    }

    if args.autonumber {
        builder = builder.autonumber(true);
    }

    // Parse actors
    for actor_spec in &args.actor {
        let participant = parse_participant_spec(actor_spec, true)?;
        builder = builder.participant(participant);
    }

    // Parse participants
    for participant_spec in &args.participant {
        let participant = parse_participant_spec(participant_spec, false)?;
        builder = builder.participant(participant);
    }

    // Parse messages
    for msg_spec in &args.message {
        let message = parse_message_spec(msg_spec)?;
        builder = builder.message(message);
    }

    // Parse notes
    for note_spec in &args.note {
        let note = parse_note_spec(note_spec)?;
        builder = builder.note(note);
    }

    Ok(builder.build())
}

fn parse_diagram(content: &str, format: &str) -> Result<SequenceDiagram, MermaidError> {
    match format.to_lowercase().as_str() {
        "json" => SequenceDiagram::from_json(content),
        "yaml" | "yml" => SequenceDiagram::from_yaml(content),
        "toml" => SequenceDiagram::from_toml(content),
        _ => Err(MermaidError::InvalidInput(format!("Unsupported format: {}", format))),
    }
}

fn parse_participant_spec(spec: &str, is_actor: bool) -> Result<Participant, MermaidError> {
    // Format: "id:label"
    let parts: Vec<&str> = spec.splitn(2, ':').collect();
    if parts.is_empty() {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid participant spec '{}'. Expected format: 'id:label'",
            spec
        )));
    }

    let id = parts[0].trim().to_string();
    let participant = if is_actor { Participant::actor(&id) } else { Participant::new(&id) };

    if parts.len() > 1 && !parts[1].is_empty() {
        Ok(participant.with_label(parts[1].trim()))
    } else {
        Ok(participant)
    }
}

fn parse_message_spec(spec: &str) -> Result<Message, MermaidError> {
    // Format: "from->to:type:text"
    let arrow_pos = spec.find("->").ok_or_else(|| {
        MermaidError::InvalidInput(format!(
            "Invalid message spec '{}'. Expected '->' between participants",
            spec
        ))
    })?;

    let from = spec[..arrow_pos].trim().to_string();
    let rest = &spec[arrow_pos + 2..];

    let parts: Vec<&str> = rest.splitn(3, ':').collect();
    if parts.is_empty() {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid message spec '{}'. Expected format: 'from->to:type:text'",
            spec
        )));
    }

    let to = parts[0].trim().to_string();
    let mut message = Message::new(from, to);

    if parts.len() > 1 && !parts[1].is_empty() {
        if let Some(msg_type) = MessageType::parse(parts[1].trim()) {
            message = message.with_type(msg_type);
        }
    }

    if parts.len() > 2 && !parts[2].is_empty() {
        message = message.with_text(parts[2].trim());
    }

    Ok(message)
}

fn parse_note_spec(spec: &str) -> Result<Note, MermaidError> {
    // Format: "position:over:text"
    let parts: Vec<&str> = spec.splitn(3, ':').collect();
    if parts.len() < 3 {
        return Err(MermaidError::InvalidInput(format!(
            "Invalid note spec '{}'. Expected format: 'position:over:text'",
            spec
        )));
    }

    let position = NotePosition::parse(parts[0].trim()).unwrap_or(NotePosition::Over);
    let over = parts[1].trim().to_string();
    let text = parts[2].trim().to_string();

    Ok(Note::over_participant(position, over, text))
}
