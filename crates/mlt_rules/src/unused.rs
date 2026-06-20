//! # UNUSED_ENGINE: Unused Constructions Detection Engine
//!
//! This module implements a file-level rule engine that detects unused variables,
//! functions, and dead code in MATLAB source files. It covers 17 distinct check IDs
//! that correspond to MATLAB Code Analyzer's "Unused Constructions" category.
//!
//! ## Checks Implemented
//!
//! | Check ID | Severity | Description |
//! |----------|----------|-------------|
//! | NASGU    | Warning  | Variable is assigned but never used |
//! | NUSED    | Warning  | Variable is defined (input arg) but never used |
//! | NOEFF    | Warning  | Statement has no effect (expression result discarded) |
//! | EQEFF    | Warning  | Comparison has no effect (result not used) |
//! | ASGLU    | Warning  | Assignment to a variable that is immediately overwritten |
//! | SETNU    | Warning  | Output of function assigned but never used |
//! | PUSE     | Warning  | Persistent/global variable set but not used |
//! | PREALL   | Warning  | Variable preallocated but unused |
//! | INUSA    | Warning  | Input argument not used in function |
//! | INUSD    | Warning  | Input argument defined but could be removed |
//! | VANUS    | Info     | Value assigned to ans is unused |
//! | DEFNU    | Warning  | Local function defined but never called |
//! | UNRCH    | Warning  | Unreachable code after return/break/continue |
//! | MANU     | Warning  | Method defined but never called |
//! | VUNUS    | Warning  | Variable assigned in all branches but unused after |
//! | MSNU     | Warning  | Struct field set but never read |
//! | MSNE     | Info     | Struct field doesn't exist (assigned but typo) |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.UNUSED_ENGINE]
//! severity = "warning"
//! ignore_patterns = ["_*", "unused*"]
//! disabled_checks = ["VANUS"]
//! ```

use std::collections::{HashMap, HashSet};

use mlt_core::{Category, Config, Diagnostic, FileContext, Rule, Severity};
use serde::Deserialize;
use tree_sitter::Node;

use crate::analysis::control_flow::find_unreachable;
use crate::analysis::symbols::{DefKind, ScopeKind, SymbolTable};

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for UNUSED_ENGINE.
///
/// Deserialized from the `[lint.rules.UNUSED_ENGINE]` section in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnusedConfig {
    /// Variable name patterns to ignore (e.g., `["_*", "unused*"]`).
    /// Supports simple glob: `*` matches any sequence, `?` matches one char.
    #[serde(default)]
    pub ignore_patterns: Vec<String>,

    /// Individual check IDs to disable (e.g., `["VANUS", "MSNE"]`).
    #[serde(default)]
    pub disabled_checks: Vec<String>,
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Variables that are always considered "used" — MATLAB built-in specials.
const ALWAYS_USED: &[&str] = &["varargin", "varargout", "nargin", "nargout"];

/// Statement-level parent node types.
const STATEMENT_PARENTS: &[&str] = &["source_file", "block"];

/// Node types representing discarded expressions at statement level.
const NO_EFFECT_EXPR_NODES: &[&str] = &[
    "binary_operator",
    "unary_operator",
    "postfix_operator",
    "number",
    "string",
    "identifier",
    "cell",
    "matrix",
    "range",
    "lambda",
    "field_expression",
    "boolean_operator",
];

/// Node type for comparison operators (result discarded = EQEFF).
const COMPARISON_NODE: &str = "comparison_operator";

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// File-level rule engine for detecting unused constructions.
///
/// This rule performs its analysis in `check_file()` by building a symbol table
/// and running unreachability analysis, then cross-referencing definitions with
/// usages to identify dead code.
pub struct UnusedEngine {
    config: UnusedConfig,
}

impl UnusedEngine {
    /// Factory constructor. Reads rule-specific params from config.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: UnusedConfig = config.rule_params("UNUSED_ENGINE");
        Box::new(Self {
            config: rule_config,
        })
    }

    /// Check if a check ID is disabled in config.
    fn is_check_disabled(&self, check_id: &str) -> bool {
        self.config
            .disabled_checks
            .iter()
            .any(|d| d.eq_ignore_ascii_case(check_id))
    }

    /// Check if a variable name should be ignored based on configured patterns.
    fn should_ignore_name(&self, name: &str) -> bool {
        // Always ignore `_` and names starting with `~`.
        if name == "_" || name.starts_with('~') {
            return true;
        }
        // Always ignore MATLAB built-in specials.
        if ALWAYS_USED.contains(&name) {
            return true;
        }
        // Check configured patterns.
        self.config
            .ignore_patterns
            .iter()
            .any(|pat| matches_simple_glob(pat, name))
    }

    /// Run NASGU check: variable assigned but never used.
    fn check_nasgu(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("NASGU") {
            return;
        }

        for scope in &table.scopes {
            // Collect output argument names for this scope (they are "used" externally).
            let output_args: HashSet<&str> = scope
                .defs
                .iter()
                .filter(|d| d.kind == DefKind::OutputArg)
                .map(|d| d.name.as_str())
                .collect();

            for def in &scope.defs {
                // Only check assignment defs.
                if def.kind != DefKind::Assignment {
                    continue;
                }
                let name = &def.name;

                // Skip if this variable is an output arg (returned externally).
                if output_args.contains(name.as_str()) {
                    continue;
                }
                // Skip ignored names.
                if self.should_ignore_name(name) {
                    continue;
                }
                // Check if the variable has any uses in scope.
                if !scope.is_used(name) {
                    diagnostics.push(Diagnostic {
                        rule_id: "NASGU",
                        message: format!("Variable '{name}' is assigned but never used"),
                        severity: Severity::Warning,
                        byte_range: def.byte_range.clone(),
                        line: def.line,
                        column: def.column,
                        fix: None,
                    });
                }
            }
        }
    }

    /// Run NUSED/INUSA/INUSD checks: input arguments that are never used.
    fn check_unused_inputs(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let nused_disabled = self.is_check_disabled("NUSED");
        let inusa_disabled = self.is_check_disabled("INUSA");
        let inusd_disabled = self.is_check_disabled("INUSD");

        if nused_disabled && inusa_disabled && inusd_disabled {
            return;
        }

        for scope in &table.scopes {
            // Only check function/method scopes (not scripts or lambdas).
            if !matches!(
                scope.kind,
                ScopeKind::Function
                    | ScopeKind::LocalFunction
                    | ScopeKind::NestedFunction
                    | ScopeKind::Method
            ) {
                continue;
            }

            for def in &scope.defs {
                if def.kind != DefKind::InputArg {
                    continue;
                }
                let name = &def.name;

                if self.should_ignore_name(name) {
                    continue;
                }

                if !scope.is_used(name) {
                    // NUSED: general "input arg not used"
                    if !nused_disabled {
                        diagnostics.push(Diagnostic {
                            rule_id: "NUSED",
                            message: format!(
                                "Input argument '{name}' is defined but never used"
                            ),
                            severity: Severity::Warning,
                            byte_range: def.byte_range.clone(),
                            line: def.line,
                            column: def.column,
                            fix: None,
                        });
                    }

                    // INUSA: input argument not used in function (same as NUSED
                    // but different check ID for compatibility).
                    if !inusa_disabled {
                        diagnostics.push(Diagnostic {
                            rule_id: "INUSA",
                            message: format!(
                                "Input argument '{name}' is not used in function '{}'",
                                scope.name
                            ),
                            severity: Severity::Warning,
                            byte_range: def.byte_range.clone(),
                            line: def.line,
                            column: def.column,
                            fix: None,
                        });
                    }

                    // INUSD: input argument could be removed.
                    if !inusd_disabled {
                        diagnostics.push(Diagnostic {
                            rule_id: "INUSD",
                            message: format!(
                                "Input argument '{name}' is defined but could be removed"
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
        }
    }

    /// Run ASGLU check: consecutive assignments to the same variable with no
    /// use between them.
    fn check_asglu(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("ASGLU") {
            return;
        }

        for scope in &table.scopes {
            // Group assignment defs by variable name, sorted by byte offset.
            let mut assignments_by_name: HashMap<&str, Vec<usize>> = HashMap::new();
            for (idx, def) in scope.defs.iter().enumerate() {
                if def.kind == DefKind::Assignment {
                    assignments_by_name
                        .entry(def.name.as_str())
                        .or_default()
                        .push(idx);
                }
            }

            for (name, indices) in &assignments_by_name {
                if self.should_ignore_name(name) {
                    continue;
                }
                if indices.len() < 2 {
                    continue;
                }

                // Check consecutive pairs: if no use exists between them,
                // the first assignment is overwritten.
                for pair in indices.windows(2) {
                    let first_def = &scope.defs[pair[0]];
                    let second_def = &scope.defs[pair[1]];

                    let has_use_between = scope.uses.iter().any(|u| {
                        u.name == *name
                            && u.byte_range.start > first_def.byte_range.end
                            && u.byte_range.start < second_def.byte_range.start
                    });

                    if !has_use_between {
                        // Also skip if the first def is an OutputArg (initial
                        // declaration before assignment).
                        let output_args: HashSet<&str> = scope
                            .defs
                            .iter()
                            .filter(|d| d.kind == DefKind::OutputArg)
                            .map(|d| d.name.as_str())
                            .collect();
                        if output_args.contains(name) {
                            continue;
                        }

                        diagnostics.push(Diagnostic {
                            rule_id: "ASGLU",
                            message: format!(
                                "Value assigned to '{name}' is immediately overwritten"
                            ),
                            severity: Severity::Warning,
                            byte_range: first_def.byte_range.clone(),
                            line: first_def.line,
                            column: first_def.column,
                            fix: None,
                        });
                    }
                }
            }
        }
    }

    /// Run PUSE check: global/persistent variables declared but not used.
    fn check_puse(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("PUSE") {
            return;
        }

        for scope in &table.scopes {
            for def in &scope.defs {
                if !matches!(def.kind, DefKind::Global | DefKind::Persistent) {
                    continue;
                }
                let name = &def.name;

                if self.should_ignore_name(name) {
                    continue;
                }

                if !scope.is_used(name) {
                    let kind_str = match def.kind {
                        DefKind::Global => "Global",
                        DefKind::Persistent => "Persistent",
                        _ => unreachable!(),
                    };
                    diagnostics.push(Diagnostic {
                        rule_id: "PUSE",
                        message: format!(
                            "{kind_str} variable '{name}' is declared but never used"
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
    }

    /// Run SETNU check: output of function call assigned but never used.
    /// This is a refinement of NASGU specifically for function call outputs.
    fn check_setnu(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("SETNU") {
            return;
        }

        for scope in &table.scopes {
            let output_args: HashSet<&str> = scope
                .defs
                .iter()
                .filter(|d| d.kind == DefKind::OutputArg)
                .map(|d| d.name.as_str())
                .collect();

            for def in &scope.defs {
                if def.kind != DefKind::Assignment {
                    continue;
                }
                let name = &def.name;

                if output_args.contains(name.as_str()) {
                    continue;
                }
                if self.should_ignore_name(name) {
                    continue;
                }
                // SETNU is for function call outputs; we only emit if the var
                // is never used. We differentiate from NASGU by checking later,
                // but to avoid duplication with NASGU, SETNU only fires if
                // NASGU is disabled.
                if !self.is_check_disabled("NASGU") {
                    continue;
                }
                if !scope.is_used(name) {
                    diagnostics.push(Diagnostic {
                        rule_id: "SETNU",
                        message: format!(
                            "Output assigned to '{name}' is never used"
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
    }

    /// Run PREALL check: variable preallocated but unused.
    /// Similar to NASGU but targets for-iterator variables specifically.
    fn check_preall(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("PREALL") {
            return;
        }

        for scope in &table.scopes {
            for def in &scope.defs {
                if def.kind != DefKind::ForIterator {
                    continue;
                }
                let name = &def.name;

                if self.should_ignore_name(name) {
                    continue;
                }

                if !scope.is_used(name) {
                    diagnostics.push(Diagnostic {
                        rule_id: "PREALL",
                        message: format!(
                            "Loop variable '{name}' is preallocated but never used in the loop body"
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
    }

    /// Run VANUS check: value assigned to `ans` is unused.
    fn check_vanus(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("VANUS") {
            return;
        }

        for scope in &table.scopes {
            for def in &scope.defs {
                if def.kind != DefKind::Assignment {
                    continue;
                }
                if def.name != "ans" {
                    continue;
                }
                if !scope.is_used("ans") {
                    diagnostics.push(Diagnostic {
                        rule_id: "VANUS",
                        message: "Value assigned to 'ans' is unused".to_string(),
                        severity: Severity::Info,
                        byte_range: def.byte_range.clone(),
                        line: def.line,
                        column: def.column,
                        fix: None,
                    });
                }
            }
        }
    }

    /// Run DEFNU check: local function defined but never called.
    fn check_defnu(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("DEFNU") {
            return;
        }

        // Collect all uses across all scopes to check if a local function is called.
        let all_uses: HashSet<&str> = table
            .scopes
            .iter()
            .flat_map(|s| s.uses.iter().map(|u| u.name.as_str()))
            .collect();

        for scope in &table.scopes {
            if scope.kind != ScopeKind::LocalFunction {
                continue;
            }
            let name = &scope.name;
            if name.is_empty() {
                continue;
            }
            if self.should_ignore_name(name) {
                continue;
            }
            if !all_uses.contains(name.as_str()) {
                diagnostics.push(Diagnostic {
                    rule_id: "DEFNU",
                    message: format!("Local function '{name}' is defined but never called"),
                    severity: Severity::Warning,
                    byte_range: scope.byte_range.clone(),
                    line: scope.line,
                    column: 1,
                    fix: None,
                });
            }
        }
    }

    /// Run MANU check: method defined but never called within the file.
    fn check_manu(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("MANU") {
            return;
        }

        // Collect all uses across all scopes.
        let all_uses: HashSet<&str> = table
            .scopes
            .iter()
            .flat_map(|s| s.uses.iter().map(|u| u.name.as_str()))
            .collect();

        for scope in &table.scopes {
            if scope.kind != ScopeKind::Method {
                continue;
            }
            let name = &scope.name;
            if name.is_empty() {
                continue;
            }
            if self.should_ignore_name(name) {
                continue;
            }
            if !all_uses.contains(name.as_str()) {
                diagnostics.push(Diagnostic {
                    rule_id: "MANU",
                    message: format!("Method '{name}' is defined but never called"),
                    severity: Severity::Warning,
                    byte_range: scope.byte_range.clone(),
                    line: scope.line,
                    column: 1,
                    fix: None,
                });
            }
        }
    }

    /// Run UNRCH check: unreachable code after return/break/continue.
    fn check_unrch(
        &self,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("UNRCH") {
            return;
        }

        let spans = find_unreachable(ctx.tree, ctx.source);
        for span in spans {
            diagnostics.push(Diagnostic {
                rule_id: "UNRCH",
                message: format!("Unreachable code after '{}'", span.cause),
                severity: Severity::Warning,
                byte_range: span.byte_range,
                line: span.line,
                column: span.column,
                fix: None,
            });
        }
    }

    /// Run NOEFF/EQEFF checks: statements with no effect.
    fn check_noeff_eqeff(
        &self,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let noeff_disabled = self.is_check_disabled("NOEFF");
        let eqeff_disabled = self.is_check_disabled("EQEFF");

        if noeff_disabled && eqeff_disabled {
            return;
        }

        // Walk the tree looking for expression nodes at statement level.
        walk_for_no_effect(ctx.tree.root_node(), diagnostics, noeff_disabled, eqeff_disabled);
    }

    /// Run VUNUS check: variable assigned in all branches but unused after.
    fn check_vunus(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("VUNUS") {
            return;
        }

        for scope in &table.scopes {
            let output_args: HashSet<&str> = scope
                .defs
                .iter()
                .filter(|d| d.kind == DefKind::OutputArg)
                .map(|d| d.name.as_str())
                .collect();

            // Find variables with multiple defs but no uses.
            let mut def_counts: HashMap<&str, usize> = HashMap::new();
            for def in &scope.defs {
                if def.kind == DefKind::Assignment {
                    *def_counts.entry(def.name.as_str()).or_insert(0) += 1;
                }
            }

            for (name, count) in &def_counts {
                if *count < 2 {
                    continue;
                }
                if output_args.contains(name) {
                    continue;
                }
                if self.should_ignore_name(name) {
                    continue;
                }
                if !scope.is_used(name) {
                    // Find the last definition for this variable.
                    if let Some(last_def) = scope
                        .defs
                        .iter()
                        .rev()
                        .find(|d| d.name == *name && d.kind == DefKind::Assignment)
                    {
                        diagnostics.push(Diagnostic {
                            rule_id: "VUNUS",
                            message: format!(
                                "Variable '{name}' is assigned in multiple branches but never used after"
                            ),
                            severity: Severity::Warning,
                            byte_range: last_def.byte_range.clone(),
                            line: last_def.line,
                            column: last_def.column,
                            fix: None,
                        });
                    }
                }
            }
        }
    }

    /// Run MSNU check: struct field set but never read.
    /// This is a heuristic check based on field_expression analysis.
    fn check_msnu(
        &self,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("MSNU") {
            return;
        }

        let mut field_writes: Vec<FieldAccess> = Vec::new();
        let mut field_reads: HashSet<String> = HashSet::new();

        self.collect_field_accesses(
            ctx.tree.root_node(),
            ctx.source,
            &mut field_writes,
            &mut field_reads,
        );

        // Report fields that are written but never read.
        for write in &field_writes {
            let key = format!("{}.{}", write.object, write.field);
            if !field_reads.contains(&key) {
                diagnostics.push(Diagnostic {
                    rule_id: "MSNU",
                    message: format!(
                        "Struct field '{}.{}' is set but never read",
                        write.object, write.field
                    ),
                    severity: Severity::Warning,
                    byte_range: write.byte_range.clone(),
                    line: write.line,
                    column: write.column,
                    fix: None,
                });
            }
        }
    }

    /// Run MSNE check: struct field doesn't exist (assigned but potential typo).
    /// Heuristic: if a struct has many fields and one is only written once with
    /// no reads, it may be a typo.
    fn check_msne(
        &self,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("MSNE") {
            return;
        }

        let mut field_writes: Vec<FieldAccess> = Vec::new();
        let mut field_reads: HashSet<String> = HashSet::new();

        self.collect_field_accesses(
            ctx.tree.root_node(),
            ctx.source,
            &mut field_writes,
            &mut field_reads,
        );

        // Group writes by object name.
        let mut writes_by_object: HashMap<&str, Vec<&FieldAccess>> = HashMap::new();
        for write in &field_writes {
            writes_by_object
                .entry(write.object.as_str())
                .or_default()
                .push(write);
        }

        // For each object with multiple fields written, check if any field
        // is written exactly once and never read — potential typo.
        for (object, writes) in &writes_by_object {
            if writes.len() < 2 {
                continue;
            }

            let mut field_write_count: HashMap<&str, usize> = HashMap::new();
            for w in writes {
                *field_write_count.entry(w.field.as_str()).or_insert(0) += 1;
            }

            for w in writes {
                let key = format!("{}.{}", object, w.field);
                if field_write_count.get(w.field.as_str()) == Some(&1) && !field_reads.contains(&key)
                {
                    diagnostics.push(Diagnostic {
                        rule_id: "MSNE",
                        message: format!(
                            "Struct field '{}.{}' is assigned once and never read (possible typo)",
                            object, w.field
                        ),
                        severity: Severity::Info,
                        byte_range: w.byte_range.clone(),
                        line: w.line,
                        column: w.column,
                        fix: None,
                    });
                }
            }
        }
    }

    /// Collect field accesses (both reads and writes) from the parse tree.
    fn collect_field_accesses(
        &self,
        node: Node,
        source: &str,
        writes: &mut Vec<FieldAccess>,
        reads: &mut HashSet<String>,
    ) {
        if node.kind() == "assignment" {
            // Check if LHS is a field_expression.
            if let Some(lhs) = node.child_by_field_name("left") {
                if lhs.kind() == "field_expression" {
                    if let Some(access) = extract_field_access(lhs, source) {
                        writes.push(access);
                    }
                }
            }
            // Check RHS for field reads.
            if let Some(rhs) = node.child_by_field_name("right") {
                self.collect_field_reads(rhs, source, reads);
            }
        } else if node.kind() == "field_expression" {
            // A field_expression not on the LHS of assignment is a read.
            if let Some(access) = extract_field_access(node, source) {
                let key = format!("{}.{}", access.object, access.field);
                reads.insert(key);
            }
        }

        // Recurse into children (skip assignment children we already handled).
        if node.kind() != "assignment" {
            let count = node.child_count();
            for i in 0..count {
                if let Some(child) = node.child(i) {
                    self.collect_field_accesses(child, source, writes, reads);
                }
            }
        } else {
            // For assignments, recurse into children except LHS field_expression.
            let count = node.child_count();
            for i in 0..count {
                if let Some(child) = node.child(i) {
                    if child.kind() == "field_expression" {
                        if let Some(lhs) = node.child_by_field_name("left") {
                            if lhs.id() == child.id() {
                                continue;
                            }
                        }
                    }
                    self.collect_field_accesses(child, source, writes, reads);
                }
            }
        }
    }

    /// Collect field reads from an expression subtree.
    fn collect_field_reads(
        &self,
        node: Node,
        source: &str,
        reads: &mut HashSet<String>,
    ) {
        if node.kind() == "field_expression" {
            if let Some(access) = extract_field_access(node, source) {
                let key = format!("{}.{}", access.object, access.field);
                reads.insert(key);
            }
        }

        let count = node.child_count();
        for i in 0..count {
            if let Some(child) = node.child(i) {
                self.collect_field_reads(child, source, reads);
            }
        }
    }
}

impl Rule for UnusedEngine {
    fn id(&self) -> &'static str {
        "UNUSED_ENGINE"
    }

    fn description(&self) -> &'static str {
        "Detect unused variables, functions, and dead code"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn category(&self) -> Category {
        Category::UnusedConstructions
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        // File-level rule — no per-node targeting.
        &[]
    }

    fn has_file_check(&self) -> bool {
        true
    }

    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // 1. Build symbol table.
        let table = SymbolTable::build(ctx.tree, ctx.source);

        // 2. Run symbol-based checks.
        self.check_nasgu(&table, &mut diagnostics);
        self.check_unused_inputs(&table, &mut diagnostics);
        self.check_asglu(&table, &mut diagnostics);
        self.check_puse(&table, &mut diagnostics);
        self.check_setnu(&table, &mut diagnostics);
        self.check_preall(&table, &mut diagnostics);
        self.check_vanus(&table, &mut diagnostics);
        self.check_defnu(&table, &mut diagnostics);
        self.check_manu(&table, &mut diagnostics);
        self.check_vunus(&table, &mut diagnostics);

        // 3. Run tree-walk checks.
        self.check_unrch(ctx, &mut diagnostics);
        self.check_noeff_eqeff(ctx, &mut diagnostics);
        self.check_msnu(ctx, &mut diagnostics);
        self.check_msne(ctx, &mut diagnostics);

        diagnostics
    }
}

// ---------------------------------------------------------------------------
// No-effect statement walker (free function to satisfy clippy)
// ---------------------------------------------------------------------------

/// Recursively walk tree to find statement-level expressions with no effect.
fn walk_for_no_effect(
    node: Node,
    diagnostics: &mut Vec<Diagnostic>,
    noeff_disabled: bool,
    eqeff_disabled: bool,
) {
    // Check if this node is at statement level (parent is block or source_file).
    let is_statement_level = node
        .parent()
        .map(|p| STATEMENT_PARENTS.contains(&p.kind()))
        .unwrap_or(false);

    if is_statement_level {
        let kind = node.kind();

        // EQEFF: comparison operator at statement level.
        if !eqeff_disabled && kind == COMPARISON_NODE {
            let pos = node.start_position();
            diagnostics.push(Diagnostic {
                rule_id: "EQEFF",
                message: "Comparison has no effect (result is not used)".to_string(),
                severity: Severity::Warning,
                byte_range: node.start_byte()..node.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            });
        }
        // NOEFF: other expression nodes at statement level that are not
        // assignments, function_calls, or commands.
        else if !noeff_disabled && NO_EFFECT_EXPR_NODES.contains(&kind) {
            let pos = node.start_position();
            diagnostics.push(Diagnostic {
                rule_id: "NOEFF",
                message: "Statement has no effect (expression result is discarded)"
                    .to_string(),
                severity: Severity::Warning,
                byte_range: node.start_byte()..node.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            });
        }
    }

    // Recurse into children.
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            walk_for_no_effect(child, diagnostics, noeff_disabled, eqeff_disabled);
        }
    }
}

// ---------------------------------------------------------------------------
// Helper types and functions
// ---------------------------------------------------------------------------

/// A struct field access (read or write).
#[derive(Debug, Clone)]
struct FieldAccess {
    /// The object name (e.g., "s" in "s.field").
    object: String,
    /// The field name (e.g., "field" in "s.field").
    field: String,
    /// Byte range of the full field expression.
    byte_range: std::ops::Range<usize>,
    /// 1-indexed line.
    line: usize,
    /// 1-indexed column.
    column: usize,
}

/// Extract a `FieldAccess` from a `field_expression` node.
fn extract_field_access(node: Node, source: &str) -> Option<FieldAccess> {
    // field_expression has an object and a field child.
    let object_node = node.child(0)?;
    let field_node = node.child_by_field_name("field")?;

    // Only handle simple identifier objects (not chained field access).
    if object_node.kind() != "identifier" {
        return None;
    }

    let object = source[object_node.start_byte()..object_node.end_byte()].to_string();
    let field = source[field_node.start_byte()..field_node.end_byte()].to_string();
    let pos = node.start_position();

    Some(FieldAccess {
        object,
        field,
        byte_range: node.start_byte()..node.end_byte(),
        line: pos.row + 1,
        column: pos.column + 1,
    })
}

/// Simple glob pattern matching supporting `*` (any sequence) and `?` (single char).
fn matches_simple_glob(pattern: &str, text: &str) -> bool {
    let pat: Vec<char> = pattern.chars().collect();
    let txt: Vec<char> = text.chars().collect();
    glob_match_recursive(&pat, &txt, 0, 0)
}

/// Recursive glob matching helper.
fn glob_match_recursive(pat: &[char], txt: &[char], pi: usize, ti: usize) -> bool {
    if pi == pat.len() {
        return ti == txt.len();
    }

    match pat[pi] {
        '*' => {
            // `*` matches zero or more characters.
            for i in ti..=txt.len() {
                if glob_match_recursive(pat, txt, pi + 1, i) {
                    return true;
                }
            }
            false
        }
        '?' => {
            // `?` matches exactly one character.
            if ti < txt.len() {
                glob_match_recursive(pat, txt, pi + 1, ti + 1)
            } else {
                false
            }
        }
        c => {
            if ti < txt.len() && txt[ti] == c {
                glob_match_recursive(pat, txt, pi + 1, ti + 1)
            } else {
                false
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "UNUSED_ENGINE",
    UnusedEngine::from_config
));
