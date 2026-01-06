use serde::{Deserialize, Serialize};

use crate::core::{Config, Diagram, Direction, FromConfig, MermaidError, Style, Theme};

use super::{ClassAssignment, ClassDef, Link, LinkStyle, LinkStyleDef, Node, NodeShape, Subgraph};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FlowChart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub direction: Direction,
    #[serde(default)]
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub links: Vec<Link>,
    #[serde(default)]
    pub subgraphs: Vec<Subgraph>,
    #[serde(default)]
    pub styles: Vec<NodeStyle>,
    /// Class definitions (classDef)
    #[serde(default)]
    pub class_defs: Vec<ClassDef>,
    /// Class assignments (class A,B className)
    #[serde(default)]
    pub class_assignments: Vec<ClassAssignment>,
    /// Link styling (linkStyle 0 stroke:#f00)
    #[serde(default)]
    pub link_styles: Vec<LinkStyleDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    /// Raw mermaid passthrough (if set, ignores other fields)
    #[serde(skip)]
    raw_mermaid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStyle {
    pub target: String,
    #[serde(flatten)]
    pub style: Style,
}

impl FlowChart {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> FlowChartBuilder {
        FlowChartBuilder::new()
    }

    pub fn from_raw_mermaid(mermaid: String) -> Self {
        Self {
            raw_mermaid: Some(mermaid),
            ..Default::default()
        }
    }

    pub fn from_json(json: &str) -> Result<Self, MermaidError> {
        let chart: Self = serde_json::from_str(json)?;
        Ok(chart)
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, MermaidError> {
        let chart: Self = serde_yaml::from_str(yaml)?;
        Ok(chart)
    }

    pub fn from_toml(toml: &str) -> Result<Self, MermaidError> {
        let chart: Self = toml::from_str(toml)?;
        Ok(chart)
    }

    /// Render a subgraph with its nodes and nested subgraphs
    fn render_subgraph_with_nodes(&self, subgraph: &Subgraph, base_indent: &str) -> String {
        let title = subgraph.title.as_deref().unwrap_or(&subgraph.id);
        let mut output = format!("{}subgraph {} [\"{}\"]\n", base_indent, subgraph.id, title);

        let inner_indent = format!("{}    ", base_indent);

        if let Some(dir) = &subgraph.direction {
            output.push_str(&format!("{}direction {}\n", inner_indent, dir));
        }

        // Render nodes belonging to this subgraph
        for node in &self.nodes {
            if subgraph.nodes.contains(&node.id) {
                output.push_str(&format!("{}{}\n", inner_indent, node.to_mermaid()));
            }
        }

        // Render nested subgraphs recursively
        for nested in &subgraph.subgraphs {
            output.push_str(&self.render_subgraph_with_nodes(nested, &inner_indent));
        }

        output.push_str(&format!("{}end\n", base_indent));
        output
    }
}

impl Diagram for FlowChart {
    fn to_mermaid(&self) -> String {
        // If raw mermaid was provided, return it directly
        if let Some(raw) = &self.raw_mermaid {
            return raw.clone();
        }

        let mut output = String::new();

        // Start with flowchart and direction
        output.push_str(&format!("flowchart {}\n", self.direction));

        // Collect nodes that are in subgraphs
        let mut nodes_in_subgraphs: Vec<String> = Vec::new();
        for sg in &self.subgraphs {
            nodes_in_subgraphs.extend(sg.nodes.clone());
        }

        // Render nodes not in any subgraph
        for node in &self.nodes {
            if !nodes_in_subgraphs.contains(&node.id) {
                output.push_str(&format!("    {}\n", node.to_mermaid()));
            }
        }

        // Render subgraphs (with nested subgraphs support)
        for subgraph in &self.subgraphs {
            output.push_str(&self.render_subgraph_with_nodes(subgraph, "    "));
        }

        // Render links
        for link in &self.links {
            output.push_str(&format!("    {}\n", link.to_mermaid()));
        }

        // Render styles
        for node_style in &self.styles {
            let css = node_style.style.to_css();
            if !css.is_empty() {
                output.push_str(&format!("    style {} {}\n", node_style.target, css));
            }
        }

        // Render class definitions
        for class_def in &self.class_defs {
            output.push_str(&format!("    {}\n", class_def.to_mermaid()));
        }

        // Render class assignments
        for class_assignment in &self.class_assignments {
            output.push_str(&format!("    {}\n", class_assignment.to_mermaid()));
        }

        // Render link styles
        for link_style in &self.link_styles {
            output.push_str(&format!("    {}\n", link_style.to_mermaid()));
        }

        output
    }

    fn diagram_type(&self) -> &'static str {
        "flowchart"
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }
}

impl FromConfig for FlowChart {
    fn from_json(json: &str) -> Result<Self, MermaidError> {
        Self::from_json(json)
    }

    fn from_yaml(yaml: &str) -> Result<Self, MermaidError> {
        Self::from_yaml(yaml)
    }

    fn from_toml(toml: &str) -> Result<Self, MermaidError> {
        Self::from_toml(toml)
    }
}

#[derive(Debug, Default)]
pub struct FlowChartBuilder {
    title: Option<String>,
    direction: Direction,
    nodes: Vec<Node>,
    links: Vec<Link>,
    subgraphs: Vec<Subgraph>,
    styles: Vec<NodeStyle>,
    class_defs: Vec<ClassDef>,
    class_assignments: Vec<ClassAssignment>,
    link_styles: Vec<LinkStyleDef>,
    config: Option<Config>,
}

impl FlowChartBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn node(mut self, node: Node) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn node_simple(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.nodes.push(Node::new(id, label, NodeShape::default()));
        self
    }

    pub fn node_with_shape(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        shape: NodeShape,
    ) -> Self {
        self.nodes.push(Node::new(id, label, shape));
        self
    }

    pub fn link(mut self, link: Link) -> Self {
        self.links.push(link);
        self
    }

    pub fn link_simple(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.links.push(Link::new(from, to));
        self
    }

    pub fn link_with_label(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        self.links.push(Link::new(from, to).with_label(label));
        self
    }

    pub fn link_with_style(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        style: LinkStyle,
    ) -> Self {
        self.links.push(Link::new(from, to).with_style(style));
        self
    }

    pub fn subgraph(mut self, subgraph: Subgraph) -> Self {
        self.subgraphs.push(subgraph);
        self
    }

    pub fn style(mut self, target: impl Into<String>, style: Style) -> Self {
        self.styles.push(NodeStyle {
            target: target.into(),
            style,
        });
        self
    }

    /// Add a class definition (classDef className fill:#f9f)
    pub fn class_def(mut self, name: impl Into<String>, style: Style) -> Self {
        self.class_defs.push(ClassDef::new(name, style));
        self
    }

    /// Assign a class to one or more nodes (class A,B className)
    pub fn class_assignment(mut self, class_name: impl Into<String>, nodes: Vec<String>) -> Self {
        self.class_assignments
            .push(ClassAssignment::new(class_name, nodes));
        self
    }

    /// Add link styling for a specific link index (linkStyle 0 stroke:#f00)
    pub fn link_style(mut self, index: usize, style: Style) -> Self {
        self.link_styles.push(LinkStyleDef::new(index, style));
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        let config = self.config.get_or_insert_with(Config::default);
        config.theme = theme;
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> FlowChart {
        FlowChart {
            title: self.title,
            direction: self.direction,
            nodes: self.nodes,
            links: self.links,
            subgraphs: self.subgraphs,
            styles: self.styles,
            class_defs: self.class_defs,
            class_assignments: self.class_assignments,
            link_styles: self.link_styles,
            config: self.config,
            raw_mermaid: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flowchart_basic() {
        let chart = FlowChart::builder()
            .direction(Direction::TopBottom)
            .node_with_shape("a", "Start", NodeShape::Stadium)
            .node_with_shape("b", "End", NodeShape::Stadium)
            .link_simple("a", "b")
            .build();

        let mermaid = chart.to_mermaid();
        assert!(mermaid.contains("flowchart TB"));
        assert!(mermaid.contains("a([\"Start\"])"));
        assert!(mermaid.contains("b([\"End\"])"));
        assert!(mermaid.contains("a --> b"));
    }

    #[test]
    fn flowchart_with_labels() {
        let chart = FlowChart::builder()
            .node_simple("A", "Hello")
            .node_simple("B", "World")
            .link_with_label("A", "B", "connects to")
            .build();

        let mermaid = chart.to_mermaid();
        assert!(mermaid.contains("-->|connects to|"));
    }

    #[test]
    fn flowchart_with_subgraph() {
        let chart = FlowChart::builder()
            .direction(Direction::LeftRight)
            .node_simple("A", "Node A")
            .node_simple("B", "Node B")
            .node_simple("C", "Node C")
            .subgraph(
                Subgraph::new("sg1")
                    .with_title("Group 1")
                    .with_nodes(vec!["A".to_string(), "B".to_string()]),
            )
            .link_simple("A", "B")
            .link_simple("B", "C")
            .build();

        let mermaid = chart.to_mermaid();
        assert!(mermaid.contains("subgraph sg1"));
        assert!(mermaid.contains("Group 1"));
        assert!(mermaid.contains("end"));
    }

    #[test]
    fn flowchart_with_styles() {
        let chart = FlowChart::builder()
            .node_simple("A", "Styled")
            .style("A", Style::builder().fill("#f9f").stroke("#333").build())
            .build();

        let mermaid = chart.to_mermaid();
        assert!(mermaid.contains("style A"));
    }

    #[test]
    fn flowchart_from_json() {
        let json = r#"{
            "direction": "LR",
            "nodes": [
                {"id": "A", "label": "Start", "shape": "stadium"},
                {"id": "B", "label": "End", "shape": "stadium"}
            ],
            "links": [
                {"from": "A", "to": "B"}
            ]
        }"#;

        let chart = FlowChart::from_json(json).unwrap();
        assert_eq!(chart.direction, Direction::LeftRight);
        assert_eq!(chart.nodes.len(), 2);
        assert_eq!(chart.links.len(), 1);
    }

    #[test]
    fn flowchart_from_yaml() {
        let yaml = r#"
direction: TB
title: Test Flow
nodes:
  - id: A
    label: Start
    shape: stadium
  - id: B
    label: Process
    shape: rectangle
  - id: C
    label: End
    shape: stadium
links:
  - from: A
    to: B
  - from: B
    to: C
    label: "next step"
"#;

        let chart = FlowChart::from_yaml(yaml).unwrap();
        assert_eq!(chart.title, Some("Test Flow".to_string()));
        assert_eq!(chart.nodes.len(), 3);
        assert_eq!(chart.links.len(), 2);
    }

    #[test]
    fn flowchart_raw_mermaid() {
        let raw = "flowchart LR\n    A --> B";
        let chart = FlowChart::from_raw_mermaid(raw.to_string());
        assert_eq!(chart.to_mermaid(), raw);
    }

    #[test]
    fn flowchart_with_class_def() {
        let chart = FlowChart::builder()
            .node_simple("A", "Start")
            .node_simple("B", "End")
            .link_simple("A", "B")
            .class_def(
                "highlight",
                Style::builder().fill("#f9f").stroke("#333").build(),
            )
            .build();

        let mermaid = chart.to_mermaid();
        assert!(mermaid.contains("classDef highlight fill:#f9f,stroke:#333"));
    }

    #[test]
    fn flowchart_with_class_assignment() {
        let chart = FlowChart::builder()
            .node_simple("A", "Start")
            .node_simple("B", "Process")
            .node_simple("C", "End")
            .link_simple("A", "B")
            .link_simple("B", "C")
            .class_def("important", Style::builder().fill("#ff0").build())
            .class_assignment("important", vec!["A".to_string(), "C".to_string()])
            .build();

        let mermaid = chart.to_mermaid();
        assert!(mermaid.contains("classDef important fill:#ff0"));
        assert!(mermaid.contains("class A,C important"));
    }

    #[test]
    fn flowchart_with_link_style() {
        let chart = FlowChart::builder()
            .node_simple("A", "Start")
            .node_simple("B", "End")
            .link_simple("A", "B")
            .link_style(
                0,
                Style::builder()
                    .stroke("#ff0000")
                    .stroke_width("2px")
                    .build(),
            )
            .build();

        let mermaid = chart.to_mermaid();
        assert!(mermaid.contains("linkStyle 0 stroke:#ff0000,stroke-width:2px"));
    }

    #[test]
    fn flowchart_with_node_class_shorthand() {
        let chart = FlowChart::builder()
            .node(Node::new("A", "Styled", NodeShape::Rectangle).with_class("highlight"))
            .class_def("highlight", Style::builder().fill("#f9f").build())
            .build();

        let mermaid = chart.to_mermaid();
        assert!(mermaid.contains("a[\"Styled\"]:::highlight"));
        assert!(mermaid.contains("classDef highlight fill:#f9f"));
    }

    #[test]
    fn flowchart_with_nested_subgraphs() {
        let inner = Subgraph::new("inner")
            .with_title("Inner Group")
            .with_nodes(vec!["B".to_string()]);

        let outer = Subgraph::new("outer")
            .with_title("Outer Group")
            .with_nodes(vec!["A".to_string()])
            .with_subgraph(inner);

        let chart = FlowChart::builder()
            .direction(Direction::TopBottom)
            .node_simple("A", "Node A")
            .node_simple("B", "Node B")
            .node_simple("C", "Node C")
            .subgraph(outer)
            .link_simple("A", "B")
            .link_simple("B", "C")
            .build();

        let mermaid = chart.to_mermaid();
        assert!(mermaid.contains("subgraph outer"));
        assert!(mermaid.contains("subgraph inner"));
        assert!(mermaid.contains("Outer Group"));
        assert!(mermaid.contains("Inner Group"));
        // Should have two end statements
        assert_eq!(mermaid.matches("end").count(), 2);
    }
}
