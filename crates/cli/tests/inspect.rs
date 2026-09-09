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

#[test]
fn bindings_command_uses_selector_and_preserves_unicode_input() {
    let input = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/bindings/unicode-crlf-bom/input.tsx");
    let before = fs::read(&input).unwrap();
    for (module, confirmed) in [("@example/ui", true), ("./barrel", false)] {
        let output = cli()
            .arg("bindings")
            .arg(&input)
            .args([module, "Button"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        let report = rewrite_core::classify_tsx(
            std::str::from_utf8(&before).unwrap(),
            &rewrite_core::ImportSelector {
                module_specifier: module.to_owned(),
                imported_name: "Button".to_owned(),
            },
        );
        assert_eq!(actual, serde_json::to_value(report).unwrap());
        assert_eq!(actual["bindings"][0]["bindingConfirmed"], confirmed);
        assert_eq!(actual["bindings"][1]["bindingConfirmed"], false);
        assert_ne!(actual["bindings"][0]["status"], "ready");
        assert_eq!(fs::read(&input).unwrap(), before);
    }
}

#[test]
fn bindings_invalid_input_has_no_partial_results() {
    let output = cli()
        .arg("bindings")
        .arg(fixture("parse-error"))
        .args(["@example/ui", "Button"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["status"], "invalid");
    assert_eq!(report["reasonCode"], "parse_error");
    assert!(!report["message"].as_str().unwrap().is_empty());
    assert!(report["bindings"].as_array().unwrap().is_empty());
}

#[test]
fn bindings_requires_exact_arguments_and_tsx_mode() {
    for args in [
        vec!["bindings"],
        vec!["bindings", "a.tsx"],
        vec!["bindings", "a.tsx", "@example/ui"],
        vec!["bindings", "a.tsx", "@example/ui", "Button", "extra"],
        vec!["bindings", "a.ts", "@example/ui", "Button"],
    ] {
        let output = cli().args(args).output().unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
}

#[cfg(unix)]
#[test]
fn bindings_rejects_non_utf8_selectors() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};
    for invalid_module in [true, false] {
        let bad = OsString::from_vec(vec![0xff]);
        let (module, name) = if invalid_module {
            (bad, OsString::from("Button"))
        } else {
            (OsString::from("@example/ui"), bad)
        };
        let output = cli()
            .arg("bindings")
            .arg(fixture("direct"))
            .arg(module)
            .arg(name)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("must be UTF-8"));
    }
}
