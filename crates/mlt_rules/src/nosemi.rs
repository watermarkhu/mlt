//! # NOSEMI: Trailing Semicolon Missing
//!
//! In MATLAB, statements that produce output without a trailing semicolon will
//! print their result to the console. This is almost always unintentional in
//! production code and can cause significant performance degradation in loops.
//!
//! ## Examples
//!
//! Bad:
//! ```matlab
//! x = compute_value()
//! data = load('file.mat')
//! ```
//!
//! Good:
//! ```matlab
//! x = compute_value();
//! data = load('file.mat');
//! ```
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.NOSEMI]
//! severity = "info"
//! ignore_functions = ["disp", "fprintf", "warning", "error"]
//! ```

use mlt_core::{Category, Config, Diagnostic, Fix, NodeContext, Rule, Severity};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for NOSEMI.
///
/// Deserialized from the `[lint.rules.NOSEMI]` section in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct NosemiConfig {
    /// Function names to ignore (statements calling these functions won't be
    /// flagged even without a semicolon). Useful for intentional output
    /// functions like `disp`, `fprintf`, etc.
    #[serde(default)]
    pub ignore_functions: Vec<String>,
}

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// Rule NOSEMI: flags statements at the top level of a block that do not end
/// with a trailing semicolon.
///
/// Targets `assignment`, `function_call`, and `command` nodes. Only fires when
/// the node is a direct child of `source_file` or `block` (i.e., at statement
/// level, not as a sub-expression).
pub struct Nosemi {
    config: NosemiConfig,
}

/// Statement-level node types that should typically end with a semicolon.
const TARGET_NODES: &[&str] = &["assignment", "function_call", "command"];

/// Parent node types that indicate statement-level context.
const STATEMENT_PARENTS: &[&str] = &["source_file", "block"];

impl Nosemi {
    /// Factory constructor. Reads rule-specific params from config.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: NosemiConfig = config.rule_params("NOSEMI");
        Box::new(Self {
            config: rule_config,
        })
    }
}

impl Rule for Nosemi {
    fn id(&self) -> &'static str {
        "NOSEMI"
    }

    fn description(&self) -> &'static str {
        "Statement without trailing semicolon may produce unintended console output"
    }

    fn severity(&self) -> Severity {
        Severity::Info
    }

    fn category(&self) -> Category {
        Category::Formatting
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        TARGET_NODES
    }

    fn check(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        let node = ctx.node;

        // Only fire at statement level: parent must be source_file or block.
        // This prevents flagging function_call nodes that are sub-expressions
        // (e.g., `x = foo(bar(1))` — bar(1) is not a statement).
        let is_statement_level = node
            .parent()
            .map(|p| STATEMENT_PARENTS.contains(&p.kind()))
            .unwrap_or(false);

        if !is_statement_level {
            return Vec::new();
        }

        // If this is a function_call or command, check if the function name
        // is in the ignore list.
        if !self.config.ignore_functions.is_empty() {
            if let Some(func_name) = extract_function_name(node, ctx.source) {
                if self.config.ignore_functions.iter().any(|f| f == func_name) {
                    return Vec::new();
                }
            }
        }

        // Check if the statement is followed by a semicolon.
        let has_semicolon = has_trailing_semicolon(node, ctx.source);

        if has_semicolon {
            return Vec::new();
        }

        // Compute position for the diagnostic.
        let start = node.start_position();
        let end_byte = node.end_byte();

        vec![Diagnostic {
            rule_id: self.id(),
            message: self.description().to_string(),
            severity: self.severity(),
            byte_range: node.start_byte()..end_byte,
            line: start.row + 1, // tree-sitter is 0-indexed
            column: start.column + 1,
            fix: Some(Fix::insert(end_byte, ";")),
        }]
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract the function name from a `function_call` or `command` node.
fn extract_function_name<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> Option<&'a str> {
    match node.kind() {
        "function_call" => {
            // The `name` field of a function_call node.
            let name_node = node.child_by_field_name("name")?;
            // For simple calls, the name is an identifier.
            // For method calls (obj.method), take the full text.
            Some(&source[name_node.start_byte()..name_node.end_byte()])
        }
        "command" => {
            // The first child of a command is the command_name.
            let name_node = node.child(0)?;
            if name_node.kind() == "command_name" {
                Some(&source[name_node.start_byte()..name_node.end_byte()])
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Determine whether a statement node is followed by a semicolon.
///
/// In tree-sitter-matlab's grammar, semicolons (`;`) and newlines are statement
/// terminators that appear as sibling nodes after the statement. We check the
/// source text immediately following the node for a `;` (skipping whitespace
/// on the same line).
fn has_trailing_semicolon(node: tree_sitter::Node, source: &str) -> bool {
    // Strategy 1: Check next named/unnamed siblings for ";"
    let mut sibling = node.next_sibling();
    while let Some(sib) = sibling {
        let kind = sib.kind();
        // Skip over comment nodes that might appear between statement and semicolon
        if kind == "comment" || kind == "line_continuation" {
            sibling = sib.next_sibling();
            continue;
        }
        // If the next meaningful sibling is ";", the statement is terminated.
        if kind == ";" {
            return true;
        }
        // Any other sibling means no semicolon follows.
        break;
    }

    // Strategy 2: Fallback — scan the source text after the node end.
    // This handles cases where the semicolon might not be a separate node.
    let after = &source[node.end_byte()..];
    for ch in after.chars() {
        match ch {
            ';' => return true,
            // Stop at newline — semicolon must be on the same line.
            '\n' | '\r' => return false,
            // Skip inline whitespace.
            ' ' | '\t' => continue,
            // Any other character means something else follows.
            _ => return false,
        }
    }

    false
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new("NOSEMI", Nosemi::from_config));
