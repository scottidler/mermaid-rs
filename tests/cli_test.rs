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
    assert!(stdout.contains("flowchart"));
    assert!(stdout.contains("pie"));
    assert!(stdout.contains("sequence"));
    assert!(stdout.contains("state"));
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
        "pie", "--data", "A:50", "--data", "B:50", "--title", "Test", "--format", "mermaid",
        "--stdout",
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("pie"));
    // mermaid-py puts title in frontmatter, not on pie line
    assert!(stdout.contains("title: Test") || stdout.contains("title Test"));
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
    let output = run_cli(&[
        "pie", "--data", "invalid", "--format", "mermaid", "--stdout",
    ]);

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

// Flowchart CLI tests

#[test]
fn cli_flowchart_help() {
    let output = run_cli(&["flowchart", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("--node"));
    assert!(stdout.contains("--link"));
    assert!(stdout.contains("--direction"));
}

#[test]
fn cli_flowchart_mermaid_output() {
    let output = run_cli(&[
        "flowchart",
        "--node",
        "A:Start:stadium",
        "--node",
        "B:End:stadium",
        "--link",
        "A->B",
        "--format",
        "mermaid",
        "--stdout",
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("flowchart"));
    // mermaid-py lowercases node IDs
    assert!(stdout.contains("a([\"Start\"])"));
    assert!(stdout.contains("a --> b"));
}

#[test]
fn cli_flowchart_with_direction() {
    let output = run_cli(&[
        "flowchart",
        "--direction",
        "LR",
        "--node",
        "A:Test",
        "--format",
        "mermaid",
        "--stdout",
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("flowchart LR"));
}

#[test]
fn cli_flowchart_with_link_label() {
    let output = run_cli(&[
        "flowchart",
        "--node",
        "A:Start",
        "--node",
        "B:End",
        "--link",
        "A->B::goes to",
        "--format",
        "mermaid",
        "--stdout",
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("|goes to|"));
}

// Sequence CLI tests

#[test]
fn cli_sequence_help() {
    let output = run_cli(&["sequence", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("--actor"));
    assert!(stdout.contains("--participant"));
    assert!(stdout.contains("--message"));
    assert!(stdout.contains("--autonumber"));
}

#[test]
fn cli_sequence_mermaid_output() {
    let output = run_cli(&[
        "sequence",
        "--actor",
        "User",
        "--participant",
        "Server",
        "--message",
        "User->Server::Request",
        "--format",
        "mermaid",
        "--stdout",
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("sequenceDiagram"));
    assert!(stdout.contains("actor User"));
    assert!(stdout.contains("participant Server"));
}

#[test]
fn cli_sequence_with_autonumber() {
    let output = run_cli(&[
        "sequence",
        "--autonumber",
        "--participant",
        "A",
        "--participant",
        "B",
        "--message",
        "A->B::Hello",
        "--format",
        "mermaid",
        "--stdout",
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("autonumber"));
}

// State CLI tests

#[test]
fn cli_state_help() {
    let output = run_cli(&["state", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("--state"));
    assert!(stdout.contains("--transition"));
    assert!(stdout.contains("--direction"));
}

#[test]
fn cli_state_mermaid_output() {
    let output = run_cli(&[
        "state",
        "--state",
        "Active",
        "--state",
        "Inactive",
        "--transition",
        "[*]->Inactive",
        "--transition",
        "Inactive->Active:start",
        "--format",
        "mermaid",
        "--stdout",
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("stateDiagram-v2"));
    // mermaid-py lowercases state IDs
    assert!(stdout.contains("[*] --> inactive"));
    assert!(stdout.contains("inactive --> active : start"));
}

#[test]
fn cli_state_with_direction() {
    let output = run_cli(&[
        "state",
        "--direction",
        "LR",
        "--state",
        "On",
        "--state",
        "Off",
        "--transition",
        "On->Off",
        "--format",
        "mermaid",
        "--stdout",
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("direction LR"));
}
