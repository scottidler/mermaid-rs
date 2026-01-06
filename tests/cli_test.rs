use std::process::Command;

/// Helper to run the CLI binary
fn run_cli(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(args)
        .output()
        .expect("Failed to execute command")
}

#[test]
fn cli_help() {
    let output = run_cli(&["--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("mermaid"));
    assert!(stdout.contains("pie"));
    assert!(stdout.contains("render"));
}

#[test]
fn cli_version() {
    let output = run_cli(&["--version"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("mermaid"));
}

#[test]
fn cli_pie_help() {
    let output = run_cli(&["pie", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("--data"));
    assert!(stdout.contains("--title"));
    assert!(stdout.contains("--show-data"));
}

#[test]
fn cli_render_help() {
    let output = run_cli(&["render", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("--mermaid"));
    assert!(stdout.contains("--stdin"));
}

#[test]
fn cli_pie_mermaid_output() {
    let output = run_cli(&[
        "pie", "--data", "A:50", "--data", "B:50", "--title", "Test", "--format", "mermaid", "--stdout",
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("pie"));
    assert!(stdout.contains("title Test"));
    assert!(stdout.contains("\"A\" : 50"));
    assert!(stdout.contains("\"B\" : 50"));
}

#[test]
fn cli_pie_with_show_data() {
    let output = run_cli(&[
        "pie",
        "--data",
        "Chrome:65",
        "--show-data",
        "--format",
        "mermaid",
        "--stdout",
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("showData"));
}

#[test]
fn cli_render_mermaid_passthrough() {
    let output = run_cli(&[
        "render",
        "--mermaid",
        "pie title Test\n    \"A\" : 100",
        "--format",
        "mermaid",
        "--stdout",
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("pie title Test"));
}

#[test]
fn cli_invalid_data_spec() {
    let output = run_cli(&["pie", "--data", "invalid", "--format", "mermaid", "--stdout"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid data spec") || stderr.contains("error"));
}

#[test]
fn cli_global_options() {
    let output = run_cli(&[
        "--theme", "dark", "--format", "mermaid", "pie", "--data", "A:100", "--stdout",
    ]);

    // Should succeed with global options
    assert!(output.status.success());
}
