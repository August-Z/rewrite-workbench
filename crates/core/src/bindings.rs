//! Selector classification is deliberately separate from the legacy spike and edit eligibility.

use std::collections::HashMap;

use oxc::{
    ast::{
        AstKind,
        ast::{ImportDeclarationSpecifier, JSXElementName},
    },
    semantic::Semantic,
    span::GetSpan,
};
use serde::Serialize;

use crate::{ByteSpan, Diagnostic, ReasonCode, Status, with_semantic};

/// Exact parsed import strings, not a recipe, resolver query, or filesystem path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportSelector {
    pub module_specifier: String,
    pub imported_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingReason {
    BindingClassificationOnly,
    OperationNotEvaluated,
    ModuleNotSelected,
    ImportedNameNotSelected,
    DifferentBinding,
    IntrinsicJsxTag,
    TypeOnlyImport,
    UnsupportedImportForm,
    UnsupportedMergedBinding,
    UnsupportedComponentIndirection,
    UnsupportedJsxName,
    UnresolvedBinding,
    ParseError,
    SemanticSyntaxError,
}

impl BindingReason {
    pub fn message(self) -> &'static str {
        match self {
            Self::BindingClassificationOnly => {
                "File-local binding classification completed; no rewrite operation was evaluated."
            }
            Self::OperationNotEvaluated => {
                "JSX reference binds to the selected direct named value import. Operation preconditions have not been evaluated; this is not ready for rewriting."
            }
            Self::ModuleNotSelected => {
                "The import declaration's literal module source does not match the selector. No re-export or physical module resolution was performed."
            }
            Self::ImportedNameNotSelected => {
                "The imported export name does not match the selector; local aliases do not change the export name."
            }
            Self::DifferentBinding => {
                "The JSX reference resolves to a local declaration, not the selected import binding."
            }
            Self::IntrinsicJsxTag => "This intrinsic JSX tag is not a component import reference.",
            Self::TypeOnlyImport => {
                "This import is type-only and cannot confirm a runtime component source."
            }
            Self::UnsupportedImportForm => {
                "Only ordinary direct named value imports are supported; default exports, namespace imports, import phases and TypeScript import-equals are not confirmed."
            }
            Self::UnsupportedMergedBinding => {
                "This symbol has multiple declarations. Merged import, type and value bindings are not supported, so no unique import origin is confirmed."
            }
            Self::UnsupportedComponentIndirection => {
                "This reference binds to a local variable. Assignment, destructuring and wrapper origins are not followed."
            }
            Self::UnsupportedJsxName => {
                "Only a bare component identifier is supported; member, namespaced and this-based JSX names are not confirmed."
            }
            Self::UnresolvedBinding => {
                "No file-local value binding was resolved. Re-exports do not create local bindings, and external or global sources are not inferred."
            }
            Self::ParseError => {
                "TSX parsing failed; the entire file's binding results were discarded."
            }
            Self::SemanticSyntaxError => {
                "Semantic syntax checks failed; the entire file's binding results were discarded."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportForm {
    Named,
    Default,
    Namespace,
}

/// Evidence concerns only this declaration (or the root of an unsupported member tag).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportEvidence {
    pub module_specifier: String,
    pub imported_name: Option<String>,
    pub local_name: String,
    pub form: ImportForm,
    pub type_only: bool,
    pub phase: Option<String>,
    pub declaration_span: ByteSpan,
    pub import_span: ByteSpan,
}

impl ImportEvidence {
    fn selected(&self, selector: &ImportSelector) -> bool {
        self.form == ImportForm::Named
            && !self.type_only
            && self.phase.is_none()
            && self.module_specifier == selector.module_specifier
            && self.imported_name.as_deref() == Some(selector.imported_name.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalBindingKind {
    Parameter,
    Variable,
    Function,
    Class,
    ImportEquals,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "snake_case",
    rename_all_fields = "camelCase"
)]
pub enum BindingEvidence {
    Import {
        declaration: ImportEvidence,
    },
    Local {
        declaration_span: ByteSpan,
        declaration_kind: LocalBindingKind,
    },
    Merged {
        declaration_spans: Vec<ByteSpan>,
    },
    Intrinsic,
    Unresolved,
    UnsupportedJsxName,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassifiedBinding {
    pub name: String,
    pub span: ByteSpan,
    pub status: Status,
    pub reason_code: BindingReason,
    pub message: &'static str,
    /// True only for a bare JSX reference to the selected ordinary named value import.
    /// Does not imply operation eligibility, type safety or final package identity.
    pub binding_confirmed: bool,
    pub evidence: BindingEvidence,
}

/// Version 1 of the classification report, independent of inspect/recipe/EditPlan schemas.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BindingReport {
    pub schema_version: u32,
    pub selector: ImportSelector,
    pub status: Status,
    pub reason_code: BindingReason,
    pub message: &'static str,
    pub diagnostics: Vec<Diagnostic>,
    pub bindings: Vec<ClassifiedBinding>,
}

/// Classify opening tags using file-local SymbolId identity and exact parsed import strings.
/// No filesystem access, cross-file resolution, operation checks, edits, or persistent IDs.
/// Successful binding confirmation still has `skipped / operation_not_evaluated` status.
pub fn classify_tsx(source: &str, selector: &ImportSelector) -> BindingReport {
    let (status, reason_code, diagnostics, bindings) = match with_semantic(source, |semantic| {
        classify_bindings(source, selector, semantic)
    }) {
        Ok(bindings) => (
            Status::Skipped,
            BindingReason::BindingClassificationOnly,
            Vec::new(),
            bindings,
        ),
        Err((reason, diagnostics)) => (
            Status::Invalid,
            match reason {
                ReasonCode::ParseError => BindingReason::ParseError,
                ReasonCode::SemanticSyntaxError => BindingReason::SemanticSyntaxError,
                ReasonCode::BindingSpikeOnly => unreachable!("not a parse failure"),
            },
            diagnostics,
            Vec::new(),
        ),
    };
    BindingReport {
        schema_version: 1,
        selector: selector.clone(),
        status,
        reason_code,
        message: reason_code.message(),
        diagnostics,
        bindings,
    }
}

fn classify_bindings(
    source: &str,
    selector: &ImportSelector,
    semantic: &Semantic<'_>,
) -> Vec<ClassifiedBinding> {
    let scoping = semantic.scoping();
    let mut imports = HashMap::new();
    for node in semantic.nodes().iter() {
        let AstKind::ImportDeclaration(import) = node.kind() else {
            continue;
        };
        for specifier in import.specifiers.iter().flatten() {
            let local = specifier.local();
            let (form, imported_name, type_only) = match specifier {
                ImportDeclarationSpecifier::ImportSpecifier(specifier) => {
                    let name = specifier.imported.name().to_string();
                    let form = if name == "default" {
                        ImportForm::Default
                    } else {
                        ImportForm::Named
                    };
                    (form, Some(name), specifier.import_kind.is_type())
                }
                ImportDeclarationSpecifier::ImportDefaultSpecifier(_) => {
                    (ImportForm::Default, Some("default".to_owned()), false)
                }
                ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => {
                    (ImportForm::Namespace, None, false)
                }
            };
            if let Some(id) = local.symbol_id.get() {
                imports.insert(
                    id,
                    ImportEvidence {
                        module_specifier: import.source.value.to_string(),
                        imported_name,
                        local_name: local.name.to_string(),
                        form,
                        type_only: import.import_kind.is_type() || type_only,
                        phase: import.phase.map(|phase| match phase {
                            oxc::ast::ast::ImportPhase::Source => "source".to_owned(),
                            oxc::ast::ast::ImportPhase::Defer => "defer".to_owned(),
                        }),
                        declaration_span: local.span.into(),
                        import_span: import.span.into(),
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
        let mut shadows_selected_import = false;
        let evidence = if let Some(reference) = element.name.get_identifier() {
            let symbol = reference
                .reference_id
                .get()
                .and_then(|id| scoping.get_reference(id).symbol_id());
            // Type-only imports may not resolve as value references. Scope lookup is used
            // only to explain rejection of a type-only import, never to confirm a value.
            let type_symbol = if symbol.is_none() {
                scoping
                    .find_binding(node.scope_id(), reference.name)
                    .filter(|id| imports.get(id).is_some_and(|import| import.type_only))
            } else {
                None
            };
            // Oxc can merge an import and a local declaration without a syntax error.
            // SymbolId equality alone therefore does not establish a unique import origin.
            let declarations = symbol
                .or(type_symbol)
                .map(|id| scoping.symbol_redeclarations(id))
                .filter(|declarations| declarations.len() > 1);
            if let Some(declarations) = declarations {
                let mut declaration_spans: Vec<ByteSpan> = declarations
                    .iter()
                    .map(|declaration| declaration.span.into())
                    .collect();
                declaration_spans.sort_by_key(|span| span.start_byte);
                BindingEvidence::Merged { declaration_spans }
            } else if let Some(import) = symbol.or(type_symbol).and_then(|id| imports.get(&id)) {
                BindingEvidence::Import {
                    declaration: import.clone(),
                }
            } else if let Some(id) = symbol {
                shadows_selected_import = imports.values().any(|import| {
                    import.selected(selector) && import.local_name == reference.name.as_str()
                });
                let declaration_kind = match semantic.nodes().kind(scoping.symbol_declaration(id)) {
                    AstKind::FormalParameter(_)
                    | AstKind::FormalParameterRest(_)
                    | AstKind::CatchParameter(_) => LocalBindingKind::Parameter,
                    AstKind::VariableDeclarator(_) => LocalBindingKind::Variable,
                    AstKind::Function(_) => LocalBindingKind::Function,
                    AstKind::Class(_) => LocalBindingKind::Class,
                    AstKind::TSImportEqualsDeclaration(_) => LocalBindingKind::ImportEquals,
                    _ => LocalBindingKind::Other,
                };
                BindingEvidence::Local {
                    declaration_span: scoping.symbol_span(id).into(),
                    declaration_kind,
                }
            } else {
                BindingEvidence::Unresolved
            }
        } else if matches!(element.name, JSXElementName::Identifier(_)) {
            BindingEvidence::Intrinsic
        } else {
            BindingEvidence::UnsupportedJsxName
        };

        // A member's root evidence must never confirm the full component expression.
        let reason = if !matches!(
            element.name,
            JSXElementName::IdentifierReference(_) | JSXElementName::Identifier(_)
        ) {
            BindingReason::UnsupportedJsxName
        } else {
            match &evidence {
                BindingEvidence::Import { declaration } => {
                    if declaration.type_only {
                        BindingReason::TypeOnlyImport
                    } else if declaration.form != ImportForm::Named || declaration.phase.is_some() {
                        BindingReason::UnsupportedImportForm
                    } else if declaration.module_specifier != selector.module_specifier {
                        BindingReason::ModuleNotSelected
                    } else if declaration.imported_name.as_deref()
                        != Some(selector.imported_name.as_str())
                    {
                        BindingReason::ImportedNameNotSelected
                    } else {
                        BindingReason::OperationNotEvaluated
                    }
                }
                BindingEvidence::Local {
                    declaration_kind: LocalBindingKind::ImportEquals,
                    ..
                } => BindingReason::UnsupportedImportForm,
                BindingEvidence::Local {
                    declaration_kind: LocalBindingKind::Variable,
                    ..
                } if !shadows_selected_import => BindingReason::UnsupportedComponentIndirection,
                BindingEvidence::Local { .. } => BindingReason::DifferentBinding,
                BindingEvidence::Merged { .. } => BindingReason::UnsupportedMergedBinding,
                BindingEvidence::Intrinsic => BindingReason::IntrinsicJsxTag,
                BindingEvidence::Unresolved => BindingReason::UnresolvedBinding,
                BindingEvidence::UnsupportedJsxName => BindingReason::UnsupportedJsxName,
            }
        };
        let status = match reason {
            BindingReason::ModuleNotSelected
            | BindingReason::ImportedNameNotSelected
            | BindingReason::DifferentBinding
            | BindingReason::IntrinsicJsxTag => Status::NotMatched,
            _ => Status::Skipped,
        };
        bindings.push(ClassifiedBinding {
            name: source[span.start as usize..span.end as usize].to_owned(),
            span: span.into(),
            status,
            reason_code: reason,
            message: reason.message(),
            binding_confirmed: reason == BindingReason::OperationNotEvaluated,
            evidence,
        });
    }
    bindings.sort_by_key(|binding| binding.span.start_byte);
    bindings
}
