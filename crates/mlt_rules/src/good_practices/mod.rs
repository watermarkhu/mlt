//! # Good Practices: Common best-practice checks for MATLAB code
//!
//! This module implements 59 good-practice checks from MATLAB's Code Analyzer,
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
//! | PFRIN     | The reduction variable might not be set before the PARFOR loop |
//! | PFRUS     | The reduction variable might not be used after the PARFOR loop |
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
//! | SUBSINDEX | Do not overload `subsindex` for fundamental data types |
//! | VTFIN    | Validated value should be the first input to a `validate*` function |
//! | CTOINW   | Constructed object passed to its own constructor |
//! | FXUP     | Outer loop index set inside a nested function |
//!
//! ### App Designer
//!
//! | Check ID | Description |
//! |----------|-------------|
//! | ADMTHDINV | Class method called without `app` as the first argument |
//! | ADPROP   | Property assigned through a bare identifier instead of `app.PROP` |
//! | ADPROPLC | Property read through a bare identifier instead of `app.PROP` |
//!
//! ### OOP practice
//!
//! | Check ID | Description |
//! |----------|-------------|
//! | MCNPN    | Member access on the object that is not declared in the class |
//! | MCNPR    | Assignment target on the object that is not a property |
//! | MCSNOV   | Value-class setter does not return the modified object |
//! | MCSOH    | Handle-class setter unnecessarily returns the modified object |
//! | MCVM     | Value-class method modifying the object has no output |
//! | MCCSPS   | Constant property name used as a struct in a dot-access chain |
//! | MCSUP    | Setter accesses a property other than the one it sets |
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

mod check_attf_attof;
mod check_app_designer;
mod check_chain;
mod check_comfs_semfs;
mod check_comnc;
mod check_comnot;
mod check_compnop;
mod check_cprop;
mod check_ctch;
mod check_ctpct;
mod check_discouraged_functions;
mod check_display_override;
mod check_dspmda;
mod check_dualc;
mod check_elarlog;
mod check_eval;
mod check_fncolnd;
mod check_fndef;
mod check_fval;
mod check_fxset;
mod check_gvmis;
mod check_iters;
mod check_lngnm;
mod check_load;
mod check_logical_aggregation;
mod check_m3col;
mod check_mccpe;
mod check_mccpi;
mod check_mcpo;
mod check_mcsac;
mod check_mdepin;
mod check_mexcep;
mod check_mgmd;
mod check_mherm;
mod check_mipc1;
mod check_misc_general;
mod check_mnuml;
mod check_mobsrv;
mod check_mthans;
mod check_nbrak1;
mod check_noans;
mod check_noin;
mod check_oop_practice;
mod check_pfevb;
mod check_pfgp;
mod check_pfgv;
mod check_pfrni;
mod check_parfor_reduction;
mod check_prop_validation;
mod check_rmwrn;
mod check_sepex;
mod check_shociraa;
mod check_simpt;
mod check_spevb;
mod check_spgv;
mod check_stci;
mod check_stcmp;
mod check_stisa;
mod check_strnu;
mod check_strsz;
mod check_tlev;
mod check_trync;
mod check_unonc;
mod check_unrpwr;
mod check_valst;
mod check_warning_error_tag;

// ---------------------------------------------------------------------------
// Default configuration values
// ---------------------------------------------------------------------------

pub(crate) const fn default_max_variable_name_length() -> usize {
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
pub(crate) fn node_text<'a>(node: Node, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

/// Parent node types indicating statement-level context.
pub(crate) const STATEMENT_PARENTS: &[&str] = &["source_file", "block"];

/// Warning message ID tags removed from MATLAB; RMWRN fires when a
/// `warning(...)` call uses one of these tags. Currently empty (placeholder):
/// no tags are populated yet, so RMWRN is inert until a tag is added here.
pub(crate) const REMOVED_WARNING_TAGS: &[&str] = &[];

// ---------------------------------------------------------------------------
// Node-level target types
// ---------------------------------------------------------------------------

/// Node types the engine subscribes to for per-node checks.
pub(crate) const TARGET_NODES: &[&str] = &[
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
    pub(crate) fn is_check_enabled(&self, check_id: &str) -> bool {
        !self
            .config
            .disabled_checks
            .iter()
            .any(|id| id == check_id)
    }

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

    /// Shared COMPNOP/COMPNOT logic: extract the `(call, literal, operator)`
    /// triple from a `comparison_operator` node when exactly one operand is a
    /// `function_call` and the other is the `true`/`false` literal.
    pub(crate) fn comparison_with_literal<'a>(
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

}

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

        // Parfor reduction-variable checks.
        diagnostics.extend(self.check_parfor_reduction(ctx.tree, ctx.source));

        // App Designer and OOP practice checks.
        diagnostics.extend(self.check_app_designer(ctx.tree, ctx.source));
        diagnostics.extend(self.check_oop_practice(ctx.tree, ctx.source));

        // Miscellaneous general practice checks.
        diagnostics.extend(self.check_misc_general(ctx.tree, ctx.source));

        diagnostics
    }
}

inventory::submit!(crate::RuleRegistration::new(
    "GOOD_PRACTICES_ENGINE",
    GoodPracticesEngine::from_config
));

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Check whether a node has a child of the given kind.
pub(crate) fn has_child_of_kind(node: Node, kind: &str) -> bool {
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
pub(crate) fn find_child_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
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
pub(crate) fn first_named_child(node: Node) -> Option<Node> {
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
pub(crate) fn last_named_child_not_end(node: Node) -> Option<Node> {
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
pub(crate) fn is_statement_level(node: Node) -> bool {
    node.parent()
        .map(|p| STATEMENT_PARENTS.contains(&p.kind()))
        .unwrap_or(false)
}

/// Extract the function name from a `function_call` node.
pub(crate) fn get_function_call_name<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    if node.kind() != "function_call" {
        return None;
    }
    let name_node = node.child_by_field_name("name")?;
    Some(node_text(name_node, source))
}

/// Extract the command name from a `command` node.
pub(crate) fn get_command_name<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
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
pub(crate) fn get_arguments_text<'a>(node: Node<'a>, source: &'a str) -> String {
    find_child_of_kind(node, "arguments")
        .map(|a| node_text(a, source).to_string())
        .unwrap_or_default()
}

/// Find the operator text in a binary/boolean/comparison operator node.
pub(crate) fn find_operator_text<'a>(node: Node<'a>, source: &'a str) -> String {
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
pub(crate) fn node_has_string_child(node: Node) -> bool {
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
pub(crate) fn classify_eval_usage(args_text: &str, config: &GoodPracticesConfig) -> (&'static str, &'static str) {
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
pub(crate) fn args_has_case_conversion(args_node: Node, source: &str) -> bool {
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
pub(crate) fn args_has_function_call(args_node: Node, source: &str, target_name: &str) -> bool {
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
pub(crate) fn field_chain_depth(node: Node) -> usize {
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
pub(crate) fn is_inside_array_context(node: Node) -> bool {
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
pub(crate) fn find_element_wise_boolean_in_condition(
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
pub(crate) fn find_node_in_range<'a>(root: Node<'a>, range: &std::ops::Range<usize>) -> Option<Node<'a>> {
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
pub(crate) fn collect_statements_by_line(
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
pub(crate) fn find_end_as_index(node: Node, diagnostics: &mut Vec<Diagnostic>) {
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
pub(crate) fn count_children_of_kind(node: Node, kind: &str) -> usize {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .filter(|c| c.kind() == kind)
        .count()
}

/// Get the root node of a tree by walking up from a node.
pub(crate) fn root_node_of<'a>(mut node: Node<'a>) -> Node<'a> {
    while let Some(p) = node.parent() {
        node = p;
    }
    node
}

/// Check whether a `for_statement` node is a parfor loop.
pub(crate) fn is_parfor_node(node: Node, source: &str) -> bool {
    node.kind() == "for_statement" && node_text(node, source).trim_start().starts_with("parfor")
}

/// Count the named children of a node.
pub(crate) fn count_named_children(node: Node) -> usize {
    let mut cursor = node.walk();
    node.children(&mut cursor).filter(|c| c.is_named()).count()
}

/// Get the `range` node of a for/parfor statement's iterator.
pub(crate) fn find_iterator_range(node: Node) -> Option<Node> {
    let iter = find_child_of_kind(node, "iterator")?;
    find_child_of_kind(iter, "range")
}

/// Get the last named child of a node (including `end`).
pub(crate) fn last_named_child(node: Node) -> Option<Node> {
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
pub(crate) fn extract_parfor_index_var(node: Node, source: &str) -> Option<String> {
    find_parfor_index_identifier(node).map(|n| node_text(n, source).to_string())
}

/// Find the parfor index variable identifier node.
pub(crate) fn find_parfor_index_identifier(node: Node) -> Option<Node> {
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
pub(crate) fn walk_body_for_function_calls<'a>(node: Node<'a>, out: &mut Vec<Node<'a>>) {
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
pub(crate) fn collect_nodes_of_kind<'a>(node: Node<'a>, kind: &str, out: &mut Vec<Node<'a>>) {
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
pub(crate) fn collect_parfor_nodes<'a>(root: Node<'a>, source: &str) -> Vec<Node<'a>> {
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
pub(crate) fn collect_global_persistent_vars(root: Node, source: &str) -> std::collections::HashSet<String> {
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
pub(crate) fn is_evalin_assignin_base(node: Node, source: &str) -> bool {
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
pub(crate) fn is_distributed_call(node: Node, source: &str) -> bool {
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
pub(crate) fn lhs_base_name(lhs: Node, source: &str) -> Option<String> {
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
pub(crate) fn collect_global_use_diagnostics<'a>(
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
pub(crate) fn collect_parfor_body_reads<'a>(
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
pub(crate) fn collect_simple_lhs_assignments<'a>(node: Node<'a>, source: &str) -> Vec<(String, Node<'a>)> {
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
pub(crate) fn is_assigned_in_body(node: Node, name: &str, source: &str) -> bool {
    collect_simple_lhs_assignments(node, source)
        .iter()
        .any(|(n, _)| n == name)
}

/// Check whether a variable is defined before a parfor loop starts.
///
/// Function inputs and global/persistent declarations always count as defined;
/// other definitions count if their byte range ends before the loop.
pub(crate) fn is_defined_before_parfor(sym: &SymbolTable, name: &str, parfor_start: usize) -> bool {
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
pub(crate) fn uses_after_parfor(
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
pub(crate) fn is_inside_loop(node: Node) -> bool {
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
pub(crate) fn enclosing_function(node: Node) -> Option<Node> {
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
pub(crate) fn count_format_specifiers(fmt: &str) -> usize {
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
pub(crate) fn collect_loop_assignments(
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
pub(crate) fn find_attribute_node<'a>(node: Node<'a>, name: &str, source: &'a str) -> Option<Node<'a>> {
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
pub(crate) fn block_diagnostic_position(
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
pub(crate) fn find_nth_properties_block<'a>(class_node: Node<'a>, index: usize) -> Option<Node<'a>> {
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
pub(crate) fn collect_property_event_calls(
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

#[cfg(test)]
use tree_sitter::Parser;

/// Parse MATLAB source and return the tree.
#[cfg(test)]
pub(crate) fn parse(source: &str) -> tree_sitter::Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_matlab::LANGUAGE.into())
        .expect("failed to load tree-sitter-matlab");
    parser.parse(source, None).expect("parse failed")
}

/// Create an engine with default config.
#[cfg(test)]
pub(crate) fn engine() -> GoodPracticesEngine {
    GoodPracticesEngine {
        config: GoodPracticesConfig::default(),
    }
}

/// Recursively find the first descendant of a node with the given kind.
#[cfg(test)]
pub(crate) fn find_descendant_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
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

/// Find all descendant nodes of the given kind.
#[cfg(test)]
pub(crate) fn all_descendants_of_kind<'a>(node: Node<'a>, kind: &str) -> Vec<Node<'a>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The twelve OOP/class/property check IDs implemented by GOODPRAC G1.
    const OOP_CHECK_IDS: &[&str] = &[
        "ATTF", "ATTOF", "MCPO", "MCSAC", "MOBSRV", "MDEPIN", "MCCPI", "MGMD", "MCCPE", "MTHANS",
        "MHERM", "MNUML",
    ];

    /// The 11 parfor/spmd check IDs implemented by this group.
    const G2_IDS: &[&str] = &[
        "PFEVB", "PFGP", "PFGV", "PFIIN", "PFOUS", "PFRNI", "PFTUSW", "PFUIXW", "SPEVB", "SPGV",
        "DSPMDA",
    ];

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
}
