use std::{fs, path::PathBuf, process::Command};

use rewrite_core::inspect_tsx;
use serde_json::Value;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/spike")
        .join(name)
        .join("input.tsx")
}

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rewrite-workbench"))
}

#[test]
fn cli_uses_core_and_preserves_input_bytes() {
    let input = fixture("unicode-crlf-bom");
    let before = fs::read(&input).unwrap();
    let output = cli().arg("inspect").arg(&input).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
    let expected =
        serde_json::to_value(inspect_tsx(std::str::from_utf8(&before).unwrap())).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(fs::read(input).unwrap(), before);
}

#[test]
fn invalid_input_has_json_diagnostic_and_nonzero_exit() {
    let output = cli()
        .arg("inspect")
        .arg(fixture("parse-error"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["status"], "invalid");
    assert_eq!(report["reasonCode"], "parse_error");
    assert!(!report["diagnostics"].as_array().unwrap().is_empty());
    assert!(report["bindings"].as_array().unwrap().is_empty());
}

#[test]
fn wrong_arguments_and_syntax_modes_are_rejected() {
    for args in [
        vec![],
        vec!["inspect"],
        vec!["inspect", "input.ts"],
        vec!["apply", "input.tsx"],
        vec!["inspect", "a.tsx", "b.tsx"],
    ] {
        let output = cli().args(args).output().unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
}

#[test]
fn missing_file_is_an_io_failure() {
    let missing = fixture("nonexistent-fixture");
    assert!(!missing.exists());
    let output = cli().arg("inspect").arg(missing).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Cannot read UTF-8 input"));
}
