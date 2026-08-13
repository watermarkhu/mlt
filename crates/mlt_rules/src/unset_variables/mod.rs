//! # Unset Variables Engine
//!
//! This module implements file-level checks for variables that might not be
//! defined before use. It wraps 6 related MATLAB Code Analyzer checks into a
//! single rule engine that leverages the symbol table and definite-assignment
//! analysis.
//!
//! ## Checks
//!
//! | ID | Description |
//! |----|-------------|
//! | NODEF | Variable might not be defined before use |
//! | USENS | Variable used but might not be set in all code paths |
//! | PSET | Variable set in one branch but not others |
//! | SUSENS | Script variable used before set |
//! | SVNODEF | Variable in script might not be defined |
//! | STOUT | Output variable might not be assigned |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.UNSET_VARIABLES_ENGINE]
//! severity = "warning"
//! ignore = ["myGlobalWorkspaceVar"]
//! ```

use std::collections::HashSet;

use mlt_core::{Category, Config, Diagnostic, FileContext, Rule, Severity};
use serde::Deserialize;
use tree_sitter::{Node, Tree};

use crate::analysis::symbols::SymbolTable;

// ---------------------------------------------------------------------------
// Submodules: one file per independent check
// ---------------------------------------------------------------------------

mod check_function_scopes;
mod check_if_pset;
mod check_output_args;
mod check_partial_branch_assignment;
mod check_script_scopes;
mod check_switch_pset;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Common MATLAB builtins that look like variables but are actually
/// constants/functions and should not trigger unset-variable warnings.
pub(crate) const MATLAB_BUILTINS: &[&str] = &[
    "true",
    "false",
    "pi",
    "inf",
    "Inf",
    "nan",
    "NaN",
    "eps",
    "i",
    "j",
    "end",
    "nargin",
    "nargout",
    "varargin",
    "varargout",
    "ans",
];

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for the unset-variables engine.
///
/// Deserialized from `[lint.rules.UNSET_VARIABLES_ENGINE]` in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnsetVariablesConfig {
    /// Variable names to ignore (e.g., common workspace variables).
    #[serde(default)]
    pub ignore: Vec<String>,
}

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// File-level rule engine that detects variables that might not be defined
/// before use. Emits diagnostics under multiple check IDs depending on context.
pub struct UnsetVariablesEngine {
    /// Rule-specific configuration (retained for future use / serialization).
    _config: UnsetVariablesConfig,
    /// Precomputed set of ignored names (builtins + user-configured).
    ignore_set: HashSet<String>,
}

impl UnsetVariablesEngine {
    /// Factory constructor — reads rule-specific params from config.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: UnsetVariablesConfig = config.rule_params("UNSET_VARIABLES_ENGINE");
        let mut ignore_set: HashSet<String> =
            MATLAB_BUILTINS.iter().map(|s| (*s).to_string()).collect();
        for name in &rule_config.ignore {
            ignore_set.insert(name.clone());
        }
        Box::new(Self {
            _config: rule_config,
            ignore_set,
        })
    }

    /// Check whether a variable name should be ignored.
    pub(crate) fn is_ignored(&self, name: &str) -> bool {
        self.ignore_set.contains(name)
    }

    /// Find the location of the first assignment to a variable within a compound
    /// statement (for PSET diagnostic positioning).
    pub(crate) fn find_assignment_location(
        &self,
        node: Node,
        source: &str,
        var_name: &str,
    ) -> Option<(std::ops::Range<usize>, usize, usize)> {
        // DFS search for the first assignment to var_name.
        let mut stack = vec![node];
        while let Some(current) = stack.pop() {
            if current.kind() == "assignment" {
                if let Some(lhs) = current.child_by_field_name("left") {
                    if lhs.kind() == "identifier" {
                        let name = &source[lhs.start_byte()..lhs.end_byte()];
                        if name == var_name {
                            let pos = lhs.start_position();
                            return Some((
                                lhs.start_byte()..lhs.end_byte(),
                                pos.row + 1,
                                pos.column + 1,
                            ));
                        }
                    }
                }
            }
            // Push children in reverse so we visit left-to-right.
            let count = current.child_count();
            for i in (0..count).rev() {
                if let Some(child) = current.child(i) {
                    stack.push(child);
                }
            }
        }
        None
    }
}

impl Rule for UnsetVariablesEngine {
    fn id(&self) -> &'static str {
        "UNSET_VARIABLES_ENGINE"
    }

    fn description(&self) -> &'static str {
        "Detects variables that might not be defined before use"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn category(&self) -> Category {
        Category::UnsetVariables
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        // File-level only — no per-node dispatch.
        &[]
    }

    fn can_be_disabled(&self) -> bool {
        true
    }

    fn has_file_check(&self) -> bool {
        true
    }

    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> {
        let tree = ctx.tree;
        let source = ctx.source;

        // Build the symbol table for the file.
        let symbol_table = SymbolTable::build(tree, source);

        let mut diagnostics = Vec::new();

        // 1. Function scope checks (NODEF, USENS, STOUT).
        diagnostics.extend(self.check_function_scopes(tree, source, &symbol_table));

        // 2. Script scope checks (SVNODEF, SUSENS).
        diagnostics.extend(self.check_script_scopes(&symbol_table));

        // 3. Partial-branch assignment (PSET).
        diagnostics.extend(self.check_partial_branch_assignment(tree, source));

        diagnostics
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Find a `function_definition` node in the tree that matches the given byte range.
///
/// Performs a DFS from the root, returning the first `function_definition` node
/// whose byte range matches.
pub(crate) fn find_function_node_at<'a>(
    tree: &'a Tree,
    byte_range: &std::ops::Range<usize>,
) -> Option<Node<'a>> {
    let mut stack = vec![tree.root_node()];
    while let Some(node) = stack.pop() {
        if node.kind() == "function_definition"
            && node.start_byte() == byte_range.start
            && node.end_byte() == byte_range.end
        {
            return Some(node);
        }
        // Only recurse into nodes that overlap with the target range.
        let count = node.child_count();
        for i in (0..count).rev() {
            if let Some(child) = node.child(i) {
                if child.start_byte() <= byte_range.end && child.end_byte() >= byte_range.start {
                    stack.push(child);
                }
            }
        }
    }
    None
}

/// Collect all variable names assigned in a block (shallow — only direct
/// assignment statements, not nested blocks).
pub(crate) fn collect_block_assignments(block: Node, source: &str) -> HashSet<String> {
    let mut assigned = HashSet::new();
    let count = block.child_count();
    for i in 0..count {
        let Some(child) = block.child(i) else {
            continue;
        };
        if child.kind() == "assignment" {
            if let Some(lhs) = child.child_by_field_name("left") {
                match lhs.kind() {
                    "identifier" => {
                        let name = &source[lhs.start_byte()..lhs.end_byte()];
                        assigned.insert(name.to_string());
                    }
                    "multioutput_variable" => {
                        let lhs_count = lhs.child_count();
                        for j in 0..lhs_count {
                            if let Some(id) = lhs.child(j) {
                                if id.kind() == "identifier" {
                                    let name = &source[id.start_byte()..id.end_byte()];
                                    assigned.insert(name.to_string());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    assigned
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "UNSET_VARIABLES_ENGINE",
    UnsetVariablesEngine::from_config
));

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use mlt_core::Config;
    use tree_sitter::Parser;

    /// Parse MATLAB source and return the tree.
    fn parse(source: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_matlab::LANGUAGE.into())
            .expect("failed to load tree-sitter-matlab");
        parser.parse(source, None).expect("parse failed")
    }

    /// Build an engine with default configuration for tests.
    fn engine() -> Box<dyn Rule> {
        UnsetVariablesEngine::from_config(&Config::default())
    }

    /// Run the unset variables engine on source code and return diagnostics.
    fn lint(source: &str) -> Vec<Diagnostic> {
        let tree = parse(source);
        let ctx = FileContext {
            tree: &tree,
            source,
            file_path: std::path::Path::new("test.m"),
        };
        engine().check_file(&ctx)
    }

    // -- Builtin suppression ------------------------------------------------

    #[test]
    fn builtins_not_flagged() {
        let source = "\
function foo()
    x = pi + inf;
    y = true;
    z = nargin;
end
";
        let diags = lint(source);
        assert!(
            !diags.iter().any(|d| d.message.contains("'pi'")
                || d.message.contains("'inf'")
                || d.message.contains("'true'")
                || d.message.contains("'nargin'")),
            "builtins should not be flagged, got: {diags:?}"
        );
    }

    // -- Config ignore test -------------------------------------------------

    #[test]
    fn config_ignore_suppresses_warning() {
        let source = "y = myVar + 1;\n";
        let tree = parse(source);

        // Create an engine that ignores 'myVar'.
        let mut rule_config = UnsetVariablesConfig::default();
        rule_config.ignore.push("myVar".to_string());

        let mut ignore_set: HashSet<String> =
            MATLAB_BUILTINS.iter().map(|s| (*s).to_string()).collect();
        ignore_set.insert("myVar".to_string());

        let engine = UnsetVariablesEngine {
            _config: rule_config,
            ignore_set,
        };

        let ctx = FileContext {
            tree: &tree,
            source,
            file_path: std::path::Path::new("test.m"),
        };
        let diags = engine.check_file(&ctx);
        assert!(
            !diags.iter().any(|d| d.message.contains("'myVar'")),
            "ignored variable should not be flagged, got: {diags:?}"
        );
    }
}
