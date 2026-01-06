pub mod cli;
pub mod core;
pub mod diagrams;
pub mod render;

// Re-export commonly used types
pub use core::{Config, Diagram, Direction, FromConfig, MermaidError, Style, Theme};
pub use diagrams::{
    Choice, CompositeState, ConcurrentState, FlowChart, Fork, Join, Link, LinkHead, LinkStyle, Logic, LogicType,
    Message, MessageType, Node, NodeShape, Note, NotePosition, Participant, ParticipantBox, ParticipantType, PieChart,
    SequenceDiagram, State, StateDiagram, StateType, Subgraph, Transition,
};
pub use render::{MermaidClient, RenderOptions};
