use std::{fs, path::Path};

use rewrite_core::{Status, inspect_tsx};
use serde_json::Value;

fn check_fixture(name: &str) {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/spike")
        .join(name);
    // Preserve BOM, CRLF and byte offsets; do not normalize fixture text.
    let source = fs::read_to_string(directory.join("input.tsx")).unwrap();
    let expected: Value =
        serde_json::from_str(&fs::read_to_string(directory.join("expected.json")).unwrap())
            .unwrap();
    let report = inspect_tsx(&source);
    if report.status == Status::Invalid {
        assert!(
            !report.diagnostics.is_empty(),
            "{name}: invalid input needs a diagnostic"
        );
        assert!(
            report.bindings.is_empty(),
            "{name}: no partial bindings from invalid input"
        );
        for diagnostic in &report.diagnostics {
            assert!(!diagnostic.message.is_empty());
            for span in &diagnostic.spans {
                assert!(span.start_byte <= span.end_byte);
                assert!(
                    source
                        .get(span.start_byte as usize..span.end_byte as usize)
                        .is_some()
                );
            }
        }
    } else {
        assert!(
            report.diagnostics.is_empty(),
            "{name}: unexpected diagnostics"
        );
    }
    for binding in &report.bindings {
        assert_eq!(
            source.get(binding.span.start_byte as usize..binding.span.end_byte as usize),
            Some(binding.name.as_str())
        );
    }
    let mut actual = serde_json::to_value(&report).unwrap();
    // Oxc's prose may evolve; keep project reason codes and exact binding evidence golden.
    actual.as_object_mut().unwrap().remove("diagnostics");
    assert_eq!(actual, expected, "{name}");
}

macro_rules! fixture {
    ($test:ident, $name:literal) => {
        #[test]
        fn $test() {
            check_fixture($name);
        }
    };
}

fixture!(direct_named_import, "direct");
fixture!(alias_and_parameter_shadowing, "alias-shadowing");
fixture!(block_and_nested_function_shadowing, "block-shadowing");
fixture!(literal_import_sources_are_distinct, "literal-sources");
fixture!(unicode_crlf_bom_byte_spans, "unicode-crlf-bom");
fixture!(
    unresolved_intrinsic_and_member_names,
    "unresolved-and-intrinsic"
);
fixture!(parse_error_discards_bindings, "parse-error");
fixture!(
    recoverable_parse_error_discards_bindings,
    "recoverable-parse-error"
);
fixture!(regex_syntax_is_checked, "invalid-regex");
fixture!(semantic_syntax_error_discards_bindings, "semantic-error");
