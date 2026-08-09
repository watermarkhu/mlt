//! # Good Practices: Common best-practice checks for MATLAB code
//!
//! This module implements 32 good-practice checks from MATLAB's Code Analyzer,
//! handled by a single hybrid engine (`GoodPracticesEngine`). Node-level checks
//! cover simple pattern matches (error handling, eval usage, string comparisons),
//! while file-level checks use metadata extraction and the symbol table for
//! context-aware analysis.
//!
//! ## Check IDs
//!
//! ### Error handling
//!
//! | Check ID | Description |
//! |----------|-------------|
//! | TRYNC    | `try` without `catch` |
//! | CTCH     | `catch` block is empty |
//! | WLAST    | `warning` called as last statement in function |
//! | WNTAG    | `warning` without message ID |
//! | ERTAG    | `error` without message ID |
//! | MEXCEP   | `catch` without exception variable |
//!
//! ### String / comparison
//!
//! | Check ID | Description |
//! |----------|-------------|
//! | STCMP    | Use `strcmp`/`strcmpi` instead of `==` for strings |
//! | STCI     | Use `strcmpi` for case-insensitive comparison |
//! | STISA    | Use `isa` instead of `class` + `strcmp` |
//! | STRNU    | Use `str2double` instead of `str2num` |
//!
//! ### Eval / dynamic code
//!
//! | Check ID   | Description |
//! |------------|-------------|
//! | EVLCS      | Avoid `eval` |
//! | EVLDOT     | Avoid `eval` for dynamic field access |
//! | EVLEQ      | Avoid `eval` for dynamic variable creation |
//! | EVLSYS     | Avoid `eval` for system commands |
//! | EVLDUAL    | Avoid `evalin` |
//! | EVLSEQVAR  | Avoid `eval` to create sequential variables |
//!
//! ### General practices
//!
//! | Check ID    | Description |
//! |-------------|-------------|
//! | NOANS       | Statement result assigned to `ans` |
//! | LOAD        | `load` without output variable |
//! | SEPEX       | Multiple statements on one line |
//! | NBRAK1      | Unnecessary brackets around scalar |
//! | LNGNM       | Variable name exceeds length |
//! | CHAIN       | Method chaining on one line |
//! | DISPLAY     | Override `display` is discouraged |
//! | FNDEF       | Function not defined at expected location |
//! | NOIN        | Function has no input validation |
//! | VALST       | Validate function arguments |
//! | PROP        | Property validation missing |
//! | CPROP       | Constant property could be method |
//! | FVAL        | Function value not used |
//! | FNCOLND     | `end` used as column index without dimension |
//! | COMNC       | Comment lacks space after `%` |
//! | ITERS       | Loop variable shadows outer variable |
//! | LOGPROD     | Use `all` instead of `prod` on logical |
//! | LOGMIN      | Use `all` instead of `min` on logical |
//! | LOGMAX      | Use `any` instead of `max` on logical |
//! | ELARLOG     | Element-wise `&`/`|` on logicals in if/while |
//! | SHOCIRAA    | Short-circuit in array context |
//! | UNRPWR      | Power of negative base may be complex |
//! | ADAPPREF    | Avoid `addpref` (use settings) |
//! | KEYBOARDFUN | `keyboard` left in code |
//! | GVMIS       | Global variable used but never declared |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.GOOD_PRACTICES_ENGINE]
//! severity = "warn"
//! max_variable_name_length = 63
//! disabled_checks = []
//! ```

use mlt_core::{Category, Config, Diagnostic, FileContext, NodeContext, Rule, Severity};
use serde::Deserialize;
use tree_sitter::Node;

use crate::analysis::metadata::FileMeta;
use crate::analysis::symbols::SymbolTable;

// ---------------------------------------------------------------------------
// Default configuration values
// ---------------------------------------------------------------------------

const fn default_max_variable_name_length() -> usize {
    63
}

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for the Good Practices engine.
///
/// Deserialized from `[lint.rules.GOOD_PRACTICES_ENGINE]` in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct GoodPracticesConfig {
    /// Maximum variable name length before LNGNM fires.
    #[serde(default = "default_max_variable_name_length")]
    pub max_variable_name_length: usize,

    /// Check IDs to skip (e.g., `["TRYNC", "CTCH"]`).
    #[serde(default)]
    pub disabled_checks: Vec<String>,
}

impl Default for GoodPracticesConfig {
    fn default() -> Self {
        Self {
            max_variable_name_length: default_max_variable_name_length(),
            disabled_checks: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: node text extraction
// ---------------------------------------------------------------------------

/// Extract source text for a tree-sitter node.
fn node_text<'a>(node: Node, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

/// Parent node types indicating statement-level context.
const STATEMENT_PARENTS: &[&str] = &["source_file", "block"];

// ---------------------------------------------------------------------------
// Node-level target types
// ---------------------------------------------------------------------------

/// Node types the engine subscribes to for per-node checks.
const TARGET_NODES: &[&str] = &[
    "function_call",
    "command",
    "try_statement",
    "catch_clause",
    "assignment",
    "binary_operator",
    "boolean_operator",
    "comparison_operator",
    "if_statement",
    "while_statement",
    "for_statement",
    "comment",
    "parenthesized_expression",
];

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// Good Practices Engine — a hybrid rule handling 32+ good-practice checks.
///
/// Node-level checks fire on simple pattern matches during the DFS traversal.
/// File-level checks use [`FileMeta`] and [`SymbolTable`] for context-aware
/// analysis (e.g., last-statement detection, variable shadowing).
pub struct GoodPracticesEngine {
    config: GoodPracticesConfig,
}

impl GoodPracticesEngine {
    /// Factory constructor. Reads rule-specific params from config.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: GoodPracticesConfig = config.rule_params("GOOD_PRACTICES_ENGINE");
        Box::new(Self {
            config: rule_config,
        })
    }

    /// Check whether a specific sub-check ID is enabled.
    fn is_check_enabled(&self, check_id: &str) -> bool {
        !self
            .config
            .disabled_checks
            .iter()
            .any(|id| id == check_id)
    }

    // -----------------------------------------------------------------------
    // Error handling checks (node-level)
    // -----------------------------------------------------------------------

    /// TRYNC: `try` without `catch` clause.
    fn check_trync(&self, node: Node, _source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("TRYNC") || node.kind() != "try_statement" {
            return Vec::new();
        }

        let has_catch = has_child_of_kind(node, "catch_clause");
        if has_catch {
            return Vec::new();
        }

        let pos = node.start_position();
        let keyword_end = node.start_byte() + "try".len();
        vec![Diagnostic {
            rule_id: "TRYNC",
            message: "Try block has no catch clause; errors will be silently ignored".to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..keyword_end,
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// CTCH: `catch` block is empty.
    fn check_ctch(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("CTCH") || node.kind() != "catch_clause" {
            return Vec::new();
        }

        // The catch clause's body is a `block` child. If it is empty
        // (no named children or only whitespace/comments), fire.
        let block = find_child_of_kind(node, "block");
        let is_empty = match block {
            Some(b) => {
                let mut cursor = b.walk();
                let has_statements = b
                    .children(&mut cursor)
                    .any(|c| c.kind() != "comment" && c.is_named());
                !has_statements
            }
            None => true,
        };

        if !is_empty {
            return Vec::new();
        }

        let pos = node.start_position();
        let text = node_text(node, source);
        let keyword_len = text.find(|c: char| c.is_whitespace()).unwrap_or(5);
        vec![Diagnostic {
            rule_id: "CTCH",
            message: "Catch block is empty; errors will be silently swallowed".to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.start_byte() + keyword_len,
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// MEXCEP: `catch` clause without an exception variable.
    fn check_mexcep(&self, node: Node, _source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MEXCEP") || node.kind() != "catch_clause" {
            return Vec::new();
        }

        // If the catch clause has an identifier child directly (the exception var),
        // it is captured. Otherwise, fire.
        let has_exception_var = has_child_of_kind(node, "identifier");
        if has_exception_var {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "MEXCEP",
            message: "Catch clause has no exception variable; use 'catch ME' to capture the error"
                .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.start_byte() + "catch".len(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// WNTAG / ERTAG: `warning`/`error` called without a message ID.
    ///
    /// A message ID is a string argument of the form `'comp:tag'` (contains `:`).
    fn check_warning_error_tag(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match get_function_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };

        let (check_id, label) = match func_name {
            "warning" if self.is_check_enabled("WNTAG") => ("WNTAG", "warning"),
            "error" if self.is_check_enabled("ERTAG") => ("ERTAG", "error"),
            _ => return Vec::new(),
        };

        // Check first argument: it should be a string containing ':' (message ID).
        let args_node = find_child_of_kind(node, "arguments");
        let first_arg = args_node.and_then(|a| first_named_child(a));
        let has_msg_id = first_arg
            .map(|arg| {
                let text = node_text(arg, source);
                // String literals are quoted; check for ':' inside quotes.
                (arg.kind() == "string" || arg.kind() == "string_content")
                    && text.contains(':')
            })
            .unwrap_or(false);

        if has_msg_id {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: check_id,
            message: format!(
                "{label}() called without a message identifier; use {label}('component:id', ...)"
            ),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    // -----------------------------------------------------------------------
    // String / comparison checks (node-level)
    // -----------------------------------------------------------------------

    /// STCMP: Use `strcmp`/`strcmpi` instead of `==` for string comparison.
    fn check_stcmp(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("STCMP") {
            return Vec::new();
        }
        if node.kind() != "comparison_operator" && node.kind() != "binary_operator" {
            return Vec::new();
        }

        let text = node_text(node, source);
        if !text.contains("==") {
            return Vec::new();
        }

        // Check if either operand is a string literal.
        let has_string_operand = node_has_string_child(node);
        if !has_string_operand {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "STCMP",
            message: "Use strcmp() or strcmpi() for string comparison instead of ==".to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// STRNU: Use `str2double` instead of `str2num`.
    fn check_strnu(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("STRNU") || node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match get_function_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };

        if func_name != "str2num" {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "STRNU",
            message: "Use str2double() instead of str2num(); str2num uses eval internally"
                .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    // -----------------------------------------------------------------------
    // Eval / dynamic code checks (node-level)
    // -----------------------------------------------------------------------

    /// EVLCS / EVLDOT / EVLEQ / EVLSYS / EVLSEQVAR / EVLDUAL: `eval`/`evalin` usage.
    fn check_eval(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if node.kind() != "function_call" && node.kind() != "command" {
            return Vec::new();
        }

        let func_name = match node.kind() {
            "function_call" => get_function_call_name(node, source),
            "command" => get_command_name(node, source),
            _ => None,
        };

        let func_name = match func_name {
            Some(n) => n,
            None => return Vec::new(),
        };

        let pos = node.start_position();

        // EVLDUAL: evalin
        if func_name == "evalin" && self.is_check_enabled("EVLDUAL") {
            return vec![Diagnostic {
                rule_id: "EVLDUAL",
                message: "Avoid evalin(); it is slow and hard to debug".to_string(),
                severity: Severity::Warning,
                byte_range: node.start_byte()..node.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            }];
        }

        if func_name != "eval" {
            return Vec::new();
        }

        // Determine which eval sub-check to fire based on argument patterns.
        let args_text = get_arguments_text(node, source);

        let (check_id, message) = if !self.is_check_enabled("EVLCS") {
            return Vec::new();
        } else {
            classify_eval_usage(&args_text, &self.config)
        };

        vec![Diagnostic {
            rule_id: check_id,
            message: message.to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    // -----------------------------------------------------------------------
    // General practice checks (node-level)
    // -----------------------------------------------------------------------

    /// LOAD: `load` called without output variable.
    fn check_load(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("LOAD") {
            return Vec::new();
        }

        // Only fire on statement-level load calls (not `x = load(...)`)
        let is_statement = is_statement_level(node);
        if !is_statement {
            return Vec::new();
        }

        let func_name = match node.kind() {
            "function_call" => get_function_call_name(node, source),
            "command" => get_command_name(node, source),
            _ => return Vec::new(),
        };

        if func_name != Some("load") {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "LOAD",
            message: "load() without output variable creates variables implicitly; use s = load(...)"
                .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// NOANS: Statement result assigned to `ans` implicitly.
    ///
    /// Fires when a `function_call` at statement level has no assignment target
    /// and is not in the ignore list (disp, fprintf, etc.).
    fn check_noans(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("NOANS") || node.kind() != "function_call" {
            return Vec::new();
        }

        if !is_statement_level(node) {
            return Vec::new();
        }

        // If it is an assignment target, skip (the parent is `assignment`).
        if let Some(parent) = node.parent() {
            if parent.kind() == "assignment" {
                return Vec::new();
            }
        }

        // Suppress for known void-return functions.
        let func_name = get_function_call_name(node, source).unwrap_or("");
        let void_functions = [
            "disp", "fprintf", "sprintf", "warning", "error", "close", "clear", "clc", "clf",
            "delete", "mkdir", "rmdir", "cd", "addpath", "rmpath", "save", "fclose", "fopen",
            "pause", "drawnow", "figure", "set", "plot", "hold", "xlabel", "ylabel", "title",
            "legend", "grid", "axis", "subplot",
        ];
        if void_functions.contains(&func_name) {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "NOANS",
            message: "Function result is not assigned to a variable; it will be stored in 'ans'"
                .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// LNGNM: Variable name exceeds maximum length.
    fn check_lngnm(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("LNGNM") || node.kind() != "assignment" {
            return Vec::new();
        }

        let lhs = match node.child_by_field_name("left") {
            Some(l) => l,
            None => return Vec::new(),
        };

        if lhs.kind() != "identifier" {
            return Vec::new();
        }

        let name = node_text(lhs, source);
        if name.len() <= self.config.max_variable_name_length {
            return Vec::new();
        }

        let pos = lhs.start_position();
        vec![Diagnostic {
            rule_id: "LNGNM",
            message: format!(
                "Variable name '{}' is {} characters long; maximum is {}",
                name,
                name.len(),
                self.config.max_variable_name_length
            ),
            severity: Severity::Warning,
            byte_range: lhs.start_byte()..lhs.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// NBRAK1: Unnecessary parentheses around a scalar expression.
    fn check_nbrak1(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("NBRAK1") || node.kind() != "parenthesized_expression" {
            return Vec::new();
        }

        // Check if the inner content is a single scalar (number or identifier).
        let inner = first_named_child(node);
        let is_single_scalar = inner
            .map(|n| {
                let kind = n.kind();
                (kind == "number" || kind == "identifier")
                    && n.next_named_sibling().is_none()
            })
            .unwrap_or(false);

        if !is_single_scalar {
            return Vec::new();
        }

        // Suppress when the parent requires parens (function_call args, etc.).
        if let Some(parent) = node.parent() {
            let pk = parent.kind();
            if pk == "function_call" || pk == "arguments" || pk == "function_arguments" {
                return Vec::new();
            }
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "NBRAK1",
            message: "Unnecessary parentheses around scalar expression".to_string(),
            severity: Severity::Info,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: inner.map(|n| {
                mlt_core::Fix::new(
                    node.start_byte()..node.end_byte(),
                    node_text(n, source),
                )
            }),
        }]
    }

    /// COMNC: Comment lacks space after `%`.
    fn check_comnc(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("COMNC") || node.kind() != "comment" {
            return Vec::new();
        }

        let text = node_text(node, source);

        // Ignore block comments (%{ ... %}) and pragma comments (%#...).
        if text.starts_with("%{") || text.starts_with("%#") || text.starts_with("%%") {
            return Vec::new();
        }

        // Check if `%` is followed by a non-space, non-empty character.
        let after_pct = text.strip_prefix('%').unwrap_or("");
        if after_pct.is_empty() || after_pct.starts_with(' ') || after_pct.starts_with('\t') {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "COMNC",
            message: "Comment should have a space after '%'".to_string(),
            severity: Severity::Info,
            byte_range: node.start_byte()..node.start_byte() + 2,
            line: pos.row + 1,
            column: pos.column + 1,
            fix: Some(mlt_core::Fix::new(
                node.start_byte()..node.start_byte() + 1,
                "% ",
            )),
        }]
    }

    /// ELARLOG: Element-wise `&` / `|` in if/while condition (should use `&&` / `||`).
    fn check_elarlog(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("ELARLOG") {
            return Vec::new();
        }
        if node.kind() != "if_statement" && node.kind() != "while_statement" {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        find_element_wise_boolean_in_condition(node, source, &mut diagnostics);
        diagnostics
    }

    /// LOGPROD / LOGMIN / LOGMAX: Using `prod`/`min`/`max` on logical values.
    fn check_logical_aggregation(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match get_function_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };

        let (check_id, replacement) = match func_name {
            "prod" if self.is_check_enabled("LOGPROD") => ("LOGPROD", "all"),
            "min" if self.is_check_enabled("LOGMIN") => ("LOGMIN", "all"),
            "max" if self.is_check_enabled("LOGMAX") => ("LOGMAX", "any"),
            _ => return Vec::new(),
        };

        // Heuristic: check if the argument is likely logical (named with `is`, `has`,
        // or is a comparison result). We can't know types statically, so this is
        // limited to obvious patterns.
        let args_node = find_child_of_kind(node, "arguments");
        let first_arg = args_node.and_then(|a| first_named_child(a));
        let looks_logical = first_arg
            .map(|arg| {
                let kind = arg.kind();
                if kind == "comparison_operator" || kind == "boolean_operator" {
                    return true;
                }
                if kind == "identifier" {
                    let name = node_text(arg, source);
                    return name.starts_with("is")
                        || name.starts_with("has")
                        || name == "true"
                        || name == "false";
                }
                if kind == "unary_operator" {
                    let text = node_text(arg, source);
                    return text.starts_with('~');
                }
                false
            })
            .unwrap_or(false);

        if !looks_logical {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: check_id,
            message: format!(
                "Use {replacement}() instead of {func_name}() for logical arrays"
            ),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// SHOCIRAA: Short-circuit operator (`&&`/`||`) used where element-wise is expected.
    fn check_shociraa(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("SHOCIRAA") || node.kind() != "boolean_operator" {
            return Vec::new();
        }

        let text = node_text(node, source);
        if !text.contains("&&") && !text.contains("||") {
            return Vec::new();
        }

        // Only fire inside array construction contexts (matrix, cell).
        let in_array_context = is_inside_array_context(node);
        if !in_array_context {
            return Vec::new();
        }

        let op_str = if text.contains("&&") { "&&" } else { "||" };
        let replacement = if op_str == "&&" { "&" } else { "|" };

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "SHOCIRAA",
            message: format!(
                "Short-circuit operator '{op_str}' used in array context; use '{replacement}' instead"
            ),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// UNRPWR: Power of negative base may produce complex result.
    fn check_unrpwr(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("UNRPWR") || node.kind() != "binary_operator" {
            return Vec::new();
        }

        let text = node_text(node, source);
        if !text.contains('^') {
            return Vec::new();
        }

        // Check if the left operand is a negative literal or unary negation.
        let left = node.child_by_field_name("left").or_else(|| node.child(0));
        let has_negative_base = left
            .map(|l| {
                if l.kind() == "unary_operator" {
                    let lt = node_text(l, source);
                    return lt.starts_with('-');
                }
                if l.kind() == "number" {
                    let lt = node_text(l, source);
                    return lt.starts_with('-');
                }
                false
            })
            .unwrap_or(false);

        if !has_negative_base {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "UNRPWR",
            message: "Power of a negative base may produce a complex result; use parentheses to clarify intent".to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// ADAPPREF: Avoid `addpref` (use settings API).
    /// KEYBOARDFUN: `keyboard` left in production code.
    fn check_discouraged_functions(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if node.kind() != "function_call" && node.kind() != "command" {
            return Vec::new();
        }

        let func_name = match node.kind() {
            "function_call" => get_function_call_name(node, source),
            "command" => get_command_name(node, source),
            _ => None,
        };

        let func_name = match func_name {
            Some(n) => n,
            None => return Vec::new(),
        };

        let (check_id, message) = match func_name {
            "addpref" if self.is_check_enabled("ADAPPREF") => (
                "ADAPPREF",
                "Avoid addpref(); use the settings API instead",
            ),
            "keyboard" if self.is_check_enabled("KEYBOARDFUN") => (
                "KEYBOARDFUN",
                "keyboard() left in code; remove before deployment",
            ),
            _ => return Vec::new(),
        };

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: check_id,
            message: message.to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// STCI: Use `strcmpi` for case-insensitive comparison.
    ///
    /// Detects patterns like `lower(s) == '...'` or `strcmp(lower(s), '...')`.
    fn check_stci(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("STCI") || node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match get_function_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };

        if func_name != "strcmp" {
            return Vec::new();
        }

        // Check if either argument is wrapped in lower() or upper().
        let args_node = match find_child_of_kind(node, "arguments") {
            Some(a) => a,
            None => return Vec::new(),
        };

        let has_case_conversion = args_has_case_conversion(args_node, source);
        if !has_case_conversion {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "STCI",
            message: "Use strcmpi() for case-insensitive comparison instead of strcmp(lower(...))"
                .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// STISA: Use `isa` instead of `strcmp(class(obj), '...')`.
    fn check_stisa(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("STISA") || node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match get_function_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };

        if func_name != "strcmp" && func_name != "strcmpi" {
            return Vec::new();
        }

        // Check if either argument is `class(...)`.
        let args_node = match find_child_of_kind(node, "arguments") {
            Some(a) => a,
            None => return Vec::new(),
        };

        let has_class_call = args_has_function_call(args_node, source, "class");
        if !has_class_call {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "STISA",
            message: "Use isa(obj, 'ClassName') instead of strcmp(class(obj), 'ClassName')"
                .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// CHAIN: Method chaining on one line (field_expression chains).
    fn check_chain(&self, node: Node, _source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("CHAIN") || node.kind() != "function_call" {
            return Vec::new();
        }

        // Check if the function_call's name child is a field_expression with deep nesting.
        let name_node = match node.child_by_field_name("name") {
            Some(n) => n,
            None => return Vec::new(),
        };

        if name_node.kind() != "field_expression" {
            return Vec::new();
        }

        let depth = field_chain_depth(name_node);
        if depth < 4 {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "CHAIN",
            message: format!(
                "Method chain depth is {depth}; consider breaking into intermediate variables"
            ),
            severity: Severity::Info,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    // -----------------------------------------------------------------------
    // File-level checks
    // -----------------------------------------------------------------------

    /// WLAST: `warning` is the last statement in a function (may need `error` instead).
    fn check_wlast(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("WLAST") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let mut diagnostics = Vec::new();

        for func in &meta.functions {
            self.check_wlast_in_range(tree.root_node(), source, func.byte_range.clone(), &mut diagnostics);
        }
        for func in &meta.local_functions {
            self.check_wlast_in_range(tree.root_node(), source, func.byte_range.clone(), &mut diagnostics);
        }

        diagnostics
    }

    /// Helper: check if the last statement in a byte range is a `warning()` call.
    fn check_wlast_in_range(
        &self,
        root: Node,
        source: &str,
        range: std::ops::Range<usize>,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // Find the function_definition node covering this range.
        let func_node = find_node_in_range(root, &range);
        let func_node = match func_node {
            Some(n) if n.kind() == "function_definition" => n,
            _ => return,
        };

        // Get the block child (function body).
        let block = match find_child_of_kind(func_node, "block") {
            Some(b) => b,
            None => return,
        };

        // Find the last named non-`end` child.
        let last_stmt = last_named_child_not_end(block);
        let last_stmt = match last_stmt {
            Some(s) => s,
            None => return,
        };

        // Check if it's a warning() call.
        if last_stmt.kind() != "function_call" {
            return;
        }
        let name = get_function_call_name(last_stmt, source);
        if name != Some("warning") {
            return;
        }

        let pos = last_stmt.start_position();
        diagnostics.push(Diagnostic {
            rule_id: "WLAST",
            message: "warning() is the last statement in this function; did you mean error()?"
                .to_string(),
            severity: Severity::Warning,
            byte_range: last_stmt.start_byte()..last_stmt.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        });
    }

    /// DISPLAY: Overriding `display` is discouraged (use `disp` instead).
    fn check_display_override(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("DISPLAY") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let mut diagnostics = Vec::new();

        for func in &meta.functions {
            if func.is_method && func.name == "display" {
                diagnostics.push(Diagnostic {
                    rule_id: "DISPLAY",
                    message: "Overriding display() is discouraged; override disp() instead"
                        .to_string(),
                    severity: Severity::Warning,
                    byte_range: func.byte_range.clone(),
                    line: func.line,
                    column: 1,
                    fix: None,
                });
            }
        }

        diagnostics
    }

    /// NOIN: Function has no input validation (no arguments block).
    fn check_noin(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("NOIN") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let mut diagnostics = Vec::new();

        for func in meta.functions.iter().chain(meta.local_functions.iter()) {
            // Skip constructors, setters, getters, and functions with no inputs.
            if func.is_constructor || func.is_setter || func.is_getter {
                continue;
            }
            // Skip abstract methods.
            if func.is_abstract {
                continue;
            }
            // Only flag if the function has inputs but no arguments block.
            if !func.inputs.is_empty() && !func.has_arguments_block {
                diagnostics.push(Diagnostic {
                    rule_id: "NOIN",
                    message: format!(
                        "Function '{}' has input arguments but no 'arguments' validation block",
                        func.name
                    ),
                    severity: Severity::Info,
                    byte_range: func.byte_range.clone(),
                    line: func.line,
                    column: 1,
                    fix: None,
                });
            }
        }

        diagnostics
    }

    /// PROP: Property without type/size validation.
    fn check_prop_validation(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("PROP") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let mut diagnostics = Vec::new();

        for prop in meta.all_properties() {
            if prop.type_constraint.is_none()
                && prop.validators.is_empty()
                && prop.dimensions.is_none()
            {
                diagnostics.push(Diagnostic {
                    rule_id: "PROP",
                    message: format!(
                        "Property '{}' has no type, size, or validation constraints",
                        prop.name
                    ),
                    severity: Severity::Info,
                    byte_range: prop.byte_range.clone(),
                    line: prop.line,
                    column: 1,
                    fix: None,
                });
            }
        }

        diagnostics
    }

    /// CPROP: Constant property with complex initialization could be a method.
    fn check_cprop(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("CPROP") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let mut diagnostics = Vec::new();

        if let Some(ref class) = meta.class {
            for block in &class.properties_blocks {
                let is_constant = block
                    .attributes
                    .iter()
                    .any(|a| a.name == "Constant" && !a.negated);
                if !is_constant {
                    continue;
                }
                for prop in &block.properties {
                    // Flag if the default value is a function call (complex init).
                    if let Some(ref val) = prop.default_value {
                        if val.contains('(') {
                            diagnostics.push(Diagnostic {
                                rule_id: "CPROP",
                                message: format!(
                                    "Constant property '{}' has a complex default value; consider using a static method",
                                    prop.name
                                ),
                                severity: Severity::Info,
                                byte_range: prop.byte_range.clone(),
                                line: prop.line,
                                column: 1,
                                fix: None,
                            });
                        }
                    }
                }
            }
        }

        diagnostics
    }

    /// ITERS: Loop variable shadows an outer variable.
    fn check_iters(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("ITERS") {
            return Vec::new();
        }

        let sym = SymbolTable::build(tree, source);
        let mut diagnostics = Vec::new();

        for (idx, scope) in sym.scopes.iter().enumerate() {
            // For each for-iterator definition, check if the same name exists
            // in a parent scope.
            for def in &scope.defs {
                if def.kind != crate::analysis::symbols::DefKind::ForIterator {
                    continue;
                }
                // Walk parent scopes.
                let mut parent_idx = scope.parent;
                while let Some(pidx) = parent_idx {
                    let parent_scope = &sym.scopes[pidx];
                    // Skip if it's the same scope index (shouldn't happen).
                    if pidx == idx {
                        break;
                    }
                    if parent_scope.is_defined(&def.name) {
                        diagnostics.push(Diagnostic {
                            rule_id: "ITERS",
                            message: format!(
                                "Loop variable '{}' shadows a variable in an outer scope",
                                def.name
                            ),
                            severity: Severity::Warning,
                            byte_range: def.byte_range.clone(),
                            line: def.line,
                            column: def.column,
                            fix: None,
                        });
                        break;
                    }
                    parent_idx = parent_scope.parent;
                }
            }
        }

        diagnostics
    }

    /// GVMIS: Global variable used but never declared.
    fn check_gvmis(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("GVMIS") {
            return Vec::new();
        }

        let sym = SymbolTable::build(tree, source);
        let mut diagnostics = Vec::new();

        // Collect all global variable declarations across all scopes.
        let mut global_names: std::collections::HashSet<String> = std::collections::HashSet::new();
        for scope in &sym.scopes {
            for def in &scope.defs {
                if def.kind == crate::analysis::symbols::DefKind::Global {
                    global_names.insert(def.name.clone());
                }
            }
        }

        // For each scope, check for usages of variables that are only defined
        // as `global` in other scopes but not declared `global` in this scope.
        for scope in &sym.scopes {
            let local_globals: std::collections::HashSet<&str> = scope
                .defs
                .iter()
                .filter(|d| d.kind == crate::analysis::symbols::DefKind::Global)
                .map(|d| d.name.as_str())
                .collect();

            for usage in &scope.uses {
                if global_names.contains(&usage.name)
                    && !local_globals.contains(usage.name.as_str())
                    && !scope.is_defined(&usage.name)
                {
                    diagnostics.push(Diagnostic {
                        rule_id: "GVMIS",
                        message: format!(
                            "Variable '{}' is used as a global in another scope but not declared global here",
                            usage.name
                        ),
                        severity: Severity::Warning,
                        byte_range: usage.byte_range.clone(),
                        line: usage.line,
                        column: usage.column,
                        fix: None,
                    });
                }
            }
        }

        diagnostics
    }

    /// SEPEX: Multiple statements on one line.
    fn check_sepex(&self, tree: &tree_sitter::Tree, _source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("SEPEX") {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        let mut line_statements: std::collections::HashMap<usize, Vec<(usize, usize)>> =
            std::collections::HashMap::new();

        // Walk all statement-level nodes and group by line.
        collect_statements_by_line(tree.root_node(), &mut line_statements);

        for (line, stmts) in &line_statements {
            if stmts.len() > 1 {
                // Report on the second statement onwards.
                for &(start, end) in &stmts[1..] {
                    diagnostics.push(Diagnostic {
                        rule_id: "SEPEX",
                        message: "Multiple statements on one line; separate for clarity"
                            .to_string(),
                        severity: Severity::Info,
                        byte_range: start..end,
                        line: *line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }

        diagnostics
    }

    /// FNDEF: Function not defined at expected location.
    fn check_fndef(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("FNDEF") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let mut diagnostics = Vec::new();

        // In a function file, check that the main function name matches the file name.
        if let Some(main_func) = meta.main_function() {
            // We don't have the file name in FileContext, so skip file-name matching.
            // Instead check that local functions come after the main function.
            for local in &meta.local_functions {
                if local.line < main_func.line {
                    diagnostics.push(Diagnostic {
                        rule_id: "FNDEF",
                        message: format!(
                            "Local function '{}' defined before the main function",
                            local.name
                        ),
                        severity: Severity::Warning,
                        byte_range: local.byte_range.clone(),
                        line: local.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }

        diagnostics
    }

    /// VALST: Function uses `nargin`/`nargout` checks instead of arguments block.
    fn check_valst(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("VALST") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let sym = SymbolTable::build(tree, source);
        let mut diagnostics = Vec::new();

        for func in meta.functions.iter().chain(meta.local_functions.iter()) {
            if func.has_arguments_block || func.is_abstract {
                continue;
            }

            // Check if the function uses nargin/nargout for validation.
            if let Some(scope) = sym.scope_at(func.byte_range.start) {
                let uses_nargin = scope.is_used("nargin") || scope.is_used("nargout");
                if uses_nargin {
                    diagnostics.push(Diagnostic {
                        rule_id: "VALST",
                        message: format!(
                            "Function '{}' uses nargin/nargout for validation; consider an 'arguments' block",
                            func.name
                        ),
                        severity: Severity::Info,
                        byte_range: func.byte_range.clone(),
                        line: func.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }

        diagnostics
    }

    /// FVAL: Function return value not used (assigned but never read).
    fn check_fval(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("FVAL") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let sym = SymbolTable::build(tree, source);
        let mut diagnostics = Vec::new();

        for func in meta.functions.iter().chain(meta.local_functions.iter()) {
            // Check each output argument: is it defined but never read in the function?
            if let Some(scope) = sym.scope_at(func.byte_range.start) {
                for output_name in &func.outputs {
                    if output_name == "~" {
                        continue;
                    }
                    // Output args are always "defined" as OutputArg. Check if also assigned.
                    let assigned = scope.defs.iter().any(|d| {
                        d.name == *output_name
                            && d.kind == crate::analysis::symbols::DefKind::Assignment
                    });
                    if !assigned && !scope.is_used(output_name) {
                        diagnostics.push(Diagnostic {
                            rule_id: "FVAL",
                            message: format!(
                                "Output variable '{}' is declared but never assigned in function '{}'",
                                output_name, func.name
                            ),
                            severity: Severity::Warning,
                            byte_range: func.byte_range.clone(),
                            line: func.line,
                            column: 1,
                            fix: None,
                        });
                    }
                }
            }
        }

        diagnostics
    }

    /// FNCOLND: `end` used as column index without explicit dimension specification.
    fn check_fncolnd(&self, tree: &tree_sitter::Tree, _source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("FNCOLND") {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        find_end_as_index(tree.root_node(), &mut diagnostics);
        diagnostics
    }
}

// ---------------------------------------------------------------------------
// Rule trait implementation
// ---------------------------------------------------------------------------

impl Rule for GoodPracticesEngine {
    fn id(&self) -> &'static str {
        "GOOD_PRACTICES_ENGINE"
    }

    fn description(&self) -> &'static str {
        "Common best-practice checks for MATLAB code"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn category(&self) -> Category {
        Category::GoodPractices
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        TARGET_NODES
    }

    fn has_file_check(&self) -> bool {
        true
    }

    fn check(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        let node = ctx.node;
        let source = ctx.source;
        let mut diagnostics = Vec::new();

        // Error handling checks.
        diagnostics.extend(self.check_trync(node, source));
        diagnostics.extend(self.check_ctch(node, source));
        diagnostics.extend(self.check_mexcep(node, source));
        diagnostics.extend(self.check_warning_error_tag(node, source));

        // String / comparison checks.
        diagnostics.extend(self.check_stcmp(node, source));
        diagnostics.extend(self.check_strnu(node, source));
        diagnostics.extend(self.check_stci(node, source));
        diagnostics.extend(self.check_stisa(node, source));

        // Eval / dynamic code checks.
        diagnostics.extend(self.check_eval(node, source));

        // General practice checks.
        diagnostics.extend(self.check_load(node, source));
        diagnostics.extend(self.check_noans(node, source));
        diagnostics.extend(self.check_lngnm(node, source));
        diagnostics.extend(self.check_nbrak1(node, source));
        diagnostics.extend(self.check_comnc(node, source));
        diagnostics.extend(self.check_elarlog(node, source));
        diagnostics.extend(self.check_logical_aggregation(node, source));
        diagnostics.extend(self.check_shociraa(node, source));
        diagnostics.extend(self.check_unrpwr(node, source));
        diagnostics.extend(self.check_discouraged_functions(node, source));
        diagnostics.extend(self.check_chain(node, source));

        diagnostics
    }

    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        diagnostics.extend(self.check_wlast(ctx.tree, ctx.source));
        diagnostics.extend(self.check_display_override(ctx.tree, ctx.source));
        diagnostics.extend(self.check_noin(ctx.tree, ctx.source));
        diagnostics.extend(self.check_prop_validation(ctx.tree, ctx.source));
        diagnostics.extend(self.check_cprop(ctx.tree, ctx.source));
        diagnostics.extend(self.check_iters(ctx.tree, ctx.source));
        diagnostics.extend(self.check_gvmis(ctx.tree, ctx.source));
        diagnostics.extend(self.check_sepex(ctx.tree, ctx.source));
        diagnostics.extend(self.check_fndef(ctx.tree, ctx.source));
        diagnostics.extend(self.check_valst(ctx.tree, ctx.source));
        diagnostics.extend(self.check_fval(ctx.tree, ctx.source));
        diagnostics.extend(self.check_fncolnd(ctx.tree, ctx.source));

        diagnostics
    }
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "GOOD_PRACTICES_ENGINE",
    GoodPracticesEngine::from_config
));

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Check whether a node has a child of the given kind.
fn has_child_of_kind(node: Node, kind: &str) -> bool {
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            if child.kind() == kind {
                return true;
            }
        }
    }
    false
}

/// Find the first child of a node with the given kind.
fn find_child_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            if child.kind() == kind {
                return Some(child);
            }
        }
    }
    None
}

/// Get the first named child of a node.
fn first_named_child(node: Node) -> Option<Node> {
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            if child.is_named() {
                return Some(child);
            }
        }
    }
    None
}

/// Get the last named child that is not `end`.
fn last_named_child_not_end(node: Node) -> Option<Node> {
    let mut result = None;
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            if child.is_named() && child.kind() != "end" {
                result = Some(child);
            }
        }
    }
    result
}

/// Check if a node is at statement level (parent is `source_file` or `block`).
fn is_statement_level(node: Node) -> bool {
    node.parent()
        .map(|p| STATEMENT_PARENTS.contains(&p.kind()))
        .unwrap_or(false)
}

/// Extract the function name from a `function_call` node.
fn get_function_call_name<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    if node.kind() != "function_call" {
        return None;
    }
    let name_node = node.child_by_field_name("name")?;
    Some(node_text(name_node, source))
}

/// Extract the command name from a `command` node.
fn get_command_name<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    if node.kind() != "command" {
        return None;
    }
    let first = node.child(0)?;
    if first.kind() == "command_name" {
        Some(node_text(first, source))
    } else {
        None
    }
}

/// Get the full arguments text from a function_call node.
fn get_arguments_text<'a>(node: Node<'a>, source: &'a str) -> String {
    find_child_of_kind(node, "arguments")
        .map(|a| node_text(a, source).to_string())
        .unwrap_or_default()
}

/// Check if a node has any string-literal children (for STCMP).
fn node_has_string_child(node: Node) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        if kind == "string" || kind == "char_vector" {
            return true;
        }
        // Also check for string literal wrapped in parens, etc.
        if child.is_named() && node_has_string_child(child) {
            return true;
        }
    }
    false
}

/// Classify the type of eval usage based on argument text patterns.
fn classify_eval_usage(args_text: &str, config: &GoodPracticesConfig) -> (&'static str, &'static str) {
    // EVLDOT: dynamic field access patterns like `eval(['s.' fieldname])`
    if (args_text.contains("s.") || args_text.contains(".("))
        && config.disabled_checks.iter().all(|c| c != "EVLDOT")
    {
        return (
            "EVLDOT",
            "Avoid eval() for dynamic field access; use s.(fieldname) instead",
        );
    }

    // EVLSYS: system command patterns
    if (args_text.contains("system") || args_text.contains("dos") || args_text.contains("unix"))
        && config.disabled_checks.iter().all(|c| c != "EVLSYS")
    {
        return (
            "EVLSYS",
            "Avoid eval() for system commands; use system() directly",
        );
    }

    // EVLEQ: dynamic variable creation like `eval([varname ' = ...'])`
    if args_text.contains(" = ") && config.disabled_checks.iter().all(|c| c != "EVLEQ") {
        return (
            "EVLEQ",
            "Avoid eval() for dynamic variable creation; use containers.Map or struct fields",
        );
    }

    // EVLSEQVAR: sequential variable creation like `eval(['x' num2str(i)])`
    if (args_text.contains("num2str") || args_text.contains("int2str"))
        && config.disabled_checks.iter().all(|c| c != "EVLSEQVAR")
    {
        return (
            "EVLSEQVAR",
            "Avoid eval() to create sequential variables; use cell arrays or struct fields",
        );
    }

    // EVLCS: general eval usage (fallback)
    (
        "EVLCS",
        "Avoid eval(); it is slow, hard to debug, and a security risk",
    )
}

/// Check if arguments contain a `lower()` or `upper()` call (for STCI).
fn args_has_case_conversion(args_node: Node, source: &str) -> bool {
    let mut cursor = args_node.walk();
    for child in args_node.children(&mut cursor) {
        if child.kind() == "function_call" {
            if let Some(name) = get_function_call_name(child, source) {
                if name == "lower" || name == "upper" {
                    return true;
                }
            }
        }
    }
    false
}

/// Check if arguments contain a call to a specific function (for STISA).
fn args_has_function_call(args_node: Node, source: &str, target_name: &str) -> bool {
    let mut cursor = args_node.walk();
    for child in args_node.children(&mut cursor) {
        if child.kind() == "function_call" {
            if let Some(name) = get_function_call_name(child, source) {
                if name == target_name {
                    return true;
                }
            }
        }
    }
    false
}

/// Count the depth of a field_expression chain (for CHAIN).
fn field_chain_depth(node: Node) -> usize {
    if node.kind() != "field_expression" {
        return 0;
    }
    // The object child of a field_expression may itself be a field_expression.
    let mut depth = 1;
    if let Some(obj) = node.child_by_field_name("object").or_else(|| node.child(0)) {
        depth += field_chain_depth(obj);
    }
    depth
}

/// Check if a node is inside a matrix or cell context (for SHOCIRAA).
fn is_inside_array_context(node: Node) -> bool {
    let mut current = node.parent();
    while let Some(p) = current {
        let kind = p.kind();
        if kind == "matrix" || kind == "cell" {
            return true;
        }
        if kind == "function_definition" || kind == "source_file" {
            break;
        }
        current = p.parent();
    }
    false
}

/// Find element-wise boolean operators (`&`, `|`) in if/while conditions.
fn find_element_wise_boolean_in_condition(
    node: Node,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // The condition of if/while is typically the first expression child.
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "boolean_operator" {
            let text = node_text(child, source);
            // Single `&` or `|` (not `&&` or `||`)
            let has_element_wise =
                (text.contains('&') && !text.contains("&&"))
                    || (text.contains('|') && !text.contains("||"));
            if has_element_wise {
                let pos = child.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "ELARLOG",
                    message: "Use short-circuit operators (&&, ||) instead of element-wise (&, |) in if/while conditions".to_string(),
                    severity: Severity::Warning,
                    byte_range: child.start_byte()..child.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        }
        // Don't recurse into nested if/while/for/blocks — just check immediate condition.
        if child.kind() == "block" || child.kind() == "elseif_clause" || child.kind() == "else_clause" {
            continue;
        }
        find_element_wise_boolean_in_condition(child, source, diagnostics);
    }
}

/// Find the function_definition node overlapping a byte range.
fn find_node_in_range<'a>(root: Node<'a>, range: &std::ops::Range<usize>) -> Option<Node<'a>> {
    if root.kind() == "function_definition"
        && root.start_byte() == range.start
        && root.end_byte() == range.end
    {
        return Some(root);
    }
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.start_byte() > range.end || child.end_byte() < range.start {
            continue;
        }
        if let Some(found) = find_node_in_range(child, range) {
            return Some(found);
        }
    }
    None
}

/// Collect statements grouped by line number (for SEPEX).
fn collect_statements_by_line(
    node: Node,
    line_statements: &mut std::collections::HashMap<usize, Vec<(usize, usize)>>,
) {
    let kind = node.kind();

    // Only collect at statement level in source_file or block.
    if kind == "source_file" || kind == "block" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let ck = child.kind();
            if ck == ";"
                || ck == ","
                || ck == "comment"
                || ck == "end"
                || ck == "line_continuation"
                || !child.is_named()
            {
                continue;
            }
            let line = child.start_position().row + 1;
            line_statements
                .entry(line)
                .or_default()
                .push((child.start_byte(), child.end_byte()));
        }
    }

    // Recurse into structural nodes.
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let ck = child.kind();
        if ck == "block"
            || ck == "function_definition"
            || ck == "if_statement"
            || ck == "for_statement"
            || ck == "while_statement"
            || ck == "switch_statement"
            || ck == "try_statement"
            || ck == "catch_clause"
            || ck == "methods"
            || ck == "class_definition"
        {
            collect_statements_by_line(child, line_statements);
        }
    }
}

/// Find `end` used as an index argument (FNCOLND).
fn find_end_as_index(node: Node, diagnostics: &mut Vec<Diagnostic>) {
    if node.kind() == "end" {
        // Check if parent is an indexing context (function_call arguments).
        if let Some(parent) = node.parent() {
            if parent.kind() == "arguments" {
                // Check if grandparent is a function_call (which could be indexing).
                if let Some(gp) = parent.parent() {
                    if gp.kind() == "function_call" {
                        // Count the number of argument separators (commas) to determine
                        // if this is a multi-dimensional index. `end` without explicit
                        // dimension is fine for 1-D indexing but ambiguous for multi-D.
                        let comma_count = count_children_of_kind(parent, ",");
                        if comma_count > 0 {
                            let pos = node.start_position();
                            diagnostics.push(Diagnostic {
                                rule_id: "FNCOLND",
                                message: "'end' used in multi-dimensional indexing; specify the dimension explicitly".to_string(),
                                severity: Severity::Warning,
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

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        find_end_as_index(child, diagnostics);
    }
}

/// Count children of a specific kind.
fn count_children_of_kind(node: Node, kind: &str) -> usize {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .filter(|c| c.kind() == kind)
        .count()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    /// Parse MATLAB source and return the tree.
    fn parse(source: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_matlab::LANGUAGE.into())
            .expect("failed to load tree-sitter-matlab");
        parser.parse(source, None).expect("parse failed")
    }

    /// Create an engine with default config.
    fn engine() -> GoodPracticesEngine {
        GoodPracticesEngine {
            config: GoodPracticesConfig::default(),
        }
    }

    // -- TRYNC: try without catch ------------------------------------------

    #[test]
    fn test_trync_fires_on_try_without_catch() {
        let source = "try\n    x = 1;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        // Find the try_statement node.
        let try_node = find_child_of_kind(root, "try_statement").unwrap();
        let diags = eng.check_trync(try_node, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "TRYNC");
    }

    #[test]
    fn test_trync_silent_on_try_with_catch() {
        let source = "try\n    x = 1;\ncatch ME\n    disp(ME);\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let try_node = find_child_of_kind(root, "try_statement").unwrap();
        let diags = eng.check_trync(try_node, source);
        assert!(diags.is_empty());
    }

    // -- CTCH: empty catch -------------------------------------------------

    #[test]
    fn test_ctch_fires_on_empty_catch() {
        let source = "try\n    x = 1;\ncatch\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let try_node = find_child_of_kind(root, "try_statement").unwrap();
        let catch_node = find_child_of_kind(try_node, "catch_clause").unwrap();
        let diags = eng.check_ctch(catch_node, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "CTCH");
    }

    // -- MEXCEP: catch without exception variable --------------------------

    #[test]
    fn test_mexcep_fires_without_variable() {
        let source = "try\n    x = 1;\ncatch\n    disp('err');\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let try_node = find_child_of_kind(root, "try_statement").unwrap();
        let catch_node = find_child_of_kind(try_node, "catch_clause").unwrap();
        let diags = eng.check_mexcep(catch_node, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MEXCEP");
    }

    // -- EVLCS: eval usage -------------------------------------------------

    #[test]
    fn test_evlcs_fires_on_eval() {
        let source = "eval('x = 1');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_child_of_kind(root, "function_call").unwrap();
        let diags = eng.check_eval(fc, source);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].rule_id.starts_with("EVL"));
    }

    // -- LOAD: load without output -----------------------------------------

    #[test]
    fn test_load_fires_without_output() {
        let source = "load('data.mat');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_child_of_kind(root, "function_call").unwrap();
        let diags = eng.check_load(fc, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "LOAD");
    }

    #[test]
    fn test_load_silent_with_output() {
        let source = "s = load('data.mat');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        // The function_call is inside an assignment, not at statement level.
        let assignment = find_child_of_kind(root, "assignment").unwrap();
        let fc = find_child_of_kind(assignment, "function_call");
        if let Some(fc) = fc {
            let diags = eng.check_load(fc, source);
            assert!(diags.is_empty());
        }
    }

    // -- COMNC: comment without space after % ------------------------------

    #[test]
    fn test_comnc_fires_on_no_space() {
        let source = "%comment without space\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comment = find_child_of_kind(root, "comment").unwrap();
        let diags = eng.check_comnc(comment, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "COMNC");
    }

    #[test]
    fn test_comnc_silent_with_space() {
        let source = "% comment with space\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comment = find_child_of_kind(root, "comment").unwrap();
        let diags = eng.check_comnc(comment, source);
        assert!(diags.is_empty());
    }

    // -- KEYBOARDFUN: keyboard left in code --------------------------------

    #[test]
    fn test_keyboardfun_fires() {
        let source = "keyboard;\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        // `keyboard` may be parsed as a command or function_call depending on grammar.
        let node = find_child_of_kind(root, "function_call")
            .or_else(|| find_child_of_kind(root, "command"));
        let node = node.expect("keyboard should parse as function_call or command");
        let diags = eng.check_discouraged_functions(node, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "KEYBOARDFUN");
    }

    // -- STRNU: str2num usage ----------------------------------------------

    #[test]
    fn test_strnu_fires_on_str2num() {
        let source = "x = str2num('123');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        // Find the function_call inside the assignment.
        let assignment = find_child_of_kind(root, "assignment").unwrap();
        let mut cursor = assignment.walk();
        let fc = assignment
            .children(&mut cursor)
            .find(|c| c.kind() == "function_call");
        if let Some(fc) = fc {
            let diags = eng.check_strnu(fc, source);
            assert_eq!(diags.len(), 1);
            assert_eq!(diags[0].rule_id, "STRNU");
        }
    }

    // -- Disabled check ----------------------------------------------------

    #[test]
    fn test_disabled_check_skipped() {
        let eng = GoodPracticesEngine {
            config: GoodPracticesConfig {
                max_variable_name_length: 63,
                disabled_checks: vec!["TRYNC".to_string()],
            },
        };

        let source = "try\n    x = 1;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let try_node = find_child_of_kind(root, "try_statement").unwrap();
        let diags = eng.check_trync(try_node, source);
        assert!(diags.is_empty());
    }
}
