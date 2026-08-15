//! # FORMATTING_ENGINE: Formatting Suggestion Checks
//!
//! ```mlt
//! id = "FORMATTING_ENGINE"
//! title = "Formatting Suggestion Checks"
//! category = "formatting"
//! severity = "info"
//! fix = true
//! icon = "lucide/align-left"
//! slug = "formatting"
//! ```
//!
//! ## Rule
//!
//! Suggests formatting improvements for MATLAB code. All 7 checks share a
//! single `FormattingEngine` that runs as a file-level rule, walking the whole
//! tree once and applying every enabled check per node. Each diagnostic
//! carries the specific check ID (e.g. `NOCOMMA`, `NO4LP`). Individual checks
//! can be enabled, disabled, or reconfigured through their own
//! `[lint.rules.*]` tables.
//!
//! ## Check IDs
//!
//! | Check ID | Severity | Fix | Description |
//! | --- | --- | --- | --- |
//! | NOCOMMA | info | yes | Extra comma is unnecessary. |
//! | NO4LP | info | yes | Parentheses are not needed in a FOR statement. |
//! | ALIGN | info | yes | This keyword might not be aligned with its matching END on line VAR_NUMBER. |
//! | NOPTS | info | yes | Add a semicolon after the statement to hide the output (in a script). |
//! | NOPRT | info | yes | Add a semicolon after the statement to hide the output (in a function). |
//! | PRTCAL | info | yes | Add a semicolon after the function call to hide the output. |
//! | NCOMMA | info | yes | Best practice is to separate output variables with commas. |
//!
//! ## Fix
//!
//! Each check rewrites the flagged construct directly:
//!
//! - NOCOMMA inserts a comma between space-separated row elements (`[1 2 3]` → `[1, 2, 3]`).
//! - NCOMMA inserts a comma between space-separated function arguments.
//! - NO4LP rewrites the indentation prefix of misindented statements.
//! - ALIGN re-indents `elseif`/`else` clauses to the column of their `if`.
//! - NOPTS and NOPRT strip unnecessary parentheses (`(x)` → `x`).
//! - PRTCAL rewrites a string-only call to command syntax (`disp('hello')` → `disp hello`).
//!
//! ## Examples
//!
//! ### Incorrect
//!
//! ```matlab
//! x = [1 2 3];
//! y = (a);
//! if (x > 0)
//!     disp('hello');
//! end
//! ```
//!
//! ### Correct
//!
//! ```matlab
//! x = [1, 2, 3];
//! y = a;
//! if x > 0
//!     disp hello;
//! end
//! ```
//!
//! ### Fixed
//!
//! ```diff
//! - x = [1 2 3];
//! + x = [1, 2, 3];
//! - y = (a);
//! + y = a;
//! - if (x > 0)
//! + if x > 0
//! ```
//!
//! ## Configuration
//!
//! Each check is configured independently through its own rule table:
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

mod check_align;
mod check_ncomma;
mod check_no4lp;
mod check_nocomma;
mod check_noprt;
mod check_nopts;
mod check_prtcal;

use mlt_core::{Category, Config, Diagnostic, FileContext, Fix, Rule, Severity};
use serde::Deserialize;
use tree_sitter::Node;

// ---------------------------------------------------------------------------
// Check IDs
// ---------------------------------------------------------------------------

/// All formatting check IDs handled by this engine.
pub(crate) const CHECK_IDS: &[&str] = &[
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
pub(crate) fn default_indent_size() -> usize {
    4
}

/// Parent node types that indicate statement-level context.
pub(crate) const STATEMENT_PARENTS: &[&str] = &["source_file", "block"];

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
    fn visit_recursive(&self, node: Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
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
// Helpers
// ---------------------------------------------------------------------------

/// Returns `true` if the node kind is bracket/punctuation that should be skipped.
pub(crate) fn is_punctuation(kind: &str) -> bool {
    matches!(kind, "[" | "]" | "{" | "}" | "(" | ")" | ";" | "," | "...")
}

/// Find the direct `arguments` child of a `function_call` node.
///
/// The grammar does not expose `arguments` as a named field on
/// `function_call`, so it must be located by walking the children.
pub(crate) fn find_arguments_child(node: Node) -> Option<Node> {
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
pub(crate) fn line_start_byte(source: &str, byte_offset: usize) -> usize {
    source[..byte_offset]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0)
}

/// Extract the text inside parentheses, stripping the outer `(` and `)`.
pub(crate) fn inner_paren_text(paren_node: Node, source: &str) -> String {
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
pub(crate) fn find_inner_expr(paren_node: Node) -> Option<Node> {
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
// Shared test helpers
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use mlt_core::Config;
    use tree_sitter::Parser;

    /// Parse MATLAB source and return the tree.
    pub(super) fn parse(source: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_matlab::LANGUAGE.into())
            .expect("failed to load tree-sitter-matlab");
        parser.parse(source, None).expect("parse failed")
    }

    /// Run the formatting engine on source code and return diagnostics.
    pub(super) fn lint(source: &str) -> Vec<Diagnostic> {
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

    /// Returns `true` if any diagnostic carries the given rule ID.
    pub(super) fn has_id(diags: &[Diagnostic], id: &str) -> bool {
        diags.iter().any(|d| d.rule_id == id)
    }
}
