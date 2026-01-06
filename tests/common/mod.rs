// Test utilities shared across integration tests

use mermaid_rs::diagrams::pie::PieChart;

/// Create a simple test pie chart
#[allow(dead_code)]
pub fn simple_pie_chart() -> PieChart {
    PieChart::builder()
        .title("Test Chart")
        .data("A", 30.0)
        .data("B", 70.0)
        .build()
}

/// Create a pie chart with show_data enabled
#[allow(dead_code)]
pub fn detailed_pie_chart() -> PieChart {
    PieChart::builder()
        .title("Browser Market Share")
        .show_data(true)
        .data("Chrome", 65.0)
        .data("Firefox", 20.0)
        .data("Safari", 10.0)
        .data("Other", 5.0)
        .build()
}
