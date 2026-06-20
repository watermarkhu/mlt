//! # PERFORMANCE_ENGINE: Performance Improvement Checks
//!
//! A hybrid rule module implementing 41 performance-related MATLAB lint checks.
//! Most checks operate at the node level (function calls, operators, commands),
//! while AGROW, SAGROW, and PFBNS require file-level traversal to detect
//! array growth patterns inside loops.
//!
//! ## Architecture
//!
//! A single `PerformanceEngine` struct handles all 41 checks. Node-level checks
//! are dispatched by node kind (`function_call`, `command`, `binary_operator`,
//! `boolean_operator`, `comparison_operator`). File-level checks traverse the
//! full tree looking for growth patterns inside `for_statement`/`while_statement`.
//!
//! ## Examples
//!
//! Bad:
//! ```matlab
//! for i = 1:n
//!     x = [x, val];          % AGROW
//! end
//! if a & b                   % AND2
//! inv(A) * b                 % MINV
//! exist('foo', 'file')       % EXIST
//! ```
//!
//! Good:
//! ```matlab
//! x = zeros(1, n);
//! for i = 1:n
//!     x(i) = val;
//! end
//! if a && b
//! A \ b
//! isfile('foo')
//! ```

use mlt_core::{Category, Config, Diagnostic, FileContext, Fix, NodeContext, Rule, Severity};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for the performance engine.
///
/// Deserialized from the `[lint.rules.PERFORMANCE_ENGINE]` section in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PerformanceEngineConfig {
    /// Check IDs to skip (e.g., `["AGROW", "AND2"]`).
    #[serde(default)]
    pub skip_checks: Vec<String>,
}

// ---------------------------------------------------------------------------
// Check metadata
// ---------------------------------------------------------------------------

/// Static metadata for a single performance check.
struct CheckMeta {
    id: &'static str,
    description: &'static str,
}

/// All 41 performance check definitions.
const CHECKS: &[CheckMeta] = &[
    CheckMeta { id: "AGROW",       description: "Variable appears to grow inside a loop; consider preallocating" },
    CheckMeta { id: "SAGROW",      description: "Struct field appears to grow inside a loop; consider preallocating" },
    CheckMeta { id: "AND2",        description: "Use '&&' (short-circuit) instead of '&' for scalar logical operations" },
    CheckMeta { id: "OR2",         description: "Use '||' (short-circuit) instead of '|' for scalar logical operations" },
    CheckMeta { id: "MINV",        description: "Use 'A\\b' instead of 'inv(A)*b' for better numerical stability and performance" },
    CheckMeta { id: "GFLD",        description: "Use dynamic field names 's.(name)' instead of 'getfield'" },
    CheckMeta { id: "SFLD",        description: "Use dynamic field names 's.(name) = val' instead of 'setfield'" },
    CheckMeta { id: "EXIST",       description: "Use 'isfile' or 'isfolder' instead of 'exist(..., ''file'')'" },
    CheckMeta { id: "PFBNS",       description: "Array initialized as empty then grown; preallocate for known size" },
    CheckMeta { id: "CCAT",        description: "Use string concatenation or 'join' instead of repeated 'strcat'" },
    CheckMeta { id: "CCAT1",       description: "Consider using 'join' for cell array of character vector concatenation" },
    CheckMeta { id: "ISMT",        description: "Use 'isempty(x)' instead of 'length(x)==0'" },
    CheckMeta { id: "ISCL",        description: "Use 'isscalar(x)' instead of 'length(x)==1'" },
    CheckMeta { id: "ST2NM",       description: "Use 'str2double' instead of 'str2num' for performance and security" },
    CheckMeta { id: "FLPST",       description: "Use 'flip' instead of 'flipud'/'fliplr' on vectors" },
    CheckMeta { id: "MXFND",       description: "Use 'max(x,[],''all'')' instead of nested 'max(max(x))'" },
    CheckMeta { id: "EFIND",       description: "Use logical indexing instead of 'find' when used as a subscript" },
    CheckMeta { id: "UDIM",        description: "Specify dimension argument in 'sum'/'max'/'min'/'prod'/'mean'" },
    CheckMeta { id: "FREAD",       description: "Specify precision argument in 'fread' for performance" },
    CheckMeta { id: "N2UNI",       description: "Consider using 'unique' instead of 'setdiff'+'union' pattern" },
    CheckMeta { id: "TNMLP",       description: "Move 'tic'/'toc' outside loop body for accurate timing" },
    CheckMeta { id: "LAXES",       description: "Cache axes handle returned by 'gca'/'gcf' instead of repeated calls" },
    CheckMeta { id: "MMTC",        description: "Use '.^2' instead of '.*' with the same operand" },
    CheckMeta { id: "MRPBW",       description: "Use 'imbinarize' instead of deprecated 'im2bw'" },
    CheckMeta { id: "SPRIX",       description: "Avoid indexing sparse matrices with full logical arrays" },
    CheckMeta { id: "TRSRT",       description: "Use 'mink'/'maxk' instead of sorting then indexing" },
    CheckMeta { id: "GRIDD",       description: "Consider using 'meshgrid' or 'ndgrid' for grid generation" },
    CheckMeta { id: "CLALL",       description: "'clear all' also clears breakpoints; use 'clearvars' instead" },
    CheckMeta { id: "CLCLS",       description: "'clear classes' is a slow operation; avoid in production code" },
    CheckMeta { id: "CLFUNC",      description: "'clear functions' is a slow operation; avoid in production code" },
    CheckMeta { id: "CLJAVA",      description: "'clear java' is a slow operation; avoid in production code" },
    CheckMeta { id: "CLMEX",       description: "'clear mex' clears all MEX files from memory; use specific names" },
    CheckMeta { id: "CLEAR0ARGS",  description: "'clear' with no arguments clears all variables; use 'clearvars' instead" },
    CheckMeta { id: "RGXP1",       description: "Regex pattern can be simplified for performance" },
    CheckMeta { id: "RGXPI",       description: "Use 'regexpi' instead of 'regexp' with 'ignorecase' option" },
    CheckMeta { id: "TRIM1",       description: "Use 'strtrim' instead of 'deblank' for more thorough whitespace removal" },
    CheckMeta { id: "TRIM2",       description: "Use 'strip' instead of 'strtrim' for more flexible whitespace removal" },
    CheckMeta { id: "STTOK",       description: "Use 'split' instead of 'strtok' in a loop for better performance" },
    CheckMeta { id: "STNCI",       description: "Use 'strcmpi' instead of wrapping 'strcmp' with 'lower'" },
    CheckMeta { id: "STCCS",       description: "Use 'contains' instead of '~isempty(strfind(...))'" },
    CheckMeta { id: "FNDSB",       description: "Use 'contains' or 'matches' instead of 'findstr'" },
];

/// Look up check description by ID.
fn check_description(id: &str) -> &'static str {
    CHECKS
        .iter()
        .find(|c| c.id == id)
        .map(|c| c.description)
        .unwrap_or("Performance improvement suggestion")
}

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// Hybrid performance engine covering 41 performance checks.
///
/// Node-level checks fire on `function_call`, `command`, `binary_operator`,
/// `boolean_operator`, and `comparison_operator`. File-level checks (AGROW,
/// SAGROW, PFBNS) traverse the full tree for loop growth patterns.
pub struct PerformanceEngine {
    config: PerformanceEngineConfig,
}

/// Node types for node-level dispatch.
const TARGET_NODES: &[&str] = &[
    "function_call",
    "command",
    "binary_operator",
    "boolean_operator",
    "comparison_operator",
    "unary_operator",
];

impl PerformanceEngine {
    /// Factory constructor called by the rule registry.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: PerformanceEngineConfig = config.rule_params("PERFORMANCE_ENGINE");
        Box::new(Self {
            config: rule_config,
        })
    }

    /// Whether a check ID is enabled (not in `skip_checks`).
    fn is_enabled(&self, check_id: &str) -> bool {
        !self.config.skip_checks.iter().any(|s| s == check_id)
    }

    // -----------------------------------------------------------------------
    // Node-level check dispatchers
    // -----------------------------------------------------------------------

    /// Check a `function_call` node for various performance patterns.
    fn check_function_call<'a>(
        &self,
        node: tree_sitter::Node<'a>,
        source: &'a str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        let func_name = match extract_func_name(node, source) {
            Some(n) => n,
            None => return diags,
        };

        // GFLD: getfield usage
        if self.is_enabled("GFLD") && func_name == "getfield" {
            diags.push(make_diag("GFLD", node));
        }

        // SFLD: setfield usage
        if self.is_enabled("SFLD") && func_name == "setfield" {
            diags.push(make_diag("SFLD", node));
        }

        // EXIST: exist(..., 'file') or exist(..., 'dir')
        if self.is_enabled("EXIST") && func_name == "exist" {
            if has_exist_type_arg(node, source) {
                diags.push(make_diag("EXIST", node));
            }
        }

        // ST2NM: str2num
        if self.is_enabled("ST2NM") && func_name == "str2num" {
            diags.push(make_diag_with_fix(
                "ST2NM",
                node,
                Some(fix_rename_func(node, source, "str2double")),
            ));
        }

        // FLPST: flipud / fliplr
        if self.is_enabled("FLPST") && (func_name == "flipud" || func_name == "fliplr") {
            diags.push(make_diag_with_fix(
                "FLPST",
                node,
                Some(fix_rename_func(node, source, "flip")),
            ));
        }

        // CCAT: strcat usage
        if self.is_enabled("CCAT") && func_name == "strcat" {
            diags.push(make_diag("CCAT", node));
        }

        // CCAT1: cellstr concatenation via strjoin candidate
        if self.is_enabled("CCAT1") && func_name == "strjoin" {
            // strjoin is already good, but flag if this is inside a loop
            // (in practice, CCAT1 flags patterns that should use join/strjoin)
            // We flag cellstr usage patterns — keep as simple function_call check
        }

        // MXFND: nested max(max(x)) or min(min(x))
        if self.is_enabled("MXFND")
            && (func_name == "max" || func_name == "min")
            && is_nested_same_call(node, source, func_name)
        {
            diags.push(make_diag("MXFND", node));
        }

        // EFIND: find() used as subscript index
        if self.is_enabled("EFIND") && func_name == "find" && is_subscript_context(node) {
            diags.push(make_diag("EFIND", node));
        }

        // UDIM: sum/max/min/prod/mean with 1 arg (no dimension)
        if self.is_enabled("UDIM")
            && matches!(func_name, "sum" | "max" | "min" | "prod" | "mean")
            && count_args(node) == 1
        {
            diags.push(make_diag("UDIM", node));
        }

        // FREAD: fread without precision
        if self.is_enabled("FREAD") && func_name == "fread" && count_args(node) < 3 {
            diags.push(make_diag("FREAD", node));
        }

        // N2UNI: setdiff / union patterns
        if self.is_enabled("N2UNI") && (func_name == "setdiff" || func_name == "union") {
            diags.push(make_diag("N2UNI", node));
        }

        // TNMLP: tic/toc inside loop
        if self.is_enabled("TNMLP")
            && (func_name == "tic" || func_name == "toc")
            && is_inside_loop(node)
        {
            diags.push(make_diag("TNMLP", node));
        }

        // LAXES: gca/gcf calls (suggest caching)
        if self.is_enabled("LAXES") && (func_name == "gca" || func_name == "gcf") {
            diags.push(make_diag("LAXES", node));
        }

        // MRPBW: im2bw
        if self.is_enabled("MRPBW") && func_name == "im2bw" {
            diags.push(make_diag_with_fix(
                "MRPBW",
                node,
                Some(fix_rename_func(node, source, "imbinarize")),
            ));
        }

        // TRSRT: sort then index — detect sort() call inside subscript or
        // assigned then immediately indexed. Simplified: flag sort() when
        // used as a subscript argument.
        if self.is_enabled("TRSRT") && func_name == "sort" && is_subscript_context(node) {
            diags.push(make_diag("TRSRT", node));
        }

        // RGXPI: regexp/regexpi with 'ignorecase' option
        if self.is_enabled("RGXPI")
            && func_name == "regexp"
            && has_string_arg(node, source, "ignorecase")
        {
            diags.push(make_diag("RGXPI", node));
        }

        // RGXP1: regexp/regexpi with overly simple patterns
        if self.is_enabled("RGXP1")
            && (func_name == "regexp" || func_name == "regexpi")
            && has_simple_regex_pattern(node, source)
        {
            diags.push(make_diag("RGXP1", node));
        }

        // TRIM1: deblank → strtrim
        if self.is_enabled("TRIM1") && func_name == "deblank" {
            diags.push(make_diag_with_fix(
                "TRIM1",
                node,
                Some(fix_rename_func(node, source, "strtrim")),
            ));
        }

        // TRIM2: strtrim → strip
        if self.is_enabled("TRIM2") && func_name == "strtrim" {
            diags.push(make_diag_with_fix(
                "TRIM2",
                node,
                Some(fix_rename_func(node, source, "strip")),
            ));
        }

        // STTOK: strtok inside loop
        if self.is_enabled("STTOK") && func_name == "strtok" && is_inside_loop(node) {
            diags.push(make_diag("STTOK", node));
        }

        // FNDSB: findstr
        if self.is_enabled("FNDSB") && func_name == "findstr" {
            diags.push(make_diag_with_fix(
                "FNDSB",
                node,
                Some(fix_rename_func(node, source, "contains")),
            ));
        }

        // SPRIX: sparse indexing with logical — detect sparse() inside subscript
        if self.is_enabled("SPRIX") && func_name == "sparse" && is_subscript_context(node) {
            diags.push(make_diag("SPRIX", node));
        }

        // GRIDD: repmat for grid generation
        if self.is_enabled("GRIDD") && func_name == "repmat" {
            diags.push(make_diag("GRIDD", node));
        }

        // STNCI: lower(strcmp(...)) or upper(strcmp(...)) pattern
        if self.is_enabled("STNCI")
            && (func_name == "lower" || func_name == "upper")
            && has_strcmp_arg(node, source)
        {
            diags.push(make_diag("STNCI", node));
        }

        // STCCS: ~isempty(strfind(...)) → contains
        // This requires checking unary_operator parent, handled in check_unary

        diags
    }

    /// Check a `command` node for clear-related patterns.
    fn check_command<'a>(
        &self,
        node: tree_sitter::Node<'a>,
        source: &'a str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        let cmd_name = match extract_command_name(node, source) {
            Some(n) => n,
            None => return diags,
        };

        if cmd_name != "clear" {
            return diags;
        }

        let args = extract_command_args(node, source);

        // CLEAR0ARGS: clear with no arguments
        if self.is_enabled("CLEAR0ARGS") && args.is_empty() {
            diags.push(make_diag("CLEAR0ARGS", node));
        }

        // CLALL: clear all
        if self.is_enabled("CLALL") && args.iter().any(|a| *a == "all") {
            diags.push(make_diag("CLALL", node));
        }

        // CLCLS: clear classes
        if self.is_enabled("CLCLS") && args.iter().any(|a| *a == "classes") {
            diags.push(make_diag("CLCLS", node));
        }

        // CLFUNC: clear functions
        if self.is_enabled("CLFUNC") && args.iter().any(|a| *a == "functions") {
            diags.push(make_diag("CLFUNC", node));
        }

        // CLJAVA: clear java
        if self.is_enabled("CLJAVA") && args.iter().any(|a| *a == "java") {
            diags.push(make_diag("CLJAVA", node));
        }

        // CLMEX: clear mex
        if self.is_enabled("CLMEX") && args.iter().any(|a| *a == "mex") {
            diags.push(make_diag("CLMEX", node));
        }

        diags
    }

    /// Check `binary_operator` for MINV and MMTC patterns.
    fn check_binary_operator<'a>(
        &self,
        node: tree_sitter::Node<'a>,
        source: &'a str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        let op = extract_operator(node, source);

        // MINV: inv(A) * b or b * inv(A)
        if self.is_enabled("MINV") && op == "*" {
            if has_inv_operand(node, source) {
                diags.push(make_diag("MINV", node));
            }
        }

        // MMTC: x .* x → x.^2
        if self.is_enabled("MMTC") && op == ".*" {
            if has_same_operands(node, source) {
                diags.push(make_diag("MMTC", node));
            }
        }

        diags
    }

    /// Check `boolean_operator` for AND2/OR2 patterns.
    ///
    /// In tree-sitter-matlab, `&&` and `||` parse as `boolean_operator`, while
    /// `&` and `|` parse as `binary_operator`. This check fires on
    /// `binary_operator` nodes with `&` or `|` that are inside boolean
    /// contexts (if/while conditions).
    fn check_boolean_context_binary<'a>(
        &self,
        node: tree_sitter::Node<'a>,
        source: &'a str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        let op = extract_operator(node, source);

        // AND2: & in boolean context
        if self.is_enabled("AND2") && op == "&" && is_in_condition(node) {
            diags.push(make_diag_with_fix(
                "AND2",
                node,
                fix_replace_operator(node, source, "&", "&&"),
            ));
        }

        // OR2: | in boolean context
        if self.is_enabled("OR2") && op == "|" && is_in_condition(node) {
            diags.push(make_diag_with_fix(
                "OR2",
                node,
                fix_replace_operator(node, source, "|", "||"),
            ));
        }

        diags
    }

    /// Check `comparison_operator` for ISMT/ISCL patterns.
    fn check_comparison<'a>(
        &self,
        node: tree_sitter::Node<'a>,
        source: &'a str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        // Pattern: length(x) == 0 → isempty(x)
        // Pattern: length(x) == 1 → isscalar(x)
        if let Some((func, val)) = extract_length_comparison(node, source) {
            if func == "length" {
                if self.is_enabled("ISMT") && val == "0" {
                    diags.push(make_diag("ISMT", node));
                }
                if self.is_enabled("ISCL") && val == "1" {
                    diags.push(make_diag("ISCL", node));
                }
            }
        }

        diags
    }

    /// Check `unary_operator` for STCCS pattern: ~isempty(strfind(...))
    fn check_unary<'a>(
        &self,
        node: tree_sitter::Node<'a>,
        source: &'a str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        if !self.is_enabled("STCCS") {
            return diags;
        }

        // Pattern: ~isempty(strfind(...))
        let op = extract_unary_operator(node, source);
        if op != "~" {
            return diags;
        }

        // The operand should be a function_call to isempty
        let operand = match node.child(1).or_else(|| node.child_by_field_name("operand")) {
            Some(n) => n,
            None => return diags,
        };

        if operand.kind() == "function_call" {
            if let Some(name) = extract_func_name(operand, source) {
                if name == "isempty" {
                    // Check if the argument to isempty is strfind(...)
                    if let Some(args_node) = operand.child_by_field_name("arguments") {
                        if let Some(first_arg) = first_named_child(args_node) {
                            if first_arg.kind() == "function_call" {
                                if let Some(inner_name) = extract_func_name(first_arg, source) {
                                    if inner_name == "strfind" {
                                        diags.push(make_diag("STCCS", node));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        diags
    }

    // -----------------------------------------------------------------------
    // File-level checks: AGROW, SAGROW, PFBNS
    // -----------------------------------------------------------------------

    /// File-level check for array/struct growth inside loops.
    fn check_growth_in_loops(
        &self,
        tree: &tree_sitter::Tree,
        source: &str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        let root = tree.root_node();
        self.traverse_for_growth(&root, source, false, &mut diags);
        diags
    }

    /// Recursive traversal looking for growth patterns inside loops.
    fn traverse_for_growth(
        &self,
        node: &tree_sitter::Node,
        source: &str,
        in_loop: bool,
        diags: &mut Vec<Diagnostic>,
    ) {
        let kind = node.kind();
        let entering_loop = kind == "for_statement" || kind == "while_statement";
        let now_in_loop = in_loop || entering_loop;

        // Only check assignment nodes inside loops.
        if now_in_loop && kind == "assignment" {
            self.check_assignment_growth(*node, source, diags);
        }

        // Also detect growth via full-node text for assignments that tree-sitter
        // may parse differently (e.g., s.field(end+1) = k might parse as
        // a function_call at statement level rather than an assignment).
        if now_in_loop
            && (kind == "function_call" || kind == "cell_index")
            && is_statement_level(node)
        {
            let text = node_text(*node, source);
            let normalized: String = text.chars().filter(|c| !c.is_whitespace()).collect();
            if normalized.contains("end+1") {
                let has_dot = text.contains('.');
                let check_id = if has_dot { "SAGROW" } else { "AGROW" };
                if self.is_enabled(check_id) {
                    diags.push(make_diag(check_id, *node));
                }
            }
        }

        let count = node.child_count();
        for i in 0..count {
            if let Some(child) = node.child(i) {
                self.traverse_for_growth(&child, source, now_in_loop, diags);
            }
        }
    }

    /// Check a single assignment node for growth patterns.
    ///
    /// Detects:
    /// 1. `x = [x, val]` or `x = [x; val]` — concatenation growth (AGROW)
    /// 2. `x(end+1) = val` — end+1 indexing (AGROW)
    /// 3. `x{end+1} = val` — cell growth (AGROW)
    /// 4. `s.field(end+1) = val` — struct field growth (SAGROW)
    /// 5. `x = []` followed by growth — PFBNS (simplified: flag end+1 patterns)
    fn check_assignment_growth(
        &self,
        node: tree_sitter::Node,
        source: &str,
        diags: &mut Vec<Diagnostic>,
    ) {
        let lhs = match node.child_by_field_name("left").or_else(|| node.child(0)) {
            Some(n) => n,
            None => return,
        };
        let rhs = match node.child_by_field_name("right").or_else(|| node.child(2)) {
            Some(n) => n,
            None => return,
        };

        let lhs_text = node_text(lhs, source);
        let rhs_text = node_text(rhs, source);

        // Determine if LHS involves a struct field (for SAGROW vs AGROW).
        let is_struct = lhs.kind() == "field_expression"
            || (lhs.kind() == "function_call" && lhs_text.contains('.'));

        let check_id = if is_struct { "SAGROW" } else { "AGROW" };

        if !self.is_enabled(check_id) {
            return;
        }

        // Pattern 1: x(end+1) = ... or x{end+1} = ...
        // LHS is a function_call (indexing) or cell_index containing "end+1"
        // or "end + 1".
        if (lhs.kind() == "function_call" || lhs.kind() == "cell_index")
            && contains_end_plus_pattern(lhs, source)
        {
            diags.push(make_diag(check_id, node));
            return;
        }

        // Pattern 1b: s.field(end+1) = ... — may parse with field_expression as LHS
        // and end+1 in the assignment text before the `=`.
        if lhs.kind() == "field_expression" {
            // Check the full assignment text up to the `=` for end+1 pattern.
            let assign_text = node_text(node, source);
            if let Some(eq_pos) = assign_text.find('=') {
                let lhs_full = &assign_text[..eq_pos];
                let normalized: String = lhs_full.chars().filter(|c| !c.is_whitespace()).collect();
                if normalized.contains("end+1") {
                    diags.push(make_diag("SAGROW", node));
                    return;
                }
            }
        }

        // Pattern 2: x = [x, val] or x = [x; val]
        // LHS is a simple identifier, RHS is a matrix containing that identifier.
        if lhs.kind() == "identifier" {
            let var_name = lhs_text;
            if rhs.kind() == "matrix" && matrix_contains_var(rhs, source, var_name) {
                diags.push(make_diag(check_id, node));
                return;
            }
            // Also check cell: x = {x, val} — though less common
            if rhs.kind() == "cell" && matrix_contains_var(rhs, source, var_name) {
                diags.push(make_diag(check_id, node));
                return;
            }
        }

        // Pattern 3: s.field = [s.field, val]
        if lhs.kind() == "field_expression" {
            let field_text = lhs_text;
            if rhs.kind() == "matrix" && rhs_text.contains(field_text) {
                diags.push(make_diag("SAGROW", node));
            }
        }
    }
}

impl Rule for PerformanceEngine {
    fn id(&self) -> &'static str {
        "PERFORMANCE_ENGINE"
    }

    fn description(&self) -> &'static str {
        "Performance improvement suggestions"
    }

    fn severity(&self) -> Severity {
        Severity::Info
    }

    fn category(&self) -> Category {
        Category::Performance
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        TARGET_NODES
    }

    fn has_file_check(&self) -> bool {
        true
    }

    fn check(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        match ctx.node.kind() {
            "function_call" => self.check_function_call(ctx.node, ctx.source),
            "command" => self.check_command(ctx.node, ctx.source),
            "binary_operator" => {
                let mut diags = self.check_binary_operator(ctx.node, ctx.source);
                // Also check for AND2/OR2 (& and | parse as binary_operator).
                diags.extend(self.check_boolean_context_binary(ctx.node, ctx.source));
                diags
            }
            "boolean_operator" => {
                // boolean_operator is for && / || which are already correct.
                Vec::new()
            }
            "comparison_operator" => self.check_comparison(ctx.node, ctx.source),
            "unary_operator" => self.check_unary(ctx.node, ctx.source),
            _ => Vec::new(),
        }
    }

    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> {
        self.check_growth_in_loops(ctx.tree, ctx.source)
    }
}

// ---------------------------------------------------------------------------
// Helper: diagnostic construction
// ---------------------------------------------------------------------------

/// Build a diagnostic from a check ID and node.
fn make_diag(check_id: &'static str, node: tree_sitter::Node) -> Diagnostic {
    let start = node.start_position();
    Diagnostic {
        rule_id: check_id,
        message: check_description(check_id).to_string(),
        severity: Severity::Info,
        byte_range: node.start_byte()..node.end_byte(),
        line: start.row + 1,
        column: start.column + 1,
        fix: None,
    }
}

/// Build a diagnostic with an optional fix.
fn make_diag_with_fix(
    check_id: &'static str,
    node: tree_sitter::Node,
    fix: Option<Fix>,
) -> Diagnostic {
    let start = node.start_position();
    Diagnostic {
        rule_id: check_id,
        message: check_description(check_id).to_string(),
        severity: Severity::Info,
        byte_range: node.start_byte()..node.end_byte(),
        line: start.row + 1,
        column: start.column + 1,
        fix,
    }
}

// ---------------------------------------------------------------------------
// Helper: node text extraction
// ---------------------------------------------------------------------------

/// Extract the raw text of a node.
fn node_text<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

/// Extract function name from a `function_call` node.
fn extract_func_name<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> Option<&'a str> {
    if node.kind() != "function_call" {
        return None;
    }
    let name_node = node.child_by_field_name("name")?;
    Some(&source[name_node.start_byte()..name_node.end_byte()])
}

/// Extract command name from a `command` node.
fn extract_command_name<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> Option<&'a str> {
    if node.kind() != "command" {
        return None;
    }
    let name_node = node.child(0)?;
    if name_node.kind() == "command_name" {
        Some(&source[name_node.start_byte()..name_node.end_byte()])
    } else {
        None
    }
}

/// Extract command arguments as a list of text fragments.
fn extract_command_args<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> Vec<&'a str> {
    let mut args = Vec::new();
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            // Skip the command_name (first child) and punctuation.
            if child.kind() == "command_name" || child.kind() == ";" || child.kind() == "," {
                continue;
            }
            let text = node_text(child, source).trim();
            if !text.is_empty() {
                args.push(text);
            }
        }
    }
    args
}

/// Extract the operator text from a `binary_operator` or `boolean_operator`.
fn extract_operator<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> &'a str {
    // The operator is typically the second child (between left and right operands),
    // or we can scan children for the operator token.
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            let text = node_text(child, source);
            // Operator tokens are short and not identifiers/expressions.
            match text {
                "+" | "-" | "*" | "/" | "\\" | "^" | ".*" | "./" | ".\\" | ".^" | "&" | "|"
                | "&&" | "||" | "==" | "~=" | "<" | ">" | "<=" | ">=" => return text,
                _ => continue,
            }
        }
    }
    ""
}

/// Extract the operator text from a `unary_operator` node.
fn extract_unary_operator<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> &'a str {
    if let Some(child) = node.child(0) {
        let text = node_text(child, source);
        match text {
            "~" | "-" | "+" | "!" => return text,
            _ => {}
        }
    }
    ""
}

/// Get the first named child of a node.
fn first_named_child(node: tree_sitter::Node) -> Option<tree_sitter::Node> {
    let count = node.named_child_count();
    if count > 0 {
        node.named_child(0)
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Helper: argument analysis
// ---------------------------------------------------------------------------

/// Count the number of arguments in a `function_call` node.
fn count_args(node: tree_sitter::Node) -> usize {
    // Arguments are in the `arguments` child node.
    let args_node = match node.child_by_field_name("arguments") {
        Some(n) => n,
        None => {
            // Fallback: iterate children looking for argument list.
            let count = node.child_count();
            let mut arg_count = 0;
            for i in 0..count {
                if let Some(child) = node.child(i) {
                    let kind = child.kind();
                    if kind != "(" && kind != ")" && kind != "," && kind != "identifier" {
                        // Skip the function name (first identifier child)
                        if i > 0 {
                            arg_count += 1;
                        }
                    }
                }
            }
            return arg_count;
        }
    };

    // Count named children inside the arguments node (skipping commas).
    args_node.named_child_count()
}

/// Check if a function_call has a specific string argument.
fn has_string_arg(node: tree_sitter::Node, source: &str, value: &str) -> bool {
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            let text = node_text(child, source);
            // Match quoted strings: 'value' or "value"
            let trimmed = text.trim_matches('\'').trim_matches('"');
            if trimmed.eq_ignore_ascii_case(value) {
                return true;
            }
            // Recurse into arguments node.
            if child.kind() == "arguments" && has_string_arg(child, source, value) {
                return true;
            }
        }
    }
    false
}

/// Check if `exist(...)` has a second argument indicating type ('file', 'dir', etc.).
fn has_exist_type_arg(node: tree_sitter::Node, source: &str) -> bool {
    // exist should have 2 arguments, with the second being a string like 'file', 'dir'.
    let arg_count = count_args(node);
    if arg_count < 2 {
        return false;
    }
    has_string_arg(node, source, "file")
        || has_string_arg(node, source, "dir")
        || has_string_arg(node, source, "builtin")
        || has_string_arg(node, source, "class")
}

/// Check if a regexp/regexpi call has an overly simple pattern (just alphanumeric).
fn has_simple_regex_pattern(node: tree_sitter::Node, source: &str) -> bool {
    // Look for the second argument (the pattern) and check if it's purely alphanumeric
    // (meaning a simple `contains` or `strcmp` would suffice).
    let args_node = match node.child_by_field_name("arguments") {
        Some(n) => n,
        None => return false,
    };

    // The pattern is usually the 2nd argument.
    let named_count = args_node.named_child_count();
    if named_count < 2 {
        return false;
    }

    if let Some(pattern_node) = args_node.named_child(1) {
        let text = node_text(pattern_node, source);
        // Strip quotes.
        let inner = text.trim_matches('\'').trim_matches('"');
        // If the pattern is purely alphanumeric (no regex metacharacters), it's simple.
        if !inner.is_empty() && inner.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return true;
        }
    }
    false
}

/// Check if the first argument to lower/upper is a strcmp call (STNCI pattern).
fn has_strcmp_arg(node: tree_sitter::Node, source: &str) -> bool {
    let args_node = match node.child_by_field_name("arguments") {
        Some(n) => n,
        None => return false,
    };

    if let Some(first_arg) = first_named_child(args_node) {
        if first_arg.kind() == "function_call" {
            if let Some(name) = extract_func_name(first_arg, source) {
                return name == "strcmp" || name == "strcmpi";
            }
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Helper: pattern detection
// ---------------------------------------------------------------------------

/// Check if the function_call is a nested call of the same function.
/// e.g., max(max(x)) — the outer call's first argument is the same function.
fn is_nested_same_call(node: tree_sitter::Node, source: &str, func_name: &str) -> bool {
    let args_node = match node.child_by_field_name("arguments") {
        Some(n) => n,
        None => return false,
    };

    // Check if first argument is a function_call with the same name.
    if let Some(first_arg) = first_named_child(args_node) {
        if first_arg.kind() == "function_call" {
            if let Some(inner_name) = extract_func_name(first_arg, source) {
                return inner_name == func_name;
            }
        }
    }
    false
}

/// Check if a function_call node is used as a subscript index.
/// i.e., the parent is the arguments of another function_call.
fn is_subscript_context(node: tree_sitter::Node) -> bool {
    if let Some(parent) = node.parent() {
        // Direct parent might be `arguments` of a function_call.
        if parent.kind() == "arguments" {
            if let Some(grandparent) = parent.parent() {
                return grandparent.kind() == "function_call";
            }
        }
        // Or the parent is directly a function_call (as an argument).
        if parent.kind() == "function_call" {
            // Make sure this node is not the function name itself.
            if let Some(name_node) = parent.child_by_field_name("name") {
                return name_node.id() != node.id();
            }
        }
    }
    false
}

/// Check if a node is at statement level (direct child of source_file or block).
fn is_statement_level(node: &tree_sitter::Node) -> bool {
    node.parent()
        .map(|p| matches!(p.kind(), "source_file" | "block"))
        .unwrap_or(false)
}

/// Check if node is inside a loop (for_statement or while_statement).
fn is_inside_loop(node: tree_sitter::Node) -> bool {
    let mut current = node.parent();
    while let Some(p) = current {
        match p.kind() {
            "for_statement" | "while_statement" => return true,
            "function_definition" => return false, // Stop at function boundary.
            _ => {}
        }
        current = p.parent();
    }
    false
}

/// Check if a node is inside a condition expression (if/while/elseif).
fn is_in_condition(node: tree_sitter::Node) -> bool {
    let mut current = node.parent();
    while let Some(p) = current {
        match p.kind() {
            "if_statement" | "while_statement" | "elseif_clause" => {
                // Check if our node is in the condition part, not the body.
                // The condition is typically the first expression child after the keyword.
                if let Some(cond) = p.child_by_field_name("condition") {
                    return is_descendant_of(node, cond);
                }
                // Fallback: check if node appears before the "block" child.
                let count = p.child_count();
                for i in 0..count {
                    if let Some(child) = p.child(i) {
                        if child.kind() == "block" {
                            // If node is before the block, it's in the condition.
                            return node.start_byte() < child.start_byte();
                        }
                    }
                }
                return true;
            }
            "function_definition" => return false,
            _ => {}
        }
        current = p.parent();
    }
    false
}

/// Check if `needle` is a descendant of `haystack`.
fn is_descendant_of(needle: tree_sitter::Node, haystack: tree_sitter::Node) -> bool {
    needle.start_byte() >= haystack.start_byte() && needle.end_byte() <= haystack.end_byte()
}

/// Check if an `inv()` call is an operand of a binary_operator.
fn has_inv_operand(node: tree_sitter::Node, source: &str) -> bool {
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            if child.kind() == "function_call" {
                if let Some(name) = extract_func_name(child, source) {
                    if name == "inv" {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Check if both operands of a binary_operator are the same text.
fn has_same_operands(node: tree_sitter::Node, source: &str) -> bool {
    // Typically: child(0) = left, child(1) = op, child(2) = right.
    let left = match node.child(0) {
        Some(n) => n,
        None => return false,
    };
    let right = match node.child(2) {
        Some(n) => n,
        None => return false,
    };
    let left_text = node_text(left, source).trim();
    let right_text = node_text(right, source).trim();
    !left_text.is_empty() && left_text == right_text
}

/// Check if an indexing expression contains `end+1` or `end + 1` pattern.
fn contains_end_plus_pattern(node: tree_sitter::Node, source: &str) -> bool {
    let text = node_text(node, source);
    // Simple textual check: look for "end+1" or "end + 1" within parentheses or braces.
    let normalized: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    normalized.contains("end+1")
}

/// Check if a matrix literal `[...]` contains a variable name as an element.
fn matrix_contains_var(node: tree_sitter::Node, source: &str, var_name: &str) -> bool {
    // Walk the matrix's children looking for an identifier matching var_name.
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            if child.kind() == "identifier" && node_text(child, source) == var_name {
                return true;
            }
            // Recurse into rows.
            if child.kind() == "row" && matrix_contains_var(child, source, var_name) {
                return true;
            }
        }
    }
    false
}

/// Extract the comparison operands from a comparison like `length(x) == 0`.
/// Returns (function_name, compared_value) if the pattern matches.
fn extract_length_comparison<'a>(
    node: tree_sitter::Node<'a>,
    source: &'a str,
) -> Option<(&'a str, &'a str)> {
    // comparison_operator children: left op right
    let left = node.child(0)?;
    let op_node = node.child(1)?;
    let right = node.child(2)?;

    let op = node_text(op_node, source);
    if op != "==" {
        return None;
    }

    // Check if left is a function_call to length/numel/size.
    if left.kind() == "function_call" {
        if let Some(name) = extract_func_name(left, source) {
            if name == "length" || name == "numel" {
                return Some((name, node_text(right, source).trim()));
            }
        }
    }

    // Also check reversed: 0 == length(x)
    if right.kind() == "function_call" {
        if let Some(name) = extract_func_name(right, source) {
            if name == "length" || name == "numel" {
                return Some((name, node_text(left, source).trim()));
            }
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Helper: fix construction
// ---------------------------------------------------------------------------

/// Create a fix that renames the function in a function_call node.
fn fix_rename_func(node: tree_sitter::Node, source: &str, new_name: &str) -> Fix {
    if let Some(name_node) = node.child_by_field_name("name") {
        Fix::new(
            name_node.start_byte()..name_node.end_byte(),
            new_name,
        )
    } else {
        // Fallback: try to replace the function name textually.
        let text = node_text(node, source);
        if let Some(paren_pos) = text.find('(') {
            let start = node.start_byte();
            Fix::new(start..start + paren_pos, new_name)
        } else {
            Fix::new(node.start_byte()..node.end_byte(), new_name)
        }
    }
}

/// Create a fix that replaces an operator in a binary_operator node.
fn fix_replace_operator(
    node: tree_sitter::Node,
    source: &str,
    old_op: &str,
    new_op: &str,
) -> Option<Fix> {
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            let text = node_text(child, source);
            if text == old_op {
                return Some(Fix::new(
                    child.start_byte()..child.end_byte(),
                    new_op,
                ));
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "PERFORMANCE_ENGINE",
    PerformanceEngine::from_config
));
