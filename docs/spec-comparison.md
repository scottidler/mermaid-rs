# Mermaid Spec vs mermaid-rs Implementation

This document compares the official Mermaid.js specification against what mermaid-rs currently implements.

**Legend:**
- ✅ Implemented
- ⚠️ Partial
- ❌ Not implemented

---

## Supported Diagram Types

| Diagram Type | Mermaid Spec | mermaid-rs |
|--------------|--------------|------------|
| Flowchart | ✅ | ✅ |
| Sequence | ✅ | ✅ |
| State | ✅ | ✅ |
| ER (Entity Relationship) | ✅ | ✅ |
| Pie Chart | ✅ | ✅ |
| Mindmap | ✅ | ✅ |
| User Journey | ✅ | ✅ |
| Requirement | ✅ | ✅ |
| Class Diagram | ✅ | ❌ |
| Gantt | ✅ | ❌ |
| Gitgraph | ✅ | ❌ |
| Timeline | ✅ | ❌ |
| Quadrant Chart | ✅ | ❌ |
| Sankey | ✅ | ❌ |
| XY Chart | ✅ | ❌ |
| Block Diagram | ✅ | ❌ |
| Packet | ✅ | ❌ |
| Kanban | ✅ | ❌ |
| Architecture | ✅ | ❌ |

---

## Flowchart

### Node Shapes

| Shape | Syntax | mermaid-rs |
|-------|--------|------------|
| Rectangle | `A[text]` | ✅ |
| Rounded | `A(text)` | ✅ |
| Stadium | `A([text])` | ✅ |
| Subroutine | `A[[text]]` | ✅ |
| Cylinder | `A[(text)]` | ✅ |
| Circle | `A((text))` | ✅ |
| Asymmetric | `A>text]` | ✅ |
| Rhombus/Diamond | `A{text}` | ✅ |
| Hexagon | `A{{text}}` | ✅ |
| Parallelogram | `A[/text/]` | ✅ |
| Parallelogram Alt | `A[\text\]` | ✅ |
| Trapezoid | `A[/text\]` | ✅ |
| Trapezoid Alt | `A[\text/]` | ✅ |
| Double Circle | `A(((text)))` | ✅ |
| v11+ Semantic Shapes | `A@{shape: decision}` | ❌ |

### Link Types

| Link | Syntax | mermaid-rs |
|------|--------|------------|
| Arrow | `A --> B` | ✅ |
| Open | `A --- B` | ✅ |
| Dotted | `A -.-> B` | ✅ |
| Thick | `A ==> B` | ✅ |
| Invisible | `A ~~~ B` | ✅ |
| With text | `A -->|text| B` | ✅ |
| Circle end | `A --o B` | ✅ |
| Cross end | `A --x B` | ✅ |
| Bidirectional | `A <--> B` | ✅ |
| Multi-directional combos | `A o--o B` | ✅ |

### Styling

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Inline style | `style A fill:#f9f` | ✅ |
| classDef | `classDef className fill:#f9f` | ✅ |
| class assignment | `class A,B className` | ✅ |
| Shorthand class | `A:::className` | ✅ |
| linkStyle | `linkStyle 0 stroke:#f00` | ✅ |

### Subgraphs

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Basic subgraph | `subgraph id[title]...end` | ✅ |
| Direction in subgraph | `direction TB` | ✅ |
| Nested subgraphs | subgraph inside subgraph | ✅ |
| Subgraph styling | via classDef | ⚠️ |
| Edges to subgraphs | `A --> subgraphId` | ⚠️ |

### Interaction

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Click href | `click A "url"` | ✅ |
| Click target | `click A "url" _blank` | ✅ |
| Click callback | `click A callback` | ❌ |
| Tooltips | `click A callback "tooltip"` | ❌ |

### Other

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| FontAwesome icons | `A[fa:fa-user]` | ❌ |
| Image nodes | `A@{img: "url"}` | ❌ |
| Markdown in labels | `A["**bold**"]` | ❌ |
| Comments | `%% comment` | ✅ (passthrough) |
| Direction | `flowchart LR` | ✅ |

---

## Sequence Diagram

### Participants

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Participant | `participant A` | ✅ |
| Actor | `actor A` | ✅ |
| Alias | `participant A as Alice` | ✅ |
| Participant types | `participant A as boundary` | ❌ |
| Create/destroy | `create participant A` | ❌ |

### Grouping

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Box | `box title...end` | ✅ |
| Box color | `box rgb(...)` | ⚠️ |

### Messages

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Solid arrow | `->>` | ✅ |
| Dotted arrow | `-->>` | ✅ |
| Solid line | `->` | ✅ |
| Dotted line | `-->` | ✅ |
| Cross | `-x` / `--x` | ✅ |
| Async | `-)` / `--)` | ✅ |
| Bidirectional | `<<->>` | ❌ |

### Activations

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Activate/deactivate | `activate A` / `deactivate A` | ✅ |
| Shorthand | `->>+` / `-->>-` | ✅ |

### Notes

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Note right/left | `Note right of A: text` | ✅ |
| Note over | `Note over A,B: text` | ✅ |

### Logic Blocks

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Loop | `loop...end` | ✅ |
| Alt/else | `alt...else...end` | ✅ |
| Opt | `opt...end` | ✅ |
| Par/and | `par...and...end` | ✅ |
| Critical/option | `critical...option...end` | ✅ |
| Break | `break...end` | ✅ |
| Rect (highlight) | `rect rgb(...)` | ❌ |

### Other

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Autonumber | `autonumber` | ✅ |
| Links | `link A: label @ url` | ❌ |
| Line breaks | `text<br/>more` | ⚠️ |

---

## State Diagram

### States

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Simple state | `state "label" as s1` | ✅ |
| Start | `[*] --> s1` | ✅ |
| End | `s1 --> [*]` | ✅ |
| Composite state | `state s1 { ... }` | ✅ |
| Concurrent | `state s1 { --  }` | ✅ |

### Transitions

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Basic | `s1 --> s2` | ✅ |
| With label | `s1 --> s2: label` | ✅ |

### Special Nodes

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Fork | `state fork <<fork>>` | ✅ |
| Join | `state join <<join>>` | ✅ |
| Choice | `state choice <<choice>>` | ✅ |

### Other

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Direction | `direction LR` | ✅ |
| Notes | `note right of s1: text` | ❌ |
| Styling | classDef/class | ❌ |

---

## ER Diagram

### Entities

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Entity | `ENTITY { }` | ✅ |
| Attributes | `type name` | ✅ |
| Primary key | `PK` | ✅ |
| Foreign key | `FK` | ✅ |
| Unique key | `UK` | ✅ |
| Comments | `type name "comment"` | ❌ |
| Entity alias | `ENTITY [alias]` | ❌ |

### Relationships

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Identifying | `A ||--|| B` | ✅ |
| Non-identifying | `A ||..|| B` | ✅ |
| Zero or one | `|o` / `o|` | ✅ |
| Exactly one | `||` | ✅ |
| Zero or more | `}o` / `o{` | ✅ |
| One or more | `}|` / `|{` | ✅ |
| Label | `: "label"` | ✅ |

---

## Pie Chart

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Title | `title "text"` | ✅ |
| Show data | `showData` | ✅ |
| Slices | `"label" : value` | ✅ |

---

## Mindmap

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Root node | `mindmap\n  Root` | ✅ |
| Child nodes | indentation | ✅ |
| Square shape | `[text]` | ✅ |
| Rounded shape | `(text)` | ✅ |
| Circle shape | `((text))` | ✅ |
| Cloud shape | `)text(` | ✅ |
| Hexagon shape | `{{text}}` | ✅ |
| Icons | `::icon(fa-user)` | ❌ |
| Classes | `:::className` | ❌ |
| Markdown | `**bold**` | ❌ |

---

## User Journey

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Title | `title text` | ✅ |
| Section | `section name` | ✅ |
| Task | `task: score: actors` | ✅ |
| Multiple actors | `actor1, actor2` | ✅ |

---

## Requirement Diagram

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Requirement | `requirement name { }` | ✅ |
| Functional req | `functionalRequirement` | ✅ |
| Interface req | `interfaceRequirement` | ✅ |
| Performance req | `performanceRequirement` | ✅ |
| Physical req | `physicalRequirement` | ✅ |
| Design constraint | `designConstraint` | ✅ |
| Element | `element name { }` | ✅ |
| Risk levels | `Low/Medium/High` | ✅ |
| Verify methods | `Analysis/Inspection/Test/Demonstration` | ✅ |
| Relationships | `contains/copies/derives/satisfies/verifies/refines/traces` | ✅ |

---

## Global Features

| Feature | Syntax | mermaid-rs |
|---------|--------|------------|
| Title frontmatter | `---\ntitle: x\n---` | ✅ |
| Theme config | `theme: dark` | ✅ |
| Init directive | `%%{init: {...}}%%` | ❌ |
| Accessible title | `accTitle: text` | ❌ |
| Accessible description | `accDescr: text` | ❌ |

---

## Summary

### Coverage by Diagram Type

| Diagram | Coverage |
|---------|----------|
| Flowchart | ~90% |
| Sequence | ~90% |
| State | ~85% |
| ER | ~90% |
| Pie | ~100% |
| Mindmap | ~70% |
| Journey | ~95% |
| Requirement | ~95% |

### Major Gaps (Priority)

1. **Icons** - FontAwesome integration
2. **Init directive** - Runtime configuration
3. **Rect (highlight)** - Sequence diagram highlight regions
4. **Bidirectional sequence messages** - `<<->>` syntax
5. **v11+ Semantic Shapes** - `A@{shape: decision}` syntax

### Not Planned

- Class diagrams (use PlantUML)
- Gantt charts (use dedicated tools)
- Gitgraph (niche use case)
- Interactive callbacks (security concerns)
