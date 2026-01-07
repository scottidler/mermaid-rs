use reqwest::Client;

use crate::core::{Diagram, MermaidError};
use crate::render::encoder::encode_diagram;

pub struct MermaidClient {
    client: Client,
    server: String,
}

#[derive(Debug, Clone, Default)]
pub struct RenderOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub scale: Option<f32>,
    pub background_color: Option<String>,
}

impl RenderOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn background_color(mut self, color: impl Into<String>) -> Self {
        self.background_color = Some(color.into());
        self
    }
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

    pub fn server(&self) -> &str {
        &self.server
    }

    pub async fn render_svg(
        &self,
        diagram: &dyn Diagram,
        options: &RenderOptions,
    ) -> Result<String, MermaidError> {
        let script = diagram.build_script();
        self.render_svg_from_script(&script, options).await
    }

    pub async fn render_svg_from_script(
        &self,
        script: &str,
        options: &RenderOptions,
    ) -> Result<String, MermaidError> {
        let encoded = encode_diagram(script);
        let url = self.build_url("svg", &encoded, options);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(MermaidError::RenderFailed(format!(
                "Server returned status: {}",
                response.status()
            )));
        }

        Ok(response.text().await?)
    }

    pub async fn render_png(
        &self,
        diagram: &dyn Diagram,
        options: &RenderOptions,
    ) -> Result<Vec<u8>, MermaidError> {
        let script = diagram.build_script();
        self.render_png_from_script(&script, options).await
    }

    pub async fn render_png_from_script(
        &self,
        script: &str,
        options: &RenderOptions,
    ) -> Result<Vec<u8>, MermaidError> {
        let encoded = encode_diagram(script);
        let url = self.build_url("img", &encoded, options);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(MermaidError::RenderFailed(format!(
                "Server returned status: {}",
                response.status()
            )));
        }

        Ok(response.bytes().await?.to_vec())
    }

    /// Build the URL for a render request
    pub fn build_render_url(
        &self,
        diagram: &dyn Diagram,
        format: &str,
        options: &RenderOptions,
    ) -> String {
        let script = diagram.build_script();
        let encoded = encode_diagram(&script);
        self.build_url(format, &encoded, options)
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
            // mermaid.ink expects hex without # (e.g., "1e1e1e" not "#1e1e1e")
            let bg_value = bg.strip_prefix('#').unwrap_or(bg);
            params.push(format!("bgColor={}", bg_value));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Config;

    struct TestDiagram;

    impl Diagram for TestDiagram {
        fn to_mermaid(&self) -> String {
            "graph TD\n  A --> B".to_string()
        }

        fn diagram_type(&self) -> &'static str {
            "flowchart"
        }

        fn title(&self) -> Option<&str> {
            None
        }

        fn config(&self) -> Option<&Config> {
            None
        }
    }

    #[test]
    fn client_default_server() {
        let client = MermaidClient::new(None);
        assert_eq!(client.server(), "https://mermaid.ink");
    }

    #[test]
    fn client_custom_server() {
        let client = MermaidClient::new(Some("https://custom.example.com".to_string()));
        assert_eq!(client.server(), "https://custom.example.com");
    }

    #[test]
    fn build_url_without_options() {
        let client = MermaidClient::new(Some("https://mermaid.ink".to_string()));
        let diagram = TestDiagram;
        let url = client.build_render_url(&diagram, "svg", &RenderOptions::default());
        assert!(url.starts_with("https://mermaid.ink/svg/"));
        assert!(!url.contains('?'));
    }

    #[test]
    fn build_url_with_options() {
        let client = MermaidClient::new(Some("https://mermaid.ink".to_string()));
        let diagram = TestDiagram;
        let options = RenderOptions::new().width(800).height(600);
        let url = client.build_render_url(&diagram, "svg", &options);
        assert!(url.contains("width=800"));
        assert!(url.contains("height=600"));
    }
}
