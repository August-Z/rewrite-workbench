//! Read-only TSX parsing and binding feasibility spike for RWB-001.

use std::collections::HashMap;

use oxc::{
    allocator::Allocator,
    ast::{
        AstKind,
        ast::{ImportDeclarationSpecifier, JSXElementName},
    },
    diagnostics::OxcDiagnostic,
    parser::{ParseOptions, Parser},
    semantic::SemanticBuilder,
    span::{GetSpan, SourceType, Span},
};
use serde::Serialize;

/// Coordinates refer only to the unchanged UTF-8 input of this analysis call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ByteSpan {
    pub start_byte: u32,
    pub end_byte: u32,
}

impl From<Span> for ByteSpan {
    fn from(span: Span) -> Self {
        Self {
            start_byte: span.start,
            end_byte: span.end,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Skipped,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonCode {
    BindingSpikeOnly,
    ParseError,
    SemanticSyntaxError,
}

#[derive(Debug, Serialize)]
pub struct Diagnostic {
    pub message: String,
    pub spans: Vec<ByteSpan>,
}

/// Binding observations are evidence, not rewrite candidates or eligibility decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "snake_case",
    rename_all_fields = "camelCase"
)]
pub enum BindingOrigin {
    NamedImport {
        module_specifier: String,
        imported_name: String,
        type_only: bool,
        declaration_span: ByteSpan,
    },
    /// Includes local declarations and imports outside this spike's named-import index.
    OtherBinding {
        declaration_span: ByteSpan,
    },
    Unresolved,
    Intrinsic,
    UnsupportedJsxName,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct BindingObservation {
    pub name: String,
    pub span: ByteSpan,
    pub origin: BindingOrigin,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisReport {
    pub schema_version: u32,
    pub status: Status,
    pub reason_code: ReasonCode,
    pub diagnostics: Vec<Diagnostic>,
    pub bindings: Vec<BindingObservation>,
}

fn invalid(reason_code: ReasonCode, diagnostics: &[OxcDiagnostic]) -> AnalysisReport {
    AnalysisReport {
        schema_version: 1,
        status: Status::Invalid,
        reason_code,
        diagnostics: diagnostics
            .iter()
            .map(|diagnostic| Diagnostic {
                message: diagnostic.to_string(),
                spans: diagnostic
                    .labels
                    .iter()
                    .map(|label| ByteSpan {
                        start_byte: label.offset(),
                        end_byte: label.offset() + label.len(),
                    })
                    .collect(),
            })
            .collect(),
        bindings: Vec::new(),
    }
}

/// Parse one immutable TSX module and inspect opening-tag bindings using Oxc Semantic.
///
/// Does not read files, resolve modules, select a recipe, create edits, or write output.
/// A valid input is `skipped / binding_spike_only`: no operation has been checked.
/// Any parser or semantic diagnostic invalidates the whole report's binding evidence.
pub fn inspect_tsx(source: &str) -> AnalysisReport {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, SourceType::tsx())
        .with_options(ParseOptions {
            parse_regular_expression: true,
            ..ParseOptions::default()
        })
        .parse();
    if parsed.fatal_error || !parsed.diagnostics.is_empty() {
        return invalid(ReasonCode::ParseError, &parsed.diagnostics);
    }

    let built = SemanticBuilder::new()
        // Oxc 0.149 does not retain nodes by default; both passes below need them.
        .with_build_nodes(true)
        .with_check_syntax_error(true)
        .build(&parsed.program);
    if !built.diagnostics.is_empty() {
        return invalid(ReasonCode::SemanticSyntaxError, &built.diagnostics);
    }
    let semantic = built.semantic;
    let scoping = semantic.scoping();
    let mut imports = HashMap::new();
    for node in semantic.nodes().iter() {
        let AstKind::ImportDeclaration(import) = node.kind() else {
            continue;
        };
        // Source/defer imports require their own classification in RWB-002.
        if import.phase.is_some() {
            continue;
        }
        for specifier in import.specifiers.iter().flatten() {
            let ImportDeclarationSpecifier::ImportSpecifier(specifier) = specifier else {
                continue;
            };
            if let Some(symbol_id) = specifier.local.symbol_id.get() {
                imports.insert(
                    symbol_id,
                    BindingOrigin::NamedImport {
                        module_specifier: import.source.value.to_string(),
                        imported_name: specifier.imported.name().to_string(),
                        type_only: import.import_kind.is_type() || specifier.import_kind.is_type(),
                        declaration_span: specifier.local.span.into(),
                    },
                );
            }
        }
    }

    let mut bindings = Vec::new();
    for node in semantic.nodes().iter() {
        let AstKind::JSXOpeningElement(element) = node.kind() else {
            continue;
        };
        let span = element.name.span();
        let origin =
            match &element.name {
                JSXElementName::Identifier(_) => BindingOrigin::Intrinsic,
                JSXElementName::IdentifierReference(reference) => {
                    let symbol = reference
                        .reference_id
                        .get()
                        .and_then(|id| scoping.get_reference(id).symbol_id());
                    match symbol {
                        Some(id) => imports.get(&id).cloned().unwrap_or_else(|| {
                            BindingOrigin::OtherBinding {
                                declaration_span: scoping.symbol_span(id).into(),
                            }
                        }),
                        None => BindingOrigin::Unresolved,
                    }
                }
                _ => BindingOrigin::UnsupportedJsxName,
            };
        bindings.push(BindingObservation {
            name: source[span.start as usize..span.end as usize].to_owned(),
            span: span.into(),
            origin,
        });
    }
    bindings.sort_by_key(|binding| binding.span.start_byte);
    AnalysisReport {
        schema_version: 1,
        status: Status::Skipped,
        reason_code: ReasonCode::BindingSpikeOnly,
        diagnostics: Vec::new(),
        bindings,
    }
}
