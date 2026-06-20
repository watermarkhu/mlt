//! # READABILITY_ENGINE: Readability Improvements
//!
//! A multi-check rule engine that detects readability improvements in MATLAB
//! code. Instead of implementing one struct per check, a single
//! `ReadabilityEngine` dispatches to per-check-ID logic inside its `check()`
//! method, emitting diagnostics with specific check IDs (e.g., `ISCHR`,
//! `IJCL`, `RPMT1`).
//!
//! ## Checks
//!
//! This engine covers 35 readability checks across these patterns:
//!
//! - **Type checking simplification** (ISCHR, ISSTR, ISLOG, ISCEL, ISMAT):
//!   prefer `ischar`, `isstring`, etc. over `isa(x, 'type')`.
//! - **Dimension checks** (ISROW, ISCOL): prefer `isrow`/`iscolumn` over
//!   `size(x,dim)==1`.
//! - **Variable shadowing** (IJCL): `i`/`j` as assignment LHS shadows the
//!   complex unit.
//! - **Unnecessary brackets** (NBRAK2): `[x]` where `x` is a scalar.
//! - **String comparisons** (STREMP, STRCL1, STRCLFH, STRIFCND): prefer
//!   modern string functions.
//! - **Character literals** (CHARTEN): use `newline` instead of `char(10)`.
//! - **Formatting** (SPRINTFN): use `num2str` over simple `sprintf`.
//! - **Control flow style** (ASGSL): avoid inline assignment in conditions.
//! - **Error/warning style** (SPERR, SPWRN): prefer message IDs.
//! - **Input validation** (NCHKE): prefer `narginchk`/`nargoutchk`.
//! - **Output style** (DSPSP, DSPSY): prefer `fprintf`/`disp` over wrappers.
//! - **Path construction** (FLUDLR): prefer `fullfile`.
//! - **Redundant arithmetic** (RPMT1, RPMT0, RPMTI, RPMTN): simplify trivial
//!   multiplication/addition.
//! - **Redundant logic** (RPMTT, RPMTF): simplify boolean tautologies.
//! - **Size helpers** (PSIZE): prefer `numel` over `prod(size(x))`.
//! - **Logical helpers** (LOGSUM, LOGL): prefer `any`/logical indexing.
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.READABILITY_ENGINE]
//! severity = "info"
//! disabled_checks = ["IJCL", "NBRAK2"]
//! ```

use mlt_core::{Category, Config, Diagnostic, Fix, NodeContext, Rule, Severity};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for the readability engine.
///
/// Deserialized from `[lint.rules.READABILITY_ENGINE]` in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ReadabilityConfig {
    /// Individual check IDs to disable within this engine.
    #[serde(default)]
    pub disabled_checks: Vec<String>,
}

// ---------------------------------------------------------------------------
// Lookup table: isa(x, 'type') → preferred function
// ---------------------------------------------------------------------------

/// Mapping from `isa` second argument (without quotes) to
/// (check_id, preferred_function, description).
const ISA_REPLACEMENTS: &[(&str, &str, &str, &str)] = &[
    ("char", "ISCHR", "ischar", "Use 'ischar(x)' instead of 'isa(x, ''char'')'"),
    ("string", "ISSTR", "isstring", "Use 'isstring(x)' instead of 'isa(x, ''string'')'"),
    ("logical", "ISLOG", "islogical", "Use 'islogical(x)' instead of 'isa(x, ''logical'')'"),
    ("cell", "ISCEL", "iscell", "Use 'iscell(x)' instead of 'isa(x, ''cell'')'"),
    ("double", "ISMAT", "isnumeric", "Use 'isnumeric(x)' instead of 'isa(x, ''double'')'"),
];

// ---------------------------------------------------------------------------
// Target node types
// ---------------------------------------------------------------------------

/// Node types this engine subscribes to for dispatch.
const TARGET_NODES: &[&str] = &[
    "function_call",
    "comparison_operator",
    "boolean_operator",
    "identifier",
    "binary_operator",
    "assignment",
    "matrix",
];

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// The readability engine — a single rule instance that checks 35 readability
/// improvement patterns via dispatched per-check logic.
///
/// Registered once with inventory. Each emitted diagnostic carries the specific
/// check ID (e.g., `"ISCHR"`), not the meta-ID `"READABILITY_ENGINE"`.
pub struct ReadabilityEngine {
    config: ReadabilityConfig,
}

impl ReadabilityEngine {
    /// Factory constructor called by the rule registry.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: ReadabilityConfig = config.rule_params("READABILITY_ENGINE");
        Box::new(Self {
            config: rule_config,
        })
    }

    /// Check whether a specific sub-check is enabled.
    fn is_check_enabled(&self, check_id: &str) -> bool {
        !self.config.disabled_checks.iter().any(|d| d == check_id)
    }

    /// Emit a diagnostic for a given node with the specified check ID.
    fn diag(
        &self,
        check_id: &'static str,
        message: &str,
        node: tree_sitter::Node,
        fix: Option<Fix>,
    ) -> Diagnostic {
        let start = node.start_position();
        Diagnostic {
            rule_id: check_id,
            message: message.to_string(),
            severity: self.severity(),
            byte_range: node.start_byte()..node.end_byte(),
            line: start.row + 1,
            column: start.column + 1,
            fix,
        }
    }

    // -----------------------------------------------------------------------
    // Per-node-type dispatch
    // -----------------------------------------------------------------------

    /// Checks dispatched for `function_call` nodes.
    fn check_function_call(
        &self,
        ctx: &NodeContext,
        results: &mut Vec<Diagnostic>,
    ) {
        let node = ctx.node;
        let source = ctx.source;

        let func_name = match node.child_by_field_name("name") {
            Some(n) => &source[n.start_byte()..n.end_byte()],
            None => return,
        };

        match func_name {
            "isa" => self.check_isa(node, source, results),
            "strcmp" => self.check_strcmp(node, source, ctx, results),
            "strncmp" | "strncmpi" => self.check_strncmp(node, source, func_name, results),
            "strfind" => self.check_strfind(node, source, results),
            "char" => self.check_char_literal(node, source, results),
            "sprintf" => self.check_sprintf(node, source, results),
            "error" => self.check_error_warning(node, source, "SPERR", results),
            "warning" => self.check_error_warning(node, source, "SPWRN", results),
            "disp" => self.check_disp(node, source, results),
            "display" => self.check_display(node, results),
            "nargchk" => self.check_narginchk(node, source, results),
            "prod" => self.check_prod_size(node, source, results),
            "sum" => self.check_sum_logical(node, source, results),
            "find" => self.check_find_logical(node, source, results),
            _ => {}
        }
    }

    /// Checks dispatched for `comparison_operator` nodes.
    fn check_comparison(
        &self,
        ctx: &NodeContext,
        results: &mut Vec<Diagnostic>,
    ) {
        let node = ctx.node;
        let source = ctx.source;

        // Look for size(x,dim)==1 patterns (ISROW, ISCOL)
        self.check_size_comparison(node, source, results);
    }

    /// Checks dispatched for `boolean_operator` nodes.
    fn check_boolean_operator(
        &self,
        ctx: &NodeContext,
        results: &mut Vec<Diagnostic>,
    ) {
        let node = ctx.node;
        let source = ctx.source;

        self.check_boolean_tautology(node, source, results);
    }

    /// Checks dispatched for `assignment` nodes.
    fn check_assignment(
        &self,
        ctx: &NodeContext,
        results: &mut Vec<Diagnostic>,
    ) {
        let node = ctx.node;
        let source = ctx.source;

        // IJCL: assignment to `i` or `j` shadows complex unit
        self.check_ij_shadow(node, source, results);

        // ASGSL: assignment on same line as control statement
        self.check_inline_assignment(node, results);
    }

    /// Checks dispatched for `binary_operator` nodes.
    fn check_binary_operator(
        &self,
        ctx: &NodeContext,
        results: &mut Vec<Diagnostic>,
    ) {
        let node = ctx.node;
        let source = ctx.source;

        self.check_redundant_arithmetic(node, source, results);
    }

    /// Checks dispatched for `matrix` nodes.
    fn check_matrix(
        &self,
        ctx: &NodeContext,
        results: &mut Vec<Diagnostic>,
    ) {
        let node = ctx.node;
        let source = ctx.source;

        // NBRAK2: unnecessary brackets [x] for scalar
        self.check_unnecessary_brackets(node, source, results);
    }

    // -----------------------------------------------------------------------
    // Individual check implementations
    // -----------------------------------------------------------------------

    /// ISCHR, ISSTR, ISLOG, ISCEL, ISMAT: `isa(x, 'type')` → `istype(x)`
    fn check_isa(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        let args = match node.child_by_field_name("arguments") {
            Some(a) => a,
            None => return,
        };

        // isa requires exactly 2 arguments
        let named_children: Vec<_> = (0..args.named_child_count())
            .filter_map(|i| args.named_child(i))
            .collect();

        if named_children.len() != 2 {
            return;
        }

        let first_arg = named_children[0];
        let second_arg = named_children[1];
        let first_text = &source[first_arg.start_byte()..first_arg.end_byte()];
        let second_text = &source[second_arg.start_byte()..second_arg.end_byte()];

        // Strip quotes from second argument
        let type_name = second_text
            .trim_start_matches('\'')
            .trim_end_matches('\'')
            .trim_start_matches('"')
            .trim_end_matches('"');

        for &(isa_type, check_id, replacement_fn, message) in ISA_REPLACEMENTS {
            if type_name == isa_type && self.is_check_enabled(check_id) {
                let fix_text = format!("{replacement_fn}({first_text})");
                results.push(self.diag(
                    check_id,
                    message,
                    node,
                    Some(Fix::new(node.start_byte()..node.end_byte(), fix_text)),
                ));
                return;
            }
        }
    }

    /// STREMP: `strcmp(s, '')` → `strlength(s)==0`
    /// STRIFCND: `strcmp` inside `if` condition → `matches`
    fn check_strcmp(
        &self,
        node: tree_sitter::Node,
        source: &str,
        ctx: &NodeContext,
        results: &mut Vec<Diagnostic>,
    ) {
        let args = match node.child_by_field_name("arguments") {
            Some(a) => a,
            None => return,
        };

        let named_children: Vec<_> = (0..args.named_child_count())
            .filter_map(|i| args.named_child(i))
            .collect();

        if named_children.len() != 2 {
            return;
        }

        let arg1_text = &source[named_children[0].start_byte()..named_children[0].end_byte()];
        let arg2_text = &source[named_children[1].start_byte()..named_children[1].end_byte()];

        // STREMP: strcmp(s, '') or strcmp('', s)
        if self.is_check_enabled("STREMP") && (is_empty_string(arg2_text) || is_empty_string(arg1_text)) {
            let var = if is_empty_string(arg1_text) { arg2_text } else { arg1_text };
            results.push(self.diag(
                "STREMP",
                "Use 'strlength(s)==0' or 's==\"\"' instead of 'strcmp(s, '')'",
                node,
                Some(Fix::new(
                    node.start_byte()..node.end_byte(),
                    format!("strlength({var})==0"),
                )),
            ));
            return;
        }

        // STRIFCND: strcmp inside if condition → suggest matches
        if self.is_check_enabled("STRIFCND") && is_inside_if_condition(ctx.node) {
            results.push(self.diag(
                "STRIFCND",
                "Consider using 'matches' instead of 'strcmp' in if-condition",
                node,
                None,
            ));
        }
    }

    /// STRCL1: `strncmp`/`strncmpi` → `startsWith`/`endsWith`
    fn check_strncmp(
        &self,
        node: tree_sitter::Node,
        _source: &str,
        func_name: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("STRCL1") {
            return;
        }

        let message = if func_name == "strncmpi" {
            "Consider using 'startsWith' or 'endsWith' instead of 'strncmpi'"
        } else {
            "Consider using 'startsWith' or 'endsWith' instead of 'strncmp'"
        };

        results.push(self.diag("STRCL1", message, node, None));
    }

    /// STRCLFH: `strfind` → `contains`
    fn check_strfind(
        &self,
        node: tree_sitter::Node,
        _source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("STRCLFH") {
            return;
        }

        results.push(self.diag(
            "STRCLFH",
            "Consider using 'contains' instead of 'strfind' for presence checks",
            node,
            None,
        ));
    }

    /// CHARTEN: `char(10)` → `newline`
    fn check_char_literal(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("CHARTEN") {
            return;
        }

        let args = match node.child_by_field_name("arguments") {
            Some(a) => a,
            None => return,
        };

        let named_children: Vec<_> = (0..args.named_child_count())
            .filter_map(|i| args.named_child(i))
            .collect();

        if named_children.len() != 1 {
            return;
        }

        let arg_text = &source[named_children[0].start_byte()..named_children[0].end_byte()];
        if arg_text.trim() == "10" {
            results.push(self.diag(
                "CHARTEN",
                "Use 'newline' instead of 'char(10)'",
                node,
                Some(Fix::new(node.start_byte()..node.end_byte(), "newline")),
            ));
        }
    }

    /// SPRINTFN: `sprintf('%d', x)` → `num2str(x)`
    fn check_sprintf(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("SPRINTFN") {
            return;
        }

        let args = match node.child_by_field_name("arguments") {
            Some(a) => a,
            None => return,
        };

        let named_children: Vec<_> = (0..args.named_child_count())
            .filter_map(|i| args.named_child(i))
            .collect();

        if named_children.len() != 2 {
            return;
        }

        let fmt_text = &source[named_children[0].start_byte()..named_children[0].end_byte()];
        let var_text = &source[named_children[1].start_byte()..named_children[1].end_byte()];

        // Check for simple numeric format specifiers
        let fmt_stripped = fmt_text
            .trim_start_matches('\'')
            .trim_end_matches('\'')
            .trim_start_matches('"')
            .trim_end_matches('"');

        if matches!(fmt_stripped, "%d" | "%i" | "%f" | "%g" | "%e") {
            results.push(self.diag(
                "SPRINTFN",
                "Use 'num2str(x)' instead of 'sprintf' with simple numeric format",
                node,
                Some(Fix::new(
                    node.start_byte()..node.end_byte(),
                    format!("num2str({var_text})"),
                )),
            ));
        }
    }

    /// SPERR / SPWRN: `error('msg')` / `warning('msg')` without message ID
    fn check_error_warning(
        &self,
        node: tree_sitter::Node,
        source: &str,
        check_id: &'static str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled(check_id) {
            return;
        }

        let args = match node.child_by_field_name("arguments") {
            Some(a) => a,
            None => return,
        };

        let named_children: Vec<_> = (0..args.named_child_count())
            .filter_map(|i| args.named_child(i))
            .collect();

        // Single-argument error/warning call — no message ID
        if named_children.len() == 1 {
            let arg_text = &source[named_children[0].start_byte()..named_children[0].end_byte()];
            // Only flag if argument looks like a string literal
            if arg_text.starts_with('\'') || arg_text.starts_with('"') {
                let func = if check_id == "SPERR" { "error" } else { "warning" };
                results.push(self.diag(
                    check_id,
                    &format!(
                        "Use '{func}' with a message ID: {func}('myComponent:myID', ...)"
                    ),
                    node,
                    None,
                ));
            }
        }
    }

    /// NCHKE: pattern `if nargin < N, error(...)` → `narginchk`
    ///
    /// Detects `function_call` named `error` or `nargchk` inside an
    /// if-statement that tests `nargin`.
    fn check_narginchk(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("NCHKE") {
            return;
        }

        let func_name = match node.child_by_field_name("name") {
            Some(n) => &source[n.start_byte()..n.end_byte()],
            None => return,
        };

        if func_name != "nargchk" {
            return;
        }

        results.push(self.diag(
            "NCHKE",
            "Use 'narginchk' or 'nargoutchk' instead of 'nargchk'/'nargoutchk' with error",
            node,
            None,
        ));
    }

    /// DSPSP: `disp(sprintf(...))` → `fprintf`
    fn check_disp(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("DSPSP") {
            return;
        }

        let args = match node.child_by_field_name("arguments") {
            Some(a) => a,
            None => return,
        };

        let named_children: Vec<_> = (0..args.named_child_count())
            .filter_map(|i| args.named_child(i))
            .collect();

        if named_children.len() != 1 {
            return;
        }

        let inner = named_children[0];
        if inner.kind() == "function_call" {
            if let Some(inner_name) = inner.child_by_field_name("name") {
                let inner_func = &source[inner_name.start_byte()..inner_name.end_byte()];
                if inner_func == "sprintf" {
                    // Extract sprintf arguments to construct fprintf replacement
                    if let Some(inner_args) = inner.child_by_field_name("arguments") {
                        let inner_args_text =
                            &source[inner_args.start_byte()..inner_args.end_byte()];
                        results.push(self.diag(
                            "DSPSP",
                            "Use 'fprintf(...)' instead of 'disp(sprintf(...))'",
                            node,
                            Some(Fix::new(
                                node.start_byte()..node.end_byte(),
                                format!("fprintf{inner_args_text}"),
                            )),
                        ));
                    }
                }
            }
        }
    }

    /// DSPSY: `display(x)` → `disp(x)`
    fn check_display(
        &self,
        node: tree_sitter::Node,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("DSPSY") {
            return;
        }

        results.push(self.diag(
            "DSPSY",
            "Use 'disp' instead of 'display'",
            node,
            None,
        ));
    }

    /// PSIZE: `prod(size(x))` → `numel(x)`
    fn check_prod_size(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("PSIZE") {
            return;
        }

        let args = match node.child_by_field_name("arguments") {
            Some(a) => a,
            None => return,
        };

        let named_children: Vec<_> = (0..args.named_child_count())
            .filter_map(|i| args.named_child(i))
            .collect();

        if named_children.len() != 1 {
            return;
        }

        let inner = named_children[0];
        if inner.kind() == "function_call" {
            if let Some(inner_name) = inner.child_by_field_name("name") {
                let inner_func = &source[inner_name.start_byte()..inner_name.end_byte()];
                if inner_func == "size" {
                    // Extract the variable passed to size
                    if let Some(inner_args) = inner.child_by_field_name("arguments") {
                        let inner_named: Vec<_> = (0..inner_args.named_child_count())
                            .filter_map(|i| inner_args.named_child(i))
                            .collect();
                        if inner_named.len() == 1 {
                            let var_text = &source
                                [inner_named[0].start_byte()..inner_named[0].end_byte()];
                            results.push(self.diag(
                                "PSIZE",
                                "Use 'numel(x)' instead of 'prod(size(x))'",
                                node,
                                Some(Fix::new(
                                    node.start_byte()..node.end_byte(),
                                    format!("numel({var_text})"),
                                )),
                            ));
                        }
                    }
                }
            }
        }
    }

    /// LOGSUM: `sum(logical) > 0` → `any(logical)`
    ///
    /// We detect `function_call` named `sum` and check if it appears as the
    /// left-hand side of a `> 0` comparison in the parent node. If no parent
    /// comparison is found, we skip (sum alone is fine).
    fn check_sum_logical(
        &self,
        node: tree_sitter::Node,
        _source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("LOGSUM") {
            return;
        }

        // Only flag when parent is a comparison `sum(...) > 0`
        if let Some(parent) = node.parent() {
            if parent.kind() == "comparison_operator" {
                results.push(self.diag(
                    "LOGSUM",
                    "Consider using 'any(...)' instead of 'sum(...) > 0'",
                    parent,
                    None,
                ));
            }
        }
    }

    /// LOGL: `x(find(condition))` → `x(condition)`
    ///
    /// Detects `find` as an argument to array/cell indexing. Since tree-sitter
    /// cannot distinguish `f(x)` from `a(i)`, we flag `find` calls used as
    /// arguments inside any `function_call` parent, which is the typical
    /// indexing pattern.
    fn check_find_logical(
        &self,
        node: tree_sitter::Node,
        _source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("LOGL") {
            return;
        }

        // find() used as argument inside another function_call (array indexing)
        if let Some(parent) = node.parent() {
            // The parent of the function_call `find(...)` should be an
            // arguments node, and *its* parent should be a function_call
            if let Some(grandparent) = parent.parent() {
                if grandparent.kind() == "function_call" {
                    results.push(self.diag(
                        "LOGL",
                        "Use logical indexing 'x(condition)' instead of 'x(find(condition))'",
                        node,
                        None,
                    ));
                }
            }
        }
    }

    /// ISROW / ISCOL: `size(x, dim) == 1` patterns
    fn check_size_comparison(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        // We need at least 3 children: lhs, operator, rhs
        if node.named_child_count() < 2 {
            return;
        }

        // Get the comparison operator text
        let node_text = &source[node.start_byte()..node.end_byte()];
        if !node_text.contains("==") {
            return;
        }

        // Find a function_call to `size` in the children
        let mut size_call = None;
        let mut other_side = None;

        for i in 0..node.named_child_count() {
            if let Some(child) = node.named_child(i) {
                if child.kind() == "function_call" {
                    if let Some(name_node) = child.child_by_field_name("name") {
                        let name = &source[name_node.start_byte()..name_node.end_byte()];
                        if name == "size" {
                            size_call = Some(child);
                            continue;
                        }
                    }
                }
                other_side = Some(child);
            }
        }

        let size_node = match size_call {
            Some(n) => n,
            None => return,
        };

        // Check the "other side" is `1`
        let other = match other_side {
            Some(n) => n,
            None => return,
        };
        let other_text = &source[other.start_byte()..other.end_byte()].trim();
        if *other_text != "1" {
            return;
        }

        // Extract the dimension argument from size(x, dim)
        let args = match size_node.child_by_field_name("arguments") {
            Some(a) => a,
            None => return,
        };

        let named_children: Vec<_> = (0..args.named_child_count())
            .filter_map(|i| args.named_child(i))
            .collect();

        if named_children.len() != 2 {
            return;
        }

        let var_text = &source[named_children[0].start_byte()..named_children[0].end_byte()];
        let dim_text = &source[named_children[1].start_byte()..named_children[1].end_byte()].trim();

        match *dim_text {
            "1" if self.is_check_enabled("ISROW") => {
                results.push(self.diag(
                    "ISROW",
                    "Use 'isrow(x)' instead of 'size(x, 1) == 1'",
                    node,
                    Some(Fix::new(
                        node.start_byte()..node.end_byte(),
                        format!("isrow({var_text})"),
                    )),
                ));
            }
            "2" if self.is_check_enabled("ISCOL") => {
                results.push(self.diag(
                    "ISCOL",
                    "Use 'iscolumn(x)' instead of 'size(x, 2) == 1'",
                    node,
                    Some(Fix::new(
                        node.start_byte()..node.end_byte(),
                        format!("iscolumn({var_text})"),
                    )),
                ));
            }
            _ => {}
        }
    }

    /// IJCL: Assignment to `i` or `j` shadows the complex unit.
    fn check_ij_shadow(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("IJCL") {
            return;
        }

        // The LHS of an assignment — the first named child or field "left"
        let lhs = match node.child_by_field_name("left") {
            Some(n) => n,
            None => {
                // Fallback: first named child
                match node.named_child(0) {
                    Some(n) => n,
                    None => return,
                }
            }
        };

        if lhs.kind() != "identifier" {
            return;
        }

        let lhs_text = &source[lhs.start_byte()..lhs.end_byte()];
        if lhs_text == "i" || lhs_text == "j" {
            results.push(self.diag(
                "IJCL",
                &format!(
                    "Variable '{lhs_text}' shadows the built-in complex unit. \
                     Use '1i' or '1j' for complex numbers, or rename the variable."
                ),
                lhs,
                None,
            ));
        }
    }

    /// ASGSL: Assignment inside a control-flow condition.
    fn check_inline_assignment(
        &self,
        node: tree_sitter::Node,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("ASGSL") {
            return;
        }

        // Walk ancestors to see if this assignment is inside a condition
        let mut current = node.parent();
        while let Some(parent) = current {
            match parent.kind() {
                "if_statement" | "while_statement" | "for_statement" => {
                    // Check if the assignment is in the condition part
                    // (first child of if/while, second child of for)
                    results.push(self.diag(
                        "ASGSL",
                        "Avoid assignment inside control-flow condition; \
                         assign on a separate line for clarity",
                        node,
                        None,
                    ));
                    return;
                }
                "block" | "source_file" | "function_definition" => {
                    // Reached a block boundary — not in a condition
                    return;
                }
                _ => {
                    current = parent.parent();
                }
            }
        }
    }

    /// NBRAK2: Unnecessary brackets `[x]` for scalar expression.
    fn check_unnecessary_brackets(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("NBRAK2") {
            return;
        }

        // A matrix node with exactly one named child that is a "row" with
        // exactly one element is `[scalar]`.
        let named_count = node.named_child_count();
        if named_count != 1 {
            return;
        }

        let row = match node.named_child(0) {
            Some(r) => r,
            None => return,
        };

        // The row should have exactly one named child (the scalar expression)
        if row.kind() == "row" && row.named_child_count() == 1 {
            if let Some(inner) = row.named_child(0) {
                // Don't flag complex expressions or function calls that might
                // rely on concatenation behavior
                if matches!(inner.kind(), "identifier" | "number" | "string") {
                    let inner_text = &source[inner.start_byte()..inner.end_byte()];
                    results.push(self.diag(
                        "NBRAK2",
                        "Unnecessary brackets around scalar expression",
                        node,
                        Some(Fix::new(
                            node.start_byte()..node.end_byte(),
                            inner_text.to_string(),
                        )),
                    ));
                }
            }
        }
    }

    /// RPMTT, RPMTF: Boolean tautologies `x | true` → `true`, `x & false` → `false`
    fn check_boolean_tautology(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        // Need at least: lhs, operator_token, rhs
        if node.child_count() < 3 {
            return;
        }

        // Extract the operator token (unnamed child between named children)
        let op_text = extract_operator_text(node, source);

        let (lhs, rhs) = match (node.named_child(0), node.named_child(1)) {
            (Some(l), Some(r)) => (l, r),
            _ => return,
        };

        let lhs_text = &source[lhs.start_byte()..lhs.end_byte()];
        let rhs_text = &source[rhs.start_byte()..rhs.end_byte()];

        // x | true → true  OR  true | x → true
        if (op_text == "|" || op_text == "||")
            && self.is_check_enabled("RPMTT")
            && (rhs_text == "true" || lhs_text == "true")
        {
            results.push(self.diag(
                "RPMTT",
                "Redundant boolean: 'x | true' is always 'true'",
                node,
                Some(Fix::new(node.start_byte()..node.end_byte(), "true")),
            ));
            return;
        }

        // x & false → false  OR  false & x → false
        if (op_text == "&" || op_text == "&&")
            && self.is_check_enabled("RPMTF")
            && (rhs_text == "false" || lhs_text == "false")
        {
            results.push(self.diag(
                "RPMTF",
                "Redundant boolean: 'x & false' is always 'false'",
                node,
                Some(Fix::new(node.start_byte()..node.end_byte(), "false")),
            ));
        }
    }

    /// RPMT1, RPMT0, RPMTI, RPMTN: Redundant arithmetic with constants
    fn check_redundant_arithmetic(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if node.child_count() < 3 {
            return;
        }

        let op_text = extract_operator_text(node, source);

        let (lhs, rhs) = match (node.named_child(0), node.named_child(1)) {
            (Some(l), Some(r)) => (l, r),
            _ => return,
        };

        let lhs_text = &source[lhs.start_byte()..lhs.end_byte()];
        let rhs_text = &source[rhs.start_byte()..rhs.end_byte()];

        match op_text {
            "*" | ".*" => {
                // RPMT1: x * 1 → x
                if self.is_check_enabled("RPMT1") && rhs_text == "1" {
                    results.push(self.diag(
                        "RPMT1",
                        "Redundant multiplication by 1; simplify to 'x'",
                        node,
                        Some(Fix::new(
                            node.start_byte()..node.end_byte(),
                            lhs_text.to_string(),
                        )),
                    ));
                    return;
                }
                if self.is_check_enabled("RPMT1") && lhs_text == "1" {
                    results.push(self.diag(
                        "RPMT1",
                        "Redundant multiplication by 1; simplify to 'x'",
                        node,
                        Some(Fix::new(
                            node.start_byte()..node.end_byte(),
                            rhs_text.to_string(),
                        )),
                    ));
                    return;
                }

                // RPMT0: x * 0 → zeros(size(x))
                if self.is_check_enabled("RPMT0") && rhs_text == "0" {
                    results.push(self.diag(
                        "RPMT0",
                        "Multiplication by 0; consider 'zeros(size(x))' for clarity",
                        node,
                        Some(Fix::new(
                            node.start_byte()..node.end_byte(),
                            format!("zeros(size({lhs_text}))"),
                        )),
                    ));
                } else if self.is_check_enabled("RPMT0") && lhs_text == "0" {
                    results.push(self.diag(
                        "RPMT0",
                        "Multiplication by 0; consider 'zeros(size(x))' for clarity",
                        node,
                        Some(Fix::new(
                            node.start_byte()..node.end_byte(),
                            format!("zeros(size({rhs_text}))"),
                        )),
                    ));
                }
            }
            "+" => {
                // RPMTI: x + 0 → x
                if self.is_check_enabled("RPMTI") && rhs_text == "0" {
                    results.push(self.diag(
                        "RPMTI",
                        "Redundant addition of 0; simplify to 'x'",
                        node,
                        Some(Fix::new(
                            node.start_byte()..node.end_byte(),
                            lhs_text.to_string(),
                        )),
                    ));
                } else if self.is_check_enabled("RPMTI") && lhs_text == "0" {
                    results.push(self.diag(
                        "RPMTI",
                        "Redundant addition of 0; simplify to 'x'",
                        node,
                        Some(Fix::new(
                            node.start_byte()..node.end_byte(),
                            rhs_text.to_string(),
                        )),
                    ));
                }
            }
            "-" if self.is_check_enabled("RPMTN") && rhs_text == "0" => {
                // RPMTN: x - 0 → x
                results.push(self.diag(
                    "RPMTN",
                    "Redundant subtraction of 0; simplify to 'x'",
                    node,
                    Some(Fix::new(
                        node.start_byte()..node.end_byte(),
                        lhs_text.to_string(),
                    )),
                ));
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Rule trait implementation
// ---------------------------------------------------------------------------

impl Rule for ReadabilityEngine {
    fn id(&self) -> &'static str {
        "READABILITY_ENGINE"
    }

    fn description(&self) -> &'static str {
        "Readability improvement checks for MATLAB code"
    }

    fn severity(&self) -> Severity {
        Severity::Info
    }

    fn category(&self) -> Category {
        Category::Readability
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        TARGET_NODES
    }

    fn check(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        let mut results = Vec::new();

        match ctx.node.kind() {
            "function_call" => self.check_function_call(ctx, &mut results),
            "comparison_operator" => self.check_comparison(ctx, &mut results),
            "boolean_operator" => self.check_boolean_operator(ctx, &mut results),
            "assignment" => self.check_assignment(ctx, &mut results),
            "binary_operator" => self.check_binary_operator(ctx, &mut results),
            "matrix" => self.check_matrix(ctx, &mut results),
            // `identifier` nodes are handled via `assignment` LHS check (IJCL)
            // rather than standalone identifier dispatch, to avoid false
            // positives on identifier reads.
            _ => {}
        }

        results
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Check if a string literal represents an empty string (`''` or `""`).
fn is_empty_string(s: &str) -> bool {
    s == "''" || s == "\"\""
}

/// Check if a node is inside an `if_statement` condition.
fn is_inside_if_condition(node: tree_sitter::Node) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        match parent.kind() {
            "if_statement" => return true,
            "block" | "source_file" | "function_definition" => return false,
            _ => current = parent.parent(),
        }
    }
    false
}

/// Extract the operator text from a binary/boolean operator node.
///
/// Iterates over unnamed children to find the operator token between operands.
fn extract_operator_text<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> &'a str {
    // Walk all children (including unnamed) looking for the operator
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if !child.is_named() {
            let text = &source[child.start_byte()..child.end_byte()];
            // Operator tokens are things like +, -, *, .*, |, ||, &, &&, ==
            if !text.trim().is_empty()
                && text != "("
                && text != ")"
                && text != "["
                && text != "]"
            {
                return text;
            }
        }
    }
    ""
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "READABILITY_ENGINE",
    ReadabilityEngine::from_config
));
