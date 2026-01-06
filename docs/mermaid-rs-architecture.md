# mermaid-rs Architecture Document

> Rust CLI rewrite of [mermaid-py](https://github.com/ouhammmourachid/mermaid-py)
> Reference implementation: `/home/saidler/repos/ouhammmourachid/mermaid-py`

---

## Overview

A Rust CLI tool that generates Mermaid diagrams programmatically and renders them via the mermaid.ink service. Distributed as a single static binary with no runtime dependencies.

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| HTTP Client | Async (`tokio` + `reqwest`) | Enables concurrent diagram rendering, modern Rust idiom |
| Input Methods | All: CLI flags, JSON/YAML files, stdin pipe | Maximum flexibility for different workflows |
| Output Methods | All: File, stdout, clipboard, browser | Support scripting and interactive use |
| Diagram Definition | File-first, structured args, DSL passthrough | File for complex, args for simple, DSL for mermaid experts |

---

## Crate Dependencies

```toml
[package]
name = "mermaid-rs"
version = "0.1.0"
edition = "2021"

[dependencies]
# CLI
clap = { version = "4", features = ["derive", "env"] }

# Async runtime
tokio = { version = "1", features = ["rt-multi-thread", "macros", "fs"] }

# HTTP client
reqwest = { version = "0.12", features = ["json"] }

# Encoding
base64 = "0.22"
urlencoding = "2"

# Serialization (config files)
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
toml = "0.8"

# Error handling
thiserror = "2"
anyhow = "1"

# Clipboard
arboard = "3"

# Open in browser
open = "5"

# Color output
owo-colors = "4"

[dev-dependencies]
tempfile = "3"
pretty_assertions = "1"
tokio-test = "0.4"
wiremock = "0.6"  # HTTP mocking
```

---

## Project Structure

```
mermaid-rs/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── src/
│   ├── main.rs                    # Entry point, tokio runtime setup
│   ├── lib.rs                     # Public library API
│   ├── cli/
│   │   ├── mod.rs                 # CLI module exports
│   │   ├── args.rs                # Clap argument definitions
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── flowchart.rs       # `mermaid flowchart` subcommand
│   │   │   ├── sequence.rs        # `mermaid sequence` subcommand
│   │   │   ├── state.rs           # `mermaid state` subcommand
│   │   │   ├── erdiagram.rs       # `mermaid er` subcommand
│   │   │   ├── pie.rs             # `mermaid pie` subcommand
│   │   │   ├── mindmap.rs         # `mermaid mindmap` subcommand
│   │   │   ├── journey.rs         # `mermaid journey` subcommand
│   │   │   ├── requirement.rs     # `mermaid requirement` subcommand
│   │   │   └── render.rs          # `mermaid render` (raw .mmd files)
│   │   └── output.rs              # Output handling (file, stdout, clipboard, browser)
│   ├── core/
│   │   ├── mod.rs
│   │   ├── diagram.rs             # `Diagram` trait definition
│   │   ├── config.rs              # Theme, colors, global config
│   │   ├── style.rs               # CSS-like node/element styling
│   │   ├── direction.rs           # TB, BT, LR, RL enum
│   │   └── error.rs               # Error types with thiserror
│   ├── render/
│   │   ├── mod.rs
│   │   ├── client.rs              # Async HTTP client for mermaid.ink
│   │   ├── encoder.rs             # Base64 + URL encoding
│   │   └── response.rs            # SVG/PNG response handling
│   ├── diagrams/
│   │   ├── mod.rs                 # Re-exports all diagram types
│   │   ├── flowchart/
│   │   │   ├── mod.rs
│   │   │   ├── flowchart.rs       # FlowChart struct
│   │   │   ├── node.rs            # Node struct + NodeShape enum (14 variants)
│   │   │   ├── link.rs            # Link struct + LinkStyle enum
│   │   │   └── subgraph.rs        # Subgraph support (not in Python version)
│   │   ├── sequence/
│   │   │   ├── mod.rs
│   │   │   ├── sequence.rs        # SequenceDiagram struct
│   │   │   ├── participant.rs     # Actor, Participant, Box
│   │   │   ├── message.rs         # Link/Message with arrow types
│   │   │   ├── note.rs            # Note with position
│   │   │   ├── rect.rs            # Colored rectangle regions
│   │   │   └── logic.rs           # Alt, Loop, Break, Critical, Opt, Par
│   │   ├── state/
│   │   │   ├── mod.rs
│   │   │   ├── state_diagram.rs   # StateDiagram struct
│   │   │   ├── state.rs           # State, Start, End
│   │   │   ├── composite.rs       # Composite state (contains sub-states)
│   │   │   ├── concurrent.rs      # Concurrent state (parallel groups)
│   │   │   └── transition.rs      # Transition, Choice, Fork, Join
│   │   ├── erdiagram/
│   │   │   ├── mod.rs
│   │   │   ├── erdiagram.rs       # ERDiagram struct
│   │   │   ├── entity.rs          # Entity with attributes
│   │   │   └── relationship.rs    # Relationship with cardinality
│   │   ├── pie/
│   │   │   ├── mod.rs
│   │   │   └── piechart.rs        # PieChart struct
│   │   ├── mindmap/
│   │   │   ├── mod.rs
│   │   │   ├── mindmap.rs         # Mindmap struct
│   │   │   └── level.rs           # Level with shape (7 variants)
│   │   ├── journey/
│   │   │   ├── mod.rs
│   │   │   ├── user_journey.rs    # UserJourney struct
│   │   │   ├── section.rs         # Section
│   │   │   └── task.rs            # Task with score
│   │   └── requirement/
│   │       ├── mod.rs
│   │       ├── requirement_diagram.rs
│   │       ├── requirement.rs     # Requirement with Type, Risk, VerifyMethod
│   │       ├── element.rs         # Element
│   │       └── link.rs            # Relationship links
│   └── input/
│       ├── mod.rs
│       ├── file.rs                # JSON/YAML/TOML file parsing
│       ├── stdin.rs               # Stdin pipe handling
│       └── parser.rs              # DSL string parsing (optional)
├── tests/
│   ├── common/
│   │   └── mod.rs                 # Test utilities
│   ├── flowchart_test.rs
│   ├── sequence_test.rs
│   ├── state_test.rs
│   ├── erdiagram_test.rs
│   ├── pie_test.rs
│   ├── mindmap_test.rs
│   ├── journey_test.rs
│   ├── requirement_test.rs
│   ├── render_test.rs             # Integration tests with mermaid.ink
│   └── cli_test.rs                # CLI argument parsing tests
└── examples/
    ├── simple_flowchart.rs
    ├── complex_sequence.rs
    └── config_files/
        ├── flowchart.yaml
        ├── flowchart.json
        └── sequence.toml
```

---

## CLI Interface

### Global Options

```bash
mermaid [OPTIONS] <COMMAND>

Options:
  -s, --server <URL>       Mermaid.ink server URL [env: MERMAID_INK_SERVER]
                           [default: https://mermaid.ink]
  -t, --theme <THEME>      Diagram theme [default, forest, dark, neutral, base]
  -o, --output <PATH>      Output file path (extension determines format: .svg, .png, .mmd)
      --stdout             Write to stdout instead of file
      --clipboard          Copy result to clipboard
      --open               Open result in default browser
  -f, --format <FORMAT>    Output format [svg, png, mermaid] [default: svg]
      --width <PX>         Output width in pixels
      --height <PX>        Output height in pixels
      --scale <FLOAT>      Scale factor (0.1 to 3.0)
  -q, --quiet              Suppress non-error output
  -v, --verbose            Increase verbosity (-v, -vv, -vvv)
  -h, --help               Print help
  -V, --version            Print version

Commands:
  flowchart    Generate a flowchart diagram
  sequence     Generate a sequence diagram
  state        Generate a state diagram
  er           Generate an entity-relationship diagram
  pie          Generate a pie chart
  mindmap      Generate a mindmap
  journey      Generate a user journey diagram
  requirement  Generate a requirement diagram
  render       Render a raw .mmd file or mermaid string
```

### Subcommand: `flowchart`

```bash
mermaid flowchart [OPTIONS]

Input Sources (mutually exclusive, priority order):
  -i, --input <FILE>       Read diagram definition from JSON/YAML/TOML file
      --stdin              Read diagram definition from stdin (JSON/YAML)
      --mermaid <STRING>   Raw mermaid syntax passthrough

Structured Arguments (when not using file/stdin):
  -n, --node <SPEC>...     Add node: "id:label:shape" (shape optional)
                           Shapes: rectangle, rounded, stadium, subroutine,
                                   cylinder, circle, asymmetric, rhombus,
                                   hexagon, parallelogram, parallelogram-alt,
                                   trapezoid, trapezoid-alt, double-circle
  -l, --link <SPEC>...     Add link: "from->to:style:label" (style/label optional)
                           Styles: arrow, dotted, thick, invisible
      --subgraph <SPEC>    Add subgraph: "id:title:node1,node2,..."
  -d, --direction <DIR>    Flow direction [TB, BT, LR, RL] [default: TB]
      --title <TITLE>      Diagram title
```

**Examples:**

```bash
# Structured arguments
mermaid flowchart \
  --node "A:Start:stadium" \
  --node "B:Process:rectangle" \
  --node "C:End:stadium" \
  --link "A->B:arrow:Begin" \
  --link "B->C" \
  --title "Simple Flow" \
  -o flow.svg

# From YAML file
mermaid flowchart -i diagram.yaml -o flow.png --theme dark

# From stdin (pipe from another tool)
cat diagram.json | mermaid flowchart --stdin -o flow.svg

# Raw mermaid passthrough
mermaid flowchart --mermaid "graph TD; A-->B; B-->C" -o flow.svg

# Multiple outputs
mermaid flowchart -i diagram.yaml -o flow.svg --clipboard --open
```

### Subcommand: `sequence`

```bash
mermaid sequence [OPTIONS]

Input Sources:
  -i, --input <FILE>       Read from JSON/YAML/TOML file
      --stdin              Read from stdin
      --mermaid <STRING>   Raw mermaid passthrough

Structured Arguments:
  -a, --actor <SPEC>...        Add actor: "id:label"
  -p, --participant <SPEC>...  Add participant: "id:label"
      --box <SPEC>...          Add box: "color:title:participant1,participant2"
  -m, --message <SPEC>...      Add message: "from->to:type:text"
                               Types: solid, dotted, solid-arrow, dotted-arrow,
                                      solid-cross, dotted-cross, solid-open, dotted-open
      --note <SPEC>...         Add note: "position:over:text"
                               Positions: left, right, over
      --activate <ID>          Activate participant
      --deactivate <ID>        Deactivate participant
      --autonumber             Enable message autonumbering
      --title <TITLE>          Diagram title
```

### Subcommand: `state`

```bash
mermaid state [OPTIONS]

Input Sources:
  -i, --input <FILE>       Read from JSON/YAML/TOML file
      --stdin              Read from stdin
      --mermaid <STRING>   Raw mermaid passthrough

Structured Arguments:
  -s, --state <SPEC>...        Add state: "id:description"
  -t, --transition <SPEC>...   Add transition: "from->to:label"
      --choice <SPEC>           Add choice: "id:condition1->state1,condition2->state2"
      --fork <ID>              Add fork point
      --join <ID>              Add join point
      --composite <SPEC>       Add composite state: "id:title:state1,state2"
      --concurrent <SPEC>      Add concurrent state: "id:group1|group2"
      --direction <DIR>        Direction [TB, BT, LR, RL] [default: TB]
      --title <TITLE>          Diagram title
```

### Subcommand: `er`

```bash
mermaid er [OPTIONS]

Input Sources:
  -i, --input <FILE>       Read from JSON/YAML/TOML file
      --stdin              Read from stdin
      --mermaid <STRING>   Raw mermaid passthrough

Structured Arguments:
  -e, --entity <SPEC>...       Add entity: "name:attr1:type1,attr2:type2"
  -r, --relationship <SPEC>... Add relationship: "entity1:cardinality1:entity2:cardinality2:label"
                               Cardinalities: zero-one, exactly-one, zero-many, one-many
      --title <TITLE>          Diagram title
```

### Subcommand: `pie`

```bash
mermaid pie [OPTIONS]

Input Sources:
  -i, --input <FILE>       Read from JSON/YAML/TOML file
      --stdin              Read from stdin
      --mermaid <STRING>   Raw mermaid passthrough

Structured Arguments:
  -d, --data <SPEC>...     Add data: "label:value" (can repeat)
      --title <TITLE>      Chart title
      --show-data          Show percentage values
```

**Example:**

```bash
mermaid pie \
  --data "Chrome:65" \
  --data "Firefox:20" \
  --data "Safari:10" \
  --data "Other:5" \
  --title "Browser Market Share" \
  --show-data \
  -o browsers.svg
```

### Subcommand: `mindmap`

```bash
mermaid mindmap [OPTIONS]

Input Sources:
  -i, --input <FILE>       Read from JSON/YAML/TOML file
      --stdin              Read from stdin
      --mermaid <STRING>   Raw mermaid passthrough

Structured Arguments:
      --root <LABEL>       Root node label
  -l, --level <SPEC>...    Add level: "depth:label:shape"
                           Shapes: square, rounded, circle, bang, cloud, hexagon, default
      --title <TITLE>      Diagram title
```

### Subcommand: `journey`

```bash
mermaid journey [OPTIONS]

Input Sources:
  -i, --input <FILE>       Read from JSON/YAML/TOML file
      --stdin              Read from stdin
      --mermaid <STRING>   Raw mermaid passthrough

Structured Arguments:
  -s, --section <SPEC>...  Add section: "title:task1:score1:actors1,task2:score2:actors2"
      --title <TITLE>      Journey title
```

### Subcommand: `requirement`

```bash
mermaid requirement [OPTIONS]

Input Sources:
  -i, --input <FILE>       Read from JSON/YAML/TOML file
      --stdin              Read from stdin
      --mermaid <STRING>   Raw mermaid passthrough

Structured Arguments:
  -r, --requirement <SPEC>...  Add requirement: "id:text:type:risk:verify"
                               Types: requirement, functional, interface, performance, physical, design
                               Risk: low, medium, high
                               Verify: analysis, inspection, test, demonstration
  -e, --element <SPEC>...      Add element: "id:type:docref"
  -l, --link <SPEC>...         Add link: "source->target:type"
                               Types: traces, derives, satisfies, verifies, refines, contains, copies
      --title <TITLE>          Diagram title
```

### Subcommand: `render`

```bash
mermaid render [OPTIONS] [FILE]

Arguments:
  [FILE]                   Path to .mmd file (omit for stdin)

Options:
      --stdin              Read raw mermaid from stdin
  -m, --mermaid <STRING>   Raw mermaid string to render
```

**Examples:**

```bash
# Render .mmd file
mermaid render diagram.mmd -o diagram.svg

# Pipe raw mermaid
echo "graph TD; A-->B" | mermaid render --stdin -o diagram.png

# Inline mermaid
mermaid render --mermaid "pie title Pets; Dog: 50; Cat: 30; Fish: 20" --open
```

---

## Input File Formats

### JSON Format

```json
{
  "type": "flowchart",
  "title": "User Authentication",
  "direction": "TB",
  "config": {
    "theme": "forest"
  },
  "nodes": [
    { "id": "start", "label": "Start", "shape": "stadium" },
    { "id": "login", "label": "Login Form", "shape": "rectangle" },
    { "id": "validate", "label": "Validate", "shape": "rhombus" },
    { "id": "success", "label": "Dashboard", "shape": "rectangle" },
    { "id": "error", "label": "Show Error", "shape": "rectangle" },
    { "id": "end", "label": "End", "shape": "stadium" }
  ],
  "links": [
    { "from": "start", "to": "login" },
    { "from": "login", "to": "validate" },
    { "from": "validate", "to": "success", "label": "Valid" },
    { "from": "validate", "to": "error", "label": "Invalid" },
    { "from": "error", "to": "login" },
    { "from": "success", "to": "end" }
  ],
  "styles": [
    { "target": "validate", "fill": "#f9f", "stroke": "#333" }
  ]
}
```

### YAML Format

```yaml
type: flowchart
title: User Authentication
direction: TB
config:
  theme: forest

nodes:
  - id: start
    label: Start
    shape: stadium
  - id: login
    label: Login Form
    shape: rectangle
  - id: validate
    label: Validate
    shape: rhombus
  - id: success
    label: Dashboard
  - id: error
    label: Show Error
  - id: end
    label: End
    shape: stadium

links:
  - from: start
    to: login
  - from: login
    to: validate
  - from: validate
    to: success
    label: Valid
  - from: validate
    to: error
    label: Invalid
  - from: error
    to: login
  - from: success
    to: end

styles:
  - target: validate
    fill: "#f9f"
    stroke: "#333"
```

### TOML Format

```toml
type = "flowchart"
title = "User Authentication"
direction = "TB"

[config]
theme = "forest"

[[nodes]]
id = "start"
label = "Start"
shape = "stadium"

[[nodes]]
id = "login"
label = "Login Form"
shape = "rectangle"

[[links]]
from = "start"
to = "login"

[[links]]
from = "login"
to = "validate"
```

### Sequence Diagram YAML Example

```yaml
type: sequence
title: API Request Flow
autonumber: true

participants:
  - type: actor
    id: user
    label: User
  - type: participant
    id: client
    label: Client App
  - type: participant
    id: api
    label: API Server
  - type: participant
    id: db
    label: Database

boxes:
  - color: "rgb(200, 220, 255)"
    title: Frontend
    members: [client]
  - color: "rgb(220, 255, 220)"
    title: Backend
    members: [api, db]

messages:
  - from: user
    to: client
    type: solid-arrow
    text: Click Submit
  - from: client
    to: api
    type: solid-arrow
    text: POST /api/data
    activate: api
  - from: api
    to: db
    type: solid-arrow
    text: INSERT query
    activate: db
  - from: db
    to: api
    type: dotted-arrow
    text: Success
    deactivate: db
  - from: api
    to: client
    type: dotted-arrow
    text: 200 OK
    deactivate: api
  - from: client
    to: user
    type: solid-arrow
    text: Show confirmation

notes:
  - position: right
    over: [api]
    text: Validates JWT token

logic:
  - type: alt
    condition: "Success"
    messages:
      - from: api
        to: client
        text: Return data
    else:
      condition: "Failure"
      messages:
        - from: api
          to: client
          text: Return error
```

---

## Core Trait Definition

```rust
// src/core/diagram.rs

use async_trait::async_trait;
use crate::core::{Config, Style};
use crate::error::MermaidError;

/// Trait implemented by all diagram types
pub trait Diagram: Send + Sync {
    /// Returns the mermaid syntax string for this diagram
    fn to_mermaid(&self) -> String;

    /// Returns the diagram type identifier (e.g., "flowchart", "sequenceDiagram")
    fn diagram_type(&self) -> &'static str;

    /// Returns optional title
    fn title(&self) -> Option<&str>;

    /// Returns optional configuration
    fn config(&self) -> Option<&Config>;

    /// Builds the complete mermaid script including frontmatter
    fn build_script(&self) -> String {
        let mut script = String::new();

        // Add YAML frontmatter if title or config present
        if self.title().is_some() || self.config().is_some() {
            script.push_str("---\n");
            if let Some(title) = self.title() {
                script.push_str(&format!("title: {}\n", title));
            }
            if let Some(config) = self.config() {
                script.push_str(&config.to_yaml());
            }
            script.push_str("---\n\n");
        }

        script.push_str(&self.to_mermaid());
        script
    }
}

/// Trait for diagram types that can be deserialized from config files
pub trait FromConfig: Diagram + Sized {
    fn from_json(json: &str) -> Result<Self, MermaidError>;
    fn from_yaml(yaml: &str) -> Result<Self, MermaidError>;
    fn from_toml(toml: &str) -> Result<Self, MermaidError>;
}
```

---

## Key Type Definitions

### FlowChart Types

```rust
// src/diagrams/flowchart/node.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub shape: NodeShape,
    pub style: Option<Style>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeShape {
    #[default]
    Rectangle,       // [label]
    Rounded,         // (label)
    Stadium,         // ([label])
    Subroutine,      // [[label]]
    Cylinder,        // [(label)]
    Circle,          // ((label))
    Asymmetric,      // >label]
    Rhombus,         // {label}
    Hexagon,         // {{label}}
    Parallelogram,   // [/label/]
    ParallelogramAlt,// [\label\]
    Trapezoid,       // [/label\]
    TrapezoidAlt,    // [\label/]
    DoubleCircle,    // (((label)))
}

impl NodeShape {
    pub fn wrap(&self, label: &str) -> String {
        match self {
            Self::Rectangle => format!("[\"{}\"", label),
            Self::Rounded => format!("(\"{}\")", label),
            Self::Stadium => format!("([\"{}\")", label),
            Self::Subroutine => format!("[[\"{}\"]]", label),
            Self::Cylinder => format!("[(\"{}\")", label),
            Self::Circle => format!("((\"{}\")", label),
            Self::Asymmetric => format!(">\"{}\"", label),
            Self::Rhombus => format!("{{\"{}\"}", label),
            Self::Hexagon => format!("{{{{\"{}\"}}}}", label),
            Self::Parallelogram => format!("[/\"{}\"/]", label),
            Self::ParallelogramAlt => format!("[\\\"{}\"\\]", label),
            Self::Trapezoid => format!("[/\"{}\"\\]", label),
            Self::TrapezoidAlt => format!("[\\\"{}\"//]", label),
            Self::DoubleCircle => format!("(((\"{}\")))", label),
        }
    }
}

// src/diagrams/flowchart/link.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub from: String,
    pub to: String,
    pub style: LinkStyle,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LinkStyle {
    #[default]
    Arrow,      // -->
    Dotted,     // -.->
    Thick,      // ==>
    Invisible,  // ~~~
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LinkHead {
    Arrow,      // >
    Circle,     // o
    Cross,      // x
    None,       // (no head)
}
```

### Direction Enum

```rust
// src/core/direction.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Direction {
    #[default]
    #[serde(rename = "TB")]
    TopBottom,
    #[serde(rename = "BT")]
    BottomTop,
    #[serde(rename = "LR")]
    LeftRight,
    #[serde(rename = "RL")]
    RightLeft,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TopBottom => write!(f, "TB"),
            Self::BottomTop => write!(f, "BT"),
            Self::LeftRight => write!(f, "LR"),
            Self::RightLeft => write!(f, "RL"),
        }
    }
}
```

### Config and Theme

```rust
// src/core/config.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub theme: Theme,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_variables: Option<ThemeVariables>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Default,
    Forest,
    Dark,
    Neutral,
    Base,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeVariables {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tertiary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<String>,
}

impl Config {
    pub fn to_yaml(&self) -> String {
        let mut yaml = String::new();
        yaml.push_str(&format!("theme: {}\n", self.theme.as_str()));
        if let Some(vars) = &self.theme_variables {
            yaml.push_str("themeVariables:\n");
            if let Some(c) = &vars.primary_color {
                yaml.push_str(&format!("  primaryColor: \"{}\"\n", c));
            }
            // ... other variables
        }
        yaml
    }
}
```

### Style

```rust
// src/core/style.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dasharray: Option<String>,
}

impl Style {
    pub fn to_css(&self) -> String {
        let mut parts = Vec::new();
        if let Some(fill) = &self.fill {
            parts.push(format!("fill:{}", fill));
        }
        if let Some(color) = &self.color {
            parts.push(format!("color:{}", color));
        }
        if let Some(stroke) = &self.stroke {
            parts.push(format!("stroke:{}", stroke));
        }
        if let Some(sw) = &self.stroke_width {
            parts.push(format!("stroke-width:{}", sw));
        }
        if let Some(sd) = &self.stroke_dasharray {
            parts.push(format!("stroke-dasharray:{}", sd));
        }
        parts.join(",")
    }
}
```

---

## Render Client

```rust
// src/render/client.rs

use reqwest::Client;
use crate::core::Diagram;
use crate::render::encoder::encode_diagram;
use crate::error::MermaidError;

pub struct MermaidClient {
    client: Client,
    server: String,
}

#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub scale: Option<f32>,
    pub background_color: Option<String>,
}

impl MermaidClient {
    pub fn new(server: Option<String>) -> Self {
        let server = server.unwrap_or_else(|| {
            std::env::var("MERMAID_INK_SERVER")
                .unwrap_or_else(|_| "https://mermaid.ink".to_string())
        });

        Self {
            client: Client::new(),
            server,
        }
    }

    pub async fn render_svg(
        &self,
        diagram: &dyn Diagram,
        options: &RenderOptions,
    ) -> Result<String, MermaidError> {
        let script = diagram.build_script();
        let encoded = encode_diagram(&script);
        let url = self.build_url("svg", &encoded, options);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(MermaidError::RenderFailed(response.status().to_string()));
        }

        Ok(response.text().await?)
    }

    pub async fn render_png(
        &self,
        diagram: &dyn Diagram,
        options: &RenderOptions,
    ) -> Result<Vec<u8>, MermaidError> {
        let script = diagram.build_script();
        let encoded = encode_diagram(&script);
        let url = self.build_url("img", &encoded, options);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(MermaidError::RenderFailed(response.status().to_string()));
        }

        Ok(response.bytes().await?.to_vec())
    }

    fn build_url(&self, endpoint: &str, encoded: &str, options: &RenderOptions) -> String {
        let mut url = format!("{}/{}/{}", self.server, endpoint, encoded);
        let mut params = Vec::new();

        if let Some(w) = options.width {
            params.push(format!("width={}", w));
        }
        if let Some(h) = options.height {
            params.push(format!("height={}", h));
        }
        if let Some(s) = options.scale {
            params.push(format!("scale={}", s));
        }
        if let Some(bg) = &options.background_color {
            params.push(format!("bgColor={}", bg));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        url
    }
}

// src/render/encoder.rs

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

pub fn encode_diagram(script: &str) -> String {
    URL_SAFE_NO_PAD.encode(script.as_bytes())
}
```

---

## Error Handling

```rust
// src/core/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MermaidError {
    #[error("Failed to parse input file: {0}")]
    ParseError(String),

    #[error("Invalid diagram configuration: {0}")]
    ConfigError(String),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Render failed: {0}")]
    RenderFailed(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Clipboard error: {0}")]
    ClipboardError(String),

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("YAML parse error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("TOML parse error: {0}")]
    TomlError(#[from] toml::de::Error),
}
```

---

## Output Handling

```rust
// src/cli/output.rs

use std::path::Path;
use tokio::fs;
use arboard::Clipboard;
use crate::error::MermaidError;

pub enum OutputTarget {
    File(String),
    Stdout,
    Clipboard,
    Browser,
}

pub struct OutputHandler {
    targets: Vec<OutputTarget>,
}

impl OutputHandler {
    pub fn new(
        file: Option<String>,
        stdout: bool,
        clipboard: bool,
        open_browser: bool,
    ) -> Self {
        let mut targets = Vec::new();

        if let Some(path) = file {
            targets.push(OutputTarget::File(path));
        }
        if stdout {
            targets.push(OutputTarget::Stdout);
        }
        if clipboard {
            targets.push(OutputTarget::Clipboard);
        }
        if open_browser {
            targets.push(OutputTarget::Browser);
        }

        // Default to stdout if nothing specified
        if targets.is_empty() {
            targets.push(OutputTarget::Stdout);
        }

        Self { targets }
    }

    pub async fn write_svg(&self, content: &str) -> Result<(), MermaidError> {
        for target in &self.targets {
            match target {
                OutputTarget::File(path) => {
                    fs::write(path, content).await?;
                }
                OutputTarget::Stdout => {
                    println!("{}", content);
                }
                OutputTarget::Clipboard => {
                    let mut clipboard = Clipboard::new()
                        .map_err(|e| MermaidError::ClipboardError(e.to_string()))?;
                    clipboard.set_text(content)
                        .map_err(|e| MermaidError::ClipboardError(e.to_string()))?;
                }
                OutputTarget::Browser => {
                    // Write to temp file and open
                    let temp_path = std::env::temp_dir().join("mermaid-output.svg");
                    fs::write(&temp_path, content).await?;
                    open::that(&temp_path)?;
                }
            }
        }
        Ok(())
    }

    pub async fn write_png(&self, content: &[u8]) -> Result<(), MermaidError> {
        for target in &self.targets {
            match target {
                OutputTarget::File(path) => {
                    fs::write(path, content).await?;
                }
                OutputTarget::Stdout => {
                    use std::io::Write;
                    std::io::stdout().write_all(content)?;
                }
                OutputTarget::Clipboard => {
                    // PNG to clipboard requires image crate integration
                    // For now, skip or implement with arboard's image feature
                    eprintln!("Warning: PNG clipboard not yet implemented");
                }
                OutputTarget::Browser => {
                    let temp_path = std::env::temp_dir().join("mermaid-output.png");
                    fs::write(&temp_path, content).await?;
                    open::that(&temp_path)?;
                }
            }
        }
        Ok(())
    }

    pub async fn write_mermaid(&self, content: &str) -> Result<(), MermaidError> {
        for target in &self.targets {
            match target {
                OutputTarget::File(path) => {
                    fs::write(path, content).await?;
                }
                OutputTarget::Stdout => {
                    println!("{}", content);
                }
                OutputTarget::Clipboard => {
                    let mut clipboard = Clipboard::new()
                        .map_err(|e| MermaidError::ClipboardError(e.to_string()))?;
                    clipboard.set_text(content)
                        .map_err(|e| MermaidError::ClipboardError(e.to_string()))?;
                }
                OutputTarget::Browser => {
                    // Open mermaid.live with the diagram
                    let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
                        .encode(content.as_bytes());
                    let url = format!("https://mermaid.live/edit#base64:{}", encoded);
                    open::that(&url)?;
                }
            }
        }
        Ok(())
    }
}
```

---

## Main Entry Point

```rust
// src/main.rs

use clap::Parser;
use mermaid_rs::cli::{Cli, Commands};
use mermaid_rs::error::MermaidError;

#[tokio::main]
async fn main() -> Result<(), MermaidError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Flowchart(args) => {
            mermaid_rs::cli::commands::flowchart::run(args, &cli.global).await
        }
        Commands::Sequence(args) => {
            mermaid_rs::cli::commands::sequence::run(args, &cli.global).await
        }
        Commands::State(args) => {
            mermaid_rs::cli::commands::state::run(args, &cli.global).await
        }
        Commands::Er(args) => {
            mermaid_rs::cli::commands::erdiagram::run(args, &cli.global).await
        }
        Commands::Pie(args) => {
            mermaid_rs::cli::commands::pie::run(args, &cli.global).await
        }
        Commands::Mindmap(args) => {
            mermaid_rs::cli::commands::mindmap::run(args, &cli.global).await
        }
        Commands::Journey(args) => {
            mermaid_rs::cli::commands::journey::run(args, &cli.global).await
        }
        Commands::Requirement(args) => {
            mermaid_rs::cli::commands::requirement::run(args, &cli.global).await
        }
        Commands::Render(args) => {
            mermaid_rs::cli::commands::render::run(args, &cli.global).await
        }
    }
}
```

---

## Implementation Order

### Phase 1: Foundation
1. Set up Cargo project with dependencies
2. Implement `core/` module (Diagram trait, Config, Style, Direction, Error)
3. Implement `render/` module (HTTP client, encoder)
4. Implement `cli/output.rs` (all output targets)
5. Basic CLI skeleton with global options

### Phase 2: First Diagram (PieChart)
1. Implement `diagrams/pie/` (simplest diagram type)
2. Implement `mermaid pie` subcommand with all input methods
3. Full integration test with mermaid.ink
4. Validate all output methods work

### Phase 3: Core Diagrams
1. Implement `diagrams/flowchart/` (most commonly used)
2. Implement `diagrams/sequence/` (complex: logic blocks)
3. Implement `diagrams/state/` (complex: composite/concurrent)

### Phase 4: Remaining Diagrams
1. Implement `diagrams/erdiagram/`
2. Implement `diagrams/mindmap/`
3. Implement `diagrams/journey/`
4. Implement `diagrams/requirement/`

### Phase 5: Polish
1. Implement `render` subcommand (raw .mmd files)
2. Comprehensive error messages
3. Shell completions (clap feature)
4. Man page generation (clap_mangen)
5. Documentation and examples

---

## Testing Strategy

### Unit Tests
- Each diagram type's `to_mermaid()` output
- Node shape wrapping
- Link formatting
- Config serialization
- Input file parsing

### Integration Tests
- Full CLI invocation with assert_cmd
- HTTP mocking with wiremock for offline tests
- Real mermaid.ink calls (marked `#[ignore]`)

### Example Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flowchart_generates_correct_mermaid() {
        let chart = FlowChart::builder()
            .direction(Direction::TopBottom)
            .node(Node::new("a", "Start", NodeShape::Stadium))
            .node(Node::new("b", "End", NodeShape::Stadium))
            .link(Link::new("a", "b"))
            .build();

        let expected = r#"flowchart TB
  a(["Start"])
  b(["End"])
  a --> b"#;

        assert_eq!(chart.to_mermaid(), expected);
    }

    #[tokio::test]
    async fn renders_svg_from_service() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::path_regex;

        let mock_server = MockServer::start().await;

        Mock::given(path_regex(r"^/svg/.*"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<svg></svg>"))
            .mount(&mock_server)
            .await;

        let client = MermaidClient::new(Some(mock_server.uri()));
        let chart = FlowChart::builder()
            .node(Node::new("a", "Test", NodeShape::Rectangle))
            .build();

        let result = client.render_svg(&chart, &RenderOptions::default()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("<svg"));
    }
}
```

---

## Future Enhancements (Out of Scope for Initial Version)

- [ ] Additional diagram types (Gantt, Git graph, C4, etc.)
- [ ] Local rendering via wasm-based mermaid.js (no network required)
- [ ] Watch mode for live reload during development
- [ ] Config file (~/.config/mermaid-rs/config.toml) for defaults
- [ ] Plugin system for custom diagram types
- [ ] Language server for editor integration
- [ ] Interactive TUI mode

---

## Reference Links

- [mermaid-py source](https://github.com/ouhammmourachid/mermaid-py)
- [Mermaid.js documentation](https://mermaid.js.org/intro/)
- [mermaid.ink API](https://mermaid.ink/)
- [Clap documentation](https://docs.rs/clap/latest/clap/)
- [Reqwest documentation](https://docs.rs/reqwest/latest/reqwest/)
