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
//! This engine covers 36 readability checks across these patterns:
//!
//! - **Type checking simplification** (ISCHR, ISSTR, ISLOG, ISCEL, ISMAT):
//!   prefer `ischar`, `isstring`, etc. over `isa(x, 'type')`.
//! - **Dimension checks** (ISROW, ISCOL): prefer `isrow`/`iscolumn` over
//!   `size(x,dim)==1`.
//! - **Variable shadowing** (IJCL): `i`/`j` as assignment LHS shadows the
//!   complex unit.
//! - **Unnecessary brackets** (NBRAK2): `[x]` where `x` is a scalar.
//! - **String comparisons** (STREMP, STRCL1, STRCLFH, STRIFCND, STLOW):
//!   prefer modern string functions; flag unnecessary UPPER/LOWER calls.
//! - **Character literals** (CHARTEN): use `newline` instead of `char(10)`.
//! - **Formatting** (SPRINTFN): use `num2str` over simple `sprintf`.
//! - **Control flow style** (ASGSL): avoid inline assignment in conditions.
//! - **Error/warning style** (SPERR, SPWRN): prefer message IDs.
//! - **Input validation** (NCHKE): prefer `narginchk`/`nargoutchk`.
//! - **Output style** (DSPSP, DSPSY): prefer `fprintf`/`disp` over wrappers.
//! - **Flip/rotate** (FLUDLR): prefer `rot90(x, 2)` over `flipud(fliplr(x))` /
//!   `fliplr(flipud(x))`.
//! - **Redundant arithmetic** (RPMT1, RPMT0, RPMTI, RPMTN): simplify trivial
//!   multiplication/addition.
//! - **Redundant logic** (RPMTT, RPMTF): simplify boolean tautologies.
//! - **Size helpers** (PSIZE): prefer `numel` over `prod(size(x))`.
//! - **Logical helpers** (LOGSUM, LOGL): prefer `any`/logical indexing.
//! - **Arguments attribute** (FVINR): add an `(Input)` attribute to `arguments`
//!   blocks that have no attribute for readability.
//! - **Ambiguous identifiers** (MFAMB): flag identifiers used as function
//!   calls that are also defined as variables.
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.READABILITY_ENGINE]
//! severity = "info"
//! disabled_checks = ["IJCL", "NBRAK2"]
//! ```

use mlt_core::{Category, Config, Diagnostic, FileContext, Fix, NodeContext, Rule, Severity};
use serde::Deserialize;

use crate::analysis::symbols::{DefKind, SymbolTable};

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
    "arguments_statement",
];

/// Built-in function names that are always treated as functions, never as
/// ambiguous variable-or-function callees.
const BUILTIN_DENYLIST: &[&str] = &[
    "length", "size", "numel", "abs", "sum", "max", "min", "mean",
];

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// The readability engine — a single rule instance that checks 36 readability
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

    /// Run the file-level MFAMB check: flag bare-identifier callees that are
    /// also defined as variables in scope.
    fn check_mfamb(
        &self,
        tree: &tree_sitter::Tree,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("MFAMB") {
            return;
        }
        let table = SymbolTable::build(tree, source);
        walk_for_mfamb(tree.root_node(), source, &table, results);
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
            "flipud" | "fliplr" => self.check_fludlr(node, source, func_name, results),
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

        // COMNL: newline after comma acts as a row separator
        self.check_comnl(node, source, results);
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
        let args = match find_arguments(node) {
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
        let args = match find_arguments(node) {
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

        // STLOW: unnecessary UPPER/LOWER call in a comparison
        self.check_stlow(&named_children, source, results);
    }

    /// STLOW: `strcmp(upper(x), 'ABC')` — the UPPER/LOWER call is unnecessary
    /// when the compared literal is already entirely in that case.
    fn check_stlow(
        &self,
        named_children: &[tree_sitter::Node],
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("STLOW") {
            return;
        }

        for i in 0..2 {
            let arg = named_children[i];
            if arg.kind() != "function_call" {
                continue;
            }

            let func_name = match arg.child_by_field_name("name") {
                Some(n) => &source[n.start_byte()..n.end_byte()],
                None => continue,
            };

            let uppercase = match func_name {
                "upper" => true,
                "lower" => false,
                _ => continue,
            };

            let inner_args = match find_arguments(arg) {
                Some(a) => a,
                None => continue,
            };
            if inner_args.named_child_count() != 1 {
                continue;
            }
            let target = match inner_args.named_child(0) {
                Some(c) => c,
                None => continue,
            };
            let target_text = &source[target.start_byte()..target.end_byte()];

            let other = named_children[1 - i];
            if other.kind() != "string" {
                continue;
            }
            let other_text = &source[other.start_byte()..other.end_byte()];
            let literal = other_text.trim_matches('\'').trim_matches('"');

            if is_all_case(literal, uppercase) {
                results.push(self.diag(
                    "STLOW",
                    "In this comparison the call to UPPER/LOWER is unnecessary.",
                    arg,
                    Some(Fix::new(
                        arg.start_byte()..arg.end_byte(),
                        target_text.to_string(),
                    )),
                ));
            }
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

        let args = match find_arguments(node) {
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

        let args = match find_arguments(node) {
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

        let args = match find_arguments(node) {
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

        let args = match find_arguments(node) {
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
                    if let Some(inner_args) = find_arguments(inner) {
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

        let args = match find_arguments(node) {
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
                    if let Some(inner_args) = find_arguments(inner) {
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

    /// FLUDLR: `flipud(fliplr(x))` / `fliplr(flipud(x))` → `rot90(x, 2)`
    ///
    /// Flipping both vertically and horizontally is a 180-degree rotation,
    /// which reads more clearly as `rot90(x, 2)`.
    fn check_fludlr(
        &self,
        node: tree_sitter::Node,
        source: &str,
        func_name: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FLUDLR") {
            return;
        }

        let args = match find_arguments(node) {
            Some(a) => a,
            None => return,
        };

        let named_children: Vec<_> = (0..args.named_child_count())
            .filter_map(|i| args.named_child(i))
            .collect();

        // Outer call must have exactly one argument: the nested flip call
        if named_children.len() != 1 {
            return;
        }

        let inner = named_children[0];
        if inner.kind() != "function_call" {
            return;
        }

        let inner_name = match inner.child_by_field_name("name") {
            Some(n) => &source[n.start_byte()..n.end_byte()],
            None => return,
        };

        // Match the exact flipud/fliplr pair, each with exactly one argument
        let is_pair = match func_name {
            "flipud" => inner_name == "fliplr",
            "fliplr" => inner_name == "flipud",
            _ => false,
        };
        if !is_pair {
            return;
        }

        let inner_args = match find_arguments(inner) {
            Some(a) => a,
            None => return,
        };

        let inner_named: Vec<_> = (0..inner_args.named_child_count())
            .filter_map(|i| inner_args.named_child(i))
            .collect();

        if inner_named.len() != 1 {
            return;
        }

        let inner_arg_text = &source[inner_named[0].start_byte()..inner_named[0].end_byte()];

        results.push(self.diag(
            "FLUDLR",
            "Use 'rot90(x, 2)' instead of 'flipud(fliplr(x))' or 'fliplr(flipud(x))'",
            node,
            Some(Fix::new(
                node.start_byte()..node.end_byte(),
                format!("rot90({inner_arg_text}, 2)"),
            )),
        ));
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
        let args = match find_arguments(size_node) {
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

    /// COMNL: A newline following a comma in a matrix acts as a row
    /// separator; suggest replacing the comma with a semicolon or using an
    /// ellipsis to continue the row.
    fn check_comnl(
        &self,
        node: tree_sitter::Node,
        source: &str,
        results: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("COMNL") {
            return;
        }

        // Collect `row` children in source order using children() + kind
        // filter (NOT named_children): `line_continuation`/`...` is a NAMED
        // extra and would leak into named_children().
        let rows: Vec<tree_sitter::Node> = node
            .children(&mut node.walk())
            .filter(|c| c.kind() == "row")
            .collect();

        for i in 0..rows.len().saturating_sub(1) {
            let row = rows[i];
            let next_row = rows[i + 1];

            let Some(comma) = find_trailing_comma(row) else { continue };

            // Source gap between this row and the next row.
            let gap = &source[row.end_byte()..next_row.start_byte()];

            // Fire ONLY when a NEWLINE acts as the row separator:
            //   - gap contains '\n' (row separator is a newline), AND
            //   - gap does NOT contain ';' (semicolon is explicit row
            //     separator; replacing the comma would create a broken `;;`),
            //   AND
            //   - gap does NOT contain '...' (ellipsis escapes the newline).
            if gap.contains('\n') && !gap.contains(';') && !gap.contains("...") {
                let range = comma.start_byte()..comma.end_byte();
                results.push(self.diag(
                    "COMNL",
                    "Newline following comma acts as a row separator. Replace the comma with a semicolon to make the row separation clearer. Alternatively, use an ellipsis (...) to continue the current row on the next line.",
                    comma,
                    Some(Fix::new(range, ";")),
                ));
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

    /// FVINR: Add `(Input)` attribute to `arguments` block for readability.
    fn check_fvinr(&self, ctx: &NodeContext, results: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("FVINR") {
            return;
        }
        let node = ctx.node;

        // If any `attributes` child exists, the block already has (Input) or (Output).
        let has_attributes = node
            .named_children(&mut node.walk())
            .any(|c| c.kind() == "attributes");
        if has_attributes {
            return;
        }

        // The `arguments` keyword is the first (unnamed) child.
        let arguments_keyword = match node.child(0) {
            Some(c) if c.kind() == "arguments" => c,
            _ => return,
        };

        // CRITICAL: fix text is " (Input)" — LEADING space, no trailing space.
        // The keyword is immediately followed by a newline, so inserting a
        // trailing space would produce `arguments(Input)` (invalid).
        let fix = Fix::insert(arguments_keyword.end_byte(), " (Input)");
        results.push(self.diag(
            "FVINR",
            "For readability, add Input attribute to the input arguments block.",
            arguments_keyword,
            Some(fix),
        ));
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

    fn has_file_check(&self) -> bool {
        true
    }

    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> {
        let mut results = Vec::new();
        self.check_mfamb(ctx.tree, ctx.source, &mut results);
        results
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
            "arguments_statement" => self.check_fvinr(ctx, &mut results),
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

/// Find the `arguments` child of a `function_call` node.
///
/// In tree-sitter-matlab the call arguments are a child node of kind
/// `arguments` rather than a named field, so `child_by_field_name("arguments")`
/// never matches. This helper locates them by child kind instead.
fn find_arguments(node: tree_sitter::Node) -> Option<tree_sitter::Node> {
    node.named_children(&mut node.walk())
        .find(|c| c.kind() == "arguments")
}

/// Check if a string literal represents an empty string (`''` or `""`).
fn is_empty_string(s: &str) -> bool {
    s == "''" || s == "\"\""
}

/// Return true when `s` contains at least one alphabetic character and every
/// alphabetic character is in the case requested by `uppercase`.
fn is_all_case(s: &str, uppercase: bool) -> bool {
    let mut has_alpha = false;
    for c in s.chars() {
        if c.is_alphabetic() {
            has_alpha = true;
            if c.is_lowercase() == uppercase {
                return false;
            }
        }
    }
    has_alpha
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

/// Find the trailing comma of a `row` node, skipping trailing `comment` and
/// `line_continuation` children.
///
/// The comma token is unnamed, so all children (not just named children) must
/// be inspected. Returns `None` when the row does not end in a comma.
fn find_trailing_comma(row: tree_sitter::Node) -> Option<tree_sitter::Node> {
    for j in (0..row.child_count()).rev() {
        let child = row.child(j)?;
        match child.kind() {
            "comment" | "line_continuation" => continue,
            "," => return Some(child),
            _ => return None,
        }
    }
    None
}

// ---------------------------------------------------------------------------
// MFAMB helpers
// ---------------------------------------------------------------------------

/// Recursively walk the tree looking for `function_call` nodes.
fn walk_for_mfamb(
    node: tree_sitter::Node,
    source: &str,
    table: &SymbolTable,
    results: &mut Vec<Diagnostic>,
) {
    if node.kind() == "function_call" {
        check_one_mfamb(node, source, table, results);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk_for_mfamb(child, source, table, results);
    }
}

/// MFAMB: flag a `function_call` whose bare-identifier callee is also a
/// defined variable in scope.
fn check_one_mfamb(
    node: tree_sitter::Node,
    source: &str,
    table: &SymbolTable,
    results: &mut Vec<Diagnostic>,
) {
    let name_node = match node.child_by_field_name("name") {
        Some(n) => n,
        None => return,
    };
    // Only bare-identifier callees are ambiguous.
    if name_node.kind() != "identifier" {
        return;
    }
    // Skip indexed assignments: `x(i) = ...` — function_call is the LHS.
    if let Some(parent) = node.parent() {
        if parent.kind() == "assignment" {
            if let Some(left) = parent.child_by_field_name("left") {
                if left.id() == node.id() {
                    return;
                }
            }
        }
    }
    let name = &source[name_node.start_byte()..name_node.end_byte()];
    if BUILTIN_DENYLIST.contains(&name) {
        return;
    }
    // Primary gate: name is a variable (non-function) in this scope or an
    // enclosing scope.
    if !is_variable_in_scope(name, name_node.start_byte(), table) {
        return;
    }
    let pos = name_node.start_position();
    results.push(Diagnostic {
        rule_id: "MFAMB",
        message: format!(
            "Code Analyzer cannot determine whether {name} is a variable or a function, and assumes it is a function."
        ),
        severity: Severity::Info,
        byte_range: name_node.start_byte()..name_node.end_byte(),
        line: pos.row + 1,
        column: pos.column + 1,
        fix: None,
    });
}

/// Check whether `name` is defined as a variable (not a nested function) in
/// the scope containing `byte_offset` or any enclosing scope.
fn is_variable_in_scope(name: &str, byte_offset: usize, table: &SymbolTable) -> bool {
    let mut current = table.scope_at(byte_offset);
    while let Some(scope) = current {
        if scope
            .defs
            .iter()
            .any(|d| d.name == name && d.kind != DefKind::NestedFunction)
        {
            return true;
        }
        current = scope.parent.and_then(|idx| table.scopes.get(idx));
    }
    false
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "READABILITY_ENGINE",
    ReadabilityEngine::from_config
));

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{has_id, lint_file, lint_nodes};
    use mlt_core::Config;

    fn engine() -> Box<dyn Rule> {
        ReadabilityEngine::from_config(&Config::default())
    }

    // -- MFAMB ---------------------------------------------------------------

    #[test]
    fn mfamb_fires_when_name_assigned_and_called() {
        let diags = lint_file(&*engine(), "somevar = 5;\ny = somevar(1);\n");
        assert!(has_id(&diags, "MFAMB"), "got: {diags:?}");
    }

    #[test]
    fn mfamb_does_not_fire_for_builtin_call() {
        let diags = lint_file(&*engine(), "y = length(x);\n");
        assert!(!has_id(&diags, "MFAMB"), "got: {diags:?}");
    }

    #[test]
    fn mfamb_does_not_fire_for_field_expression_call() {
        let diags = lint_file(&*engine(), "somevar = 5;\ny = somevar.method();\n");
        assert!(!has_id(&diags, "MFAMB"), "got: {diags:?}");
    }

    #[test]
    fn mfamb_does_not_fire_when_name_not_a_variable() {
        let diags = lint_file(&*engine(), "y = mylocal(x);\n");
        assert!(!has_id(&diags, "MFAMB"), "got: {diags:?}");
    }

    #[test]
    fn mfamb_does_not_fire_for_indexed_assignment_lhs() {
        let diags = lint_file(&*engine(), "x = 1:10;\nx(1) = 5;\n");
        assert!(!has_id(&diags, "MFAMB"), "got: {diags:?}");
    }

    #[test]
    fn mfamb_disabled_in_config_does_not_fire() {
        let config = Config::from_toml(
            "[lint.rules.READABILITY_ENGINE]\ndisabled_checks = [\"MFAMB\"]\n",
        )
        .expect("valid config");
        let rule = ReadabilityEngine::from_config(&config);
        let diags = lint_file(&*rule, "somevar = 5;\ny = somevar(1);\n");
        assert!(!has_id(&diags, "MFAMB"), "got: {diags:?}");
    }

    // -- ISCHR ---------------------------------------------------------------

    #[test]
    fn ischr_isa_char_fires() {
        let diags = lint_nodes(&*engine(), "x = isa(a, 'char');\n");
        assert!(has_id(&diags, "ISCHR"), "got: {diags:?}");
    }

    #[test]
    fn ischr_isa_other_type_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = isa(a, 'numeric');\n");
        assert!(!has_id(&diags, "ISCHR"), "got: {diags:?}");
    }

    // -- ISSTR ---------------------------------------------------------------

    #[test]
    fn isstr_isa_string_fires() {
        let diags = lint_nodes(&*engine(), "x = isa(a, 'string');\n");
        assert!(has_id(&diags, "ISSTR"), "got: {diags:?}");
    }

    #[test]
    fn isstr_isa_other_type_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = isa(a, 'numeric');\n");
        assert!(!has_id(&diags, "ISSTR"), "got: {diags:?}");
    }

    // -- ISLOG ---------------------------------------------------------------

    #[test]
    fn islog_isa_logical_fires() {
        let diags = lint_nodes(&*engine(), "x = isa(a, 'logical');\n");
        assert!(has_id(&diags, "ISLOG"), "got: {diags:?}");
    }

    #[test]
    fn islog_isa_other_type_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = isa(a, 'numeric');\n");
        assert!(!has_id(&diags, "ISLOG"), "got: {diags:?}");
    }

    // -- ISCEL ---------------------------------------------------------------

    #[test]
    fn iscel_isa_cell_fires() {
        let diags = lint_nodes(&*engine(), "x = isa(a, 'cell');\n");
        assert!(has_id(&diags, "ISCEL"), "got: {diags:?}");
    }

    #[test]
    fn iscel_isa_other_type_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = isa(a, 'numeric');\n");
        assert!(!has_id(&diags, "ISCEL"), "got: {diags:?}");
    }

    // -- ISMAT ---------------------------------------------------------------

    #[test]
    fn ismat_isa_double_fires() {
        let diags = lint_nodes(&*engine(), "x = isa(a, 'double');\n");
        assert!(has_id(&diags, "ISMAT"), "got: {diags:?}");
    }

    #[test]
    fn ismat_isa_other_type_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = isa(a, 'numeric');\n");
        assert!(!has_id(&diags, "ISMAT"), "got: {diags:?}");
    }

    // -- ISROW ---------------------------------------------------------------

    #[test]
    fn isrow_size_dim1_equals_one_fires() {
        let diags = lint_nodes(&*engine(), "x = size(a, 1) == 1;\n");
        assert!(has_id(&diags, "ISROW"), "got: {diags:?}");
    }

    #[test]
    fn isrow_size_dim1_equals_two_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = size(a, 1) == 2;\n");
        assert!(!has_id(&diags, "ISROW"), "got: {diags:?}");
    }

    // -- ISCOL ---------------------------------------------------------------

    #[test]
    fn iscol_size_dim2_equals_one_fires() {
        let diags = lint_nodes(&*engine(), "x = size(a, 2) == 1;\n");
        assert!(has_id(&diags, "ISCOL"), "got: {diags:?}");
    }

    #[test]
    fn iscol_size_dim2_equals_zero_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = size(a, 2) == 0;\n");
        assert!(!has_id(&diags, "ISCOL"), "got: {diags:?}");
    }

    // -- IJCL ----------------------------------------------------------------

    #[test]
    fn ijcl_assign_to_i_fires() {
        let diags = lint_nodes(&*engine(), "i = 5;\n");
        assert!(has_id(&diags, "IJCL"), "got: {diags:?}");
    }

    #[test]
    fn ijcl_assign_to_j_fires() {
        let diags = lint_nodes(&*engine(), "j = zeros(3);\n");
        assert!(has_id(&diags, "IJCL"), "got: {diags:?}");
    }

    #[test]
    fn ijcl_assign_to_other_name_does_not_fire() {
        let diags = lint_nodes(&*engine(), "k = 5;\n");
        assert!(!has_id(&diags, "IJCL"), "got: {diags:?}");
    }

    // -- NBRAK2 --------------------------------------------------------------

    #[test]
    fn nbrak2_brackets_around_scalar_fires() {
        let diags = lint_nodes(&*engine(), "x = [5];\n");
        assert!(has_id(&diags, "NBRAK2"), "got: {diags:?}");
    }

    #[test]
    fn nbrak2_brackets_around_identifier_fires() {
        let diags = lint_nodes(&*engine(), "x = [v];\n");
        assert!(has_id(&diags, "NBRAK2"), "got: {diags:?}");
    }

    #[test]
    fn nbrak2_multi_element_matrix_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = [1 2];\n");
        assert!(!has_id(&diags, "NBRAK2"), "got: {diags:?}");
    }

    // -- STREMP --------------------------------------------------------------

    #[test]
    fn stremp_strcmp_empty_fires() {
        let diags = lint_nodes(&*engine(), "x = strcmp(s, '');\n");
        assert!(has_id(&diags, "STREMP"), "got: {diags:?}");
    }

    #[test]
    fn stremp_strcmp_nonempty_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = strcmp(s, 'a');\n");
        assert!(!has_id(&diags, "STREMP"), "got: {diags:?}");
    }

    // -- STRIFCND ------------------------------------------------------------

    #[test]
    fn strifcnd_strcmp_in_if_condition_fires() {
        let source = "if strcmp(a, b)\n    x = 1;\nend\n";
        let diags = lint_nodes(&*engine(), source);
        assert!(has_id(&diags, "STRIFCND"), "got: {diags:?}");
    }

    #[test]
    fn strifcnd_strcmp_at_statement_level_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = strcmp(a, b);\n");
        assert!(!has_id(&diags, "STRIFCND"), "got: {diags:?}");
    }

    // -- STRCL1 --------------------------------------------------------------

    #[test]
    fn strcl1_strncmp_fires() {
        let diags = lint_nodes(&*engine(), "x = strncmp(a, b, 3);\n");
        assert!(has_id(&diags, "STRCL1"), "got: {diags:?}");
    }

    #[test]
    fn strcl1_strncmpi_fires() {
        let diags = lint_nodes(&*engine(), "x = strncmpi(a, b, 3);\n");
        assert!(has_id(&diags, "STRCL1"), "got: {diags:?}");
    }

    #[test]
    fn strcl1_starts_with_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = startsWith(a, b);\n");
        assert!(!has_id(&diags, "STRCL1"), "got: {diags:?}");
    }

    // -- STRCLFH -------------------------------------------------------------

    #[test]
    fn strclfh_strfind_fires() {
        let diags = lint_nodes(&*engine(), "x = strfind(a, b);\n");
        assert!(has_id(&diags, "STRCLFH"), "got: {diags:?}");
    }

    #[test]
    fn strclfh_contains_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = contains(a, b);\n");
        assert!(!has_id(&diags, "STRCLFH"), "got: {diags:?}");
    }

    // -- CHARTEN -------------------------------------------------------------

    #[test]
    fn charten_char_10_fires() {
        let diags = lint_nodes(&*engine(), "x = char(10);\n");
        assert!(has_id(&diags, "CHARTEN"), "got: {diags:?}");
    }

    #[test]
    fn charten_char_other_value_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = char(9);\n");
        assert!(!has_id(&diags, "CHARTEN"), "got: {diags:?}");
    }

    // -- SPRINTFN ------------------------------------------------------------

    #[test]
    fn sprintfn_simple_numeric_format_fires() {
        let diags = lint_nodes(&*engine(), "x = sprintf('%d', y);\n");
        assert!(has_id(&diags, "SPRINTFN"), "got: {diags:?}");
    }

    #[test]
    fn sprintfn_string_format_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = sprintf('%s', y);\n");
        assert!(!has_id(&diags, "SPRINTFN"), "got: {diags:?}");
    }

    // -- SPERR ---------------------------------------------------------------

    #[test]
    fn sperr_error_without_message_id_fires() {
        let diags = lint_nodes(&*engine(), "error('my message');\n");
        assert!(has_id(&diags, "SPERR"), "got: {diags:?}");
    }

    #[test]
    fn sperr_error_with_message_id_does_not_fire() {
        let diags = lint_nodes(&*engine(), "error('MyComp:myID', 'message');\n");
        assert!(!has_id(&diags, "SPERR"), "got: {diags:?}");
    }

    // -- SPWRN ---------------------------------------------------------------

    #[test]
    fn spwrn_warning_without_message_id_fires() {
        let diags = lint_nodes(&*engine(), "warning('my message');\n");
        assert!(has_id(&diags, "SPWRN"), "got: {diags:?}");
    }

    #[test]
    fn spwrn_warning_with_message_id_does_not_fire() {
        let diags = lint_nodes(&*engine(), "warning('MyComp:myID', 'message');\n");
        assert!(!has_id(&diags, "SPWRN"), "got: {diags:?}");
    }

    // -- NCHKE ---------------------------------------------------------------

    #[test]
    fn nchke_nargchk_fires() {
        let diags = lint_nodes(&*engine(), "x = nargchk(1, 2, nargin);\n");
        assert!(has_id(&diags, "NCHKE"), "got: {diags:?}");
    }

    #[test]
    fn nchke_narginchk_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = narginchk(1, 2);\n");
        assert!(!has_id(&diags, "NCHKE"), "got: {diags:?}");
    }

    // -- DSPSP ---------------------------------------------------------------

    #[test]
    fn dspsp_disp_sprintf_fires() {
        let diags = lint_nodes(&*engine(), "disp(sprintf('%d', x));\n");
        assert!(has_id(&diags, "DSPSP"), "got: {diags:?}");
    }

    #[test]
    fn dspsp_disp_direct_does_not_fire() {
        let diags = lint_nodes(&*engine(), "disp(x);\n");
        assert!(!has_id(&diags, "DSPSP"), "got: {diags:?}");
    }

    // -- DSPSY ---------------------------------------------------------------

    #[test]
    fn dspsy_display_fires() {
        let diags = lint_nodes(&*engine(), "display(x);\n");
        assert!(has_id(&diags, "DSPSY"), "got: {diags:?}");
    }

    #[test]
    fn dspsy_disp_does_not_fire() {
        let diags = lint_nodes(&*engine(), "disp(x);\n");
        assert!(!has_id(&diags, "DSPSY"), "got: {diags:?}");
    }

    // -- PSIZE ---------------------------------------------------------------

    #[test]
    fn psize_prod_size_fires() {
        let diags = lint_nodes(&*engine(), "x = prod(size(a));\n");
        assert!(has_id(&diags, "PSIZE"), "got: {diags:?}");
    }

    #[test]
    fn psize_prod_without_size_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = prod(a);\n");
        assert!(!has_id(&diags, "PSIZE"), "got: {diags:?}");
    }

    // -- LOGSUM --------------------------------------------------------------

    #[test]
    fn logsum_sum_greater_than_zero_fires() {
        let diags = lint_nodes(&*engine(), "x = sum(y) > 0;\n");
        assert!(has_id(&diags, "LOGSUM"), "got: {diags:?}");
    }

    #[test]
    fn logsum_sum_alone_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = sum(y);\n");
        assert!(!has_id(&diags, "LOGSUM"), "got: {diags:?}");
    }

    // -- LOGL ----------------------------------------------------------------

    #[test]
    fn logl_find_in_indexing_fires() {
        let diags = lint_nodes(&*engine(), "x = m(find(c));\n");
        assert!(has_id(&diags, "LOGL"), "got: {diags:?}");
    }

    #[test]
    fn logl_find_at_statement_level_does_not_fire() {
        let diags = lint_nodes(&*engine(), "y = find(c);\n");
        assert!(!has_id(&diags, "LOGL"), "got: {diags:?}");
    }

    // -- RPMTT ---------------------------------------------------------------

    #[test]
    fn rpmtt_or_true_fires() {
        let diags = lint_nodes(&*engine(), "x = a || true;\n");
        assert!(has_id(&diags, "RPMTT"), "got: {diags:?}");
    }

    #[test]
    fn rpmtt_or_other_variable_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = a || b;\n");
        assert!(!has_id(&diags, "RPMTT"), "got: {diags:?}");
    }

    // -- RPMTF ---------------------------------------------------------------

    #[test]
    fn rpmtf_and_false_fires() {
        let diags = lint_nodes(&*engine(), "x = a && false;\n");
        assert!(has_id(&diags, "RPMTF"), "got: {diags:?}");
    }

    #[test]
    fn rpmtf_and_other_variable_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = a && b;\n");
        assert!(!has_id(&diags, "RPMTF"), "got: {diags:?}");
    }

    // -- RPMT1 ---------------------------------------------------------------

    #[test]
    fn rpmt1_mul_by_one_fires() {
        let diags = lint_nodes(&*engine(), "x = a * 1;\n");
        assert!(has_id(&diags, "RPMT1"), "got: {diags:?}");
    }

    #[test]
    fn rpmt1_one_times_x_fires() {
        let diags = lint_nodes(&*engine(), "x = 1 * a;\n");
        assert!(has_id(&diags, "RPMT1"), "got: {diags:?}");
    }

    #[test]
    fn rpmt1_mul_by_two_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = a * 2;\n");
        assert!(!has_id(&diags, "RPMT1"), "got: {diags:?}");
    }

    // -- RPMT0 ---------------------------------------------------------------

    #[test]
    fn rpmt0_mul_by_zero_fires() {
        let diags = lint_nodes(&*engine(), "x = a * 0;\n");
        assert!(has_id(&diags, "RPMT0"), "got: {diags:?}");
    }

    #[test]
    fn rpmt0_mul_by_two_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = a * 2;\n");
        assert!(!has_id(&diags, "RPMT0"), "got: {diags:?}");
    }

    // -- RPMTI ---------------------------------------------------------------

    #[test]
    fn rpmti_add_zero_fires() {
        let diags = lint_nodes(&*engine(), "x = a + 0;\n");
        assert!(has_id(&diags, "RPMTI"), "got: {diags:?}");
    }

    #[test]
    fn rpmti_add_two_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = a + 2;\n");
        assert!(!has_id(&diags, "RPMTI"), "got: {diags:?}");
    }

    // -- RPMTN ---------------------------------------------------------------

    #[test]
    fn rpmtn_sub_zero_fires() {
        let diags = lint_nodes(&*engine(), "x = a - 0;\n");
        assert!(has_id(&diags, "RPMTN"), "got: {diags:?}");
    }

    #[test]
    fn rpmtn_sub_two_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = a - 2;\n");
        assert!(!has_id(&diags, "RPMTN"), "got: {diags:?}");
    }

    // -- FVINR ---------------------------------------------------------------

    #[test]
    fn fvinr_no_attribute_fires() {
        let source = "function f(a)\n    arguments\n        a (1,1)\n    end\nend\n";
        let diags = lint_nodes(&*engine(), source);
        assert!(has_id(&diags, "FVINR"), "got: {diags:?}");
    }

    #[test]
    fn fvinr_with_input_attribute_does_not_fire() {
        let source = "function f(a)\n    arguments (Input)\n        a (1,1)\n    end\nend\n";
        let diags = lint_nodes(&*engine(), source);
        assert!(!has_id(&diags, "FVINR"), "got: {diags:?}");
    }

    #[test]
    fn fvinr_with_output_attribute_does_not_fire() {
        let source = "function f(a)\n    arguments (Output)\n        a\n    end\nend\n";
        let diags = lint_nodes(&*engine(), source);
        assert!(!has_id(&diags, "FVINR"), "got: {diags:?}");
    }

    #[test]
    fn fvinr_disabled_in_config_does_not_fire() {
        let config = Config::from_toml(
            "[lint.rules.READABILITY_ENGINE]\ndisabled_checks = [\"FVINR\"]\n",
        )
        .expect("valid config");
        let rule = ReadabilityEngine::from_config(&config);
        let source = "function f(a)\n    arguments\n        a (1,1)\n    end\nend\n";
        let diags = lint_nodes(&*rule, source);
        assert!(!has_id(&diags, "FVINR"), "got: {diags:?}");
    }

    #[test]
    fn fvinr_fix_inserts_input_attribute() {
        let source = "function f(a)\n    arguments\n        a (1,1)\n    end\nend\n";
        let diags = lint_nodes(&*engine(), source);
        let diag = diags
            .iter()
            .find(|d| d.rule_id == "FVINR")
            .expect("FVINR should fire");
        let fix = diag.fix.as_ref().expect("FVINR should carry an auto-fix");
        assert_eq!(fix.replacement, " (Input)");
        let mut fixed = String::from(source);
        fixed.replace_range(fix.byte_range.clone(), &fix.replacement);
        assert!(
            fixed.contains("arguments (Input)"),
            "got: {fixed:?}"
        );
    }

    // -- FLUDLR --------------------------------------------------------------

    #[test]
    fn fludlr_flipud_fliplr_fires() {
        let diags = lint_nodes(&*engine(), "y = flipud(fliplr(x));\n");
        assert!(has_id(&diags, "FLUDLR"), "got: {diags:?}");
    }

    #[test]
    fn fludlr_fliplr_flipud_fires() {
        let diags = lint_nodes(&*engine(), "y = fliplr(flipud(x));\n");
        assert!(has_id(&diags, "FLUDLR"), "got: {diags:?}");
    }

    #[test]
    fn fludlr_single_flipud_does_not_fire() {
        let diags = lint_nodes(&*engine(), "y = flipud(x);\n");
        assert!(!has_id(&diags, "FLUDLR"), "got: {diags:?}");
    }

    #[test]
    fn fludlr_single_fliplr_does_not_fire() {
        let diags = lint_nodes(&*engine(), "y = fliplr(x);\n");
        assert!(!has_id(&diags, "FLUDLR"), "got: {diags:?}");
    }

    #[test]
    fn fludlr_two_arg_outer_does_not_fire() {
        let diags = lint_nodes(&*engine(), "y = flipud(fliplr(x), 2);\n");
        assert!(!has_id(&diags, "FLUDLR"), "got: {diags:?}");
    }

    #[test]
    fn fludlr_same_flip_pair_does_not_fire() {
        let diags = lint_nodes(&*engine(), "y = flipud(flipud(x));\n");
        assert!(!has_id(&diags, "FLUDLR"), "got: {diags:?}");
    }

    #[test]
    fn fludlr_fix_replaces_with_rot90() {
        let source = "y = flipud(fliplr(x));\n";
        let diags = lint_nodes(&*engine(), source);
        let diag = diags
            .iter()
            .find(|d| d.rule_id == "FLUDLR")
            .expect("FLUDLR should fire");
        let fix = diag.fix.as_ref().expect("FLUDLR should carry an auto-fix");
        assert_eq!(fix.replacement, "rot90(x, 2)");
        let mut fixed = String::from(source);
        fixed.replace_range(fix.byte_range.clone(), &fix.replacement);
        assert_eq!(fixed, "y = rot90(x, 2);\n", "got: {fixed:?}");
    }

    #[test]
    fn fludlr_disabled_in_config_does_not_fire() {
        let config = Config::from_toml(
            "[lint.rules.READABILITY_ENGINE]\ndisabled_checks = [\"FLUDLR\"]\n",
        )
        .expect("valid config");
        let rule = ReadabilityEngine::from_config(&config);
        let diags = lint_nodes(&*rule, "y = flipud(fliplr(x));\n");
        assert!(!has_id(&diags, "FLUDLR"), "got: {diags:?}");
    }

    // -- STLOW ---------------------------------------------------------------

    #[test]
    fn stlow_upper_with_uppercase_literal_fires() {
        let diags = lint_nodes(&*engine(), "x = strcmp(upper(str), 'ABC');\n");
        assert!(has_id(&diags, "STLOW"), "got: {diags:?}");
    }

    #[test]
    fn stlow_lower_with_lowercase_literal_fires() {
        let diags = lint_nodes(&*engine(), "x = strcmp(lower(str), 'abc');\n");
        assert!(has_id(&diags, "STLOW"), "got: {diags:?}");
    }

    #[test]
    fn stlow_mixed_case_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = strcmp(upper(str), 'AbC');\n");
        assert!(!has_id(&diags, "STLOW"), "got: {diags:?}");
    }

    #[test]
    fn stlow_opposite_case_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = strcmp(upper(str), 'abc');\n");
        assert!(!has_id(&diags, "STLOW"), "got: {diags:?}");
    }

    #[test]
    fn stlow_no_conversion_call_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = strcmp(str, 'ABC');\n");
        assert!(!has_id(&diags, "STLOW"), "got: {diags:?}");
    }

    #[test]
    fn stlow_non_literal_other_side_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = strcmp(upper(x), y);\n");
        assert!(!has_id(&diags, "STLOW"), "got: {diags:?}");
    }

    #[test]
    fn stlow_fix_replaces_call_with_inner_arg() {
        let source = "x = strcmp(upper(x), 'ABC');\n";
        let diags = lint_nodes(&*engine(), source);
        let diag = diags
            .iter()
            .find(|d| d.rule_id == "STLOW")
            .expect("STLOW should fire");
        let fix = diag.fix.as_ref().expect("STLOW should carry an auto-fix");
        assert_eq!(fix.replacement, "x");
        assert_eq!(&source[fix.byte_range.clone()], "upper(x)");
    }

    #[test]
    fn stlow_disabled_in_config_does_not_fire() {
        let config = Config::from_toml(
            "[lint.rules.READABILITY_ENGINE]\ndisabled_checks = [\"STLOW\"]\n",
        )
        .expect("valid config");
        let rule = ReadabilityEngine::from_config(&config);
        let diags = lint_nodes(&*rule, "x = strcmp(upper(str), 'ABC');\n");
        assert!(!has_id(&diags, "STLOW"), "got: {diags:?}");
    }

    // -- COMNL ---------------------------------------------------------------

    #[test]
    fn comnl_trailing_comma_before_newline_fires() {
        let diags = lint_nodes(&*engine(), "x = [1, 2,\n3, 4];\n");
        assert!(has_id(&diags, "COMNL"), "got: {diags:?}");
    }

    #[test]
    fn comnl_multiple_rows_fires_each_comma() {
        let diags = lint_nodes(&*engine(), "x = [1, 2,\n3, 4,\n5, 6];\n");
        let count = diags.iter().filter(|d| d.rule_id == "COMNL").count();
        assert_eq!(count, 2, "got: {diags:?}");
    }

    #[test]
    fn comnl_single_row_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = [1, 2, 3, 4];\n");
        assert!(!has_id(&diags, "COMNL"), "got: {diags:?}");
    }

    #[test]
    fn comnl_no_trailing_comma_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = [1, 2, 3\n4, 5, 6];\n");
        assert!(!has_id(&diags, "COMNL"), "got: {diags:?}");
    }

    #[test]
    fn comnl_semicolon_separator_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = [1, 2;\n3, 4];\n");
        assert!(!has_id(&diags, "COMNL"), "got: {diags:?}");
    }

    #[test]
    fn comnl_ellipsis_continuation_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = [1, 2, ...\n3, 4];\n");
        assert!(!has_id(&diags, "COMNL"), "got: {diags:?}");
    }

    #[test]
    fn comnl_semicolon_after_trailing_comma_does_not_fire() {
        let diags = lint_nodes(&*engine(), "x = [1, 2,;\n3, 4];\n");
        assert!(!has_id(&diags, "COMNL"), "got: {diags:?}");
    }

    #[test]
    fn comnl_disabled_via_config_does_not_fire() {
        let config = Config::from_toml(
            "[lint.rules.READABILITY_ENGINE]\ndisabled_checks = [\"COMNL\"]\n",
        )
        .expect("valid config");
        let rule = ReadabilityEngine::from_config(&config);
        let diags = lint_nodes(&*rule, "x = [1, 2,\n3, 4];\n");
        assert!(!has_id(&diags, "COMNL"), "got: {diags:?}");
    }

    #[test]
    fn comnl_fix_replaces_comma_with_semicolon() {
        let source = "x = [1, 2,\n3, 4];\n";
        let diags = lint_nodes(&*engine(), source);
        let comnl: Vec<_> = diags.iter().filter(|d| d.rule_id == "COMNL").collect();
        assert_eq!(comnl.len(), 1, "got: {diags:?}");
        let fix = comnl[0].fix.as_ref().expect("COMNL should provide a fix");
        assert_eq!(&source[fix.byte_range.clone()], ",");
        assert_eq!(fix.replacement, ";");
    }
}

