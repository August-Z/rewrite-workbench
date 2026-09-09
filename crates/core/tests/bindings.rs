use std::{fs, path::Path};

use rewrite_core::{BindingReason, ImportSelector, Status, classify_tsx};
use serde_json::Value;

fn selector(module: &str, name: &str) -> ImportSelector {
    ImportSelector {
        module_specifier: module.to_owned(),
        imported_name: name.to_owned(),
    }
}

fn fixture(name: &str) {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/bindings")
        .join(name);
    let source = fs::read_to_string(directory.join("input.tsx")).unwrap();
    let expected: Value =
        serde_json::from_str(&fs::read_to_string(directory.join("expected.json")).unwrap())
            .unwrap();
    let selected = selector(
        expected["selector"]["moduleSpecifier"].as_str().unwrap(),
        expected["selector"]["importedName"].as_str().unwrap(),
    );
    let report = classify_tsx(&source, &selected);
    assert_eq!(
        report.status,
        Status::Skipped,
        "{name}: {:?}",
        report.diagnostics
    );
    assert_eq!(report.reason_code, BindingReason::BindingClassificationOnly);
    assert!(report.diagnostics.is_empty());
    assert!(!report.message.is_empty());
    assert_eq!(report.schema_version, 1);
    assert_eq!(report.selector, selected);
    for binding in &report.bindings {
        assert_ne!(binding.status, Status::Ready);
        assert!(!binding.message.is_empty());
        assert_eq!(
            source.get(binding.span.start_byte as usize..binding.span.end_byte as usize),
            Some(binding.name.as_str())
        );
    }
    let mut actual = serde_json::to_value(&report.bindings).unwrap();
    for binding in actual.as_array_mut().unwrap() {
        binding.as_object_mut().unwrap().remove("message");
    }
    assert_eq!(actual, expected["bindings"], "{name}");
}

macro_rules! fixture_test {
    ($test:ident, $name:literal) => {
        #[test]
        fn $test() {
            fixture($name);
        }
    };
}
fixture_test!(
    direct_alias_export_names_and_literal_sources,
    "direct-sources"
);
fixture_test!(
    parameters_blocks_hoisting_catch_loops_and_scope_exit,
    "shadowing"
);
fixture_test!(
    default_namespace_import_equals_phases_and_members,
    "unsupported-imports"
);
fixture_test!(
    wrappers_assignments_destructuring_and_require,
    "indirection"
);
fixture_test!(type_only_forms_and_local_value_shadowing, "type-only");
fixture_test!(
    reexports_unresolved_dom_and_special_jsx_names,
    "unresolved-and-reexports"
);
fixture_test!(unicode_bom_crlf_and_declaration_offsets, "unicode-crlf-bom");
fixture_test!(reexports_and_jsx_text_do_not_create_usages, "no-jsx");
fixture_test!(
    merged_import_and_local_declarations_are_not_confirmed,
    "merged-bindings"
);

#[test]
fn selector_uses_export_name_and_literal_module_without_normalization() {
    let source = "import { Button as B } from './barrel'; <B />;";
    for (module, name, reason) in [
        ("@example/ui", "Button", BindingReason::ModuleNotSelected),
        ("./barrel", "Button", BindingReason::OperationNotEvaluated),
        ("./barrel", "B", BindingReason::ImportedNameNotSelected),
        ("./x/../barrel", "Button", BindingReason::ModuleNotSelected),
        ("./Barrel", "Button", BindingReason::ModuleNotSelected),
    ] {
        let report = classify_tsx(source, &selector(module, name));
        assert_eq!(report.bindings[0].reason_code, reason);
        assert_eq!(
            report.bindings[0].binding_confirmed,
            reason == BindingReason::OperationNotEvaluated
        );
    }
    // Even explicit selection of a default export does not expand supported import forms.
    let report = classify_tsx(
        "import { default as B } from '@example/ui'; <B />;",
        &selector("@example/ui", "default"),
    );
    assert_eq!(
        report.bindings[0].reason_code,
        BindingReason::UnsupportedImportForm
    );
    assert!(!report.bindings[0].binding_confirmed);
}

#[test]
fn invalid_files_discard_all_classification_including_earlier_valid_tags() {
    let prefix = "import { Button } from '@example/ui'; <Button />;\n";
    for (suffix, reason) in [
        ("<Button", BindingReason::ParseError),
        ("<Button></Different>", BindingReason::ParseError),
        ("const r = /[/;", BindingReason::ParseError),
        (
            "let repeated; let repeated;",
            BindingReason::SemanticSyntaxError,
        ),
    ] {
        let source = format!("{prefix}{suffix}");
        let report = classify_tsx(&source, &selector("@example/ui", "Button"));
        assert_eq!(report.status, Status::Invalid);
        assert_eq!(report.reason_code, reason);
        assert!(report.bindings.is_empty());
        assert!(!report.diagnostics.is_empty());
        for diagnostic in report.diagnostics {
            assert!(!diagnostic.message.is_empty());
            for span in diagnostic.spans {
                assert!(
                    source
                        .get(span.start_byte as usize..span.end_byte as usize)
                        .is_some()
                );
            }
        }
    }
}
