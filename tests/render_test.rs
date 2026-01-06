mod common;

use mermaid_rs::render::{MermaidClient, RenderOptions};
use wiremock::matchers::{method, path_regex, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn render_svg_with_mock_server() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/svg/.*"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<svg></svg>"))
        .mount(&mock_server)
        .await;

    let client = MermaidClient::new(Some(mock_server.uri()));
    let chart = common::simple_pie_chart();

    let result = client.render_svg(&chart, &RenderOptions::default()).await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("<svg"));
}

#[tokio::test]
async fn render_png_with_mock_server() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/img/.*"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![0x89, 0x50, 0x4E, 0x47]))
        .mount(&mock_server)
        .await;

    let client = MermaidClient::new(Some(mock_server.uri()));
    let chart = common::simple_pie_chart();

    let result = client.render_png(&chart, &RenderOptions::default()).await;
    assert!(result.is_ok());
    let bytes = result.unwrap();
    // Check PNG magic bytes
    assert_eq!(&bytes[0..4], &[0x89, 0x50, 0x4E, 0x47]);
}

#[tokio::test]
async fn render_with_options() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/svg/.*"))
        .and(query_param("width", "800"))
        .and(query_param("height", "600"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<svg></svg>"))
        .mount(&mock_server)
        .await;

    let client = MermaidClient::new(Some(mock_server.uri()));
    let chart = common::simple_pie_chart();
    let options = RenderOptions::new().width(800).height(600);

    let result = client.render_svg(&chart, &options).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn render_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/svg/.*"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let client = MermaidClient::new(Some(mock_server.uri()));
    let chart = common::simple_pie_chart();

    let result = client.render_svg(&chart, &RenderOptions::default()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn render_from_raw_script() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/svg/.*"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<svg></svg>"))
        .mount(&mock_server)
        .await;

    let client = MermaidClient::new(Some(mock_server.uri()));
    let script = "pie title Test\n    \"A\" : 50\n    \"B\" : 50";

    let result = client
        .render_svg_from_script(script, &RenderOptions::default())
        .await;
    assert!(result.is_ok());
}

#[test]
fn build_render_url() {
    let client = MermaidClient::new(Some("https://test.example.com".to_string()));
    let chart = common::simple_pie_chart();

    let url = client.build_render_url(&chart, "svg", &RenderOptions::default());

    assert!(url.starts_with("https://test.example.com/svg/"));
    assert!(!url.contains('?')); // No query params for default options
}

#[test]
fn build_render_url_with_options() {
    let client = MermaidClient::new(Some("https://test.example.com".to_string()));
    let chart = common::simple_pie_chart();
    let options = RenderOptions::new()
        .width(800)
        .height(600)
        .scale(2.0)
        .background_color("#ffffff");

    let url = client.build_render_url(&chart, "svg", &options);

    assert!(url.contains("width=800"));
    assert!(url.contains("height=600"));
    assert!(url.contains("scale=2"));
    assert!(url.contains("bgColor="));
}

// Integration test with real mermaid.ink - run with `cargo test -- --ignored`
#[tokio::test]
#[ignore]
async fn render_svg_real_server() {
    let client = MermaidClient::new(None); // Use default mermaid.ink
    let chart = common::simple_pie_chart();

    let result = client.render_svg(&chart, &RenderOptions::default()).await;
    assert!(result.is_ok());

    let svg = result.unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
}
