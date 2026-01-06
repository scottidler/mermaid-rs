pub mod cli;
pub mod core;
pub mod diagrams;
pub mod render;

// Re-export commonly used types
pub use core::{Config, Diagram, Direction, FromConfig, MermaidError, Style, Theme};
pub use diagrams::{
    // ER Diagram
    Attribute,
    AttributeKey,
    AttributeType,
    Cardinality,
    // State
    Choice,
    CompositeState,
    ConcurrentState,
    ERDiagram,
    // Requirement
    Element,
    ElementType,
    Entity,
    // Flowchart
    FlowChart,
    Fork,
    Join,
    // Journey
    Journey,
    Link,
    LinkHead,
    LinkStyle,
    // Sequence
    Logic,
    LogicType,
    Message,
    MessageType,
    // Mindmap
    Mindmap,
    MindmapNode,
    MindmapNodeShape,
    Node,
    NodeShape,
    Note,
    NotePosition,
    Participant,
    ParticipantBox,
    ParticipantType,
    // Pie
    PieChart,
    Relationship,
    ReqRelationship,
    Requirement,
    RequirementDiagram,
    Risk,
    Section,
    SequenceDiagram,
    State,
    StateDiagram,
    StateType,
    Subgraph,
    Task,
    Transition,
    VerifyMethod,
};
pub use render::{MermaidClient, RenderOptions};
