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

use crate::analysis::control_flow::{analyze_definite_assignment, DefiniteAssignment};
use crate::analysis::symbols::{DefKind, ScopeKind, SymbolTable};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Common MATLAB builtins that look like variables but are actually
/// constants/functions and should not trigger unset-variable warnings.
const MATLAB_BUILTINS: &[&str] = &[
    "true", "false", "pi", "inf", "Inf", "nan", "NaN", "eps", "i", "j", "end", "nargin",
    "nargout", "varargin", "varargout", "ans",
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
    fn is_ignored(&self, name: &str) -> bool {
        self.ignore_set.contains(name)
    }

    // -----------------------------------------------------------------------
    // Function scope checks
    // -----------------------------------------------------------------------

    /// Run definite-assignment analysis on function scopes and emit diagnostics.
    fn check_function_scopes(
        &self,
        tree: &Tree,
        source: &str,
        symbol_table: &SymbolTable,
    ) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for scope in &symbol_table.scopes {
            // Only process function-like scopes.
            if !matches!(
                scope.kind,
                ScopeKind::Function
                    | ScopeKind::LocalFunction
                    | ScopeKind::NestedFunction
                    | ScopeKind::Method
            ) {
                continue;
            }

            // Find the corresponding function_definition node.
            let Some(func_node) = find_function_node_at(tree, &scope.byte_range) else {
                continue;
            };

            // Run definite-assignment analysis.
            let da = analyze_definite_assignment(func_node, source);

            // Emit NODEF/USENS for possibly-unset variables.
            self.emit_possibly_unset_diagnostics(&da, &mut diagnostics);

            // Check output arguments: STOUT.
            self.check_output_args(scope, &da, &mut diagnostics);
        }

        diagnostics
    }

    /// Emit diagnostics for possibly-unset variables from definite-assignment results.
    fn emit_possibly_unset_diagnostics(
        &self,
        da: &DefiniteAssignment,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // Track which variable names have already been reported to avoid duplicates.
        let mut reported: HashSet<String> = HashSet::new();

        for unset_var in &da.possibly_unset {
            if self.is_ignored(&unset_var.name) {
                continue;
            }
            if reported.contains(&unset_var.name) {
                continue;
            }
            reported.insert(unset_var.name.clone());

            // If the variable is in the definitely-assigned set at function end
            // but was used before that point, it's USENS (used but not set in
            // all paths). Otherwise it's NODEF (never defined at all on some paths).
            let (rule_id, message) = if da.definitely_assigned.contains(&unset_var.name) {
                (
                    "USENS",
                    format!(
                        "Variable '{}' might not be set in all code paths before this use",
                        unset_var.name
                    ),
                )
            } else {
                (
                    "NODEF",
                    format!(
                        "Variable '{}' might not be defined before this use",
                        unset_var.name
                    ),
                )
            };

            diagnostics.push(Diagnostic {
                rule_id,
                message,
                severity: Severity::Warning,
                byte_range: unset_var.use_byte_range.clone(),
                line: unset_var.use_line,
                column: unset_var.use_column,
                fix: None,
            });
        }
    }

    /// Check that all output arguments are definitely assigned (STOUT).
    fn check_output_args(
        &self,
        scope: &crate::analysis::symbols::Scope,
        _da: &DefiniteAssignment,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        for def in &scope.defs {
            if def.kind != DefKind::OutputArg {
                continue;
            }
            if self.is_ignored(&def.name) {
                continue;
            }

            // Check if the output variable is definitely assigned at function end.
            // The output arg name is pre-added to the assigned set by
            // collect_function_args, so we need to check if there is an actual
            // assignment (not just the output declaration itself). We look for
            // any Assignment-kind def in the scope for this variable name.
            let has_assignment = scope
                .defs
                .iter()
                .any(|d| d.name == def.name && d.kind == DefKind::Assignment);

            // If there's no assignment and the name is not in the definitely-
            // assigned set from the analysis (excluding the initial output arg
            // declaration), emit STOUT.
            if !has_assignment && !self.has_non_output_def(scope, &def.name) {
                diagnostics.push(Diagnostic {
                    rule_id: "STOUT",
                    message: format!(
                        "Output variable '{}' might not be assigned in function '{}'",
                        def.name, scope.name
                    ),
                    severity: Severity::Warning,
                    byte_range: def.byte_range.clone(),
                    line: def.line,
                    column: def.column,
                    fix: None,
                });
            }
        }
    }

    /// Check whether a variable has any non-OutputArg definition in the scope.
    fn has_non_output_def(
        &self,
        scope: &crate::analysis::symbols::Scope,
        name: &str,
    ) -> bool {
        scope
            .defs
            .iter()
            .any(|d| d.name == name && d.kind != DefKind::OutputArg)
    }

    // -----------------------------------------------------------------------
    // Script scope checks
    // -----------------------------------------------------------------------

    /// Check script scopes for variables used before being set.
    fn check_script_scopes(&self, symbol_table: &SymbolTable) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for scope in &symbol_table.scopes {
            if scope.kind != ScopeKind::Script {
                continue;
            }

            // In scripts, variables must be defined before use within the file.
            // We check for uses that appear before any definition.
            let mut reported: HashSet<String> = HashSet::new();

            for var_use in &scope.uses {
                if self.is_ignored(&var_use.name) {
                    continue;
                }
                if reported.contains(&var_use.name) {
                    continue;
                }

                // Find the earliest definition of this variable in the scope.
                let earliest_def = scope
                    .defs
                    .iter()
                    .filter(|d| d.name == var_use.name)
                    .min_by_key(|d| d.byte_range.start);

                match earliest_def {
                    None => {
                        // Variable is used but never defined in the script → SVNODEF.
                        reported.insert(var_use.name.clone());
                        diagnostics.push(Diagnostic {
                            rule_id: "SVNODEF",
                            message: format!(
                                "Variable '{}' in script might not be defined",
                                var_use.name
                            ),
                            severity: Severity::Warning,
                            byte_range: var_use.byte_range.clone(),
                            line: var_use.line,
                            column: var_use.column,
                            fix: None,
                        });
                    }
                    Some(def) if var_use.byte_range.start < def.byte_range.start => {
                        // Variable is used before its first definition → SUSENS.
                        reported.insert(var_use.name.clone());
                        diagnostics.push(Diagnostic {
                            rule_id: "SUSENS",
                            message: format!(
                                "Script variable '{}' is used before it is set",
                                var_use.name
                            ),
                            severity: Severity::Warning,
                            byte_range: var_use.byte_range.clone(),
                            line: var_use.line,
                            column: var_use.column,
                            fix: None,
                        });
                    }
                    _ => {
                        // Variable is used after being defined — OK.
                    }
                }
            }
        }

        diagnostics
    }

    // -----------------------------------------------------------------------
    // PSET: partial-branch assignment check
    // -----------------------------------------------------------------------

    /// Check for variables set in some branches of if/switch but not all (PSET).
    ///
    /// This check walks `if_statement` and `switch_statement` nodes looking for
    /// variables that are assigned in some branches but not others.
    fn check_partial_branch_assignment(
        &self,
        tree: &Tree,
        source: &str,
    ) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        self.walk_pset(tree.root_node(), source, &mut diagnostics);
        diagnostics
    }

    /// DFS walk to find if/switch statements for PSET analysis.
    fn walk_pset(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        match node.kind() {
            "if_statement" => {
                self.check_if_pset(node, source, diagnostics);
            }
            "switch_statement" => {
                self.check_switch_pset(node, source, diagnostics);
            }
            _ => {}
        }

        // Recurse into children.
        let count = node.child_count();
        for i in 0..count {
            if let Some(child) = node.child(i) {
                self.walk_pset(child, source, diagnostics);
            }
        }
    }

    /// Check an if_statement for partial assignment (PSET).
    fn check_if_pset(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut branch_assignments: Vec<HashSet<String>> = Vec::new();
        let mut has_else = false;

        let count = node.child_count();
        for i in 0..count {
            let Some(child) = node.child(i) else {
                continue;
            };
            match child.kind() {
                "block" => {
                    // The main if-body block.
                    branch_assignments.push(collect_block_assignments(child, source));
                }
                "elseif_clause" => {
                    let inner_count = child.child_count();
                    for j in 0..inner_count {
                        if let Some(inner) = child.child(j) {
                            if inner.kind() == "block" {
                                branch_assignments
                                    .push(collect_block_assignments(inner, source));
                                break;
                            }
                        }
                    }
                }
                "else_clause" => {
                    has_else = true;
                    let inner_count = child.child_count();
                    for j in 0..inner_count {
                        if let Some(inner) = child.child(j) {
                            if inner.kind() == "block" {
                                branch_assignments
                                    .push(collect_block_assignments(inner, source));
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Only emit PSET if there's an else branch (complete coverage).
        if !has_else || branch_assignments.len() < 2 {
            return;
        }

        // Find variables assigned in some branches but not all.
        let all_vars: HashSet<String> = branch_assignments
            .iter()
            .flat_map(|s| s.iter().cloned())
            .collect();

        for var_name in &all_vars {
            if self.is_ignored(var_name) {
                continue;
            }
            let assigned_in_all = branch_assignments.iter().all(|s| s.contains(var_name));
            if !assigned_in_all {
                // Find the first branch that assigns this variable for location info.
                if let Some(loc) = self.find_assignment_location(node, source, var_name) {
                    diagnostics.push(Diagnostic {
                        rule_id: "PSET",
                        message: format!(
                            "Variable '{}' is set in some branches but not all",
                            var_name
                        ),
                        severity: Severity::Warning,
                        byte_range: loc.0,
                        line: loc.1,
                        column: loc.2,
                        fix: None,
                    });
                }
            }
        }
    }

    /// Check a switch_statement for partial assignment (PSET).
    fn check_switch_pset(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut branch_assignments: Vec<HashSet<String>> = Vec::new();
        let mut has_otherwise = false;

        let count = node.child_count();
        for i in 0..count {
            let Some(child) = node.child(i) else {
                continue;
            };
            match child.kind() {
                "case_clause" => {
                    let inner_count = child.child_count();
                    for j in 0..inner_count {
                        if let Some(inner) = child.child(j) {
                            if inner.kind() == "block" {
                                branch_assignments
                                    .push(collect_block_assignments(inner, source));
                                break;
                            }
                        }
                    }
                }
                "otherwise_clause" => {
                    has_otherwise = true;
                    let inner_count = child.child_count();
                    for j in 0..inner_count {
                        if let Some(inner) = child.child(j) {
                            if inner.kind() == "block" {
                                branch_assignments
                                    .push(collect_block_assignments(inner, source));
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Only emit PSET if there's an otherwise clause.
        if !has_otherwise || branch_assignments.len() < 2 {
            return;
        }

        let all_vars: HashSet<String> = branch_assignments
            .iter()
            .flat_map(|s| s.iter().cloned())
            .collect();

        for var_name in &all_vars {
            if self.is_ignored(var_name) {
                continue;
            }
            let assigned_in_all = branch_assignments.iter().all(|s| s.contains(var_name));
            if !assigned_in_all {
                if let Some(loc) = self.find_assignment_location(node, source, var_name) {
                    diagnostics.push(Diagnostic {
                        rule_id: "PSET",
                        message: format!(
                            "Variable '{}' is set in some branches but not all",
                            var_name
                        ),
                        severity: Severity::Warning,
                        byte_range: loc.0,
                        line: loc.1,
                        column: loc.2,
                        fix: None,
                    });
                }
            }
        }
    }

    /// Find the location of the first assignment to a variable within a compound
    /// statement (for PSET diagnostic positioning).
    fn find_assignment_location(
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
fn find_function_node_at<'a>(tree: &'a Tree, byte_range: &std::ops::Range<usize>) -> Option<Node<'a>> {
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
fn collect_block_assignments(block: Node, source: &str) -> HashSet<String> {
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

    /// Create a default config for factory tests.
    fn default_config() -> Config {
        Config::default()
    }

    /// Run the unset variables engine on source code and return diagnostics.
    fn lint(source: &str) -> Vec<Diagnostic> {
        let config = default_config();
        let rule = UnsetVariablesEngine::from_config(&config);
        let tree = parse(source);
        let ctx = FileContext {
            tree: &tree,
            source,
            file_path: std::path::Path::new("test.m"),
        };
        rule.check_file(&ctx)
    }

    // -- NODEF tests --------------------------------------------------------

    #[test]
    fn nodef_use_before_assign() {
        let source = "\
function foo()
    y = x + 1;
    x = 5;
end
";
        let diags = lint(source);
        // x is assigned later in the function, so it is "used but not set
        // in all code paths" (USENS) rather than completely undefined.
        assert!(
            diags
                .iter()
                .any(|d| (d.rule_id == "NODEF" || d.rule_id == "USENS")
                    && d.message.contains("'x'")),
            "expected NODEF or USENS for 'x', got: {diags:?}"
        );
    }

    #[test]
    fn nodef_never_defined() {
        let source = "\
function foo()
    y = unknown_var + 1;
end
";
        let diags = lint(source);
        assert!(
            diags
                .iter()
                .any(|d| d.rule_id == "NODEF" && d.message.contains("'unknown_var'")),
            "expected NODEF for 'unknown_var', got: {diags:?}"
        );
    }

    #[test]
    fn nodef_not_fired_when_assigned() {
        let source = "\
function foo()
    x = 5;
    y = x + 1;
end
";
        let diags = lint(source);
        assert!(
            !diags.iter().any(|d| d.rule_id == "NODEF" && d.message.contains("'x'")),
            "should not fire NODEF for 'x' when it is assigned before use"
        );
    }

    // -- USENS tests --------------------------------------------------------

    #[test]
    fn usens_variable_not_set_in_all_paths() {
        let source = "\
function y = foo(x)
    if x > 0
        z = 1;
    end
    y = z;
end
";
        let diags = lint(source);
        // z is assigned later on one path but used before being definitely assigned.
        let has_relevant = diags.iter().any(|d| {
            (d.rule_id == "NODEF" || d.rule_id == "USENS") && d.message.contains("'z'")
        });
        assert!(
            has_relevant,
            "expected NODEF or USENS for 'z', got: {diags:?}"
        );
    }

    // -- STOUT tests --------------------------------------------------------

    #[test]
    fn stout_output_not_assigned() {
        let source = "\
function y = foo(x)
    disp(x);
end
";
        let diags = lint(source);
        assert!(
            diags.iter().any(|d| d.rule_id == "STOUT" && d.message.contains("'y'")),
            "expected STOUT for 'y', got: {diags:?}"
        );
    }

    #[test]
    fn stout_not_fired_when_output_assigned() {
        let source = "\
function y = foo(x)
    y = x + 1;
end
";
        let diags = lint(source);
        assert!(
            !diags.iter().any(|d| d.rule_id == "STOUT" && d.message.contains("'y'")),
            "should not fire STOUT when output is assigned"
        );
    }

    // -- SVNODEF / SUSENS tests (script scope) ------------------------------

    #[test]
    fn svnodef_script_var_never_defined() {
        let source = "y = x + 1;\n";
        let diags = lint(source);
        assert!(
            diags
                .iter()
                .any(|d| d.rule_id == "SVNODEF" && d.message.contains("'x'")),
            "expected SVNODEF for 'x' in script, got: {diags:?}"
        );
    }

    #[test]
    fn susens_script_var_used_before_set() {
        let source = "y = x + 1;\nx = 5;\n";
        let diags = lint(source);
        assert!(
            diags
                .iter()
                .any(|d| d.rule_id == "SUSENS" && d.message.contains("'x'")),
            "expected SUSENS for 'x' used before set, got: {diags:?}"
        );
    }

    #[test]
    fn script_no_warning_when_defined_first() {
        let source = "x = 5;\ny = x + 1;\n";
        let diags = lint(source);
        assert!(
            !diags
                .iter()
                .any(|d| (d.rule_id == "SVNODEF" || d.rule_id == "SUSENS")
                    && d.message.contains("'x'")),
            "should not warn for 'x' in script when defined first"
        );
    }

    // -- PSET tests ---------------------------------------------------------

    #[test]
    fn pset_variable_set_in_some_branches() {
        let source = "\
function foo(x)
    if x > 0
        z = 1;
    else
        w = 2;
    end
end
";
        let diags = lint(source);
        assert!(
            diags.iter().any(|d| d.rule_id == "PSET" && d.message.contains("'z'")),
            "expected PSET for 'z' (set in if but not else), got: {diags:?}"
        );
    }

    #[test]
    fn pset_not_fired_when_set_in_all_branches() {
        let source = "\
function foo(x)
    if x > 0
        z = 1;
    else
        z = 2;
    end
end
";
        let diags = lint(source);
        assert!(
            !diags.iter().any(|d| d.rule_id == "PSET" && d.message.contains("'z'")),
            "should not fire PSET when 'z' is set in all branches"
        );
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
            !diags
                .iter()
                .any(|d| d.message.contains("'pi'")
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
