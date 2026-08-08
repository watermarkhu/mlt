//! # Formatting Suggestions Engine
//!
//! This module implements 7 formatting checks from MATLAB's Code Analyzer as
//! a single file-level rule engine. The checks are:
//!
//! | Check ID | Description |
//! |----------|-------------|
//! | NOCOMMA  | Use commas to separate elements in a row |
//! | NO4LP    | Use 4-space indentation in loop/conditional bodies |
//! | ALIGN    | Code alignment suggestion (elseif/else vs. if) |
//! | NOPTS    | Add parentheses around condition in if/while |
//! | NOPRT    | Remove unnecessary parentheses |
//! | PRTCAL   | Consider using command syntax instead of function syntax |
//! | NCOMMA   | Use comma to separate input arguments |
//!
//! ## Architecture
//!
//! A single `FormattingEngine` rule instance:
//! - Registers once with inventory as `"FORMATTING_ENGINE"`
//! - Uses `has_file_check() = true` for full-tree traversal
//! - Walks the tree once, running all applicable checks per node
//! - Emits diagnostics with the specific check ID (e.g., `"NOCOMMA"`)
//!
//! ## Configuration
//!
//! Each check can be configured individually:
//!
//! ```toml
//! [lint.rules.NOCOMMA]
//! severity = "info"
//!
//! [lint.rules.NO4LP]
//! severity = "info"
//! indent_size = 4
//!
//! [lint.rules.PRTCAL]
//! severity = "off"
//! ```

use mlt_core::{Category, Config, Diagnostic, FileContext, Fix, Rule, Severity};
use serde::Deserialize;
use tree_sitter::Node;

// ---------------------------------------------------------------------------
// Check IDs
// ---------------------------------------------------------------------------

/// All formatting check IDs handled by this engine.
const CHECK_IDS: &[&str] = &[
    "NOCOMMA", "NO4LP", "ALIGN", "NOPTS", "NOPRT", "PRTCAL", "NCOMMA",
];

// ---------------------------------------------------------------------------
// Per-check configuration
// ---------------------------------------------------------------------------

/// Configuration for the NO4LP indentation check.
#[derive(Debug, Clone, Deserialize)]
pub struct No4lpConfig {
    /// Expected indentation size in spaces.
    #[serde(default = "default_indent_size")]
    pub indent_size: usize,
}

impl Default for No4lpConfig {
    fn default() -> Self {
        Self {
            indent_size: default_indent_size(),
        }
    }
}

/// Default indentation size (4 spaces).
fn default_indent_size() -> usize {
    4
}

/// Parent node types that indicate statement-level context.
const STATEMENT_PARENTS: &[&str] = &["source_file", "block"];

// ---------------------------------------------------------------------------
// FormattingEngine rule
// ---------------------------------------------------------------------------

/// Tracks which individual checks are enabled.
struct EnabledChecks {
    nocomma: bool,
    no4lp: bool,
    align: bool,
    nopts: bool,
    noprt: bool,
    prtcal: bool,
    ncomma: bool,
}

/// The formatting suggestions engine that handles all 7 formatting checks.
///
/// Registered as `"FORMATTING_ENGINE"` with inventory. Uses file-level
/// analysis to walk the tree once and apply all enabled formatting checks.
pub struct FormattingEngine {
    enabled: EnabledChecks,
    no4lp_config: No4lpConfig,
}

impl FormattingEngine {
    /// Factory constructor invoked by inventory.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let enabled = EnabledChecks {
            nocomma: config.is_rule_enabled("NOCOMMA"),
            no4lp: config.is_rule_enabled("NO4LP"),
            align: config.is_rule_enabled("ALIGN"),
            nopts: config.is_rule_enabled("NOPTS"),
            noprt: config.is_rule_enabled("NOPRT"),
            prtcal: config.is_rule_enabled("PRTCAL"),
            ncomma: config.is_rule_enabled("NCOMMA"),
        };
        let no4lp_config: No4lpConfig = config.rule_params("NO4LP");
        Box::new(Self {
            enabled,
            no4lp_config,
        })
    }

    /// Returns true if any sub-check is enabled.
    fn any_enabled(&self) -> bool {
        self.enabled.nocomma
            || self.enabled.no4lp
            || self.enabled.align
            || self.enabled.nopts
            || self.enabled.noprt
            || self.enabled.prtcal
            || self.enabled.ncomma
    }
}

impl Rule for FormattingEngine {
    fn id(&self) -> &'static str {
        "FORMATTING_ENGINE"
    }

    fn description(&self) -> &'static str {
        "Formatting suggestion checks (NOCOMMA, NO4LP, ALIGN, NOPTS, NOPRT, PRTCAL, NCOMMA)"
    }

    fn severity(&self) -> Severity {
        Severity::Info
    }

    fn category(&self) -> Category {
        Category::Formatting
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        // File-level only — no node subscriptions.
        &[]
    }

    fn can_be_disabled(&self) -> bool {
        true
    }

    fn has_file_check(&self) -> bool {
        true
    }

    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> {
        if !self.any_enabled() {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        let root = ctx.tree.root_node();
        self.visit_recursive(root, ctx.source, &mut diagnostics);
        diagnostics
    }
}

// ---------------------------------------------------------------------------
// Recursive tree traversal
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// Walk the tree recursively, applying enabled checks to each node.
    fn visit_recursive(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let kind = node.kind();
        // Note: multiple checks can apply to the same node type, so dispatch
        // each independently rather than using mutually-exclusive match arms.

        // NOCOMMA: matrix/cell rows with space-separated elements
        if (kind == "matrix" || kind == "cell") && self.enabled.nocomma {
            self.check_nocomma(node, source, diagnostics);
        }

        // NO4LP: indentation in loop/conditional bodies
        if matches!(
            kind,
            "for_statement" | "while_statement" | "if_statement" | "switch_statement"
        ) && self.enabled.no4lp
        {
            self.check_no4lp(node, source, diagnostics);
        }

        // ALIGN: elseif/else alignment with parent if
        if kind == "if_statement" && self.enabled.align {
            self.check_align(node, source, diagnostics);
        }

        // NOPTS: unnecessary parentheses around condition
        if matches!(kind, "if_statement" | "while_statement") && self.enabled.nopts {
            self.check_nopts(node, source, diagnostics);
        }

        // NOPRT: unnecessary parentheses around simple expressions
        if kind == "parenthesis" && self.enabled.noprt {
            self.check_noprt(node, source, diagnostics);
        }

        // PRTCAL: function syntax that could be command syntax
        if kind == "function_call" && self.enabled.prtcal {
            self.check_prtcal(node, source, diagnostics);
        }

        // NCOMMA: function call args separated by spaces
        if kind == "function_call" && self.enabled.ncomma {
            self.check_ncomma(node, source, diagnostics);
        }

        // Recurse into children.
        let child_count = node.child_count();
        for i in 0..child_count {
            if let Some(child) = node.child(i) {
                self.visit_recursive(child, source, diagnostics);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// NOCOMMA: Use commas to separate elements in a row
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// In `matrix` or `cell` nodes, check if `row` children have elements
    /// separated by spaces instead of commas. Look for adjacent expression
    /// nodes without a `,` between them.
    fn check_nocomma(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let child_count = node.child_count();
        for i in 0..child_count {
            let Some(child) = node.child(i) else {
                continue;
            };
            if child.kind() == "row" {
                self.check_row_commas(child, source, diagnostics);
            }
        }
    }

    /// Check a single `row` node for missing commas between elements.
    fn check_row_commas(
        &self,
        row: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // Collect expression children (skip punctuation like "[", "]", ",", ";").
        let mut prev_expr: Option<Node> = None;
        let child_count = row.child_count();

        for i in 0..child_count {
            let Some(child) = row.child(i) else {
                continue;
            };
            let kind = child.kind();

            // Reset tracking on comma or semicolon. A zero-width comma token
            // is inserted by the grammar when an element separator is missing.
            if kind == "," {
                if child.start_byte() == child.end_byte() {
                    let pos = child.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "NOCOMMA",
                        message: "Use commas to separate elements in a row".to_string(),
                        severity: Severity::Info,
                        byte_range: child.start_byte()..child.start_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: Some(Fix::insert(child.start_byte(), ", ")),
                    });
                }
                prev_expr = None;
                continue;
            }
            if kind == ";" {
                prev_expr = None;
                continue;
            }

            // Skip non-expression tokens (brackets, whitespace nodes).
            if is_punctuation(kind) {
                continue;
            }

            // If we have a previous expression and no comma was found between,
            // check whether there was only whitespace separating them.
            if let Some(prev) = prev_expr {
                let gap = &source[prev.end_byte()..child.start_byte()];
                if !gap.is_empty() && gap.chars().all(|c| c == ' ' || c == '\t') {
                    let pos = child.start_position();
                    let insert_pos = prev.end_byte();
                    diagnostics.push(Diagnostic {
                        rule_id: "NOCOMMA",
                        message: "Use commas to separate elements in a row".to_string(),
                        severity: Severity::Info,
                        byte_range: prev.end_byte()..child.start_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: Some(Fix::new(insert_pos..insert_pos + 1, ", ")),
                    });
                }
            }
            prev_expr = Some(child);
        }
    }
}

// ---------------------------------------------------------------------------
// NO4LP: Use 4-space indentation in loop/conditional bodies
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// For `for_statement`, `while_statement`, `if_statement`, and
    /// `switch_statement`, check that the `block` body is indented by
    /// `indent_size` spaces relative to the parent keyword.
    fn check_no4lp(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let parent_col = node.start_position().column;
        let expected_indent = parent_col + self.no4lp_config.indent_size;

        // Find all `block` children (there may be multiple in if/switch).
        let child_count = node.child_count();
        for i in 0..child_count {
            let Some(child) = node.child(i) else {
                continue;
            };
            match child.kind() {
                "block" => {
                    self.check_block_indentation(child, expected_indent, source, diagnostics);
                }
                // elseif_clause and else_clause also contain blocks.
                "elseif_clause" | "else_clause" | "case_clause" | "otherwise_clause" => {
                    let inner_count = child.child_count();
                    for j in 0..inner_count {
                        if let Some(inner) = child.child(j) {
                            if inner.kind() == "block" {
                                self.check_block_indentation(
                                    inner,
                                    expected_indent,
                                    source,
                                    diagnostics,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Check that each statement in a block starts at the expected column.
    fn check_block_indentation(
        &self,
        block: Node,
        expected_indent: usize,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let child_count = block.child_count();
        for i in 0..child_count {
            let Some(stmt) = block.child(i) else {
                continue;
            };
            // Skip non-statement nodes (e.g., semicolons, newlines).
            if is_punctuation(stmt.kind()) || stmt.kind() == "comment" {
                continue;
            }

            let actual_col = stmt.start_position().column;
            if actual_col != expected_indent {
                // Only flag if the line starts with spaces (not tabs or mixed).
                let line_start = line_start_byte(source, stmt.start_byte());
                let prefix = &source[line_start..stmt.start_byte()];
                if prefix.chars().all(|c| c == ' ') {
                    let pos = stmt.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "NO4LP",
                        message: format!(
                            "Use {}-space indentation (expected column {}, found {})",
                            self.no4lp_config.indent_size, expected_indent, actual_col
                        ),
                        severity: Severity::Info,
                        byte_range: line_start..stmt.start_byte(),
                        line: pos.row + 1,
                        column: 1,
                        fix: Some(Fix::new(
                            line_start..stmt.start_byte(),
                            " ".repeat(expected_indent),
                        )),
                    });
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ALIGN: elseif/else alignment with if
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// Check that `elseif_clause` and `else_clause` are aligned with their
    /// `if_statement` parent.
    fn check_align(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let if_col = node.start_position().column;
        let child_count = node.child_count();

        for i in 0..child_count {
            let Some(child) = node.child(i) else {
                continue;
            };
            let kind = child.kind();
            if kind != "elseif_clause" && kind != "else_clause" {
                continue;
            }

            let clause_col = child.start_position().column;
            if clause_col != if_col {
                let pos = child.start_position();
                let line_start = line_start_byte(source, child.start_byte());
                diagnostics.push(Diagnostic {
                    rule_id: "ALIGN",
                    message: format!(
                        "{} should be aligned with 'if' at column {} (found column {})",
                        keyword_for_clause(kind),
                        if_col + 1,
                        clause_col + 1
                    ),
                    severity: Severity::Info,
                    byte_range: line_start..child.start_byte(),
                    line: pos.row + 1,
                    column: clause_col + 1,
                    fix: Some(Fix::new(
                        line_start..child.start_byte(),
                        " ".repeat(if_col),
                    )),
                });
            }
        }
    }
}

// ---------------------------------------------------------------------------
// NOPTS: Add parentheses around condition in if/while
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// For `if_statement` and `while_statement`, check if the condition
    /// expression is wrapped in a `parenthesis` node. If so, suggest
    /// removing the outer parens since MATLAB does not require them.
    fn check_nopts(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let condition = match node.kind() {
            "if_statement" | "while_statement" => node.child_by_field_name("condition"),
            _ => None,
        };

        let Some(cond) = condition else {
            return;
        };

        if cond.kind() == "parenthesis" {
            let pos = cond.start_position();
            // The fix removes the outer parentheses, keeping the inner content.
            let inner_text = inner_paren_text(cond, source);
            diagnostics.push(Diagnostic {
                rule_id: "NOPTS",
                message: "Unnecessary parentheses around condition in if/while statement"
                    .to_string(),
                severity: Severity::Info,
                byte_range: cond.start_byte()..cond.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: Some(Fix::new(
                    cond.start_byte()..cond.end_byte(),
                    inner_text,
                )),
            });
        }
    }
}

// ---------------------------------------------------------------------------
// NOPRT: Remove unnecessary parentheses
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// Detect `parenthesis` nodes that are unnecessary — e.g., `(x)` where
    /// x is a simple identifier or number, and the parenthesis is not a
    /// function call argument or condition.
    fn check_noprt(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // Only flag parenthesized simple expressions (identifier, number, string).
        let inner = find_inner_expr(node);
        let Some(inner_node) = inner else {
            return;
        };

        let inner_kind = inner_node.kind();
        if inner_kind != "identifier" && inner_kind != "number" && inner_kind != "string" {
            return;
        }

        // Don't flag if the parent is a function_call (arguments), or if it is
        // the condition of an if/while (that's NOPTS territory).
        if let Some(parent) = node.parent() {
            let pk = parent.kind();
            if pk == "function_call" || pk == "arguments" {
                return;
            }
            // Skip if this is a condition node (handled by NOPTS).
            if (pk == "if_statement" || pk == "while_statement")
                && parent
                    .child_by_field_name("condition")
                    .map(|c| c.id() == node.id())
                    .unwrap_or(false)
            {
                return;
            }
        }

        let pos = node.start_position();
        let inner_text = &source[inner_node.start_byte()..inner_node.end_byte()];
        diagnostics.push(Diagnostic {
            rule_id: "NOPRT",
            message: format!(
                "Unnecessary parentheses around '{}'",
                inner_text
            ),
            severity: Severity::Info,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: Some(Fix::new(
                node.start_byte()..node.end_byte(),
                inner_text.to_string(),
            )),
        });
    }
}

// ---------------------------------------------------------------------------
// PRTCAL: Consider using command syntax
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// For `function_call` nodes at statement level with only string literal
    /// arguments, suggest command syntax instead. For example:
    /// `disp('hello')` could be `disp hello`.
    fn check_prtcal(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // Only fire at statement level.
        let is_statement_level = node
            .parent()
            .map(|p| STATEMENT_PARENTS.contains(&p.kind()))
            .unwrap_or(false);
        if !is_statement_level {
            return;
        }

        // Must have an arguments node with only string/identifier children.
        let Some(args_node) = find_arguments_child(node) else {
            return;
        };

        let mut arg_texts = Vec::new();
        let mut all_strings = true;
        let arg_count = args_node.child_count();
        let mut expr_count = 0;

        for i in 0..arg_count {
            let Some(arg) = args_node.child(i) else {
                continue;
            };
            let kind = arg.kind();
            // Skip punctuation (parens, commas).
            if is_punctuation(kind) || kind == "," {
                continue;
            }
            expr_count += 1;
            if kind == "string" {
                // Extract string content without quotes.
                let text = &source[arg.start_byte()..arg.end_byte()];
                let unquoted = text
                    .trim_start_matches('\'')
                    .trim_end_matches('\'')
                    .trim_start_matches('"')
                    .trim_end_matches('"');
                // Command syntax only works for simple strings without spaces.
                if unquoted.contains(' ') || unquoted.contains('\'') {
                    all_strings = false;
                    break;
                }
                arg_texts.push(unquoted.to_string());
            } else {
                all_strings = false;
                break;
            }
        }

        // Must have at least one argument, and all must be simple strings.
        if expr_count == 0 || !all_strings {
            return;
        }

        let func_name = node
            .child_by_field_name("name")
            .map(|n| &source[n.start_byte()..n.end_byte()]);
        let Some(name) = func_name else {
            return;
        };

        let pos = node.start_position();
        let command_form = format!("{} {}", name, arg_texts.join(" "));
        diagnostics.push(Diagnostic {
            rule_id: "PRTCAL",
            message: format!(
                "Consider using command syntax: '{}'",
                command_form
            ),
            severity: Severity::Info,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: Some(Fix::new(
                node.start_byte()..node.end_byte(),
                command_form,
            )),
        });
    }
}

// ---------------------------------------------------------------------------
// NCOMMA: Use comma to separate input arguments
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// For `function_call` arguments, check if args are separated by spaces
    /// instead of commas.
    fn check_ncomma(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let Some(args_node) = find_arguments_child(node) else {
            return;
        };

        // Walk argument children looking for adjacent expressions without commas.
        let mut prev_expr: Option<Node> = None;
        let arg_count = args_node.child_count();

        for i in 0..arg_count {
            let Some(child) = args_node.child(i) else {
                continue;
            };
            let kind = child.kind();

            // Reset on comma. A zero-width comma token is inserted by the
            // grammar when an argument separator is missing.
            if kind == "," {
                if child.start_byte() == child.end_byte() {
                    let pos = child.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "NCOMMA",
                        message: "Use comma to separate input arguments".to_string(),
                        severity: Severity::Info,
                        byte_range: child.start_byte()..child.start_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: Some(Fix::insert(child.start_byte(), ", ")),
                    });
                }
                prev_expr = None;
                continue;
            }

            // Skip parentheses and other punctuation.
            if is_punctuation(kind) {
                continue;
            }

            if let Some(prev) = prev_expr {
                let gap = &source[prev.end_byte()..child.start_byte()];
                if !gap.is_empty() && gap.chars().all(|c| c == ' ' || c == '\t') {
                    let pos = child.start_position();
                    let insert_pos = prev.end_byte();
                    diagnostics.push(Diagnostic {
                        rule_id: "NCOMMA",
                        message: "Use comma to separate input arguments".to_string(),
                        severity: Severity::Info,
                        byte_range: prev.end_byte()..child.start_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: Some(Fix::new(insert_pos..insert_pos + 1, ", ")),
                    });
                }
            }
            prev_expr = Some(child);
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Returns `true` if the node kind is bracket/punctuation that should be skipped.
fn is_punctuation(kind: &str) -> bool {
    matches!(kind, "[" | "]" | "{" | "}" | "(" | ")" | ";" | "," | "...")
}

/// Find the direct `arguments` child of a `function_call` node.
///
/// The grammar does not expose `arguments` as a named field on
/// `function_call`, so it must be located by walking the children.
fn find_arguments_child(node: Node) -> Option<Node> {
    let child_count = node.child_count();
    for i in 0..child_count {
        if let Some(child) = node.child(i) {
            if child.kind() == "arguments" {
                return Some(child);
            }
        }
    }
    None
}

/// Get the byte offset of the start of the line containing `byte_offset`.
fn line_start_byte(source: &str, byte_offset: usize) -> usize {
    source[..byte_offset]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0)
}

/// Get the human-readable keyword for an if-related clause kind.
fn keyword_for_clause(kind: &str) -> &str {
    match kind {
        "elseif_clause" => "'elseif'",
        "else_clause" => "'else'",
        _ => kind,
    }
}

/// Extract the text inside parentheses, stripping the outer `(` and `)`.
fn inner_paren_text(paren_node: Node, source: &str) -> String {
    // Find the first and last non-paren children.
    let child_count = paren_node.child_count();
    if child_count < 3 {
        // Malformed — return as-is minus outer chars.
        let text = &source[paren_node.start_byte()..paren_node.end_byte()];
        return text
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(text)
            .to_string();
    }
    // Content is between the opening "(" and closing ")" nodes.
    let start = paren_node.start_byte() + 1; // skip "("
    let end = paren_node.end_byte().saturating_sub(1); // skip ")"
    source[start..end].trim().to_string()
}

/// Find the single inner expression node of a `parenthesis` node.
/// Returns `None` if the parenthesis contains more than one expression
/// or a complex expression (e.g., binary operator).
fn find_inner_expr(paren_node: Node) -> Option<Node> {
    let mut inner = None;
    let child_count = paren_node.child_count();
    for i in 0..child_count {
        let Some(child) = paren_node.child(i) else {
            continue;
        };
        let kind = child.kind();
        if kind == "(" || kind == ")" {
            continue;
        }
        // If we already found one expression, this paren has multiple — skip.
        if inner.is_some() {
            return None;
        }
        inner = Some(child);
    }
    inner
}

// ---------------------------------------------------------------------------
// Check ID validation (used in tests)
// ---------------------------------------------------------------------------

/// Returns all check IDs managed by this engine.
#[allow(dead_code)]
pub fn check_ids() -> &'static [&'static str] {
    CHECK_IDS
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "FORMATTING_ENGINE",
    FormattingEngine::from_config
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

    /// Run the formatting engine on source code and return diagnostics.
    fn lint(source: &str) -> Vec<Diagnostic> {
        let config = Config::default();
        let rule = FormattingEngine::from_config(&config);
        let tree = parse(source);
        let ctx = FileContext {
            tree: &tree,
            source,
            file_path: std::path::Path::new("test.m"),
        };
        rule.check_file(&ctx)
    }

    fn has_id(diags: &[Diagnostic], id: &str) -> bool {
        diags.iter().any(|d| d.rule_id == id)
    }

    // -- NOCOMMA -------------------------------------------------------------

    #[test]
    fn nocomma_space_separated_row() {
        let source = "x = [1 2 3];\n";
        let diags = lint(source);
        assert!(has_id(&diags, "NOCOMMA"), "got: {diags:?}");
    }

    #[test]
    fn nocomma_comma_separated_row() {
        let source = "x = [1, 2, 3];\n";
        let diags = lint(source);
        assert!(!has_id(&diags, "NOCOMMA"), "got: {diags:?}");
    }

    #[test]
    fn nocomma_cell_space_separated() {
        let source = "c = {1 2};\n";
        let diags = lint(source);
        assert!(has_id(&diags, "NOCOMMA"), "got: {diags:?}");
    }

    // -- NO4LP ---------------------------------------------------------------

    #[test]
    fn no4lp_loop_body_unindented() {
        let source = "\
function f()
for i = 1:10
x = i;
end
end
";
        let diags = lint(source);
        assert!(has_id(&diags, "NO4LP"), "got: {diags:?}");
    }

    #[test]
    fn no4lp_loop_body_indented() {
        let source = "\
function f()
    for i = 1:10
        x = i;
    end
end
";
        let diags = lint(source);
        assert!(!has_id(&diags, "NO4LP"), "got: {diags:?}");
    }

    // -- ALIGN ---------------------------------------------------------------

    #[test]
    fn align_misaligned_elseif() {
        let source = "\
function f()
    if x > 0
        a = 1;
      elseif x < 0
        a = -1;
    else
        a = 0;
    end
end
";
        let diags = lint(source);
        assert!(has_id(&diags, "ALIGN"), "got: {diags:?}");
    }

    #[test]
    fn align_aligned_clauses() {
        let source = "\
function f()
    if x > 0
        a = 1;
    elseif x < 0
        a = -1;
    else
        a = 0;
    end
end
";
        let diags = lint(source);
        assert!(!has_id(&diags, "ALIGN"), "got: {diags:?}");
    }

    // -- NOPTS ---------------------------------------------------------------

    #[test]
    fn nopts_parenthesized_condition() {
        let source = "if (x > 0)\n    y = 1;\nend\n";
        let diags = lint(source);
        assert!(has_id(&diags, "NOPTS"), "got: {diags:?}");
    }

    #[test]
    fn nopts_unparenthesized_condition() {
        let source = "if x > 0\n    y = 1;\nend\n";
        let diags = lint(source);
        assert!(!has_id(&diags, "NOPTS"), "got: {diags:?}");
    }

    // -- NOPRT ---------------------------------------------------------------

    #[test]
    fn noprt_unnecessary_parens() {
        let source = "y = (x);\n";
        let diags = lint(source);
        assert!(has_id(&diags, "NOPRT"), "got: {diags:?}");
    }

    #[test]
    fn noprt_binary_expression_kept() {
        let source = "y = (a + b) * c;\n";
        let diags = lint(source);
        assert!(!has_id(&diags, "NOPRT"), "got: {diags:?}");
    }

    // -- PRTCAL --------------------------------------------------------------

    #[test]
    fn prtcal_simple_string_call() {
        let source = "disp('hello');\n";
        let diags = lint(source);
        assert!(has_id(&diags, "PRTCAL"), "got: {diags:?}");
    }

    #[test]
    fn prtcal_non_string_argument() {
        let source = "disp(123);\n";
        let diags = lint(source);
        assert!(!has_id(&diags, "PRTCAL"), "got: {diags:?}");
    }

    // -- NCOMMA --------------------------------------------------------------
    //
    // Note: space-separated function arguments (e.g. `foo(a b)`) parse as
    // syntax errors in tree-sitter-matlab, so NCOMMA is not reachable on that
    // input; it is reported by the syntax-errors engine instead. The check is
    // verified here only for the well-formed case (no false positive).

    #[test]
    fn ncomma_args_comma_separated() {
        let source = "foo(a, b);\n";
        let diags = lint(source);
        assert!(!has_id(&diags, "NCOMMA"), "got: {diags:?}");
    }
}
