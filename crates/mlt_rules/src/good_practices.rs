//! # Good Practices: Common best-practice checks for MATLAB code
//!
//! This module implements 43 good-practice checks from MATLAB's Code Analyzer,
//! handled by a single hybrid engine (`GoodPracticesEngine`). Node-level checks
//! cover simple pattern matches (error handling, eval usage, string comparisons,
//! parfor/spmd usage), while file-level checks use metadata extraction and the
//! symbol table for context-aware analysis.
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
//! ### Parfor / SPMD / parallel practices
//!
//! | Check ID | Description |
//! |----------|-------------|
//! | PFEVB     | Using EVALIN('base') or ASSIGNIN('base') inside a PARFOR loop refers to the worker machines' base workspaces |
//! | PFGP      | Avoid assigning to GLOBAL or PERSISTENT variable inside a PARFOR loop |
//! | PFGV      | Avoid using GLOBAL variable in a PARFOR loop |
//! | PFIIN     | The input variable should be initialized before the PARFOR loop |
//! | PFOUS     | The output variable might not be used after the PARFOR loop |
//! | PFRNI     | Do not specify the increment explicitly; parfor can only use an increment of one |
//! | PFTUSW    | The temporary variable might be used after the PARFOR loop |
//! | PFUIXW    | The index variable might be used after the PARFOR loop |
//! | SPEVB     | Using EVALIN('base') or ASSIGNIN('base') inside an SPMD block refers to the worker machines' base workspaces |
//! | SPGV      | Using the GLOBAL or PERSISTENT variable in an SPMD block might fail because it is accessed on a worker machine |
//! | DSPMDA    | Distributed array must be created outside of an SPMD block |
//!
//! ### Structure / string / misc
//!
//! | Check ID | Description |
//! |----------|-------------|
//! | COMFS     | Comma makes the file a script, so functions are local |
//! | DUALC     | Command might be prematurely ended by comma |
//! | RMFLD     | `rmfield` output must be assigned back to the structure |
//! | RMWRN     | Warning tag has been removed from MATLAB |
//! | SEMFS     | Semicolon makes the file a script, so functions are local |
//! | STFLD     | `setfield` output must be assigned back to the structure |
//! | STRSZ     | Use `strcmp` to compare character vectors of different sizes |
//!
//! ### OOP / class / property
//!
//! | Check ID | Description |
//! |----------|-------------|
//! | ATTF     | Unable to determine if the expression assigned to the `Abstract` attribute evaluates to true or false |
//! | ATTOF    | Setting the class attribute `Abstract` to false is not recommended |
//! | MCPO     | `SetObservable`/`GetObservable`/`AbortSet` property has no effect in a value class |
//! | MCSAC    | `SetAccess` cannot be set on Constant properties |
//! | MOBSRV   | `SetObservable`/`GetObservable` on a Constant property has no effect |
//! | MDEPIN   | Default values should not be assigned to dependent properties |
//! | MCCPI    | Initialize the Constant property or make it an Abstract Constant property |
//! | MGMD     | `get` method should be implemented for each dependent property without private `GetAccess` |
//! | MCCPE    | Attempting to call a property or event as a function |
//! | MTHANS   | Using `ANS` as a method name is not recommended |
//! | MHERM    | Parenthesize the multiplication of a variable and its transpose |
//! | MNUML    | Use `VAR_NAME(numel(...), numel(...))` to create a square matrix |
//!
//! ### Logical / comparison / range
//!
//! | Check ID | Description |
//! |----------|-------------|
//! | COMPNOP   | Comparison with `true` simplifies to the function call itself |
//! | COMPNOT   | Comparison with `~= true` or `== false` simplifies to `~call(...)` |
//! | M3COL     | Three colons (`a:b:c:d`) in an expression is probably unintended |
//!
//! ### Function-call conventions
//!
//! | Check ID | Description |
//! |----------|-------------|
//! | CTPCT    | `sprintf`/`fprintf` format might not agree with the argument count |
//! | FXSET    | Loop index variable is changed inside of a `for` loop |
//! | SIMPT    | `import` statement does not run first in a function |
//! | TLEV     | Dynamic-code function used as a sub-expression, not a top-level statement |
//! | UNONC    | `onCleanup` output must be assigned to a variable, not `~` |
//! | MIPC1    | `computer('arch')` is platform-specific |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.GOOD_PRACTICES_ENGINE]
//! severity = "warn"
//! max_variable_name_length = 63
//! disabled_checks = []
//! ```

use mlt_core::{Category, Config, Diagnostic, FileContext, Fix, NodeContext, Rule, Severity};
use serde::Deserialize;
use tree_sitter::Node;

use crate::analysis::metadata::{ClassMeta, FileMeta};
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

/// Warning message ID tags removed from MATLAB; RMWRN fires when a
/// `warning(...)` call uses one of these tags. Currently empty (placeholder):
/// no tags are populated yet, so RMWRN is inert until a tag is added here.
const REMOVED_WARNING_TAGS: &[&str] = &[];

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
    "range",
    "if_statement",
    "while_statement",
    "for_statement",
    "spmd_statement",
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
    // Parfor / SPMD checks (node-level)
    // -----------------------------------------------------------------------

    /// PFRNI: PARFOR loop with an explicitly specified increment.
    ///
    /// A parfor range with three parts (`start:step:end`) is flagged because
    /// parfor only supports an increment of one.
    fn check_pfrni(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("PFRNI") || !is_parfor_node(node, source) {
            return Vec::new();
        }

        let range = match find_iterator_range(node) {
            Some(r) => r,
            None => return Vec::new(),
        };
        if count_named_children(range) != 3 {
            return Vec::new();
        }
        let (Some(first), Some(last)) = (first_named_child(range), last_named_child(range)) else {
            return Vec::new();
        };

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "PFRNI",
            message: "Do not specify the increment explicitly. The parfor loop can only use an increment of one.".to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: Some(mlt_core::Fix::new(
                range.start_byte()..range.end_byte(),
                format!("{}:{}", node_text(first, source), node_text(last, source)),
            )),
        }]
    }

    /// PFEVB: EVALIN('base') or ASSIGNIN('base') inside a PARFOR loop.
    fn check_pfevb(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("PFEVB") || !is_parfor_node(node, source) {
            return Vec::new();
        }

        let body = match find_child_of_kind(node, "block") {
            Some(b) => b,
            None => return Vec::new(),
        };
        let mut calls = Vec::new();
        walk_body_for_function_calls(body, &mut calls);
        calls.retain(|c| is_evalin_assignin_base(*c, source));
        calls
            .into_iter()
            .map(|c| {
                let pos = c.start_position();
                Diagnostic {
                    rule_id: "PFEVB",
                    message: "Using EVALIN('base') or ASSIGNIN('base') inside a PARFOR loop refers to the worker machines' base workspaces.".to_string(),
                    severity: Severity::Warning,
                    byte_range: c.start_byte()..c.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                }
            })
            .collect()
    }

    /// PFGP: assignment to a GLOBAL or PERSISTENT variable inside a PARFOR loop.
    fn check_pfgp(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("PFGP") || !is_parfor_node(node, source) {
            return Vec::new();
        }

        let globals = collect_global_persistent_vars(root_node_of(node), source);
        if globals.is_empty() {
            return Vec::new();
        }
        let body = match find_child_of_kind(node, "block") {
            Some(b) => b,
            None => return Vec::new(),
        };

        let mut assignments = Vec::new();
        collect_nodes_of_kind(body, "assignment", &mut assignments);
        let mut seen = std::collections::HashSet::new();
        let mut diagnostics = Vec::new();
        for assign in assignments {
            let lhs = match assign.child_by_field_name("left") {
                Some(l) => l,
                None => continue,
            };
            let name = match lhs_base_name(lhs, source) {
                Some(n) => n,
                None => continue,
            };
            if !globals.contains(&name) || !seen.insert(name.clone()) {
                continue;
            }
            let pos = lhs.start_position();
            diagnostics.push(Diagnostic {
                rule_id: "PFGP",
                message: format!(
                    "Avoid assigning to GLOBAL or PERSISTENT variable {name} inside a PARFOR loop."
                ),
                severity: Severity::Warning,
                byte_range: lhs.start_byte()..lhs.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            });
        }
        diagnostics
    }

    /// PFGV: use of a GLOBAL variable inside a PARFOR loop.
    fn check_pfgv(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("PFGV") || !is_parfor_node(node, source) {
            return Vec::new();
        }

        let globals = collect_global_persistent_vars(root_node_of(node), source);
        if globals.is_empty() {
            return Vec::new();
        }
        let body = match find_child_of_kind(node, "block") {
            Some(b) => b,
            None => return Vec::new(),
        };

        collect_global_use_diagnostics(body, source, &globals, "PFGV", true)
    }

    /// SPEVB: EVALIN('base') or ASSIGNIN('base') inside an SPMD block.
    fn check_spevb(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("SPEVB") || node.kind() != "spmd_statement" {
            return Vec::new();
        }

        let body = match find_child_of_kind(node, "block") {
            Some(b) => b,
            None => return Vec::new(),
        };
        let mut calls = Vec::new();
        walk_body_for_function_calls(body, &mut calls);
        calls.retain(|c| is_evalin_assignin_base(*c, source));
        calls
            .into_iter()
            .map(|c| {
                let pos = c.start_position();
                Diagnostic {
                    rule_id: "SPEVB",
                    message: "Using EVALIN('base') or ASSIGNIN('base') inside an SPMD block refers to the worker machines' base workspaces.".to_string(),
                    severity: Severity::Warning,
                    byte_range: c.start_byte()..c.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                }
            })
            .collect()
    }

    /// SPGV: use of a GLOBAL or PERSISTENT variable inside an SPMD block.
    fn check_spgv(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("SPGV") || node.kind() != "spmd_statement" {
            return Vec::new();
        }

        let globals = collect_global_persistent_vars(root_node_of(node), source);
        if globals.is_empty() {
            return Vec::new();
        }
        let body = match find_child_of_kind(node, "block") {
            Some(b) => b,
            None => return Vec::new(),
        };

        collect_global_use_diagnostics(body, source, &globals, "SPGV", false)
    }

    /// DSPMDA: distributed array constructed inside an SPMD block.
    fn check_dspmda(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("DSPMDA") || node.kind() != "spmd_statement" {
            return Vec::new();
        }

        let body = match find_child_of_kind(node, "block") {
            Some(b) => b,
            None => return Vec::new(),
        };
        let mut calls = Vec::new();
        walk_body_for_function_calls(body, &mut calls);
        calls.retain(|c| is_distributed_call(*c, source));
        calls
            .into_iter()
            .map(|c| {
                let pos = c.start_position();
                Diagnostic {
                    rule_id: "DSPMDA",
                    message: "Distributed array must be created outside of an SPMD block."
                        .to_string(),
                    severity: Severity::Warning,
                    byte_range: c.start_byte()..c.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                }
            })
            .collect()
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

    // -----------------------------------------------------------------------
    // Function-call convention checks (node-level)
    // -----------------------------------------------------------------------

    /// CTPCT: `sprintf`/`fprintf` format specifier count does not match the
    /// number of remaining arguments.
    fn check_ctpct(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("CTPCT") || node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match get_function_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };
        if func_name != "sprintf" && func_name != "fprintf" {
            return Vec::new();
        }

        let args_node = match find_child_of_kind(node, "arguments") {
            Some(a) => a,
            None => return Vec::new(),
        };

        let mut cursor = args_node.walk();
        let args: Vec<Node> = args_node
            .children(&mut cursor)
            .filter(|c| c.is_named())
            .collect();
        let format_index = match args.iter().position(|a| a.kind() == "string") {
            Some(i) => i,
            None => return Vec::new(),
        };

        let fmt_text = node_text(args[format_index], source).trim_matches('\'');
        let specifiers = count_format_specifiers(fmt_text);
        let arg_count = args.len() - format_index - 1;
        if specifiers == arg_count {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "CTPCT",
            message: "The format might not agree with the argument count.".to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// FXSET: the `for` loop iterator variable is assigned inside the loop body.
    fn check_fxset(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("FXSET") || node.kind() != "for_statement" {
            return Vec::new();
        }

        let iterator = match find_child_of_kind(node, "iterator") {
            Some(i) => i,
            None => return Vec::new(),
        };
        let var_node = match first_named_child(iterator) {
            Some(n) if n.kind() == "identifier" => n,
            _ => return Vec::new(),
        };
        let var_name = node_text(var_node, source).to_string();

        let body = match find_child_of_kind(node, "block") {
            Some(b) => b,
            None => return Vec::new(),
        };

        let mut diagnostics = Vec::new();
        collect_loop_assignments(body, &var_name, source, &mut diagnostics);
        diagnostics
    }

    /// SIMPT: `import` command does not run before any other code in its
    /// enclosing function.
    fn check_simpt(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("SIMPT") || node.kind() != "command" {
            return Vec::new();
        }
        if get_command_name(node, source) != Some("import") {
            return Vec::new();
        }

        // Skip file/script-level imports (no enclosing function).
        let func = match enclosing_function(node) {
            Some(f) => f,
            None => return Vec::new(),
        };
        let block = match find_child_of_kind(func, "block") {
            Some(b) => b,
            None => return Vec::new(),
        };

        // Fire unless the import is the first non-comment statement in the block.
        let mut cursor = block.walk();
        let first_stmt = block
            .children(&mut cursor)
            .find(|c| c.is_named() && c.kind() != "comment");
        if let Some(first) = first_stmt {
            if first.start_byte() == node.start_byte() && first.end_byte() == node.end_byte() {
                return Vec::new();
            }
        }

        let func_name = func
            .child_by_field_name("name")
            .map(|n| node_text(n, source))
            .unwrap_or("");

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "SIMPT",
            message: format!(
                "This import statement runs before any other code in function {func_name}. Consider placing it at the top of the function body."
            ),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// TLEV: dynamic-code function (`eval`/`evalc`/`evalin`/`feval`) used as a
    /// sub-expression rather than a top-level statement.
    fn check_tlev(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("TLEV") || node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match get_function_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };
        if !["eval", "evalc", "evalin", "feval"].contains(&func_name) {
            return Vec::new();
        }

        // A call that is the entire right-hand side of an assignment is still a
        // top-level statement; anything deeper is a sub-expression.
        if let Some(parent) = node.parent() {
            if parent.kind() == "assignment" {
                if let Some(right) = parent.child_by_field_name("right") {
                    if right.start_byte() == node.start_byte()
                        && right.end_byte() == node.end_byte()
                    {
                        return Vec::new();
                    }
                }
            }
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "TLEV",
            message: format!(
                "{func_name} could be very inefficient unless it is a top-level statement in its function."
            ),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// UNONC: `onCleanup` output must be assigned to a variable; `~` is not
    /// permitted in place of a variable.
    fn check_unonc(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("UNONC") || node.kind() != "function_call" {
            return Vec::new();
        }
        if get_function_call_name(node, source) != Some("onCleanup") {
            return Vec::new();
        }

        let parent = match node.parent() {
            Some(p) => p,
            None => return Vec::new(),
        };

        // Bare statement fires; an assignment fires only when `~` is used.
        let tilde_used = if parent.kind() == "assignment" {
            parent
                .child_by_field_name("left")
                .map(|lhs| {
                    lhs.kind() == "ignored_argument"
                        || ((lhs.kind() == "multioutput_variable" || lhs.kind() == "matrix")
                            && has_child_of_kind(lhs, "ignored_argument"))
                })
                .unwrap_or(false)
        } else {
            true
        };

        if !tilde_used {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "UNONC",
            message:
                "Assign the onCleanup output argument to a variable. Do not use the tilde operator (~) in place of a variable."
                    .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// MIPC1: `computer('arch')` returns a platform-specific value.
    fn check_mipc1(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MIPC1") || node.kind() != "function_call" {
            return Vec::new();
        }
        if get_function_call_name(node, source) != Some("computer") {
            return Vec::new();
        }

        let args_node = match find_child_of_kind(node, "arguments") {
            Some(a) => a,
            None => return Vec::new(),
        };
        let mut cursor = args_node.walk();
        let args: Vec<Node> = args_node
            .children(&mut cursor)
            .filter(|c| c.is_named())
            .collect();
        if args.len() != 1 {
            return Vec::new();
        }
        if args[0].kind() != "string" {
            return Vec::new();
        }
        if node_text(args[0], source).trim_matches('\'') != "arch" {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "MIPC1",
            message:
                "Calling the computer function with 'arch' returns 'win64', 'glnxa64', or 'maci64'."
                    .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    // -----------------------------------------------------------------------
    // Structure / string / misc checks (node-level)
    // -----------------------------------------------------------------------

    /// RMFLD: `rmfield` result must be assigned back to the structure.
    fn check_rmfld(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        self.check_struct_assign_back(node, source, "rmfield", "RMFLD")
    }

    /// STFLD: `setfield` result must be assigned back to the structure.
    fn check_stfld(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        self.check_struct_assign_back(node, source, "setfield", "STFLD")
    }

    /// Shared RMFLD/STFLD logic: fire when the call is a standalone statement
    /// (parent is `source_file`/`block`) so its result is silently discarded.
    fn check_struct_assign_back(
        &self,
        node: Node,
        source: &str,
        func_name: &str,
        check_id: &'static str,
    ) -> Vec<Diagnostic> {
        if !self.is_check_enabled(check_id) || node.kind() != "function_call" {
            return Vec::new();
        }
        if get_function_call_name(node, source) != Some(func_name) {
            return Vec::new();
        }
        if !is_statement_level(node) {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: check_id,
            message: format!(
                "{} output must be assigned back to the structure",
                func_name.to_uppercase()
            ),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// RMWRN: `warning` called with a message ID tag that has been removed from MATLAB.
    ///
    /// The tag denylist is [`REMOVED_WARNING_TAGS`], currently empty. The
    /// mechanism is implemented so tags can be populated as removals occur.
    fn check_rmwrn(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("RMWRN") || node.kind() != "function_call" {
            return Vec::new();
        }
        if get_function_call_name(node, source) != Some("warning") {
            return Vec::new();
        }

        let args_node = match find_child_of_kind(node, "arguments") {
            Some(a) => a,
            None => return Vec::new(),
        };
        let first_arg = match first_named_child(args_node) {
            Some(a) => a,
            None => return Vec::new(),
        };
        if first_arg.kind() != "string" {
            return Vec::new();
        }

        let tag = node_text(first_arg, source).trim_matches('\'');
        if !REMOVED_WARNING_TAGS.contains(&tag) {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "RMWRN",
            message: format!(
                "The warning with tag {tag} has been removed from MATLAB, so this statement has no effect."
            ),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// STRSZ: `==`/`~=` between string literals of different sizes.
    fn check_strsz(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("STRSZ") {
            return Vec::new();
        }
        if node.kind() != "comparison_operator" && node.kind() != "binary_operator" {
            return Vec::new();
        }

        let text = node_text(node, source);
        if !text.contains("==") && !text.contains("~=") {
            return Vec::new();
        }

        let mut cursor = node.walk();
        let children: Vec<Node> = node.children(&mut cursor).collect();
        let op_idx = match children
            .iter()
            .position(|c| c.kind() == "==" || c.kind() == "~=")
        {
            Some(i) if i > 0 && i + 1 < children.len() => i,
            _ => return Vec::new(),
        };

        let left = children[op_idx - 1];
        let right = children[op_idx + 1];
        if left.kind() != "string" || right.kind() != "string" {
            return Vec::new();
        }
        if node_text(left, source).len() == node_text(right, source).len() {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "STRSZ",
            message: "Use STRCMP to compare character vectors that can have different sizes."
                .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// DUALC: a command is immediately followed by a comma, which can
    /// prematurely end the command. Heuristic — see docs for limitations.
    fn check_dualc(&self, node: Node, _source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("DUALC") || node.kind() != "command" {
            return Vec::new();
        }

        let comma_after = node
            .next_sibling()
            .map(|n| n.kind() == ",")
            .unwrap_or(false);
        if !comma_after {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "DUALC",
            message: "Command might be prematurely ended by comma.".to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    // -----------------------------------------------------------------------
    // Logical / comparison / range checks (node-level)
    // -----------------------------------------------------------------------

    /// COMPNOP: `call(...) == true` simplifies to `call(...)`.
    fn check_compnop(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("COMPNOP") || node.kind() != "comparison_operator" {
            return Vec::new();
        }

        let Some((call_node, literal_node, op)) = self.comparison_with_literal(node, source) else {
            return Vec::new();
        };
        let literal = node_text(literal_node, source).trim();
        if op != "==" || literal != "true" {
            return Vec::new();
        }

        let call_text = node_text(call_node, source).to_string();
        let func_name = get_function_call_name(call_node, source).unwrap_or("function");
        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "COMPNOP",
            message: format!(
                "This logical comparison simplifies to {call_text}. Did you mean to use {func_name} to evaluate function argument: {call_text}?"
            ),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: Some(Fix::new(
                node.start_byte()..node.end_byte(),
                call_text,
            )),
        }]
    }

    /// COMPNOT: `call(...) ~= true` or `call(...) == false` simplifies to
    /// `~call(...)`.
    fn check_comnot(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("COMPNOT") || node.kind() != "comparison_operator" {
            return Vec::new();
        }

        let Some((call_node, literal_node, op)) = self.comparison_with_literal(node, source) else {
            return Vec::new();
        };
        let literal = node_text(literal_node, source).trim();
        let is_comnot = (op == "~=" && literal == "true") || (op == "==" && literal == "false");
        if !is_comnot {
            return Vec::new();
        }

        let call_text = node_text(call_node, source).to_string();
        let func_name = get_function_call_name(call_node, source).unwrap_or("function");
        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "COMPNOT",
            message: format!(
                "This logical comparison simplifies to ~{call_text}. Did you mean to use {func_name} to evaluate function argument: {call_text}?"
            ),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: Some(Fix::new(
                node.start_byte()..node.end_byte(),
                format!("~{call_text}"),
            )),
        }]
    }

    /// Shared COMPNOP/COMPNOT logic: extract the `(call, literal, operator)`
    /// triple from a `comparison_operator` node when exactly one operand is a
    /// `function_call` and the other is the `true`/`false` literal.
    fn comparison_with_literal<'a>(
        &self,
        node: Node<'a>,
        source: &'a str,
    ) -> Option<(Node<'a>, Node<'a>, String)> {
        let lhs = node.child(0)?;
        let rhs = node.child(2)?;
        let op = find_operator_text(node, source);

        let (call_node, literal_node) = match (lhs.kind(), rhs.kind()) {
            ("function_call", "identifier") => (lhs, rhs),
            ("identifier", "function_call") => (rhs, lhs),
            _ => return None,
        };
        let literal = node_text(literal_node, source).trim();
        if literal == "true" || literal == "false" {
            Some((call_node, literal_node, op))
        } else {
            None
        }
    }

    /// M3COL: an expression with three colons (`a:b:c:d`) is probably
    /// unintended. The grammar cannot represent four-element colon chains, so
    /// they parse as a `range` plus a trailing ERROR node (or a range nested
    /// inside an ERROR node); all three shapes are detected here.
    fn check_m3col(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("M3COL") || node.kind() != "range" {
            return Vec::new();
        }

        let mut colon_count = node_text(node, source).matches(':').count();
        let mut end_byte = node.end_byte();

        if let Some(next) = node.next_named_sibling() {
            if next.kind() == "ERROR" && node_text(next, source).starts_with(':') {
                colon_count += node_text(next, source).matches(':').count();
                end_byte = end_byte.max(next.end_byte());
            }
        }
        if let Some(parent) = node.parent() {
            if parent.kind() == "ERROR" {
                colon_count += node_text(parent, source).matches(':').count();
                end_byte = end_byte.max(parent.end_byte());
            } else if let Some(next) = parent.next_named_sibling() {
                if next.kind() == "ERROR" && node_text(next, source).starts_with(':') {
                    colon_count += node_text(next, source).matches(':').count();
                    end_byte = end_byte.max(next.end_byte());
                }
            }
        }

        if colon_count < 3 {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "M3COL",
            message: "Using three colons (a:b:c:d) in an expression is probably unintended"
                .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..end_byte,
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    // -----------------------------------------------------------------------
    // Structure / string / misc checks (file-level)
    // -----------------------------------------------------------------------

    /// COMFS/SEMFS: a top-level `,` (COMFS) or `;` (SEMFS) in a file that also
    /// contains a `function_definition` makes the file a script, so all
    /// functions in it become local functions.
    fn check_comfs_semfs(&self, tree: &tree_sitter::Tree, _source: &str) -> Vec<Diagnostic> {
        let root = tree.root_node();
        if root.kind() != "source_file" {
            return Vec::new();
        }

        let mut cursor = root.walk();
        let children: Vec<Node> = root.children(&mut cursor).collect();
        if !children.iter().any(|c| c.kind() == "function_definition") {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        for child in children {
            let (check_id, message) = match child.kind() {
                "," if self.is_check_enabled("COMFS") => (
                    "COMFS",
                    "This comma makes the file a script. Therefore, all functions in the file are local functions.",
                ),
                ";" if self.is_check_enabled("SEMFS") => (
                    "SEMFS",
                    "This semicolon makes the file a script. Therefore, all functions in the file are local functions.",
                ),
                _ => continue,
            };
            let pos = child.start_position();
            diagnostics.push(Diagnostic {
                rule_id: check_id,
                message: message.to_string(),
                severity: Severity::Warning,
                byte_range: child.start_byte()..child.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            });
        }
        diagnostics
    }

    // -----------------------------------------------------------------------
    // OOP / class / property checks
    // -----------------------------------------------------------------------

    /// ATTF / ATTOF: class-level `Abstract` attribute.
    ///
    /// ATTF fires when the value assigned to `Abstract` is not a recognizable
    /// boolean literal; ATTOF fires when the value is explicitly `false`.
    fn check_attf_attof(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("ATTF") && !self.is_check_enabled("ATTOF") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let Some(class) = meta.class.as_ref() else {
            return Vec::new();
        };

        let attr = match class
            .attributes
            .iter()
            .find(|a| a.name == "Abstract" && !a.negated)
        {
            Some(a) => a,
            None => return Vec::new(),
        };

        let Some(value) = attr.value.as_deref() else {
            return Vec::new();
        };

        let class_node = find_child_of_kind(tree.root_node(), "class_definition");
        let attr_node = class_node.and_then(|cn| find_attribute_node(cn, "Abstract", source));
        let (start_byte, end_byte, line, column) = match attr_node {
            Some(an) => (
                an.start_byte(),
                an.end_byte(),
                an.start_position().row + 1,
                an.start_position().column + 1,
            ),
            None => (class.byte_range.start, class.byte_range.end, class.line, 1),
        };

        let lower = value.to_lowercase();
        let is_valid_literal = matches!(
            lower.as_str(),
            "true" | "false" | "1" | "0" | "on" | "off"
        );

        let mut diagnostics = Vec::new();
        if !is_valid_literal && self.is_check_enabled("ATTF") {
            diagnostics.push(Diagnostic {
                rule_id: "ATTF",
                message: "The Code Analyzer is unable to determine if the expression assigned to the Abstract attribute evaluates to true or false.".to_string(),
                severity: Severity::Warning,
                byte_range: start_byte..end_byte,
                line,
                column,
                fix: None,
            });
        } else if lower == "false" && self.is_check_enabled("ATTOF") {
            diagnostics.push(Diagnostic {
                rule_id: "ATTOF",
                message: "Setting the class attribute Abstract to false is not recommended."
                    .to_string(),
                severity: Severity::Info,
                byte_range: start_byte..end_byte,
                line,
                column,
                fix: None,
            });
        }
        diagnostics
    }

    /// MCPO: `SetObservable`/`GetObservable`/`AbortSet` have no effect on
    /// properties of a value class.
    fn check_mcpo(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MCPO") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let Some(class) = meta.class.as_ref() else {
            return Vec::new();
        };
        if class.is_handle() {
            return Vec::new();
        }

        let class_node = find_child_of_kind(tree.root_node(), "class_definition");
        let mut diagnostics = Vec::new();

        for (block_idx, block) in class.properties_blocks.iter().enumerate() {
            let attr_name = ["SetObservable", "GetObservable", "AbortSet"]
                .iter()
                .find(|name| {
                    block
                        .attributes
                        .iter()
                        .any(|a| &a.name == *name && !a.negated)
                });
            let Some(attr_name) = attr_name else {
                continue;
            };

            let (start_byte, end_byte, line, column) =
                block_diagnostic_position(class_node, block_idx, class, source);

            diagnostics.push(Diagnostic {
                rule_id: "MCPO",
                message: format!("{attr_name} property has no effect in a value class."),
                severity: Severity::Warning,
                byte_range: start_byte..end_byte,
                line,
                column,
                fix: None,
            });
        }
        diagnostics
    }

    /// MCSAC: `SetAccess` cannot be set on `Constant` properties.
    fn check_mcsac(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MCSAC") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let Some(class) = meta.class.as_ref() else {
            return Vec::new();
        };

        let class_node = find_child_of_kind(tree.root_node(), "class_definition");
        let mut diagnostics = Vec::new();

        for (block_idx, block) in class.properties_blocks.iter().enumerate() {
            let has_constant = block
                .attributes
                .iter()
                .any(|a| a.name == "Constant" && !a.negated);
            let has_set_access = block
                .attributes
                .iter()
                .any(|a| a.name == "SetAccess" && !a.negated);
            if !has_constant || !has_set_access {
                continue;
            }

            let (start_byte, end_byte, line, column) =
                block_diagnostic_position(class_node, block_idx, class, source);

            diagnostics.push(Diagnostic {
                rule_id: "MCSAC",
                message: "SetAccess cannot be set on Constant properties.".to_string(),
                severity: Severity::Warning,
                byte_range: start_byte..end_byte,
                line,
                column,
                fix: None,
            });
        }
        diagnostics
    }

    /// MOBSRV: `SetObservable`/`GetObservable` have no effect on `Constant`
    /// properties.
    fn check_mobsrv(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MOBSRV") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let Some(class) = meta.class.as_ref() else {
            return Vec::new();
        };

        let class_node = find_child_of_kind(tree.root_node(), "class_definition");
        let mut diagnostics = Vec::new();

        for (block_idx, block) in class.properties_blocks.iter().enumerate() {
            let has_constant = block
                .attributes
                .iter()
                .any(|a| a.name == "Constant" && !a.negated);
            let has_observable = block.attributes.iter().any(|a| {
                !a.negated && (a.name == "SetObservable" || a.name == "GetObservable")
            });
            if !has_constant || !has_observable {
                continue;
            }

            let (start_byte, end_byte, line, column) =
                block_diagnostic_position(class_node, block_idx, class, source);

            diagnostics.push(Diagnostic {
                rule_id: "MOBSRV",
                message: "Using SetObservable or GetObservable on a Constant property has no effect.".to_string(),
                severity: Severity::Info,
                byte_range: start_byte..end_byte,
                line,
                column,
                fix: None,
            });
        }
        diagnostics
    }

    /// MDEPIN: dependent properties do not store values, so default values
    /// should not be assigned to them.
    fn check_mdepin(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MDEPIN") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let Some(class) = meta.class.as_ref() else {
            return Vec::new();
        };

        let mut diagnostics = Vec::new();
        for block in &class.properties_blocks {
            let is_dependent = block
                .attributes
                .iter()
                .any(|a| a.name == "Dependent" && !a.negated);
            if !is_dependent {
                continue;
            }
            for prop in &block.properties {
                if prop.has_default {
                    diagnostics.push(Diagnostic {
                        rule_id: "MDEPIN",
                        message: "Default values should not be assigned to dependent properties because dependent properties do not store the values.".to_string(),
                        severity: Severity::Warning,
                        byte_range: prop.byte_range.clone(),
                        line: prop.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }
        diagnostics
    }

    /// MCCPI: a `Constant` property must be initialized or declared `Abstract`.
    fn check_mccpi(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MCCPI") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let Some(class) = meta.class.as_ref() else {
            return Vec::new();
        };
        if class.is_abstract() {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        for block in &class.properties_blocks {
            let has_constant = block
                .attributes
                .iter()
                .any(|a| a.name == "Constant" && !a.negated);
            let block_abstract = block
                .attributes
                .iter()
                .any(|a| a.name == "Abstract" && !a.negated);
            if !has_constant || block_abstract {
                continue;
            }
            for prop in &block.properties {
                if !prop.has_default {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCCPI",
                        message: format!(
                            "Initialize the Constant property '{}' or make it an Abstract Constant property.",
                            prop.name
                        ),
                        severity: Severity::Warning,
                        byte_range: prop.byte_range.clone(),
                        line: prop.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }
        diagnostics
    }

    /// MGMD: a `get` method is required for each dependent property that does
    /// not have private `GetAccess`.
    fn check_mgmd(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MGMD") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let Some(class) = meta.class.as_ref() else {
            return Vec::new();
        };

        let method_names: std::collections::HashSet<String> = class
            .methods_blocks
            .iter()
            .flat_map(|mb| mb.methods.iter())
            .map(|m| m.name.clone())
            .collect();

        let mut diagnostics = Vec::new();
        for block in &class.properties_blocks {
            let is_dependent = block
                .attributes
                .iter()
                .any(|a| a.name == "Dependent" && !a.negated);
            if !is_dependent {
                continue;
            }
            let get_access_private = block.attributes.iter().any(|a| {
                a.name == "GetAccess"
                    && a.value
                        .as_deref()
                        .is_some_and(|v| v.eq_ignore_ascii_case("private"))
            });
            if get_access_private {
                continue;
            }
            for prop in &block.properties {
                let getter = format!("get.{}", prop.name);
                if !method_names.contains(&getter) {
                    diagnostics.push(Diagnostic {
                        rule_id: "MGMD",
                        message: format!(
                            "'get' method should be implemented for each dependent property that does not also have private 'GetAccess' attribute. Add a method named '{}'.",
                            getter
                        ),
                        severity: Severity::Warning,
                        byte_range: prop.byte_range.clone(),
                        line: prop.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }
        diagnostics
    }

    /// MTHANS: `ans` is frequently overwritten by MATLAB, so it should not be
    /// used as a method name.
    fn check_mthans(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MTHANS") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let mut diagnostics = Vec::new();

        for func in meta.all_methods() {
            if func.name == "ans" {
                diagnostics.push(Diagnostic {
                    rule_id: "MTHANS",
                    message: "Using ANS as a method name is not recommended as ANS is frequently overwritten by MATLAB.".to_string(),
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

    /// MCCPE: attempting to call a property or event as a function.
    fn check_mccpe(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MCCPE") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let Some(class) = meta.class.as_ref() else {
            return Vec::new();
        };

        let mut names: std::collections::HashSet<String> = std::collections::HashSet::new();
        for block in &class.properties_blocks {
            for prop in &block.properties {
                names.insert(prop.name.clone());
            }
        }
        for block in &class.events_blocks {
            for ev in &block.events {
                names.insert(ev.clone());
            }
        }
        if names.is_empty() {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        if let Some(class_node) = find_child_of_kind(tree.root_node(), "class_definition") {
            collect_property_event_calls(class_node, source, &names, &mut diagnostics);
        }
        diagnostics
    }

    /// MHERM: parenthesize the multiplication of a variable and its transpose
    /// to ensure the result is Hermitian.
    fn check_mherm(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MHERM") || node.kind() != "binary_operator" {
            return Vec::new();
        }

        // Already parenthesized — the rule is satisfied.
        if node
            .parent()
            .map(|p| p.kind() == "parenthesis")
            .unwrap_or(false)
        {
            return Vec::new();
        }

        let has_mul = (0..node.child_count()).any(|i| {
            node.child(i)
                .map(|c| {
                    let text = node_text(c, source);
                    text == "*" || text == ".*"
                })
                .unwrap_or(false)
        });
        if !has_mul {
            return Vec::new();
        }

        let postfix = (0..node.child_count()).find_map(|i| {
            let child = node.child(i)?;
            if child.kind() == "postfix_operator" {
                Some(child)
            } else {
                None
            }
        });
        let Some(pf) = postfix else {
            return Vec::new();
        };

        let var_name = first_named_child(pf)
            .map(|c| node_text(c, source))
            .unwrap_or("expression");

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "MHERM",
            message: format!(
                "Parenthesize the multiplication of {var_name} and its transpose to ensure the result is Hermitian."
            ),
            severity: Severity::Info,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// MNUML: to create a square matrix, pass both dimensions or use `size`.
    fn check_mnuml(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MNUML") || node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match get_function_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };
        if !["zeros", "ones", "rand", "randn", "false", "true"].contains(&func_name) {
            return Vec::new();
        }

        let args = match find_child_of_kind(node, "arguments") {
            Some(a) => a,
            None => return Vec::new(),
        };

        let mut named = Vec::new();
        let mut cursor = args.walk();
        for child in args.children(&mut cursor) {
            if child.is_named() {
                named.push(child);
            }
        }
        if named.len() != 1 {
            return Vec::new();
        }
        let arg = named[0];
        if arg.kind() != "function_call" {
            return Vec::new();
        }
        if get_function_call_name(arg, source) != Some("numel") {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "MNUML",
            message: format!(
                "To create a square matrix, use {func_name}(numel(...), numel(...)). Alternatively, use {func_name}(size(...))."
            ),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }

    /// PFIIN / PFOUS / PFTUSW / PFUIXW: PARFOR loop data-flow checks.
    ///
    /// Uses the symbol table with byte-range comparisons because the symbol
    /// table has no per-parfor scope.
    fn check_parfor_file_level(
        &self,
        tree: &tree_sitter::Tree,
        source: &str,
    ) -> Vec<Diagnostic> {
        let any_enabled = self.is_check_enabled("PFIIN")
            || self.is_check_enabled("PFOUS")
            || self.is_check_enabled("PFTUSW")
            || self.is_check_enabled("PFUIXW");
        if !any_enabled {
            return Vec::new();
        }

        let sym = SymbolTable::build(tree, source);
        let mut diagnostics = Vec::new();
        let parfors = collect_parfor_nodes(tree.root_node(), source);

        for parfor in &parfors {
            let idx_var = extract_parfor_index_var(*parfor, source);
            let body = find_child_of_kind(*parfor, "block");

            // PFIIN: input variable read in the body but never initialized
            // before the loop and not assigned inside the loop.
            if self.is_check_enabled("PFIIN") {
                if let Some(body_node) = body {
                    let mut skip = std::collections::HashSet::new();
                    if let Some(ref idx) = idx_var {
                        skip.insert(idx.clone());
                    }
                    let reads = collect_parfor_body_reads(body_node, source, &skip);
                    for (name, read_node) in reads {
                        if is_defined_before_parfor(&sym, &name, parfor.start_byte())
                            || is_assigned_in_body(body_node, &name, source)
                        {
                            continue;
                        }
                        let pos = read_node.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "PFIIN",
                            message: format!(
                                "The input variable {name} should be initialized before the PARFOR loop."
                            ),
                            severity: Severity::Warning,
                            byte_range: read_node.start_byte()..read_node.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            }

            // PFOUS / PFTUSW: simple-identifier assignments in the body and
            // whether the variable is used after the loop.
            let need_assignments = self.is_check_enabled("PFOUS") || self.is_check_enabled("PFTUSW");
            if need_assignments {
                if let Some(body_node) = body {
                    let assigned = collect_simple_lhs_assignments(body_node, source);
                    for (name, lhs_node) in assigned {
                        if idx_var.as_deref() == Some(name.as_str()) {
                            continue;
                        }
                        let uses =
                            uses_after_parfor(&sym, &name, parfor.end_byte(), tree, false);
                        if self.is_check_enabled("PFOUS") && uses.is_empty() {
                            let pos = lhs_node.start_position();
                            diagnostics.push(Diagnostic {
                                rule_id: "PFOUS",
                                message: format!(
                                    "The output variable {name} might not be used after the PARFOR loop."
                                ),
                                severity: Severity::Warning,
                                byte_range: lhs_node.start_byte()..lhs_node.end_byte(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        }
                        if self.is_check_enabled("PFTUSW") {
                            if let Some(line) = uses.first() {
                                let pos = lhs_node.start_position();
                                diagnostics.push(Diagnostic {
                                    rule_id: "PFTUSW",
                                    message: format!(
                                        "The temporary variable {name} might be used after the PARFOR loop on line {line}."
                                    ),
                                    severity: Severity::Warning,
                                    byte_range: lhs_node.start_byte()..lhs_node.end_byte(),
                                    line: pos.row + 1,
                                    column: pos.column + 1,
                                    fix: None,
                                });
                            }
                        }
                    }
                }
            }

            // PFUIXW: index variable used after the loop.
            if self.is_check_enabled("PFUIXW") {
                if let Some(ref idx) = idx_var {
                    let uses = uses_after_parfor(&sym, idx, parfor.end_byte(), tree, true);
                    if let Some(line) = uses.first() {
                        let idx_node = find_parfor_index_identifier(*parfor);
                        let target = idx_node.unwrap_or(*parfor);
                        let pos = target.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "PFUIXW",
                            message: format!(
                                "The index variable {idx} might be used after the PARFOR loop on line {line}."
                            ),
                            severity: Severity::Warning,
                            byte_range: target.start_byte()..target.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            }
        }

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

        // Function-call convention checks.
        diagnostics.extend(self.check_ctpct(node, source));
        diagnostics.extend(self.check_fxset(node, source));
        diagnostics.extend(self.check_simpt(node, source));
        diagnostics.extend(self.check_tlev(node, source));
        diagnostics.extend(self.check_unonc(node, source));
        diagnostics.extend(self.check_mipc1(node, source));

        // Structure / string / misc checks.
        diagnostics.extend(self.check_rmfld(node, source));
        diagnostics.extend(self.check_stfld(node, source));
        diagnostics.extend(self.check_rmwrn(node, source));
        diagnostics.extend(self.check_strsz(node, source));
        diagnostics.extend(self.check_dualc(node, source));

        // Logical / comparison / range checks.
        diagnostics.extend(self.check_compnop(node, source));
        diagnostics.extend(self.check_comnot(node, source));
        diagnostics.extend(self.check_m3col(node, source));

        // OOP / class / property checks.
        diagnostics.extend(self.check_mherm(node, source));
        diagnostics.extend(self.check_mnuml(node, source));

        // Parfor / SPMD checks.
        if node.kind() == "for_statement" || node.kind() == "spmd_statement" {
            diagnostics.extend(self.check_pfrni(node, source));
            diagnostics.extend(self.check_pfevb(node, source));
            diagnostics.extend(self.check_pfgp(node, source));
            diagnostics.extend(self.check_pfgv(node, source));
            diagnostics.extend(self.check_spevb(node, source));
            diagnostics.extend(self.check_spgv(node, source));
            diagnostics.extend(self.check_dspmda(node, source));
        }

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
        diagnostics.extend(self.check_comfs_semfs(ctx.tree, ctx.source));

        // OOP / class / property checks.
        diagnostics.extend(self.check_attf_attof(ctx.tree, ctx.source));
        diagnostics.extend(self.check_mcpo(ctx.tree, ctx.source));
        diagnostics.extend(self.check_mcsac(ctx.tree, ctx.source));
        diagnostics.extend(self.check_mobsrv(ctx.tree, ctx.source));
        diagnostics.extend(self.check_mdepin(ctx.tree, ctx.source));
        diagnostics.extend(self.check_mccpi(ctx.tree, ctx.source));
        diagnostics.extend(self.check_mgmd(ctx.tree, ctx.source));
        diagnostics.extend(self.check_mthans(ctx.tree, ctx.source));
        diagnostics.extend(self.check_mccpe(ctx.tree, ctx.source));

        // Parfor / SPMD file-level checks.
        diagnostics.extend(self.check_parfor_file_level(ctx.tree, ctx.source));

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

/// Find the operator text in a binary/boolean/comparison operator node.
fn find_operator_text<'a>(node: Node<'a>, source: &'a str) -> String {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if !child.is_named() {
            let text = &source[child.start_byte()..child.end_byte()];
            let trimmed = text.trim();
            if !trimmed.is_empty()
                && trimmed != "("
                && trimmed != ")"
                && trimmed != ","
            {
                return trimmed.to_string();
            }
        }
    }

    if let Some(op_node) = node.child_by_field_name("operator") {
        return node_text(op_node, source).trim().to_string();
    }

    let text = node_text(node, source);
    for op in &["==", "~=", ">=", "<=", "&&", "||", ">", "<", "&", "|", "+", "-", "*", "/", "^"] {
        if text.contains(op) {
            return (*op).to_string();
        }
    }

    String::new()
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
// Parfor / SPMD helpers
// ---------------------------------------------------------------------------

/// Get the root node of a tree by walking up from a node.
fn root_node_of<'a>(mut node: Node<'a>) -> Node<'a> {
    while let Some(p) = node.parent() {
        node = p;
    }
    node
}

/// Check whether a `for_statement` node is a parfor loop.
fn is_parfor_node(node: Node, source: &str) -> bool {
    node.kind() == "for_statement" && node_text(node, source).trim_start().starts_with("parfor")
}

/// Count the named children of a node.
fn count_named_children(node: Node) -> usize {
    let mut cursor = node.walk();
    node.children(&mut cursor).filter(|c| c.is_named()).count()
}

/// Get the `range` node of a for/parfor statement's iterator.
fn find_iterator_range(node: Node) -> Option<Node> {
    let iter = find_child_of_kind(node, "iterator")?;
    find_child_of_kind(iter, "range")
}

/// Get the last named child of a node (including `end`).
fn last_named_child(node: Node) -> Option<Node> {
    let mut result = None;
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            if child.is_named() {
                result = Some(child);
            }
        }
    }
    result
}

/// Extract the parfor index variable name.
fn extract_parfor_index_var(node: Node, source: &str) -> Option<String> {
    find_parfor_index_identifier(node).map(|n| node_text(n, source).to_string())
}

/// Find the parfor index variable identifier node.
fn find_parfor_index_identifier(node: Node) -> Option<Node> {
    let iter = find_child_of_kind(node, "iterator")?;
    let count = iter.child_count();
    for i in 0..count {
        if let Some(c) = iter.child(i) {
            if c.kind() == "identifier" {
                return Some(c);
            }
        }
    }
    None
}

/// Walk a node collecting all `function_call` and `command` descendants.
fn walk_body_for_function_calls<'a>(node: Node<'a>, out: &mut Vec<Node<'a>>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "function_call" || child.kind() == "command" {
            out.push(child);
        }
        if child.is_named() {
            walk_body_for_function_calls(child, out);
        }
    }
}

/// Collect all descendants of a node with the given kind.
fn collect_nodes_of_kind<'a>(node: Node<'a>, kind: &str, out: &mut Vec<Node<'a>>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == kind {
            out.push(child);
        }
        if child.is_named() {
            collect_nodes_of_kind(child, kind, out);
        }
    }
}

/// Collect all parfor `for_statement` nodes in a tree.
fn collect_parfor_nodes<'a>(root: Node<'a>, source: &str) -> Vec<Node<'a>> {
    let mut out = Vec::new();
    if is_parfor_node(root, source) {
        out.push(root);
    }
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.is_named() {
            out.extend(collect_parfor_nodes(child, source));
        }
    }
    out
}

/// Collect all identifiers declared `global` or `persistent` anywhere in a tree.
fn collect_global_persistent_vars(root: Node, source: &str) -> std::collections::HashSet<String> {
    let mut vars = std::collections::HashSet::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "global_operator" || child.kind() == "persistent_operator" {
            let mut c2 = child.walk();
            for id in child.children(&mut c2) {
                if id.kind() == "identifier" {
                    vars.insert(node_text(id, source).to_string());
                }
            }
        }
        if child.is_named() {
            vars.extend(collect_global_persistent_vars(child, source));
        }
    }
    vars
}

/// Check whether a function call / command is `evalin`/`assignin` with `'base'`.
fn is_evalin_assignin_base(node: Node, source: &str) -> bool {
    let func_name = match node.kind() {
        "function_call" => get_function_call_name(node, source),
        "command" => get_command_name(node, source),
        _ => return false,
    };
    let func_name = match func_name {
        Some(n) => n,
        None => return false,
    };
    if func_name != "evalin" && func_name != "assignin" {
        return false;
    }

    let first_arg = if node.kind() == "function_call" {
        find_child_of_kind(node, "arguments").and_then(|a| first_named_child(a))
    } else {
        find_child_of_kind(node, "command_argument")
    };
    let first_arg = match first_arg {
        Some(a) => a,
        None => return false,
    };
    let text = node_text(first_arg, source).trim();
    let text = text.trim_matches('\'').trim_matches('"');
    text == "base"
}

/// Check whether a function call / command constructs a distributed array.
fn is_distributed_call(node: Node, source: &str) -> bool {
    let func_name = match node.kind() {
        "function_call" => get_function_call_name(node, source),
        "command" => get_command_name(node, source),
        _ => return false,
    };
    matches!(
        func_name,
        Some("distributed") | Some("gpuArray") | Some("codistributed")
    )
}

/// Extract the base variable name assigned by an assignment LHS.
fn lhs_base_name(lhs: Node, source: &str) -> Option<String> {
    match lhs.kind() {
        "identifier" => Some(node_text(lhs, source).to_string()),
        "function_call" => get_function_call_name(lhs, source).map(|s| s.to_string()),
        "field_expression" => find_child_of_kind(lhs, "identifier")
            .map(|n| node_text(n, source).to_string()),
        _ => None,
    }
}

/// Emit use-of-global diagnostics for a body node (PFGV / SPGV).
///
/// When `skip_writes` is true, assignment LHS identifiers are excluded
/// (they are handled by the separate PFGP check).
fn collect_global_use_diagnostics<'a>(
    body: Node<'a>,
    source: &str,
    globals: &std::collections::HashSet<String>,
    rule_id: &'static str,
    skip_writes: bool,
) -> Vec<Diagnostic> {
    let mut seen = std::collections::HashSet::new();
    let mut diagnostics = Vec::new();
    let mut identifiers = Vec::new();
    collect_nodes_of_kind(body, "identifier", &mut identifiers);
    for id in identifiers {
        let name = node_text(id, source).to_string();
        if !globals.contains(&name) {
            continue;
        }
        if let Some(parent) = id.parent() {
            match parent.kind() {
                "global_operator" | "persistent_operator" => continue,
                "assignment" => {
                    if skip_writes
                        && parent
                            .child_by_field_name("left")
                            .map(|l| l.id() == id.id())
                            .unwrap_or(false)
                    {
                        continue;
                    }
                }
                "field_expression" => {
                    if parent
                        .child_by_field_name("field")
                        .map(|f| f.id() == id.id())
                        .unwrap_or(false)
                    {
                        continue;
                    }
                }
                _ => {}
            }
        }
        if !seen.insert(name.clone()) {
            continue;
        }
        let pos = id.start_position();
        let message = if rule_id == "PFGV" {
            format!("Avoid using GLOBAL variable {name} in a PARFOR loop.")
        } else {
            format!(
                "Using the GLOBAL or PERSISTENT variable {name} in an SPMD block might fail because it is accessed on a worker machine."
            )
        };
        diagnostics.push(Diagnostic {
            rule_id,
            message,
            severity: Severity::Warning,
            byte_range: id.start_byte()..id.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        });
    }
    diagnostics
}

/// Collect plain-identifier reads inside a parfor body (for PFIIN).
///
/// Function-call names, assignment LHS writes, field names, and declarations
/// are excluded; only genuine variable reads are returned, deduplicated by name.
fn collect_parfor_body_reads<'a>(
    node: Node<'a>,
    source: &str,
    skip: &std::collections::HashSet<String>,
) -> Vec<(String, Node<'a>)> {
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut identifiers = Vec::new();
    collect_nodes_of_kind(node, "identifier", &mut identifiers);
    for id in identifiers {
        let name = node_text(id, source).to_string();
        if skip.contains(&name) {
            continue;
        }
        if let Some(parent) = id.parent() {
            match parent.kind() {
                "function_call" => {
                    if parent
                        .child_by_field_name("name")
                        .map(|n| n.id() == id.id())
                        .unwrap_or(false)
                    {
                        continue;
                    }
                }
                "assignment" => {
                    if parent
                        .child_by_field_name("left")
                        .map(|l| l.id() == id.id())
                        .unwrap_or(false)
                    {
                        continue;
                    }
                }
                "global_operator" | "persistent_operator" => continue,
                "field_expression" => {
                    if parent
                        .child_by_field_name("field")
                        .map(|f| f.id() == id.id())
                        .unwrap_or(false)
                    {
                        continue;
                    }
                }
                "iterator" => continue,
                _ => {}
            }
        }
        if seen.insert(name.clone()) {
            out.push((name, id));
        }
    }
    out
}

/// Collect simple-identifier LHS assignments inside a node (for PFOUS/PFTUSW).
fn collect_simple_lhs_assignments<'a>(node: Node<'a>, source: &str) -> Vec<(String, Node<'a>)> {
    let mut out = Vec::new();
    let mut assignments = Vec::new();
    collect_nodes_of_kind(node, "assignment", &mut assignments);
    for assign in assignments {
        if let Some(lhs) = assign.child_by_field_name("left") {
            if lhs.kind() == "identifier" {
                out.push((node_text(lhs, source).to_string(), lhs));
            }
        }
    }
    out
}

/// Check whether a variable is assigned anywhere inside a node.
fn is_assigned_in_body(node: Node, name: &str, source: &str) -> bool {
    collect_simple_lhs_assignments(node, source)
        .iter()
        .any(|(n, _)| n == name)
}

/// Check whether a variable is defined before a parfor loop starts.
///
/// Function inputs and global/persistent declarations always count as defined;
/// other definitions count if their byte range ends before the loop.
fn is_defined_before_parfor(sym: &SymbolTable, name: &str, parfor_start: usize) -> bool {
    sym.scopes.iter().any(|s| {
        s.defs.iter().any(|d| {
            d.name == name
                && (matches!(
                    d.kind,
                    crate::analysis::symbols::DefKind::InputArg
                        | crate::analysis::symbols::DefKind::Global
                        | crate::analysis::symbols::DefKind::Persistent
                ) || d.byte_range.end <= parfor_start)
        })
    })
}

/// Collect lines where a variable is used after a byte offset.
///
/// When `exclude_loops` is true, uses inside any enclosing `for_statement`
/// are ignored (the variable is re-bound by a subsequent loop).
fn uses_after_parfor(
    sym: &SymbolTable,
    name: &str,
    after: usize,
    tree: &tree_sitter::Tree,
    exclude_loops: bool,
) -> Vec<usize> {
    let mut lines = Vec::new();
    for scope in &sym.scopes {
        for u in &scope.uses {
            if u.name != name || u.byte_range.start < after {
                continue;
            }
            if exclude_loops {
                if let Some(n) = tree
                    .root_node()
                    .descendant_for_byte_range(u.byte_range.start, u.byte_range.end)
                {
                    if is_inside_loop(n) {
                        continue;
                    }
                }
            }
            lines.push(u.line);
        }
    }
    lines.sort_unstable();
    lines.dedup();
    lines
}

/// Check whether a node is enclosed by a `for_statement` (loop body).
fn is_inside_loop(node: Node) -> bool {
    let mut current = node.parent();
    while let Some(p) = current {
        if p.kind() == "for_statement" {
            return true;
        }
        if p.kind() == "function_definition" || p.kind() == "source_file" {
            return false;
        }
        current = p.parent();
    }
    false
}


/// Find the nearest enclosing `function_definition` of a node (for SIMPT).
fn enclosing_function(node: Node) -> Option<Node> {
    let mut current = node.parent();
    while let Some(p) = current {
        if p.kind() == "function_definition" {
            return Some(p);
        }
        current = p.parent();
    }
    None
}

/// Count `printf`-style format specifiers in a format string (for CTPCT).
///
/// Counts `%` followed by a conversion character. Escaped `%%`, `%*` width
/// specifiers, and `%n$` positional specifiers are not counted.
fn count_format_specifiers(fmt: &str) -> usize {
    let chars: Vec<char> = fmt.chars().collect();
    let mut count = 0;
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '%' {
            i += 1;
            continue;
        }
        let j = i + 1;
        if j >= chars.len() {
            break;
        }
        if chars[j] == '%' {
            i = j + 1;
            continue;
        }
        if chars[j] == '*' {
            let mut k = j + 1;
            while k < chars.len() && !chars[k].is_ascii_alphabetic() {
                k += 1;
            }
            i = k + 1;
            continue;
        }
        let mut k = j;
        while k < chars.len() && chars[k].is_ascii_digit() {
            k += 1;
        }
        if k < chars.len() && chars[k] == '$' {
            let mut m = k + 1;
            while m < chars.len() && !chars[m].is_ascii_alphabetic() {
                m += 1;
            }
            i = m + 1;
            continue;
        }
        while k < chars.len() && "-+ #0".contains(chars[k]) {
            k += 1;
        }
        while k < chars.len() && chars[k].is_ascii_digit() {
            k += 1;
        }
        if k < chars.len() && chars[k] == '.' {
            k += 1;
            while k < chars.len() && chars[k].is_ascii_digit() {
                k += 1;
            }
        }
        if k < chars.len() && chars[k].is_ascii_alphabetic() {
            count += 1;
            i = k + 1;
        } else {
            i = j;
        }
    }
    count
}

/// Collect assignments to `var_name` inside a loop body, without descending
/// into nested function definitions or anonymous functions (for FXSET).
fn collect_loop_assignments(
    node: Node,
    var_name: &str,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if node.kind() == "function_definition" || node.kind() == "lambda" {
        return;
    }
    if node.kind() == "assignment" {
        if let Some(left) = node.child_by_field_name("left") {
            if left.kind() == "identifier" && node_text(left, source) == var_name {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "FXSET",
                    message: format!("Loop index {var_name} is changed inside of a FOR loop."),
                    severity: Severity::Warning,
                    byte_range: node.start_byte()..node.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_loop_assignments(child, var_name, source, diagnostics);
    }
}

/// Find the `attribute` node with the given name inside a node that has an
/// `attributes` child (e.g., `class_definition` or a `properties` block).
fn find_attribute_node<'a>(node: Node<'a>, name: &str, source: &'a str) -> Option<Node<'a>> {
    let attrs = find_child_of_kind(node, "attributes")?;
    let count = attrs.child_count();
    for i in 0..count {
        let child = attrs.child(i)?;
        if child.kind() != "attribute" {
            continue;
        }
        if find_child_of_kind(child, "identifier")
            .map(|id| node_text(id, source) == name)
            .unwrap_or(false)
        {
            return Some(child);
        }
    }
    None
}

/// Compute a diagnostic position for the `index`-th `properties` block of a
/// class, falling back to the class definition range if the block node cannot
/// be located.
fn block_diagnostic_position(
    class_node: Option<Node>,
    index: usize,
    class: &ClassMeta,
    _source: &str,
) -> (usize, usize, usize, usize) {
    let block_node = class_node.and_then(|cn| find_nth_properties_block(cn, index));
    match block_node {
        Some(bn) => (
            bn.start_byte(),
            bn.end_byte(),
            bn.start_position().row + 1,
            bn.start_position().column + 1,
        ),
        None => (class.byte_range.start, class.byte_range.end, class.line, 1),
    }
}

/// Find the `index`-th child of kind `properties` under a class definition.
fn find_nth_properties_block<'a>(class_node: Node<'a>, index: usize) -> Option<Node<'a>> {
    let mut seen = 0;
    let count = class_node.child_count();
    for i in 0..count {
        let child = class_node.child(i)?;
        if child.kind() == "properties" {
            if seen == index {
                return Some(child);
            }
            seen += 1;
        }
    }
    None
}

/// Walk a class definition and emit MCCPE diagnostics for `function_call`
/// nodes whose name matches a property or event name.
fn collect_property_event_calls(
    node: Node,
    source: &str,
    names: &std::collections::HashSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if node.kind() == "function_call" {
        if let Some(name) = get_function_call_name(node, source) {
            if names.contains(name) {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "MCCPE",
                    message: format!(
                        "Attempting to call a property or event {name} as a function."
                    ),
                    severity: Severity::Warning,
                    byte_range: node.start_byte()..node.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_property_event_calls(child, source, names, diagnostics);
    }
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

    /// Recursively find the first descendant of a node with the given kind.
    fn find_descendant_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
        if node.kind() == kind {
            return Some(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(found) = find_descendant_of_kind(child, kind) {
                return Some(found);
            }
        }
        None
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

    // -- RMFLD: rmfield must be assigned back ---------------------------------

    #[test]
    fn test_rmfld_fires_on_unassigned_call() {
        let source = "rmfield(s, 'a');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_child_of_kind(root, "function_call").unwrap();
        let diags = eng.check_rmfld(fc, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "RMFLD");
    }

    #[test]
    fn test_rmfld_silent_when_assigned() {
        let source = "s = rmfield(s, 'a');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let assignment = find_child_of_kind(root, "assignment").unwrap();
        let mut cursor = assignment.walk();
        let fc = assignment
            .children(&mut cursor)
            .find(|c| c.kind() == "function_call")
            .unwrap();
        let diags = eng.check_rmfld(fc, source);
        assert!(diags.is_empty());
    }

    // -- STFLD: setfield must be assigned back ---------------------------------

    #[test]
    fn test_stfld_fires_on_unassigned_call() {
        let source = "setfield(s, 'a', 1);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_child_of_kind(root, "function_call").unwrap();
        let diags = eng.check_stfld(fc, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "STFLD");
    }

    #[test]
    fn test_stfld_silent_when_assigned() {
        let source = "s = setfield(s, 'a', 1);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let assignment = find_child_of_kind(root, "assignment").unwrap();
        let mut cursor = assignment.walk();
        let fc = assignment
            .children(&mut cursor)
            .find(|c| c.kind() == "function_call")
            .unwrap();
        let diags = eng.check_stfld(fc, source);
        assert!(diags.is_empty());
    }

    // -- RMWRN: removed warning tags -------------------------------------------

    #[test]
    fn test_rmwrn_mechanism_never_fires_with_empty_denylist() {
        let source = "warning('ident:tag','msg');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_child_of_kind(root, "function_call").unwrap();
        let diags = eng.check_rmwrn(fc, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_rmwrn_not_a_warning_call() {
        let source = "error('ident:tag','msg');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_child_of_kind(root, "function_call").unwrap();
        let diags = eng.check_rmwrn(fc, source);
        assert!(diags.is_empty());
    }

    // -- STRSZ: different-size character vectors -------------------------------

    #[test]
    fn test_strsz_fires_on_different_length_strings() {
        let source = "x = 'abc' == 'abcd';\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comp = find_descendant_of_kind(root, "comparison_operator").unwrap();
        let diags = eng.check_strsz(comp, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "STRSZ");
    }

    #[test]
    fn test_strsz_silent_on_same_length_strings() {
        let source = "x = 'abc' == 'abc';\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comp = find_descendant_of_kind(root, "comparison_operator").unwrap();
        let diags = eng.check_strsz(comp, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_strsz_silent_on_strcmp() {
        let source = "x = strcmp('abc','abcd');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_strsz(fc, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_strsz_fires_on_not_equal() {
        let source = "x = 'abc' ~= 'abcd';\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comp = find_descendant_of_kind(root, "comparison_operator").unwrap();
        let diags = eng.check_strsz(comp, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "STRSZ");
    }

    // -- DUALC: command followed by comma --------------------------------------

    #[test]
    fn test_dualc_fires_on_comma_after_command() {
        let source = "disp hello, disp world\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let cmd = find_child_of_kind(root, "command").unwrap();
        let diags = eng.check_dualc(cmd, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "DUALC");
    }

    #[test]
    fn test_dualc_silent_without_comma() {
        let source = "disp hello\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let cmd = find_child_of_kind(root, "command").unwrap();
        let diags = eng.check_dualc(cmd, source);
        assert!(diags.is_empty());
    }

    // -- COMFS/SEMFS: top-level separator makes file a script ------------------

    #[test]
    fn test_comfs_semfs_fires_with_function_definition() {
        let source = "x = 1,\ny = 2;\nfunction f()\nend\n";
        let tree = parse(source);
        let eng = engine();

        let diags = eng.check_comfs_semfs(&tree, source);
        assert!(diags.iter().any(|d| d.rule_id == "COMFS"));
        assert!(diags.iter().any(|d| d.rule_id == "SEMFS"));
    }

    #[test]
    fn test_comfs_semfs_silent_without_function_definition() {
        let source = "x = 1,\ny = 2;\n";
        let tree = parse(source);
        let eng = engine();

        let diags = eng.check_comfs_semfs(&tree, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_comfs_semfs_silent_in_function_file() {
        let source = "function f()\n  x = 1;\nend\n";
        let tree = parse(source);
        let eng = engine();

        let diags = eng.check_comfs_semfs(&tree, source);
        assert!(diags.is_empty());
    }

    // -- G5 checks disabled ----------------------------------------------------

    #[test]
    fn test_g5_checks_respect_disabled_checks() {
        let eng = GoodPracticesEngine {
            config: GoodPracticesConfig {
                max_variable_name_length: 63,
                disabled_checks: vec![
                    "RMFLD".to_string(),
                    "STFLD".to_string(),
                    "STRSZ".to_string(),
                    "DUALC".to_string(),
                    "COMFS".to_string(),
                    "SEMFS".to_string(),
                ],
            },
        };

        let source = "rmfield(s, 'a');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let fc = find_child_of_kind(root, "function_call").unwrap();
        assert!(eng.check_rmfld(fc, source).is_empty());
        assert!(eng.check_stfld(fc, source).is_empty());

        let source = "x = 'abc' == 'abcd';\n";
        let tree = parse(source);
        let root = tree.root_node();
        let comp = find_descendant_of_kind(root, "comparison_operator").unwrap();
        assert!(eng.check_strsz(comp, source).is_empty());

        let source = "disp hello, disp world\n";
        let tree = parse(source);
        let root = tree.root_node();
        let cmd = find_child_of_kind(root, "command").unwrap();
        assert!(eng.check_dualc(cmd, source).is_empty());

        let source = "x = 1,\ny = 2;\nfunction f()\nend\n";
        let tree = parse(source);
        assert!(eng.check_comfs_semfs(&tree, source).is_empty());
    }

    // -- COMPNOP / COMPNOT / M3COL (G3) --------------------------------------

    #[test]
    fn test_compnop_fires_on_call_eq_true() {
        let source = "if isa(x,'double') == true\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comp = find_descendant_of_kind(root, "comparison_operator").unwrap();
        let diags = eng.check_compnop(comp, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "COMPNOP");
        assert!(diags[0].fix.is_some());
    }

    #[test]
    fn test_compnop_fix_replaces_comparison_with_call() {
        let source = "if isa(x,'double') == true\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comp = find_descendant_of_kind(root, "comparison_operator").unwrap();
        let diags = eng.check_compnop(comp, source);
        let fix = diags[0].fix.as_ref().unwrap();
        assert_eq!(&source[fix.byte_range.clone()], "isa(x,'double') == true");
        assert_eq!(fix.replacement, "isa(x,'double')");
    }

    #[test]
    fn test_compnop_silent_on_identifier_eq_true() {
        let source = "if y == true\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comp = find_descendant_of_kind(root, "comparison_operator").unwrap();
        assert!(eng.check_compnop(comp, source).is_empty());
        assert!(eng.check_comnot(comp, source).is_empty());
    }

    #[test]
    fn test_compnop_silent_on_call_eq_false() {
        // `== false` belongs to COMPNOT, not COMPNOP.
        let source = "if isa(x,'double') == false\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comp = find_descendant_of_kind(root, "comparison_operator").unwrap();
        assert!(eng.check_compnop(comp, source).is_empty());
    }

    #[test]
    fn test_comnot_fires_on_call_neq_true() {
        let source = "if isa(x,'double') ~= true\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comp = find_descendant_of_kind(root, "comparison_operator").unwrap();
        let diags = eng.check_comnot(comp, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "COMPNOT");
        let fix = diags[0].fix.as_ref().unwrap();
        assert_eq!(fix.replacement, "~isa(x,'double')");
    }

    #[test]
    fn test_comnot_fires_on_call_eq_false() {
        let source = "if isa(x,'double') == false\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comp = find_descendant_of_kind(root, "comparison_operator").unwrap();
        let diags = eng.check_comnot(comp, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "COMPNOT");
        assert_eq!(
            diags[0].fix.as_ref().unwrap().replacement,
            "~isa(x,'double')"
        );
    }

    #[test]
    fn test_comnot_silent_on_call_neq_false() {
        // `~= false` is not a COMPNOT pattern.
        let source = "if isa(x,'double') ~= false\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comp = find_descendant_of_kind(root, "comparison_operator").unwrap();
        assert!(eng.check_comnot(comp, source).is_empty());
    }

    #[test]
    fn test_compnop_comnot_silent_without_comparison() {
        let source = "if isa(x,'double')\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        assert!(find_descendant_of_kind(root, "comparison_operator").is_none());
        assert!(eng.check_compnop(root, source).is_empty());
        assert!(eng.check_comnot(root, source).is_empty());
    }

    #[test]
    fn test_m3col_fires_on_three_colons() {
        let source = "a = 1:2:3:4;\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let range = find_descendant_of_kind(root, "range").unwrap();
        let diags = eng.check_m3col(range, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "M3COL");
        assert_eq!(&source[diags[0].byte_range.clone()], "1:2:3:4");
    }

    #[test]
    fn test_m3col_silent_on_two_colons() {
        let source = "a = 1:2:10;\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let range = find_descendant_of_kind(root, "range").unwrap();
        assert!(eng.check_m3col(range, source).is_empty());
    }

    #[test]
    fn test_m3col_silent_on_one_colon() {
        let source = "a = 1:10;\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let range = find_descendant_of_kind(root, "range").unwrap();
        assert!(eng.check_m3col(range, source).is_empty());
    }

    #[test]
    fn test_m3col_fires_on_parenthesized_three_colons() {
        // `(1:2:3):4` contains three `:` in the range's own text, so it fires.
        let source = "a = (1:2:3):4;\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let mut fired = false;
        for range in all_descendants_of_kind(root, "range") {
            if !eng.check_m3col(range, source).is_empty() {
                fired = true;
            }
        }
        assert!(fired);
    }

    #[test]
    fn test_g3_target_nodes_include_range() {
        assert!(TARGET_NODES.contains(&"range"));
        assert!(TARGET_NODES.contains(&"comparison_operator"));
    }

    #[test]
    fn test_g3_checks_fire_through_dispatch() {
        let source = "if isa(x,'double') == true\nend\nif isa(x,'double') ~= true\nend\na = 1:2:3:4;\n";
        let tree = parse(source);
        let eng = engine();

        let mut ids = Vec::new();
        collect_diagnostics(&eng, tree.root_node(), source, &mut ids);
        assert!(ids.iter().any(|id| id == &"COMPNOP"), "got: {ids:?}");
        assert!(ids.iter().any(|id| id == &"COMPNOT"), "got: {ids:?}");
        assert!(ids.iter().any(|id| id == &"M3COL"), "got: {ids:?}");
    }

    #[test]
    fn test_g3_checks_respect_disabled_checks() {
        let eng = GoodPracticesEngine {
            config: GoodPracticesConfig {
                max_variable_name_length: 63,
                disabled_checks: vec![
                    "COMPNOP".to_string(),
                    "COMPNOT".to_string(),
                    "M3COL".to_string(),
                ],
            },
        };

        let source = "if isa(x,'double') == true\nend\nif isa(x,'double') ~= true\nend\na = 1:2:3:4;\n";
        let tree = parse(source);
        let mut ids = Vec::new();
        collect_diagnostics(&eng, tree.root_node(), source, &mut ids);
        assert!(ids.iter().all(|id| id != &"COMPNOP"), "got: {ids:?}");
        assert!(ids.iter().all(|id| id != &"COMPNOT"), "got: {ids:?}");
        assert!(ids.iter().all(|id| id != &"M3COL"), "got: {ids:?}");
    }

    /// Collect rule IDs from every node in the subtree, dispatching through the
    /// engine's public `check` entry point.
    fn collect_diagnostics(
        eng: &GoodPracticesEngine,
        node: Node,
        source: &str,
        ids: &mut Vec<&'static str>,
    ) {
        let ctx = NodeContext {
            node,
            source,
            file_path: std::path::Path::new("test.m"),
        };
        ids.extend(eng.check(&ctx).iter().map(|d| d.rule_id));
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_diagnostics(eng, child, source, ids);
        }
    }

    /// Find all descendant nodes of the given kind.
    fn all_descendants_of_kind<'a>(node: Node<'a>, kind: &str) -> Vec<Node<'a>> {
        let mut result = Vec::new();
        if node.kind() == kind {
            result.push(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            result.extend(all_descendants_of_kind(child, kind));
        }
        result
    }

    /// The twelve OOP/class/property check IDs implemented by GOODPRAC G1.
    const OOP_CHECK_IDS: &[&str] = &[
        "ATTF", "ATTOF", "MCPO", "MCSAC", "MOBSRV", "MDEPIN", "MCCPI", "MGMD", "MCCPE", "MTHANS",
        "MHERM", "MNUML",
    ];

    // -- ATTF / ATTOF --------------------------------------------------------

    #[test]
    fn test_attof_fires_on_abstract_false() {
        let source = "classdef (Abstract = false) Foo\nend\n";
        let tree = parse(source);
        let eng = engine();
        let diags = eng.check_attf_attof(&tree, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "ATTOF");
    }

    #[test]
    fn test_attf_fires_on_non_literal_abstract() {
        let source = "classdef (Abstract = someVar) Bar\nend\n";
        let tree = parse(source);
        let eng = engine();
        let diags = eng.check_attf_attof(&tree, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "ATTF");
    }

    #[test]
    fn test_attf_attof_silent_on_abstract_true() {
        let source = "classdef (Abstract = true) Foo\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_attf_attof(&tree, source).is_empty());
    }

    #[test]
    fn test_attf_attof_respect_disabled_checks() {
        let eng = GoodPracticesEngine {
            config: GoodPracticesConfig {
                max_variable_name_length: 63,
                disabled_checks: vec!["ATTF".to_string(), "ATTOF".to_string()],
            },
        };
        let source = "classdef (Abstract = false) Foo\nend\n";
        let tree = parse(source);
        assert!(eng.check_attf_attof(&tree, source).is_empty());
    }

    // -- MCPO ----------------------------------------------------------------

    #[test]
    fn test_mcpo_fires_on_value_class_observable() {
        let source = "classdef ValClass\n    properties (SetObservable)\n        Data\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        let diags = eng.check_mcpo(&tree, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MCPO");
    }

    #[test]
    fn test_mcpo_silent_on_handle_class() {
        let source = "classdef HClass < handle\n    properties (SetObservable)\n        Data\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_mcpo(&tree, source).is_empty());
    }

    #[test]
    fn test_mcpo_respects_disabled_check() {
        let eng = GoodPracticesEngine {
            config: GoodPracticesConfig {
                max_variable_name_length: 63,
                disabled_checks: vec!["MCPO".to_string()],
            },
        };
        let source = "classdef ValClass\n    properties (SetObservable)\n        Data\n    end\nend\n";
        let tree = parse(source);
        assert!(eng.check_mcpo(&tree, source).is_empty());
    }

    // -- MCSAC ---------------------------------------------------------------

    #[test]
    fn test_mcsac_fires_on_constant_setaccess() {
        let source = "classdef MyConst\n    properties (Constant, SetAccess = public)\n        X = 1\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        let diags = eng.check_mcsac(&tree, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MCSAC");
    }

    #[test]
    fn test_mcsac_silent_without_setaccess() {
        let source = "classdef MyConst\n    properties (Constant)\n        X = 1\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_mcsac(&tree, source).is_empty());
    }

    // -- MOBSRV --------------------------------------------------------------

    #[test]
    fn test_mobsrv_fires_on_constant_observable() {
        let source = "classdef MyConst\n    properties (Constant, GetObservable)\n        X = 1\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        let diags = eng.check_mobsrv(&tree, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MOBSRV");
    }

    #[test]
    fn test_mobsrv_silent_on_constant_only() {
        let source = "classdef MyConst\n    properties (Constant)\n        X = 1\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_mobsrv(&tree, source).is_empty());
    }

    // -- MDEPIN --------------------------------------------------------------

    #[test]
    fn test_mdepin_fires_on_dependent_default() {
        let source = "classdef DepClass\n    properties (Dependent)\n        Y = 5\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        let diags = eng.check_mdepin(&tree, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MDEPIN");
    }

    #[test]
    fn test_mdepin_silent_on_dependent_without_default() {
        let source = "classdef DepClass\n    properties (Dependent)\n        Y\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_mdepin(&tree, source).is_empty());
    }

    // -- MCCPI ---------------------------------------------------------------

    #[test]
    fn test_mccpi_fires_on_uninitialized_constant() {
        let source = "classdef MyConst\n    properties (Constant)\n        X\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        let diags = eng.check_mccpi(&tree, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MCCPI");
    }

    #[test]
    fn test_mccpi_silent_when_initialized() {
        let source = "classdef MyConst\n    properties (Constant)\n        X = 1\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_mccpi(&tree, source).is_empty());
    }

    #[test]
    fn test_mccpi_silent_on_abstract_block() {
        let source = "classdef MyConst\n    properties (Constant, Abstract)\n        X\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_mccpi(&tree, source).is_empty());
    }

    #[test]
    fn test_mccpi_silent_on_abstract_class() {
        let source = "classdef (Abstract) AbsClass\n    properties (Constant)\n        X\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_mccpi(&tree, source).is_empty());
    }

    // -- MGMD ----------------------------------------------------------------

    #[test]
    fn test_mgmd_fires_on_dependent_without_getter() {
        let source = "classdef DepClass\n    properties (Dependent)\n        Y\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        let diags = eng.check_mgmd(&tree, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MGMD");
    }

    #[test]
    fn test_mgmd_silent_when_getter_exists() {
        let source = "classdef DepClass\n    properties (Dependent)\n        Y\n    end\n    methods\n        function val = get.Y(obj)\n            val = 1;\n        end\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_mgmd(&tree, source).is_empty());
    }

    #[test]
    fn test_mgmd_silent_on_private_getaccess() {
        let source = "classdef DepClass\n    properties (Dependent, GetAccess = private)\n        Y\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_mgmd(&tree, source).is_empty());
    }

    // -- MTHANS --------------------------------------------------------------

    #[test]
    fn test_mthans_fires_on_method_named_ans() {
        let source = "classdef Foo\n    methods\n        function ans = ans(obj)\n            ans = 1;\n        end\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        let diags = eng.check_mthans(&tree, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MTHANS");
    }

    #[test]
    fn test_mthans_silent_on_regular_method() {
        let source = "classdef Foo\n    methods\n        function z = calc(obj)\n            z = 1;\n        end\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_mthans(&tree, source).is_empty());
    }

    // -- MCCPE ---------------------------------------------------------------

    #[test]
    fn test_mccpe_fires_on_property_called_as_function() {
        let source = "classdef Foo\n    properties\n        Color\n    end\n    methods\n        function go(obj)\n            y = Color(1);\n        end\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        let diags = eng.check_mccpe(&tree, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MCCPE");
    }

    #[test]
    fn test_mccpe_silent_on_field_access() {
        let source = "classdef Foo\n    properties\n        Color\n    end\n    methods\n        function go(obj)\n            x = obj.Color;\n        end\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        assert!(eng.check_mccpe(&tree, source).is_empty());
    }

    // -- MHERM ---------------------------------------------------------------

    #[test]
    fn test_mherm_fires_on_unparenthesized_transpose_multiply() {
        let source = "classdef Foo\n    methods\n        function z = compute(obj, x)\n            z = x * x';\n        end\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        let nodes = all_descendants_of_kind(tree.root_node(), "binary_operator");
        let diags: Vec<Diagnostic> = nodes
            .iter()
            .flat_map(|n| eng.check_mherm(*n, source))
            .collect();
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MHERM");
    }

    #[test]
    fn test_mherm_silent_when_parenthesized() {
        let source = "classdef Foo\n    methods\n        function z = compute(obj, x)\n            z = (x * x');\n        end\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        let nodes = all_descendants_of_kind(tree.root_node(), "binary_operator");
        let diags: Vec<Diagnostic> = nodes
            .iter()
            .flat_map(|n| eng.check_mherm(*n, source))
            .collect();
        assert!(diags.is_empty());
    }

    // -- MNUML ---------------------------------------------------------------

    #[test]
    fn test_mnuml_fires_on_single_numel_arg() {
        let source = "y = zeros(numel(x));\n";
        let tree = parse(source);
        let eng = engine();
        let nodes = all_descendants_of_kind(tree.root_node(), "function_call");
        let diags: Vec<Diagnostic> = nodes
            .iter()
            .flat_map(|n| eng.check_mnuml(*n, source))
            .collect();
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MNUML");
    }

    #[test]
    fn test_mnuml_silent_on_multiple_args() {
        let source = "y = zeros(numel(x), 1);\n";
        let tree = parse(source);
        let eng = engine();
        let nodes = all_descendants_of_kind(tree.root_node(), "function_call");
        let diags: Vec<Diagnostic> = nodes
            .iter()
            .flat_map(|n| eng.check_mnuml(*n, source))
            .collect();
        assert!(diags.is_empty());
    }

    #[test]
    fn test_mnuml_silent_on_literal_args() {
        let source = "y = zeros(3, 3);\n";
        let tree = parse(source);
        let eng = engine();
        let nodes = all_descendants_of_kind(tree.root_node(), "function_call");
        let diags: Vec<Diagnostic> = nodes
            .iter()
            .flat_map(|n| eng.check_mnuml(*n, source))
            .collect();
        assert!(diags.is_empty());
    }

    // -- Node-level dispatch (MHERM + MNUML) through public check() ----------

    #[test]
    fn test_g1_node_checks_fire_through_dispatch() {
        let source = "classdef Foo\n    methods\n        function z = compute(obj, x)\n            z = x * x';\n        end\n    end\nend\ny = zeros(numel(x));\n";
        let tree = parse(source);
        let eng = engine();

        let mut ids = Vec::new();
        collect_diagnostics(&eng, tree.root_node(), source, &mut ids);
        assert!(ids.iter().any(|id| id == &"MHERM"), "got: {ids:?}");
        assert!(ids.iter().any(|id| id == &"MNUML"), "got: {ids:?}");
    }

    // -- File-level dispatch through public check_file() ---------------------

    #[test]
    fn test_g1_file_checks_fire_through_dispatch() {
        let source = "classdef ValClass\n    properties (SetObservable)\n        Data\n    end\n    properties (Constant, SetAccess = public)\n        X = 1\n    end\n    properties (Constant, GetObservable)\n        W\n    end\n    properties (Constant)\n        Z\n    end\n    properties (Dependent)\n        Y = 5\n    end\nend\n";
        let tree = parse(source);
        let eng = engine();
        let ctx = FileContext {
            tree: &tree,
            source,
            file_path: std::path::Path::new("test.m"),
        };
        let ids: Vec<&str> = eng.check_file(&ctx).iter().map(|d| d.rule_id).collect();
        for expected in ["MCPO", "MCSAC", "MOBSRV", "MCCPI", "MDEPIN", "MGMD"] {
            assert!(ids.iter().any(|id| id == &expected), "missing {expected}, got: {ids:?}");
        }
    }

    #[test]
    fn test_g1_checks_silent_on_valid_source() {
        let source = "classdef (Abstract = true) Foo\n    properties X = 42\n    end\n    methods\n        function z = compute(obj, x)\n            z = (x * x');\n        end\n    end\nend\ny = zeros(3, 3);\n";
        let tree = parse(source);
        let eng = engine();

        let mut ids = Vec::new();
        collect_diagnostics(&eng, tree.root_node(), source, &mut ids);
        let ctx = FileContext {
            tree: &tree,
            source,
            file_path: std::path::Path::new("test.m"),
        };
        ids.extend(eng.check_file(&ctx).iter().map(|d| d.rule_id));

        let oop_hits: Vec<&str> = ids
            .iter()
            .copied()
            .filter(|id| OOP_CHECK_IDS.contains(id))
            .collect();
        assert!(oop_hits.is_empty(), "unexpected OOP diagnostics: {oop_hits:?}");
    }

    // -- G4: CTPCT format/arg count ------------------------------------------

    #[test]
    fn test_ctpct_fires_on_mismatch() {
        let source = "fprintf('%d %d', x);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_ctpct(fc, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "CTPCT");
    }

    #[test]
    fn test_ctpct_fires_on_sprintf_mismatch() {
        let source = "sprintf('%d %f', x);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_ctpct(fc, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "CTPCT");
    }

    #[test]
    fn test_ctpct_silent_on_match() {
        let source = "fprintf('%d', x);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_ctpct(fc, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_ctpct_silent_on_escaped_percent() {
        let source = "sprintf('100%% %d', x);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_ctpct(fc, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_ctpct_silent_with_file_id() {
        let source = "fprintf(fid, '%d', x);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_ctpct(fc, source);
        assert!(diags.is_empty());
    }

    // -- G4: FXSET loop index assignment -------------------------------------

    #[test]
    fn test_fxset_fires_on_iterator_assignment() {
        let source = "for i = 1:10\n  i = 5;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let for_node = find_child_of_kind(root, "for_statement").unwrap();
        let diags = eng.check_fxset(for_node, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "FXSET");
    }

    #[test]
    fn test_fxset_silent_on_read_only_iterator() {
        let source = "for i = 1:10\n  y = i;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let for_node = find_child_of_kind(root, "for_statement").unwrap();
        let diags = eng.check_fxset(for_node, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_fxset_silent_on_other_variable_assignment() {
        let source = "for i = 1:10\n  j = 5;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let for_node = find_child_of_kind(root, "for_statement").unwrap();
        let diags = eng.check_fxset(for_node, source);
        assert!(diags.is_empty());
    }

    // -- G4: SIMPT import placement ------------------------------------------

    #[test]
    fn test_simpt_fires_when_not_first_statement() {
        let source = "function f()\n  x = 1;\n  import foo.bar\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let cmd = find_descendant_of_kind(root, "command").unwrap();
        let diags = eng.check_simpt(cmd, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "SIMPT");
    }

    #[test]
    fn test_simpt_silent_when_first_statement() {
        let source = "function f()\n  import foo.bar;\n  x = 1;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let cmd = find_descendant_of_kind(root, "command").unwrap();
        let diags = eng.check_simpt(cmd, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_simpt_silent_at_script_level() {
        let source = "import foo.bar\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let cmd = find_child_of_kind(root, "command").unwrap();
        let diags = eng.check_simpt(cmd, source);
        assert!(diags.is_empty());
    }

    // -- G4: TLEV dynamic-code sub-expression ---------------------------------

    #[test]
    fn test_tlev_fires_on_bare_eval_statement() {
        let source = "eval(x + y);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_child_of_kind(root, "function_call").unwrap();
        let diags = eng.check_tlev(fc, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "TLEV");
    }

    #[test]
    fn test_tlev_fires_on_nested_eval() {
        let source = "x = f(eval(y));\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let eval_call = goodprac_g4_find_call(root, "eval", source).unwrap();
        let diags = eng.check_tlev(eval_call, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "TLEV");
    }

    #[test]
    fn test_tlev_silent_on_whole_rhs() {
        let source = "x = eval(y);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_tlev(fc, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_tlev_silent_on_evalc_whole_rhs() {
        let source = "x = evalc('y = 1');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_tlev(fc, source);
        assert!(diags.is_empty());
    }

    // -- G4: UNONC onCleanup assignment --------------------------------------

    #[test]
    fn test_unonc_fires_on_bare_call() {
        let source = "onCleanup(@f);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_unonc(fc, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "UNONC");
    }

    #[test]
    fn test_unonc_fires_on_tilde_assignment() {
        let source = "[~] = onCleanup(@f);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_unonc(fc, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "UNONC");
    }

    #[test]
    fn test_unonc_silent_when_assigned() {
        let source = "h = onCleanup(@f);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_unonc(fc, source);
        assert!(diags.is_empty());
    }

    // -- G4: MIPC1 computer('arch') ------------------------------------------

    #[test]
    fn test_mipc1_fires_on_arch() {
        let source = "c = computer('arch');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_mipc1(fc, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MIPC1");
    }

    #[test]
    fn test_mipc1_silent_on_other_argument() {
        let source = "c = computer('win');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        let diags = eng.check_mipc1(fc, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_mipc1_silent_without_parens() {
        let source = "c = computer;\n";
        let tree = parse(source);
        let root = tree.root_node();

        assert!(find_descendant_of_kind(root, "function_call").is_none());
    }

    // -- G4: dispatch + disabled ---------------------------------------------

    #[test]
    fn test_g4_checks_fire_through_dispatch() {
        let source = "for i = 1:10\n    i = 5;\nend\n\
function f()\n    x = 1;\n    import foo.bar\nend\n\
eval(x + y);\nonCleanup(@f);\n[~] = onCleanup(@f);\nc = computer('arch');\nfprintf('%d %d', x);\n";
        let tree = parse(source);
        let eng = engine();

        let mut ids = Vec::new();
        collect_diagnostics(&eng, tree.root_node(), source, &mut ids);
        for want in ["FXSET", "SIMPT", "TLEV", "UNONC", "MIPC1", "CTPCT"] {
            assert!(ids.iter().any(|id| id == &want), "expected {want} to fire, got: {ids:?}");
        }
    }

    #[test]
    fn test_g4_checks_respect_disabled_checks() {
        let eng = GoodPracticesEngine {
            config: GoodPracticesConfig {
                max_variable_name_length: 63,
                disabled_checks: vec![
                    "CTPCT".to_string(),
                    "FXSET".to_string(),
                    "SIMPT".to_string(),
                    "TLEV".to_string(),
                    "UNONC".to_string(),
                    "MIPC1".to_string(),
                ],
            },
        };

        let source = "fprintf('%d %d', x);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        assert!(eng.check_ctpct(fc, source).is_empty());

        let source = "for i = 1:10\n  i = 5;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let for_node = find_child_of_kind(root, "for_statement").unwrap();
        assert!(eng.check_fxset(for_node, source).is_empty());

        let source = "function f()\n  x = 1;\n  import foo.bar\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let cmd = find_descendant_of_kind(root, "command").unwrap();
        assert!(eng.check_simpt(cmd, source).is_empty());

        let source = "eval(x + y);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        assert!(eng.check_tlev(fc, source).is_empty());

        let source = "onCleanup(@f);\n";
        let tree = parse(source);
        let root = tree.root_node();
        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        assert!(eng.check_unonc(fc, source).is_empty());

        let source = "c = computer('arch');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let fc = find_descendant_of_kind(root, "function_call").unwrap();
        assert!(eng.check_mipc1(fc, source).is_empty());
    }

    /// G4 helper: find the first `function_call` descendant with the given name.
    fn goodprac_g4_find_call<'a>(node: Node<'a>, name: &str, source: &'a str) -> Option<Node<'a>> {
        if node.kind() == "function_call" && get_function_call_name(node, source) == Some(name) {
            return Some(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(found) = goodprac_g4_find_call(child, name, source) {
                return Some(found);
            }
        }
        None
    }

    // -- Parfor / SPMD checks (G2) ------------------------------------------

    /// The 11 parfor/spmd check IDs implemented by this group.
    const G2_IDS: &[&str] = &[
        "PFEVB", "PFGP", "PFGV", "PFIIN", "PFOUS", "PFRNI", "PFTUSW", "PFUIXW", "SPEVB", "SPGV",
        "DSPMDA",
    ];

    /// Run the full engine (node-level + file-level) and collect rule IDs.
    fn g2_collect_ids(
        eng: &GoodPracticesEngine,
        tree: &tree_sitter::Tree,
        source: &str,
    ) -> Vec<&'static str> {
        let mut ids = Vec::new();
        let mut stack = vec![tree.root_node()];
        while let Some(n) = stack.pop() {
            let ctx = NodeContext {
                node: n,
                source,
                file_path: std::path::Path::new("test.m"),
            };
            ids.extend(eng.check(&ctx).iter().map(|d| d.rule_id));
            let mut cursor = n.walk();
            for c in n.children(&mut cursor) {
                stack.push(c);
            }
        }
        let fc = FileContext {
            tree,
            source,
            file_path: std::path::Path::new("test.m"),
        };
        ids.extend(eng.check_file(&fc).iter().map(|d| d.rule_id));
        ids
    }

    /// Run the parfor file-level check and keep only diagnostics with a given ID.
    fn g2_file_ids_with(
        eng: &GoodPracticesEngine,
        tree: &tree_sitter::Tree,
        source: &str,
        check_id: &str,
    ) -> Vec<&'static str> {
        eng.check_parfor_file_level(tree, source)
            .iter()
            .filter(|d| d.rule_id == check_id)
            .map(|d| d.rule_id)
            .collect()
    }

    // -- PFRNI --------------------------------------------------------------

    #[test]
    fn test_pfrni_fires_on_explicit_step() {
        let source = "parfor i = 1:2:10\n    x(i) = i;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let parfor = find_child_of_kind(root, "for_statement").unwrap();
        let diags = eng.check_pfrni(parfor, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "PFRNI");
        assert!(diags[0].fix.is_some());
    }

    #[test]
    fn test_pfrni_silent_on_unit_step_and_regular_for() {
        let eng = engine();

        let source = "parfor i = 1:10\n    x(i) = i;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let parfor = find_child_of_kind(root, "for_statement").unwrap();
        assert!(eng.check_pfrni(parfor, source).is_empty());

        let source = "for i = 1:2:10\n    x(i) = i;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let parfor = find_child_of_kind(root, "for_statement").unwrap();
        assert!(eng.check_pfrni(parfor, source).is_empty());
    }

    // -- PFEVB --------------------------------------------------------------

    #[test]
    fn test_pfevb_fires_on_base_evalin_assignin() {
        let source = "parfor i = 1:10\n    evalin('base','x');\n    assignin('base','y',1);\n    x(i) = i;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let parfor = find_child_of_kind(root, "for_statement").unwrap();
        let diags = eng.check_pfevb(parfor, source);
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.rule_id == "PFEVB"));
    }

    #[test]
    fn test_pfevb_silent_on_caller_workspace() {
        let source = "parfor i = 1:10\n    evalin('caller','x');\n    x(i) = i;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let parfor = find_child_of_kind(root, "for_statement").unwrap();
        assert!(eng.check_pfevb(parfor, source).is_empty());
    }

    // -- PFGP ---------------------------------------------------------------

    #[test]
    fn test_pfgp_fires_on_global_assignment() {
        let source = "global gVar;\nparfor i = 1:10\n    gVar = i;\n    x(i) = gVar;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let parfor = find_child_of_kind(root, "for_statement").unwrap();
        let diags = eng.check_pfgp(parfor, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "PFGP");
        assert!(diags[0].message.contains("gVar"));
    }

    #[test]
    fn test_pfgp_silent_without_globals() {
        let source = "parfor i = 1:10\n    q = i;\n    x(i) = q;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let parfor = find_child_of_kind(root, "for_statement").unwrap();
        assert!(eng.check_pfgp(parfor, source).is_empty());
    }

    // -- PFGV ---------------------------------------------------------------

    #[test]
    fn test_pfgv_fires_on_global_use() {
        let source = "global gVar;\nparfor i = 1:10\n    gVar = i;\n    x(i) = gVar;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let parfor = find_child_of_kind(root, "for_statement").unwrap();
        let diags = eng.check_pfgv(parfor, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "PFGV");
    }

    #[test]
    fn test_pfgv_silent_without_globals() {
        let source = "parfor i = 1:10\n    q = i;\n    x(i) = q;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let parfor = find_child_of_kind(root, "for_statement").unwrap();
        assert!(eng.check_pfgv(parfor, source).is_empty());
    }

    // -- SPEVB --------------------------------------------------------------

    #[test]
    fn test_spevb_fires_on_base_evalin_in_spmd() {
        let source = "spmd\n    evalin('base','x');\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let spmd = find_child_of_kind(root, "spmd_statement").unwrap();
        let diags = eng.check_spevb(spmd, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "SPEVB");
    }

    #[test]
    fn test_spevb_silent_on_caller_workspace_in_spmd() {
        let source = "spmd\n    evalin('caller','x');\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let spmd = find_child_of_kind(root, "spmd_statement").unwrap();
        assert!(eng.check_spevb(spmd, source).is_empty());
    }

    // -- SPGV ---------------------------------------------------------------

    #[test]
    fn test_spgv_fires_on_global_use_in_spmd() {
        let source = "global g2;\nspmd\n    z = g2 + 1;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let spmd = find_child_of_kind(root, "spmd_statement").unwrap();
        let diags = eng.check_spgv(spmd, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "SPGV");
    }

    #[test]
    fn test_spgv_silent_without_globals() {
        let source = "spmd\n    z = 1 + 1;\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let spmd = find_child_of_kind(root, "spmd_statement").unwrap();
        assert!(eng.check_spgv(spmd, source).is_empty());
    }

    // -- DSPMDA -------------------------------------------------------------

    #[test]
    fn test_dspmda_fires_on_distributed_constructors_in_spmd() {
        let source = "spmd\n    d = distributed(zeros(100));\n    g = gpuArray(1);\n    c = codistributed(2);\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let spmd = find_child_of_kind(root, "spmd_statement").unwrap();
        let diags = eng.check_dspmda(spmd, source);
        assert_eq!(diags.len(), 3);
        assert!(diags.iter().all(|d| d.rule_id == "DSPMDA"));
    }

    #[test]
    fn test_dspmda_silent_when_created_outside_spmd() {
        let source = "d = distributed(zeros(100));\nspmd\n    work_with(d);\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let spmd = find_child_of_kind(root, "spmd_statement").unwrap();
        assert!(eng.check_dspmda(spmd, source).is_empty());
    }

    // -- PFIIN --------------------------------------------------------------

    #[test]
    fn test_pfiin_fires_on_uninitialized_input() {
        let source = "parfor i = 1:10\n    q = z + i;\nend\n";
        let tree = parse(source);
        let eng = engine();

        let ids = g2_file_ids_with(&eng, &tree, source, "PFIIN");
        assert_eq!(ids.len(), 1);
        let diag = eng
            .check_parfor_file_level(&tree, source)
            .into_iter()
            .find(|d| d.rule_id == "PFIIN")
            .unwrap();
        assert!(diag.message.contains("z"));
    }

    #[test]
    fn test_pfiin_silent_when_initialized_or_input_arg() {
        let eng = engine();

        let source = "z = 0;\nparfor i = 1:10\n    q = z + i;\nend\n";
        let tree = parse(source);
        assert!(g2_file_ids_with(&eng, &tree, source, "PFIIN").is_empty());

        let source = "function f(z)\nparfor i = 1:10\n    q = z + i;\nend\nend\n";
        let tree = parse(source);
        assert!(g2_file_ids_with(&eng, &tree, source, "PFIIN").is_empty());
    }

    // -- PFOUS --------------------------------------------------------------

    #[test]
    fn test_pfous_fires_on_output_not_used_after() {
        let source = "parfor i = 1:10\n    q = i;\nend\n";
        let tree = parse(source);
        let eng = engine();

        let ids = g2_file_ids_with(&eng, &tree, source, "PFOUS");
        assert_eq!(ids.len(), 1);
        let diag = eng
            .check_parfor_file_level(&tree, source)
            .into_iter()
            .find(|d| d.rule_id == "PFOUS")
            .unwrap();
        assert!(diag.message.contains("q"));
    }

    #[test]
    fn test_pfous_silent_when_used_after() {
        let source = "parfor i = 1:10\n    q = i;\nend\ndisp(q);\n";
        let tree = parse(source);
        let eng = engine();

        assert!(g2_file_ids_with(&eng, &tree, source, "PFOUS").is_empty());
    }

    // -- PFTUSW -------------------------------------------------------------

    #[test]
    fn test_pftusw_fires_on_temp_used_after() {
        let source = "parfor i = 1:10\n    tmp = compute(i);\n    x(i) = tmp;\nend\ndisp(tmp);\n";
        let tree = parse(source);
        let eng = engine();

        let ids = g2_file_ids_with(&eng, &tree, source, "PFTUSW");
        assert_eq!(ids.len(), 1);
        let diag = eng
            .check_parfor_file_level(&tree, source)
            .into_iter()
            .find(|d| d.rule_id == "PFTUSW")
            .unwrap();
        assert!(diag.message.contains("on line 5"), "got: {}", diag.message);
    }

    #[test]
    fn test_pftusw_silent_without_after_use() {
        let source = "parfor i = 1:10\n    tmp = compute(i);\n    x(i) = tmp;\nend\n";
        let tree = parse(source);
        let eng = engine();

        assert!(g2_file_ids_with(&eng, &tree, source, "PFTUSW").is_empty());
    }

    // -- PFUIXW -------------------------------------------------------------

    #[test]
    fn test_pfuiwx_fires_on_index_used_after() {
        let source = "parfor i = 1:10\n    x(i) = i;\nend\ndisp(i);\n";
        let tree = parse(source);
        let eng = engine();

        let ids = g2_file_ids_with(&eng, &tree, source, "PFUIXW");
        assert_eq!(ids.len(), 1);
        let diag = eng
            .check_parfor_file_level(&tree, source)
            .into_iter()
            .find(|d| d.rule_id == "PFUIXW")
            .unwrap();
        assert!(diag.message.contains("on line 4"), "got: {}", diag.message);
    }

    #[test]
    fn test_pfuiwx_silent_without_after_use() {
        let source = "parfor i = 1:10\n    x(i) = i;\nend\n";
        let tree = parse(source);
        let eng = engine();

        assert!(g2_file_ids_with(&eng, &tree, source, "PFUIXW").is_empty());
    }

    // -- Disabled checks ----------------------------------------------------

    #[test]
    fn test_g2_checks_respect_disabled_checks() {
        let eng = GoodPracticesEngine {
            config: GoodPracticesConfig {
                max_variable_name_length: 63,
                disabled_checks: vec!["PFRNI".to_string(), "PFUIXW".to_string()],
            },
        };

        let source = "parfor i = 1:2:10\n    evalin('base','x');\n    x(i) = i;\nend\ndisp(i);\n";
        let tree = parse(source);
        let ids = g2_collect_ids(&eng, &tree, source);
        assert!(ids.iter().all(|id| id != &"PFRNI"), "got: {ids:?}");
        assert!(ids.iter().all(|id| id != &"PFUIXW"), "got: {ids:?}");
        assert!(ids.iter().any(|id| id == &"PFEVB"), "got: {ids:?}");
    }

    // -- Integration: MUST-fire / MUST-NOT-fire examples --------------------

    #[test]
    fn test_g2_must_fire_example() {
        let eng = engine();
        let source = "parfor i = 1:2:10\n    evalin('base','x');\n    x(i) = i;\nend\nparfor i = 1:10\n    tmp = compute(i);\n    x(i) = tmp;\nend\ndisp(tmp);\nparfor i = 1:10\n    x(i) = i;\nend\ndisp(i);\nspmd\n    d = distributed(zeros(100));\nend\n";
        let tree = parse(source);
        let ids = g2_collect_ids(&eng, &tree, source);
        for must in ["PFRNI", "PFEVB", "PFTUSW", "PFUIXW", "DSPMDA"] {
            assert!(ids.contains(&must), "{must} missing from: {ids:?}");
        }
    }

    #[test]
    fn test_g2_must_not_fire_example() {
        let eng = engine();
        let source = "parfor i = 1:10\n    evalin('caller','x');\n    x(i) = i;\nend\ny = zeros(1,10);\nparfor i = 1:10\n    x(i) = y(i);\nend\nd = distributed(zeros(100));\nspmd\n    work_with(d);\nend\n";
        let tree = parse(source);
        let ids = g2_collect_ids(&eng, &tree, source);
        let g2_fired: Vec<&str> = ids
            .iter()
            .copied()
            .filter(|id| G2_IDS.contains(id))
            .collect();
        assert!(g2_fired.is_empty(), "unexpected G2 fires: {g2_fired:?}");
    }
}
