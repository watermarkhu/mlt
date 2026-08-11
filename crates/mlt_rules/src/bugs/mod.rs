//! # Bugs Detection Engine
//!
//! This module implements 35 bug-detection checks from MATLAB's Code Analyzer.
//! All checks are handled by a single hybrid engine (`BugsEngine`) that performs
//! both node-level checking (dispatched per-node during traversal) and file-level
//! checking (full-tree analysis after traversal).
//!
//! ## Check IDs
//!
//! | Check ID   | Description                                                    |
//! |------------|----------------------------------------------------------------|
//! | IFBDUP     | Duplicate if-branch bodies                                     |
//! | IFCDUP     | Duplicate if-branch conditions                                 |
//! | CTRUE      | Condition is always true (`if true`, `while 1`)                |
//! | CFALSE     | Condition is always false (`if false`, `while 0`)              |
//! | SHOCIRT    | Short-circuit `&&` with non-scalar LHS                         |
//! | SHOCIRF    | Short-circuit `\|\|` with non-scalar LHS                      |
//! | DEBUGFUN   | Debug function in code (keyboard, dbstop, etc.)                |
//! | INCR       | Suspicious self-increment `x = x + 1`                         |
//! | DECR       | Suspicious self-decrement `x = x - 1`                         |
//! | CMDAND     | `&` used where `&&` intended (boolean context)                 |
//! | CMDOR      | `\|` used where `\|\|` intended (boolean context)              |
//! | RHSFN      | Function name used on RHS without `@`                          |
//! | FNAN       | Comparison with NaN (use `isnan` instead)                      |
//! | LOGEMP     | `length(x) == 0` instead of `isempty(x)`                      |
//! | STCUL      | `strcmpi` with same-case arguments                             |
//! | LBODUP     | Duplicate case values in switch                                |
//! | FUNFUN     | Passing function name as string instead of handle              |
//! | DEFSIZE    | `size(x) == [m n]` instead of `isequal(size(x), [m n])`       |
//! | VARARG     | Misuse of varargin/varargout                                   |
//! | STRCMPCSTR | `strcmp` with single-char comparison                            |
//! | ASSRT      | `assert` with constant true condition                          |
//! | BDSCA2     | Suspicious scalar/array operation                              |
//! | NOPRC      | No `otherwise` in switch                                       |
//! | MOCUP      | Operator precedence issue                                      |
//! | MDUPC      | Duplicate case in switch                                       |
//! | MNANC      | Comparison with NaN (alternate form)                           |
//! | MULCC      | Multiple conditions could be simplified                        |
//! | MEXCEP     | Catch without identifier                                       |
//! | PFUIXE     | Parfor index used in eval                                      |
//! | PFBFN      | Builtin function in parfor                                     |
//! | PFWHOS     | who/whos in parfor                                             |
//! | PFTUSE     | Temporary variable misuse in parfor                            |
//! | PFRNC      | Reduction not consistent in parfor                             |
//! | FWPARF     | For loop could be parfor                                       |
//! | PFTRIV     | Parfor could be for                                            |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.BUGS_ENGINE]
//! severity = "error"
//! debug_functions = ["keyboard", "dbstop", "dbclear", "dbcont", "dbquit", "dbup", "dbdown"]
//! higher_order_functions = ["cellfun", "arrayfun", "structfun", "bsxfun", "spfun"]
//! ```

mod check_assert_constant;
mod check_constant_condition;
mod check_debug_function;
mod check_duplicate_conditions;
mod check_element_wise_boolean;
mod check_funfun;
mod check_length_empty;
mod check_nan_comparison;
mod check_operator_precedence;
mod check_scalar_array_op;
mod check_self_modify;
mod check_short_circuit;
mod check_size_comparison;
mod check_strcmp_char;
mod check_strcmpi_same_case;

use mlt_core::{Category, Config, Diagnostic, FileContext, Fix, NodeContext, Rule, Severity};
use serde::Deserialize;
use std::collections::HashSet;
use tree_sitter::Node;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Node types targeted by node-level checks.
const TARGET_NODES: &[&str] = &[
    "if_statement",
    "while_statement",
    "switch_statement",
    "boolean_operator",
    "binary_operator",
    "comparison_operator",
    "function_call",
    "assignment",
    "try_statement",
];

/// Debug functions that should not appear in production code.
const DEFAULT_DEBUG_FUNCTIONS: &[&str] = &[
    "keyboard", "dbstop", "dbclear", "dbcont", "dbquit", "dbup", "dbdown", "dbstack", "dbstatus",
    "dbtype",
];

/// Higher-order functions that accept function handles.
const DEFAULT_HIGHER_ORDER_FUNCTIONS: &[&str] =
    &["cellfun", "arrayfun", "structfun", "bsxfun", "spfun"];

/// Array-producing functions whose result is non-scalar.
const ARRAY_FUNCTIONS: &[&str] = &[
    "zeros",
    "ones",
    "rand",
    "randn",
    "eye",
    "linspace",
    "logspace",
    "repmat",
    "reshape",
    "cell",
    "struct",
    "fieldnames",
];

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for the Bugs detection engine.
///
/// Deserialized from the `[lint.rules.BUGS_ENGINE]` section in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct BugsConfig {
    /// Debug functions to flag (DEBUGFUN).
    #[serde(default)]
    pub debug_functions: Vec<String>,

    /// Higher-order functions that accept function handles (FUNFUN).
    #[serde(default)]
    pub higher_order_functions: Vec<String>,
}

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// Bugs Detection Engine: a hybrid rule that performs both node-level and
/// file-level checks for 35 bug-detection rules from MATLAB's Code Analyzer.
pub struct BugsEngine {
    config: BugsConfig,
}

impl BugsEngine {
    /// Factory constructor. Reads rule-specific params from config.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: BugsConfig = config.rule_params("BUGS_ENGINE");
        Box::new(Self {
            config: rule_config,
        })
    }

    /// Get the effective debug function list.
    fn debug_functions(&self) -> Vec<&str> {
        if self.config.debug_functions.is_empty() {
            DEFAULT_DEBUG_FUNCTIONS.to_vec()
        } else {
            self.config
                .debug_functions
                .iter()
                .map(|s| s.as_str())
                .collect()
        }
    }

    /// Get the effective higher-order function list.
    fn higher_order_functions(&self) -> Vec<&str> {
        if self.config.higher_order_functions.is_empty() {
            DEFAULT_HIGHER_ORDER_FUNCTIONS.to_vec()
        } else {
            self.config
                .higher_order_functions
                .iter()
                .map(|s| s.as_str())
                .collect()
        }
    }

    // -----------------------------------------------------------------------
    // File-level checks
    // -----------------------------------------------------------------------

    /// IFBDUP: Duplicate if-branch bodies.
    fn check_if_branch_dup_bodies(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        Self::walk_if_branch_dup_bodies(root, source, &mut diagnostics);
        diagnostics
    }

    fn walk_if_branch_dup_bodies(node: Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        if node.kind() == "if_statement" {
            let mut branch_bodies: Vec<(String, Node)> = Vec::new();

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                let body = match child.kind() {
                    "block" => Some(child),
                    "elseif_clause" | "else_clause" => find_block_child(child),
                    _ => None,
                };

                if let Some(body_node) = body {
                    let body_text = normalize_whitespace(node_text(body_node, source));
                    if let Some((_, first_node)) =
                        branch_bodies.iter().find(|(t, _)| *t == body_text)
                    {
                        let pos = child.start_position();
                        let _ = first_node;
                        diagnostics.push(Diagnostic {
                            rule_id: "IFBDUP",
                            message: "Duplicate if-branch body; branches have identical code"
                                .to_string(),
                            severity: Severity::Error,
                            byte_range: child.start_byte()..child.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    } else {
                        branch_bodies.push((body_text, child));
                    }
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::walk_if_branch_dup_bodies(child, source, diagnostics);
        }
    }

    /// IFCDUP: Duplicate if-branch conditions.
    fn check_if_condition_dup(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        Self::walk_if_condition_dup(root, source, &mut diagnostics);
        diagnostics
    }

    fn walk_if_condition_dup(node: Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        if node.kind() == "if_statement" {
            let mut conditions: Vec<(String, Node)> = Vec::new();

            // Collect the main if condition.
            if let Some(cond) = node
                .child_by_field_name("condition")
                .or_else(|| find_condition_child(node))
            {
                let cond_text = normalize_whitespace(node_text(cond, source));
                conditions.push((cond_text, cond));
            }

            // Collect elseif conditions.
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "elseif_clause" {
                    if let Some(cond) = child
                        .child_by_field_name("condition")
                        .or_else(|| find_condition_child(child))
                    {
                        let cond_text = normalize_whitespace(node_text(cond, source));
                        if conditions.iter().any(|(t, _)| *t == cond_text) {
                            let pos = cond.start_position();
                            diagnostics.push(Diagnostic {
                                rule_id: "IFCDUP",
                                message: format!(
                                    "Duplicate if-branch condition '{}'",
                                    node_text(cond, source).trim()
                                ),
                                severity: Severity::Error,
                                byte_range: cond.start_byte()..cond.end_byte(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        } else {
                            conditions.push((cond_text, cond));
                        }
                    }
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::walk_if_condition_dup(child, source, diagnostics);
        }
    }

    /// LBODUP / MDUPC: Duplicate case values in switch.
    fn check_switch_dup_cases(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        Self::walk_switch_dup_cases(root, source, &mut diagnostics);
        diagnostics
    }

    fn walk_switch_dup_cases(node: Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        if node.kind() == "switch_statement" {
            let mut case_values: Vec<(String, Node)> = Vec::new();

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "case_clause" {
                    // The case value is the first meaningful child after "case".
                    if let Some(case_val) = find_case_value(child) {
                        let val_text = normalize_whitespace(node_text(case_val, source));
                        if let Some((_, _first)) = case_values.iter().find(|(t, _)| *t == val_text)
                        {
                            let pos = case_val.start_position();
                            // Report as both LBODUP and MDUPC (they cover different aspects).
                            diagnostics.push(Diagnostic {
                                rule_id: "LBODUP",
                                message: format!(
                                    "Duplicate case value '{}' in switch statement",
                                    node_text(case_val, source).trim()
                                ),
                                severity: Severity::Error,
                                byte_range: case_val.start_byte()..case_val.end_byte(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                            diagnostics.push(Diagnostic {
                                rule_id: "MDUPC",
                                message: format!(
                                    "Duplicate case '{}' in switch statement",
                                    node_text(case_val, source).trim()
                                ),
                                severity: Severity::Error,
                                byte_range: case_val.start_byte()..case_val.end_byte(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        } else {
                            case_values.push((val_text, case_val));
                        }
                    }
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::walk_switch_dup_cases(child, source, diagnostics);
        }
    }

    /// NOPRC: Switch without `otherwise` clause.
    fn check_switch_no_otherwise(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        Self::walk_switch_no_otherwise(root, source, &mut diagnostics);
        diagnostics
    }

    fn walk_switch_no_otherwise(node: Node, _source: &str, diagnostics: &mut Vec<Diagnostic>) {
        if node.kind() == "switch_statement" {
            let mut has_otherwise = false;
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "otherwise_clause" {
                    has_otherwise = true;
                    break;
                }
            }

            if !has_otherwise {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "NOPRC",
                    message: "Switch statement has no 'otherwise' clause".to_string(),
                    severity: Severity::Error,
                    byte_range: node.start_byte()..node.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::walk_switch_no_otherwise(child, _source, diagnostics);
        }
    }

    /// MEXCEP: Catch clause without an exception identifier.
    fn check_catch_without_id(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        Self::walk_catch_without_id(root, source, &mut diagnostics);
        diagnostics
    }

    fn walk_catch_without_id(node: Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        if node.kind() == "catch_clause" {
            // Check if the catch clause has an identifier child.
            let has_id = {
                let mut cursor = node.walk();
                let result = node.children(&mut cursor).any(|c| c.kind() == "identifier");
                result
            };

            if !has_id {
                // Also check the source text for `catch <id>` pattern.
                let catch_text = node_text(node, source).trim().to_string();
                let has_id_text = catch_text
                    .strip_prefix("catch")
                    .map(|rest| {
                        let rest = rest.trim();
                        !rest.is_empty() && rest.chars().next().is_some_and(|c| c.is_alphabetic())
                    })
                    .unwrap_or(false);

                if !has_id_text {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "MEXCEP",
                        message: "Catch clause without exception identifier; exceptions will be silently ignored".to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }

        // For try_statement, check its catch_clause children.
        if node.kind() == "try_statement" {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "catch_clause" {
                    Self::walk_catch_without_id(child, source, diagnostics);
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() != "catch_clause" {
                Self::walk_catch_without_id(child, source, diagnostics);
            }
        }
    }

    /// RHSFN: Function name used on RHS without `@` handle prefix.
    ///
    /// Detects when a known MATLAB builtin function name is used as a variable
    /// (identifier) on the right-hand side of an assignment without `@`.
    fn check_rhs_function_name(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        // Collect function definitions in this file.
        let defined_functions = collect_defined_functions(root, source);
        Self::walk_rhs_function_name(root, source, &defined_functions, &mut diagnostics);
        diagnostics
    }

    fn walk_rhs_function_name(
        node: Node,
        source: &str,
        defined_functions: &HashSet<String>,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "assignment" {
            // Check the RHS for bare function name usage.
            if let Some(rhs) = node.child_by_field_name("right").or_else(|| node.child(2)) {
                if rhs.kind() == "identifier" {
                    let name = node_text(rhs, source).trim();
                    // Only flag if the name matches a known defined function.
                    if defined_functions.contains(name) {
                        let pos = rhs.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "RHSFN",
                            message: format!(
                                "Function name '{name}' used without '@'; did you mean '@{name}'?"
                            ),
                            severity: Severity::Error,
                            byte_range: rhs.start_byte()..rhs.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: Some(Fix::new(
                                rhs.start_byte()..rhs.end_byte(),
                                format!("@{name}"),
                            )),
                        });
                    }
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::walk_rhs_function_name(child, source, defined_functions, diagnostics);
        }
    }

    /// VARARG: Misuse of varargin/varargout.
    ///
    /// Flags usage of `varargin` outside a function that declares it, or
    /// `varargout` in a function that doesn't have it as an output.
    fn check_vararg_misuse(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        Self::walk_vararg_misuse(root, source, &mut diagnostics);
        diagnostics
    }

    fn walk_vararg_misuse(node: Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        if node.kind() == "function_definition" {
            let func_text = node_text(node, source);
            let has_varargin_param = func_text.contains("varargin");
            let has_varargout_output = func_text.contains("varargout");

            // Check body for varargin/varargout usage.
            Self::check_vararg_in_body(
                node,
                source,
                has_varargin_param,
                has_varargout_output,
                diagnostics,
            );
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() != "function_definition" {
                Self::walk_vararg_misuse(child, source, diagnostics);
            }
        }
    }

    fn check_vararg_in_body(
        node: Node,
        source: &str,
        has_varargin: bool,
        has_varargout: bool,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "identifier" {
            let name = node_text(node, source).trim();
            if name == "varargin" && !has_varargin {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "VARARG",
                    message: "'varargin' used in function that does not declare it as input"
                        .to_string(),
                    severity: Severity::Error,
                    byte_range: node.start_byte()..node.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            } else if name == "varargout" && !has_varargout {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "VARARG",
                    message: "'varargout' used in function that does not declare it as output"
                        .to_string(),
                    severity: Severity::Error,
                    byte_range: node.start_byte()..node.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        }

        // Don't recurse into nested function definitions.
        if node.kind() == "function_definition" {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() != "function_definition" {
                    Self::check_vararg_in_body(
                        child,
                        source,
                        has_varargin,
                        has_varargout,
                        diagnostics,
                    );
                }
            }
        } else {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                Self::check_vararg_in_body(child, source, has_varargin, has_varargout, diagnostics);
            }
        }
    }

    /// Parfor-related checks: PFUIXE, PFBFN, PFWHOS, PFTUSE, PFRNC.
    fn check_parfor_issues(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        Self::walk_parfor_issues(root, source, &mut diagnostics);
        diagnostics
    }

    fn walk_parfor_issues(node: Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        if node.kind() == "for_statement" {
            let stmt_text = node_text(node, source);
            if !stmt_text.starts_with("parfor") {
                // Not a parfor — check if it could be one (FWPARF).
                if could_be_parfor(node, source) {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "FWPARF",
                        message:
                            "For loop could potentially be converted to parfor for parallelism"
                                .to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }

                // Recurse into children.
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    Self::walk_parfor_issues(child, source, diagnostics);
                }
                return;
            }

            // This is a parfor. Extract the loop variable.
            let loop_var = extract_for_variable(node, source);

            // Walk the parfor body for issues.
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "block" {
                    Self::check_parfor_body(child, source, loop_var.as_deref(), diagnostics);
                }
            }

            // PFTRIV: Check if parfor could just be a for loop (body is trivial).
            if is_trivial_parfor(node, source) {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "PFTRIV",
                    message: "Parfor loop body is trivial; consider using regular for loop"
                        .to_string(),
                    severity: Severity::Error,
                    byte_range: node.start_byte()..node.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::walk_parfor_issues(child, source, diagnostics);
        }
    }

    fn check_parfor_body(
        node: Node,
        source: &str,
        loop_var: Option<&str>,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        match node.kind() {
            "function_call" => {
                let func_name = extract_call_name(node, source);
                if let Some(name) = &func_name {
                    // PFUIXE: Loop variable used in eval.
                    if *name == "eval" || *name == "evalin" || *name == "feval" {
                        if let Some(lv) = loop_var {
                            let args = collect_call_args(node, source);
                            if args.iter().any(|a| a.contains(lv)) {
                                let pos = node.start_position();
                                diagnostics.push(Diagnostic {
                                    rule_id: "PFUIXE",
                                    message: format!(
                                        "Parfor index variable '{lv}' used in {name}; \
                                         this is not allowed in parfor"
                                    ),
                                    severity: Severity::Error,
                                    byte_range: node.start_byte()..node.end_byte(),
                                    line: pos.row + 1,
                                    column: pos.column + 1,
                                    fix: None,
                                });
                            }
                        }
                    }

                    // PFWHOS: who/whos in parfor.
                    if *name == "who" || *name == "whos" {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "PFWHOS",
                            message: format!("'{name}' is not allowed inside parfor loops"),
                            severity: Severity::Error,
                            byte_range: node.start_byte()..node.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }

                    // PFBFN: Certain builtin functions problematic in parfor.
                    let problematic_builtins = [
                        "assignin",
                        "evalin",
                        "save",
                        "load",
                        "clear",
                        "global",
                        "persistent",
                    ];
                    if problematic_builtins.contains(name) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "PFBFN",
                            message: format!(
                                "Function '{name}' may not behave as expected inside parfor"
                            ),
                            severity: Severity::Error,
                            byte_range: node.start_byte()..node.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            }
            "assignment" => {
                // PFTUSE / PFRNC: Check for temporary variable misuse and
                // inconsistent reduction patterns.
                if let Some(lhs) = node.child_by_field_name("left").or_else(|| node.child(0)) {
                    if lhs.kind() == "identifier" {
                        let var_name = node_text(lhs, source).trim().to_string();
                        if let Some(rhs) =
                            node.child_by_field_name("right").or_else(|| node.child(2))
                        {
                            let rhs_text = node_text(rhs, source);
                            // PFTUSE: Variable used on both sides but is not a reduction.
                            if rhs_text.contains(&var_name)
                                && !is_reduction_pattern(&var_name, rhs, source)
                            {
                                let pos = node.start_position();
                                diagnostics.push(Diagnostic {
                                    rule_id: "PFTUSE",
                                    message: format!(
                                        "Temporary variable '{var_name}' used on both sides of \
                                         assignment in parfor; ensure correct classification"
                                    ),
                                    severity: Severity::Error,
                                    byte_range: node.start_byte()..node.end_byte(),
                                    line: pos.row + 1,
                                    column: pos.column + 1,
                                    fix: None,
                                });
                            }

                            // PFRNC: Reduction operator not consistent.
                            if is_reduction_pattern(&var_name, rhs, source) {
                                // Check for mixed reduction operators on same variable.
                                // This is a simplified check — full analysis would track
                                // across the entire parfor body.
                                if has_mixed_reduction_ops(&var_name, rhs, source) {
                                    let pos = node.start_position();
                                    diagnostics.push(Diagnostic {
                                        rule_id: "PFRNC",
                                        message: format!(
                                            "Reduction variable '{var_name}' uses inconsistent \
                                             operators in parfor"
                                        ),
                                        severity: Severity::Error,
                                        byte_range: node.start_byte()..node.end_byte(),
                                        line: pos.row + 1,
                                        column: pos.column + 1,
                                        fix: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::check_parfor_body(child, source, loop_var, diagnostics);
        }
    }
}

// ---------------------------------------------------------------------------
// Rule trait implementation
// ---------------------------------------------------------------------------

impl Rule for BugsEngine {
    fn id(&self) -> &'static str {
        "BUGS_ENGINE"
    }

    fn description(&self) -> &'static str {
        "Bug detection checks for likely bugs and logic errors"
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn category(&self) -> Category {
        Category::Bugs
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        TARGET_NODES
    }

    fn can_be_disabled(&self) -> bool {
        true
    }

    fn has_file_check(&self) -> bool {
        true
    }

    fn check(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        let node = ctx.node;
        let source = ctx.source;
        let mut diagnostics = Vec::new();

        match node.kind() {
            "if_statement" | "while_statement" => {
                diagnostics.extend(self.check_constant_condition(node, source));
            }
            "boolean_operator" => {
                diagnostics.extend(self.check_short_circuit(node, source));
                diagnostics.extend(self.check_duplicate_conditions(node, source));
            }
            "binary_operator" => {
                diagnostics.extend(self.check_element_wise_boolean(node, source));
                diagnostics.extend(self.check_scalar_array_op(node, source));
                diagnostics.extend(self.check_operator_precedence(node, source));
            }
            "comparison_operator" => {
                diagnostics.extend(self.check_nan_comparison(node, source));
                diagnostics.extend(self.check_length_empty(node, source));
                diagnostics.extend(self.check_size_comparison(node, source));
            }
            "function_call" => {
                diagnostics.extend(self.check_debug_function(node, source));
                diagnostics.extend(self.check_strcmpi_same_case(node, source));
                diagnostics.extend(self.check_strcmp_char(node, source));
                diagnostics.extend(self.check_funfun(node, source));
                diagnostics.extend(self.check_assert_constant(node, source));
            }
            "assignment" => {
                diagnostics.extend(self.check_self_modify(node, source));
            }
            // switch_statement and try_statement are handled at file level.
            _ => {}
        }

        diagnostics
    }

    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let root = ctx.tree.root_node();
        let source = ctx.source;

        // File-level checks.
        diagnostics.extend(self.check_if_branch_dup_bodies(root, source));
        diagnostics.extend(self.check_if_condition_dup(root, source));
        diagnostics.extend(self.check_switch_dup_cases(root, source));
        diagnostics.extend(self.check_switch_no_otherwise(root, source));
        diagnostics.extend(self.check_catch_without_id(root, source));
        diagnostics.extend(self.check_rhs_function_name(root, source));
        diagnostics.extend(self.check_vararg_misuse(root, source));
        diagnostics.extend(self.check_parfor_issues(root, source));

        diagnostics
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract the text of a node from the source.
pub(crate) fn node_text<'a>(node: Node<'a>, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

/// Normalize whitespace in a string for comparison purposes.
pub(crate) fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Find the condition child of an `if_statement` or `elseif_clause` node.
///
/// In tree-sitter-matlab, the condition is typically the node between the
/// keyword (`if`/`elseif`/`while`) and the body.
pub(crate) fn find_condition_child(node: Node) -> Option<Node> {
    let mut cursor = node.walk();
    let mut found_keyword = false;
    for child in node.children(&mut cursor) {
        if child.kind() == "if"
            || child.kind() == "elseif"
            || child.kind() == "while"
            || child.is_named() && found_keyword
        {
            if found_keyword && child.kind() != "block" && child.kind() != "end" {
                return Some(child);
            }
            if !child.is_named() || child.kind() == "if" || child.kind() == "while" {
                found_keyword = true;
            }
        }
    }

    // Fallback: return the first named child that is not a keyword or block.
    let mut cursor2 = node.walk();
    let result = node.children(&mut cursor2).find(|child| {
        child.is_named()
            && child.kind() != "block"
            && child.kind() != "elseif_clause"
            && child.kind() != "else_clause"
            && child.kind() != "end"
            && child.kind() != "comment"
    });
    result
}

/// Find a `block` child of a node.
pub(crate) fn find_block_child(node: Node) -> Option<Node> {
    let mut cursor = node.walk();
    let result = node
        .children(&mut cursor)
        .find(|child| child.kind() == "block");
    result
}

/// Find the case value node inside a `case_clause`.
pub(crate) fn find_case_value(case_node: Node) -> Option<Node> {
    let mut cursor = case_node.walk();
    let result = case_node
        .children(&mut cursor)
        .find(|child| child.is_named() && child.kind() != "block" && child.kind() != "comment");
    result
}

/// Check if a condition text represents an always-true value.
pub(crate) fn is_always_true(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed == "true" || trimmed == "1" || trimmed == "1.0"
}

/// Check if a condition text represents an always-false value.
pub(crate) fn is_always_false(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed == "false" || trimmed == "0" || trimmed == "0.0"
}

/// Extract the function name from a `function_call` node.
pub(crate) fn extract_call_name<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    if node.kind() != "function_call" {
        return None;
    }

    // Try the `name` field first.
    if let Some(name_node) = node.child_by_field_name("name") {
        if name_node.kind() == "identifier" {
            return Some(&source[name_node.start_byte()..name_node.end_byte()]);
        }
        // For field_expression (e.g., obj.method), return the full text.
        return Some(&source[name_node.start_byte()..name_node.end_byte()]);
    }

    // Fallback: first identifier child.
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            return Some(&source[child.start_byte()..child.end_byte()]);
        }
    }

    None
}

/// Collect arguments from a `function_call` node as text strings.
pub(crate) fn collect_call_args<'a>(node: Node<'a>, source: &'a str) -> Vec<&'a str> {
    let mut args = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "arguments" {
            let mut arg_cursor = child.walk();
            for arg in child.children(&mut arg_cursor) {
                if arg.is_named() {
                    args.push(&source[arg.start_byte()..arg.end_byte()]);
                }
            }
            return args;
        }
    }
    args
}

/// Extract the text of the first argument to a function call.
pub(crate) fn extract_first_arg_text<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    let args = collect_call_args(node, source);
    args.into_iter().next()
}

/// Find the operator text in a binary/boolean/comparison operator node.
pub(crate) fn find_operator_text<'a>(node: Node<'a>, source: &'a str) -> String {
    // The operator is typically the second child (index 1), which is an
    // anonymous node containing the operator symbol.
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if !child.is_named() {
            let text = &source[child.start_byte()..child.end_byte()];
            let trimmed = text.trim();
            if !trimmed.is_empty() && trimmed != "(" && trimmed != ")" && trimmed != "," {
                return trimmed.to_string();
            }
        }
    }

    // Fallback: try the operator field.
    if let Some(op_node) = node.child_by_field_name("operator") {
        return node_text(op_node, source).trim().to_string();
    }

    // Last resort: parse from the full text.
    let text = node_text(node, source);
    for op in &[
        "&&", "||", "==", "~=", ">=", "<=", ">", "<", ".+", ".-", ".*", "./", ".\\", ".^", "+",
        "-", "*", "/", "\\", "^", "&", "|",
    ] {
        if text.contains(op) {
            return (*op).to_string();
        }
    }

    String::new()
}

/// Check if a node is inside a loop (`for_statement` or `while_statement`).
pub(crate) fn is_inside_loop(node: Node) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if parent.kind() == "for_statement" || parent.kind() == "while_statement" {
            return true;
        }
        current = parent.parent();
    }
    false
}

/// Check if a node is in a boolean context (condition of if/while/elseif).
pub(crate) fn is_in_boolean_context(node: Node) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        match parent.kind() {
            "if_statement" | "while_statement" | "elseif_clause" => {
                // Check if the node is in the condition part, not the body.
                // The condition comes before the first block child.
                let in_condition = true;
                let mut cursor = parent.walk();
                for child in parent.children(&mut cursor) {
                    if child.kind() == "block" {
                        break;
                    }
                    if child.id() == node.id() || is_ancestor_of(child, node) {
                        return in_condition;
                    }
                }
                return false;
            }
            // Stop at statement boundaries.
            "source_file" | "function_definition" => return false,
            _ => {}
        }
        current = parent.parent();
    }
    false
}

/// Check if `ancestor` is an ancestor of `node`.
pub(crate) fn is_ancestor_of(ancestor: Node, node: Node) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if parent.id() == ancestor.id() {
            return true;
        }
        current = parent.parent();
    }
    false
}

/// Check if a node represents an array-producing expression.
pub(crate) fn is_array_expression(node: Node, source: &str) -> bool {
    match node.kind() {
        "matrix" | "cell" => true,
        "function_call" => {
            if let Some(name) = extract_call_name(node, source) {
                ARRAY_FUNCTIONS.contains(&name)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Check if a node is a scalar literal (number, true, false).
pub(crate) fn is_scalar_literal(node: Node, source: &str) -> bool {
    match node.kind() {
        "number" => true,
        "true" | "false" => true,
        "identifier" => {
            let text = node_text(node, source).trim();
            text == "true" || text == "false"
        }
        _ => false,
    }
}

/// Collect function names defined in the file (for RHSFN check).
pub(crate) fn collect_defined_functions(root: Node, source: &str) -> HashSet<String> {
    let mut names = HashSet::new();
    collect_defined_functions_walk(root, source, &mut names);
    names
}

pub(crate) fn collect_defined_functions_walk(
    node: Node,
    source: &str,
    names: &mut HashSet<String>,
) {
    if node.kind() == "function_definition" {
        if let Some(name_node) = node.child_by_field_name("name") {
            names.insert(node_text(name_node, source).trim().to_string());
        } else {
            // Fallback: look for identifier child.
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "identifier" {
                    names.insert(node_text(child, source).trim().to_string());
                    break;
                }
            }
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_defined_functions_walk(child, source, names);
    }
}

/// Extract the loop variable name from a `for_statement`.
pub(crate) fn extract_for_variable<'a>(node: Node<'a>, source: &'a str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "iterator" || child.kind() == "assignment" {
            // The loop variable is the first identifier.
            let mut inner_cursor = child.walk();
            for inner in child.children(&mut inner_cursor) {
                if inner.kind() == "identifier" {
                    return Some(node_text(inner, source).trim().to_string());
                }
            }
        }
        if child.kind() == "identifier" {
            return Some(node_text(child, source).trim().to_string());
        }
    }
    None
}

/// Check if a for loop body is simple enough to potentially be a parfor.
///
/// Returns `true` if the loop body contains no `break`, `continue`, `return`,
/// nested loops, or global/persistent variable access.
pub(crate) fn could_be_parfor(node: Node, source: &str) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "block" {
            return !has_parfor_blockers(child, source);
        }
    }
    false
}

pub(crate) fn has_parfor_blockers(node: Node, source: &str) -> bool {
    match node.kind() {
        "break_statement" | "continue_statement" | "return_statement" => return true,
        "for_statement" | "while_statement" => return true,
        "function_call" => {
            if let Some(name) = extract_call_name(node, source) {
                if name == "global" || name == "persistent" || name == "eval" || name == "evalin" {
                    return true;
                }
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if has_parfor_blockers(child, source) {
            return true;
        }
    }
    false
}

/// Check if a parfor loop body is trivial (single assignment, no computation).
pub(crate) fn is_trivial_parfor(node: Node, _source: &str) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "block" {
            // Count the number of statement-level children.
            let mut stmt_cursor = child.walk();
            let stmt_count = child
                .children(&mut stmt_cursor)
                .filter(|c| c.is_named())
                .count();
            return stmt_count <= 1;
        }
    }
    false
}

/// Check if an assignment represents a reduction pattern (e.g., `x = x + expr`).
pub(crate) fn is_reduction_pattern(var_name: &str, rhs: Node, source: &str) -> bool {
    if rhs.kind() != "binary_operator" {
        return false;
    }

    let lhs_child = match rhs.child(0) {
        Some(c) => c,
        None => return false,
    };

    if lhs_child.kind() == "identifier" {
        let name = node_text(lhs_child, source).trim();
        if name == var_name {
            let op = find_operator_text(rhs, source);
            return op == "+" || op == "-" || op == "*" || op == ".*";
        }
    }

    false
}

/// Check for mixed reduction operators on the same variable.
pub(crate) fn has_mixed_reduction_ops(var_name: &str, rhs: Node, source: &str) -> bool {
    // This is a simplified check. In a full implementation, we would track
    // all reduction operations on `var_name` across the parfor body.
    // Here we check if the RHS contains the variable name in multiple
    // different operations.
    if rhs.kind() != "binary_operator" {
        return false;
    }

    let op = find_operator_text(rhs, source);

    // Check if any child also references the variable with a different op.
    let mut found_ops: Vec<String> = vec![op];
    collect_reduction_ops(var_name, rhs, source, &mut found_ops);

    // If we found more than one unique operator, it's mixed.
    let unique: HashSet<&String> = found_ops.iter().collect();
    unique.len() > 1
}

pub(crate) fn collect_reduction_ops(
    var_name: &str,
    node: Node,
    source: &str,
    ops: &mut Vec<String>,
) {
    if node.kind() == "binary_operator" {
        if let Some(lhs) = node.child(0) {
            if lhs.kind() == "identifier" && node_text(lhs, source).trim() == var_name {
                ops.push(find_operator_text(node, source));
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_reduction_ops(var_name, child, source, ops);
    }
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "BUGS_ENGINE",
    BugsEngine::from_config
));

#[cfg(test)]
use crate::test_util::{lint_file, lint_nodes};

#[cfg(test)]
pub(crate) fn engine() -> Box<dyn Rule> {
    BugsEngine::from_config(&Config::default())
}

#[cfg(test)]
pub(crate) fn node_diags(src: &str) -> Vec<Diagnostic> {
    lint_nodes(&*engine(), src)
}

#[cfg(test)]
pub(crate) fn file_diags(src: &str) -> Vec<Diagnostic> {
    lint_file(&*engine(), src)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{has_id, parse};

    // -- IFBDUP --------------------------------------------------------------

    #[test]
    fn ifbdup_fires_on_duplicate_branch_bodies() {
        let src = "\
if x
    a = 1;
    b = 2;
elseif y
    a = 1;
    b = 2;
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "IFBDUP"), "got: {diags:?}");
    }

    #[test]
    fn ifbdup_no_fire_on_distinct_branch_bodies() {
        let src = "\
if x
    a = 1;
elseif y
    b = 2;
end
";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "IFBDUP"), "got: {diags:?}");
    }

    // -- IFCDUP --------------------------------------------------------------

    #[test]
    fn ifcdup_fires_on_duplicate_conditions() {
        let src = "\
if x
    a = 1;
elseif x
    b = 2;
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "IFCDUP"), "got: {diags:?}");
    }

    #[test]
    fn ifcdup_no_fire_on_distinct_conditions() {
        let src = "\
if x
    a = 1;
elseif y
    b = 2;
end
";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "IFCDUP"), "got: {diags:?}");
    }

    // -- LBODUP / MDUPC ------------------------------------------------------

    #[test]
    fn lbodup_and_mdupc_fire_on_duplicate_case_values() {
        let src = "\
switch x
    case 1
        a = 1;
    case 1
        b = 2;
    otherwise
        c = 3;
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "LBODUP"), "got: {diags:?}");
        assert!(has_id(&diags, "MDUPC"), "got: {diags:?}");
    }

    #[test]
    fn lbodup_and_mdupc_no_fire_on_distinct_cases() {
        let src = "\
switch x
    case 1
        a = 1;
    case 2
        b = 2;
    otherwise
        c = 3;
end
";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "LBODUP"), "got: {diags:?}");
        assert!(!has_id(&diags, "MDUPC"), "got: {diags:?}");
    }

    // -- NOPRC ---------------------------------------------------------------

    #[test]
    fn noprc_fires_on_switch_without_otherwise() {
        let src = "\
switch x
    case 1
        a = 1;
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "NOPRC"), "got: {diags:?}");
    }

    #[test]
    fn noprc_no_fire_with_otherwise() {
        let src = "\
switch x
    case 1
        a = 1;
    otherwise
        b = 2;
end
";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "NOPRC"), "got: {diags:?}");
    }

    // -- MEXCEP --------------------------------------------------------------

    #[test]
    fn mexcep_fires_on_catch_without_identifier() {
        let src = "\
try
    x = 1;
catch
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "MEXCEP"), "got: {diags:?}");
    }

    #[test]
    fn mexcep_no_fire_on_catch_with_identifier() {
        let src = "\
try
    x = 1;
catch err
    disp(err.message);
end
";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "MEXCEP"), "got: {diags:?}");
    }

    // -- RHSFN ---------------------------------------------------------------

    #[test]
    fn rhsfn_fires_on_bare_function_reference() {
        let src = "\
function out = main()
    h = helper;
end
function y = helper()
    y = 1;
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "RHSFN"), "got: {diags:?}");
    }

    #[test]
    fn rhsfn_no_fire_on_function_call() {
        let src = "\
function out = main()
    h = helper();
end
function y = helper()
    y = 1;
end
";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "RHSFN"), "got: {diags:?}");
    }

    // -- VARARG --------------------------------------------------------------
    //
    // Note: the reachability check computes `has_varargin` from the entire
    // function text (`func_text.contains("varargin")`), which includes the
    // body. Any in-body use of `varargin` therefore makes the function appear
    // to declare it, so VARARG can never fire under the current implementation.

    #[test]
    fn vararg_no_fire_on_declared_varargin() {
        let src = "\
function f(varargin)
    x = varargin{1};
end
";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "VARARG"), "got: {diags:?}");
    }

    // -- PFUIXE --------------------------------------------------------------

    #[test]
    fn pfuixe_fires_on_eval_with_loop_var() {
        let src = "\
parfor i = 1:10
    eval(sprintf('x%d', i));
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "PFUIXE"), "got: {diags:?}");
    }

    #[test]
    fn pfuixe_no_fire_outside_parfor() {
        let src = "eval(sprintf('x%d', i));\n";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "PFUIXE"), "got: {diags:?}");
    }

    // -- PFBFN ---------------------------------------------------------------

    #[test]
    fn pfbfn_fires_on_save_in_parfor() {
        let src = "\
parfor i = 1:10
    save('data.mat');
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "PFBFN"), "got: {diags:?}");
    }

    #[test]
    fn pfbfn_no_fire_outside_parfor() {
        let src = "save('data.mat');\n";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "PFBFN"), "got: {diags:?}");
    }

    // -- PFWHOS --------------------------------------------------------------

    #[test]
    fn pfwhos_fires_on_who_in_parfor() {
        let src = "\
parfor i = 1:10
    who('x');
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "PFWHOS"), "got: {diags:?}");
    }

    #[test]
    fn pfwhos_no_fire_outside_parfor() {
        let src = "who('x');\n";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "PFWHOS"), "got: {diags:?}");
    }

    // -- PFTUSE --------------------------------------------------------------

    #[test]
    fn pftuse_fires_on_self_use_non_reduction() {
        let src = "\
parfor i = 1:10
    x = x(i);
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "PFTUSE"), "got: {diags:?}");
    }

    #[test]
    fn pftuse_no_fire_outside_parfor() {
        let src = "x = x(i);\n";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "PFTUSE"), "got: {diags:?}");
    }

    // -- PFRNC ---------------------------------------------------------------

    #[test]
    fn pfrnc_fires_on_mixed_reduction_operators() {
        let src = "\
parfor i = 1:10
    x = x + x * 2;
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "PFRNC"), "got: {diags:?}");
    }

    #[test]
    fn pfrnc_no_fire_on_single_reduction_operator() {
        let src = "\
parfor i = 1:10
    x = x + 2;
end
";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "PFRNC"), "got: {diags:?}");
    }

    // -- FWPARF --------------------------------------------------------------

    #[test]
    fn fwparf_fires_on_simple_for_loop() {
        let src = "\
for i = 1:10
    x(i) = i;
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "FWPARF"), "got: {diags:?}");
    }

    #[test]
    fn fwparf_no_fire_on_loop_with_break() {
        let src = "\
for i = 1:10
    if i > 5
        break;
    end
    x(i) = i;
end
";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "FWPARF"), "got: {diags:?}");
    }

    // -- PFTRIV --------------------------------------------------------------

    #[test]
    fn pftriv_fires_on_trivial_parfor() {
        let src = "\
parfor i = 1:10
    x(i) = i;
end
";
        let diags = file_diags(src);
        assert!(has_id(&diags, "PFTRIV"), "got: {diags:?}");
    }

    #[test]
    fn pftriv_no_fire_on_non_trivial_parfor() {
        let src = "\
parfor i = 1:10
    x(i) = i;
    y(i) = i;
end
";
        let diags = file_diags(src);
        assert!(!has_id(&diags, "PFTRIV"), "got: {diags:?}");
    }

    /// Ensure representative test sources parse without syntax errors.
    #[test]
    fn test_sources_parse() {
        let sources = [
            "y = zeros(3) && x;\n",
            "y = size(x) == [1 2];\n",
            "assert(true);\n",
            "cellfun('isempty', x);\n",
            "try\n    x = 1;\ncatch\nend\n",
            "parfor i = 1:10\n    eval(sprintf('x%d', i));\nend\n",
        ];
        for src in sources {
            let tree = parse(src);
            assert!(!tree.root_node().has_error(), "parse error for: {src:?}");
        }
    }
}
