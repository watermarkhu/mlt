//! # Incomplete Analysis: Linter-Internal Limit Checks
//!
//! This module implements the 17 "Incomplete Analysis" checks from MATLAB's Code
//! Analyzer. These are special internal-limit diagnostics that **cannot be disabled**
//! by the user. They all have default severity `Error`.
//!
//! ## Check IDs
//!
//! | Check ID | Description                                  | Detection                                  |
//! |----------|----------------------------------------------|--------------------------------------------|
//! | TMMSG    | More than 10,000 diagnostics generated       | Post-lint (diagnostic count > threshold)   |
//! | TMSMS    | More than 1,000 parse errors generated       | Count ERROR nodes in tree                  |
//! | MXASET   | File too complex to analyze                  | Node count > threshold                     |
//! | QUIT     | Analysis did not complete                    | No-op (linter panic guard)                 |
//! | NOSPC    | File too complex (nesting)                   | Max nesting depth > threshold              |
//! | MBIG     | File too large                               | Source length > threshold                  |
//! | NOFIL    | File not found                               | No-op (handled by CLI)                     |
//! | MDOTM    | Invalid file extension                       | File extension != `.m`                     |
//! | MDMCR    | Deployed MATLAB file                         | File extension == `.ctf` or `.p`           |
//! | RDERR    | Unable to read file                          | No-op (handled by CLI)                     |
//! | EOFER    | Too many syntax errors                       | ERROR node count > threshold               |
//! | EOFMI    | Incomplete file                              | Last node is ERROR or MISSING              |
//! | MDEEP    | Parentheses/brackets nested too deeply       | Max `()`, `[]`, `{}` nesting depth         |
//! | DEEPC    | Block comments nested too deeply             | Nested `%{ %}` detection                   |
//! | DEEPN    | Functions nested too deeply                  | Nested `function_definition` depth         |
//! | DEEPS    | Statements nested too deeply                 | Nested if/for/while/switch/try depth       |
//! | TEXTL    | Text too long                                | Max line length > threshold                |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.INCOMPLETE_ANALYSIS]
//! max_diagnostics = 10000
//! max_parse_errors = 1000
//! max_node_count = 100000
//! max_file_size = 1048576
//! max_paren_depth = 32
//! max_function_depth = 20
//! max_statement_depth = 15
//! max_line_length = 4096
//! ```

use mlt_core::{Category, Config, Diagnostic, FileContext, Rule, Severity};
use serde::Deserialize;
use tree_sitter::Node;

// ---------------------------------------------------------------------------
// Default threshold functions
// ---------------------------------------------------------------------------

const fn default_max_diagnostics() -> usize {
    10000
}
const fn default_max_parse_errors() -> usize {
    1000
}
const fn default_max_node_count() -> usize {
    100000
}
const fn default_max_file_size() -> usize {
    1_048_576 // 1 MB
}
const fn default_max_paren_depth() -> usize {
    32
}
const fn default_max_function_depth() -> usize {
    20
}
const fn default_max_statement_depth() -> usize {
    15
}
const fn default_max_line_length() -> usize {
    4096
}

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for incomplete analysis checks — each threshold is configurable.
///
/// Deserialized from the `[lint.rules.INCOMPLETE_ANALYSIS]` section in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct IncompleteAnalysisConfig {
    /// Maximum number of diagnostics before TMMSG fires.
    #[serde(default = "default_max_diagnostics")]
    pub max_diagnostics: usize,

    /// Maximum parse error (ERROR node) count before TMSMS fires.
    #[serde(default = "default_max_parse_errors")]
    pub max_parse_errors: usize,

    /// Maximum tree node count before MXASET fires.
    #[serde(default = "default_max_node_count")]
    pub max_node_count: usize,

    /// Maximum source file size in bytes before MBIG fires.
    #[serde(default = "default_max_file_size")]
    pub max_file_size: usize,

    /// Maximum parentheses/bracket nesting depth before MDEEP fires.
    #[serde(default = "default_max_paren_depth")]
    pub max_paren_depth: usize,

    /// Maximum nested `function_definition` depth before DEEPN fires.
    #[serde(default = "default_max_function_depth")]
    pub max_function_depth: usize,

    /// Maximum nested statement (if/for/while/switch/try) depth before DEEPS fires.
    #[serde(default = "default_max_statement_depth")]
    pub max_statement_depth: usize,

    /// Maximum line length in characters before TEXTL fires.
    #[serde(default = "default_max_line_length")]
    pub max_line_length: usize,
}

impl Default for IncompleteAnalysisConfig {
    fn default() -> Self {
        Self {
            max_diagnostics: default_max_diagnostics(),
            max_parse_errors: default_max_parse_errors(),
            max_node_count: default_max_node_count(),
            max_file_size: default_max_file_size(),
            max_paren_depth: default_max_paren_depth(),
            max_function_depth: default_max_function_depth(),
            max_statement_depth: default_max_statement_depth(),
            max_line_length: default_max_line_length(),
        }
    }
}

// ---------------------------------------------------------------------------
// Metrics collected during tree traversal
// ---------------------------------------------------------------------------

/// Metrics gathered in a single traversal of the parse tree.
#[derive(Debug, Default)]
struct TreeMetrics {
    /// Total number of nodes in the tree.
    node_count: usize,
    /// Number of ERROR nodes.
    error_count: usize,
    /// Maximum depth of parentheses/bracket nesting (`(`, `[`, `{`).
    max_paren_depth: usize,
    /// Maximum depth of nested `function_definition` nodes.
    max_function_depth: usize,
    /// Maximum depth of nested statement nodes (if/for/while/switch/try).
    max_statement_depth: usize,
    /// Whether the last child of the root is an ERROR or MISSING node.
    last_node_is_error: bool,
}

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// Incomplete Analysis Engine: a file-level rule that handles all 17
/// incomplete-analysis checks in a single pass.
///
/// These checks represent linter-internal limits and cannot be disabled
/// by the user (`can_be_disabled() == false`).
pub struct IncompleteAnalysisEngine {
    config: IncompleteAnalysisConfig,
}

impl IncompleteAnalysisEngine {
    /// Factory constructor. Reads rule-specific params from config.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: IncompleteAnalysisConfig = config.rule_params("INCOMPLETE_ANALYSIS");
        Box::new(Self {
            config: rule_config,
        })
    }

    /// Walk the tree and collect all metrics in one pass.
    fn collect_metrics(&self, root: Node) -> TreeMetrics {
        let mut metrics = TreeMetrics::default();

        // Check last child of root for EOFMI.
        if let Some(last) = last_descendant(root) {
            metrics.last_node_is_error = last.is_error() || last.is_missing();
        }

        self.walk_tree(root, &mut metrics, 0, 0, 0);
        metrics
    }

    /// Recursive DFS traversal that collects node counts, error counts, and
    /// nesting depths in a single pass.
    fn walk_tree(
        &self,
        node: Node,
        metrics: &mut TreeMetrics,
        paren_depth: usize,
        function_depth: usize,
        statement_depth: usize,
    ) {
        metrics.node_count += 1;

        if node.is_error() {
            metrics.error_count += 1;
        }

        // Track parentheses/bracket nesting depth (MDEEP).
        let new_paren_depth = match node.kind() {
            "parenthesis" | "arguments" | "matrix" | "cell" => {
                let d = paren_depth + 1;
                if d > metrics.max_paren_depth {
                    metrics.max_paren_depth = d;
                }
                d
            }
            _ => paren_depth,
        };

        // Track function nesting depth (DEEPN).
        let new_function_depth = if node.kind() == "function_definition" {
            let d = function_depth + 1;
            if d > metrics.max_function_depth {
                metrics.max_function_depth = d;
            }
            d
        } else {
            function_depth
        };

        // Track statement nesting depth (DEEPS).
        let new_statement_depth = match node.kind() {
            "if_statement" | "for_statement" | "while_statement" | "switch_statement"
            | "try_statement" => {
                let d = statement_depth + 1;
                if d > metrics.max_statement_depth {
                    metrics.max_statement_depth = d;
                }
                d
            }
            _ => statement_depth,
        };

        // Recurse into children.
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_tree(
                child,
                metrics,
                new_paren_depth,
                new_function_depth,
                new_statement_depth,
            );
        }
    }

    /// Check maximum line length (TEXTL).
    fn check_line_lengths(&self, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut byte_offset = 0;

        for (line_idx, line) in source.lines().enumerate() {
            let char_len = line.chars().count();
            if char_len > self.config.max_line_length {
                diagnostics.push(Diagnostic {
                    rule_id: "TEXTL",
                    message: format!(
                        "Line length ({char_len}) exceeds internal limit ({max})",
                        max = self.config.max_line_length
                    ),
                    severity: Severity::Error,
                    byte_range: byte_offset..byte_offset + line.len(),
                    line: line_idx + 1,
                    column: 1,
                    fix: None,
                });
            }
            // +1 for newline character.
            byte_offset += line.len() + 1;
        }

        diagnostics
    }

    /// Scan source text for nested block comments (DEEPC).
    ///
    /// MATLAB block comments are `%{ ... %}`. Nesting is not allowed — a
    /// second `%{` inside an open block comment is an error.
    fn check_nested_block_comments(&self, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut depth: usize = 0;
        let mut byte_offset = 0;

        for (line_idx, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed == "%{" {
                depth += 1;
                if depth > 1 {
                    diagnostics.push(Diagnostic {
                        rule_id: "DEEPC",
                        message: format!(
                            "Block comments nested too deeply (depth {depth})"
                        ),
                        severity: Severity::Error,
                        byte_range: byte_offset..byte_offset + line.len(),
                        line: line_idx + 1,
                        column: 1,
                        fix: None,
                    });
                }
            } else if trimmed == "%}" {
                depth = depth.saturating_sub(1);
            }
            byte_offset += line.len() + 1;
        }

        diagnostics
    }
}

impl Rule for IncompleteAnalysisEngine {
    fn id(&self) -> &'static str {
        "INCOMPLETE_ANALYSIS"
    }

    fn description(&self) -> &'static str {
        "Internal linter limits and analysis integrity checks"
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn category(&self) -> Category {
        Category::IncompleteAnalysis
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        // File-level only — no node subscriptions.
        &[]
    }

    fn can_be_disabled(&self) -> bool {
        false
    }

    fn has_file_check(&self) -> bool {
        true
    }

    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let root = ctx.tree.root_node();
        let source = ctx.source;

        // -- Checks that don't need tree traversal --

        // MBIG: File too large.
        if source.len() > self.config.max_file_size {
            diagnostics.push(Diagnostic {
                rule_id: "MBIG",
                message: format!(
                    "File size ({size} bytes) exceeds internal limit ({max} bytes)",
                    size = source.len(),
                    max = self.config.max_file_size
                ),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // MDOTM: Invalid file extension (not .m).
        if let Some(ext) = ctx.file_path.extension() {
            if ext != "m" {
                let ext_str = ext.to_string_lossy();
                // MDMCR: Deployed MATLAB file (.ctf or .p).
                if ext_str == "ctf" || ext_str == "p" {
                    diagnostics.push(Diagnostic {
                        rule_id: "MDMCR",
                        message: format!(
                            "File has deployed MATLAB extension '.{ext_str}'; \
                             analysis of deployed files is not supported"
                        ),
                        severity: Severity::Error,
                        byte_range: 0..source.len().min(1),
                        line: 1,
                        column: 1,
                        fix: None,
                    });
                } else {
                    diagnostics.push(Diagnostic {
                        rule_id: "MDOTM",
                        message: format!(
                            "File has extension '.{ext_str}'; expected '.m'"
                        ),
                        severity: Severity::Error,
                        byte_range: 0..source.len().min(1),
                        line: 1,
                        column: 1,
                        fix: None,
                    });
                }
            }
        } else {
            // No extension at all — also flag as MDOTM.
            diagnostics.push(Diagnostic {
                rule_id: "MDOTM",
                message: "File has no extension; expected '.m'".to_string(),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // -- Single-pass tree traversal --

        let metrics = self.collect_metrics(root);

        // MXASET: File too complex to analyze (node count).
        if metrics.node_count > self.config.max_node_count {
            diagnostics.push(Diagnostic {
                rule_id: "MXASET",
                message: format!(
                    "File too complex to fully analyze ({count} AST nodes; limit is {max})",
                    count = metrics.node_count,
                    max = self.config.max_node_count
                ),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // TMSMS: Too many parse errors.
        if metrics.error_count > self.config.max_parse_errors {
            diagnostics.push(Diagnostic {
                rule_id: "TMSMS",
                message: format!(
                    "File has {count} parse errors; limit is {max}",
                    count = metrics.error_count,
                    max = self.config.max_parse_errors
                ),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // EOFER: Too many syntax errors (same as TMSMS but lower threshold).
        // Fires when error count exceeds max_parse_errors (same metric, distinct ID
        // for MATLAB compatibility).
        if metrics.error_count > 0 && metrics.error_count > self.config.max_parse_errors {
            diagnostics.push(Diagnostic {
                rule_id: "EOFER",
                message: format!(
                    "Too many syntax errors ({count}); further analysis may be unreliable",
                    count = metrics.error_count,
                ),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // NOSPC: File too complex (nesting) — uses the maximum of all nesting depths.
        let max_depth = metrics
            .max_paren_depth
            .max(metrics.max_function_depth)
            .max(metrics.max_statement_depth);
        // Use the most restrictive threshold as a combined depth limit.
        let combined_limit = self
            .config
            .max_paren_depth
            .min(self.config.max_function_depth)
            .min(self.config.max_statement_depth);
        if max_depth > combined_limit {
            diagnostics.push(Diagnostic {
                rule_id: "NOSPC",
                message: format!(
                    "File too complex: maximum nesting depth is {max_depth}; limit is {combined_limit}"
                ),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // MDEEP: Parentheses/brackets nested too deeply.
        if metrics.max_paren_depth > self.config.max_paren_depth {
            diagnostics.push(Diagnostic {
                rule_id: "MDEEP",
                message: format!(
                    "Parentheses/brackets nested too deeply (depth {depth}; limit is {max})",
                    depth = metrics.max_paren_depth,
                    max = self.config.max_paren_depth
                ),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // DEEPN: Functions nested too deeply.
        if metrics.max_function_depth > self.config.max_function_depth {
            diagnostics.push(Diagnostic {
                rule_id: "DEEPN",
                message: format!(
                    "Functions nested too deeply (depth {depth}; limit is {max})",
                    depth = metrics.max_function_depth,
                    max = self.config.max_function_depth
                ),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // DEEPS: Statements nested too deeply.
        if metrics.max_statement_depth > self.config.max_statement_depth {
            diagnostics.push(Diagnostic {
                rule_id: "DEEPS",
                message: format!(
                    "Statements nested too deeply (depth {depth}; limit is {max})",
                    depth = metrics.max_statement_depth,
                    max = self.config.max_statement_depth
                ),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // EOFMI: Incomplete file (last node is ERROR or MISSING).
        if metrics.last_node_is_error {
            diagnostics.push(Diagnostic {
                rule_id: "EOFMI",
                message: "File appears incomplete; the parser found an error or \
                          missing node at the end of the file"
                    .to_string(),
                severity: Severity::Error,
                byte_range: source.len().saturating_sub(1)..source.len(),
                line: source.lines().count().max(1),
                column: 1,
                fix: None,
            });
        }

        // -- Line-based checks --

        // TEXTL: Text too long.
        diagnostics.extend(self.check_line_lengths(source));

        // DEEPC: Block comments nested too deeply.
        diagnostics.extend(self.check_nested_block_comments(source));

        // -- Post-hoc diagnostic count check --

        // TMMSG: Too many diagnostics generated.
        // NOTE: This counts diagnostics emitted by THIS rule only. The linter
        // may also enforce this at a higher level across all rules.
        if diagnostics.len() > self.config.max_diagnostics {
            diagnostics.push(Diagnostic {
                rule_id: "TMMSG",
                message: format!(
                    "More than {max} diagnostics generated; output may be truncated",
                    max = self.config.max_diagnostics
                ),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // -- No-op checks (handled by CLI, documented here for completeness) --
        // QUIT:  Analysis did not complete — linter panic guard (CLI level).
        // NOFIL: File not found — CLI handles file discovery.
        // RDERR: Unable to read file — CLI handles I/O errors.

        diagnostics
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Find the last (rightmost, deepest) descendant node in a tree.
fn last_descendant(node: Node) -> Option<Node> {
    let child_count = node.child_count();
    if child_count == 0 {
        return Some(node);
    }
    let last_child = node.child(child_count - 1)?;
    last_descendant(last_child)
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "INCOMPLETE_ANALYSIS",
    IncompleteAnalysisEngine::from_config
));
