mod common;

use mermaid_rs::core::Diagram;
use mermaid_rs::diagrams::pie::{PieChart, PieData};

#[test]
fn pie_chart_empty() {
    let chart = PieChart::builder().build();
    let mermaid = chart.to_mermaid();
    assert!(mermaid.starts_with("pie"));
}

#[test]
fn pie_chart_with_title() {
    let chart = PieChart::builder().title("My Chart").build();
    // mermaid-py puts title in frontmatter, not in the diagram body
    let script = chart.build_script();
    assert!(script.contains("title: My Chart"));
}

#[test]
fn pie_chart_with_data() {
    let chart = common::simple_pie_chart();
    let mermaid = chart.to_mermaid();

    assert!(mermaid.contains("\"A\" : 30"));
    assert!(mermaid.contains("\"B\" : 70"));
}

#[test]
fn pie_chart_with_show_data() {
    let chart = common::detailed_pie_chart();
    let mermaid = chart.to_mermaid();

    assert!(mermaid.contains("showData"));
    assert!(mermaid.contains("\"Chrome\" : 65"));
}

#[test]
fn pie_chart_from_json() {
    let json = r#"{
        "title": "Languages",
        "data": [
            {"label": "Rust", "value": 40},
            {"label": "Python", "value": 30},
            {"label": "Go", "value": 30}
        ]
    }"#;

    let chart = PieChart::from_json(json).unwrap();
    let mermaid = chart.to_mermaid();
    let script = chart.build_script();

    // mermaid-py puts title in frontmatter
    assert!(script.contains("title: Languages"));
    assert!(mermaid.contains("\"Rust\" : 40"));
}

#[test]
fn pie_chart_from_yaml() {
    let yaml = r#"
title: "Languages"
show_data: true
data:
  - label: Rust
    value: 40
  - label: Python
    value: 30
"#;

    let chart = PieChart::from_yaml(yaml).unwrap();
    let mermaid = chart.to_mermaid();
    let script = chart.build_script();

    assert!(mermaid.contains("showData"));
    // mermaid-py puts title in frontmatter
    assert!(script.contains("title: Languages"));
}

#[test]
fn pie_chart_from_toml() {
    let toml = r#"
title = "Languages"

[[data]]
label = "Rust"
value = 50.0

[[data]]
label = "Go"
value = 50.0
"#;

    let chart = PieChart::from_toml(toml).unwrap();
    let mermaid = chart.to_mermaid();

    assert!(mermaid.contains("\"Rust\" : 50"));
    assert!(mermaid.contains("\"Go\" : 50"));
}

#[test]
fn pie_data_creation() {
    let data = PieData::new("Test Label", 42.5);
    assert_eq!(data.label, "Test Label");
    assert_eq!(data.value, 42.5);
}

#[test]
fn pie_chart_build_script_includes_frontmatter() {
    let chart = PieChart::builder().title("Test").data("A", 100.0).build();

    let script = chart.build_script();

    // Should include frontmatter when title is present
    assert!(script.contains("---"));
    assert!(script.contains("title: Test"));
}

#[test]
fn pie_chart_diagram_type() {
    let chart = PieChart::builder().build();
    assert_eq!(chart.diagram_type(), "pie");
}
