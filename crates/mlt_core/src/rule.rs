use std::path::Path;

use tree_sitter::{Node, Tree};

use crate::diagnostic::{Diagnostic, Severity};

// ---------------------------------------------------------------------------
// Context types
// ---------------------------------------------------------------------------

/// Context passed to [`Rule::check`] during the single-pass node traversal.
///
/// Provides the current node, the full source text (for extracting snippets),
/// and the file path. Since tree-sitter `Node` supports `.parent()`,
/// `.next_sibling()`, etc., rules can walk the tree locally as needed.
pub struct NodeContext<'a> {
    pub node: Node<'a>,
    pub source: &'a str,
    pub file_path: &'a Path,
}

/// Context passed to [`Rule::check_file`] after the full traversal completes.
///
/// Provides the parsed tree (for rules that need their own traversal strategy),
/// the full source text, and the file path.
pub struct FileContext<'a> {
    pub tree: &'a Tree,
    pub source: &'a str,
    pub file_path: &'a Path,
}

// ---------------------------------------------------------------------------
// Rule trait
// ---------------------------------------------------------------------------

/// The core trait that all lint rules must implement.
///
/// Rules declare which tree-sitter node types they are interested in via
/// [`target_node_types`]. During the single-pass traversal, the engine
/// dispatches matching nodes to [`check`]. After traversal, [`check_file`]
/// is called on every rule for file-level analysis.
///
/// # Design Notes
///
/// - **Node-level rules**: Override `check()`, return `target_node_types()`.
/// - **File-level rules**: Override `check_file()`, return empty `target_node_types()`.
/// - **Hybrid rules**: Override both (unusual, but supported).
///
/// Rules must be `Send + Sync` to support future parallel file processing.
pub trait Rule: Send + Sync {
    /// Unique rule identifier (e.g., `"M001"`).
    fn id(&self) -> &'static str;

    /// Human-readable one-line description of what this rule checks.
    fn description(&self) -> &'static str;

    /// Default severity for diagnostics produced by this rule.
    fn severity(&self) -> Severity;

    /// Tree-sitter node type names this rule subscribes to.
    ///
    /// Return an empty slice for file-level-only rules (which use `check_file`).
    /// The engine uses this to build an index for O(1) dispatch during traversal.
    fn target_node_types(&self) -> &'static [&'static str];

    /// Check a single node encountered during the single-pass traversal.
    ///
    /// Only called for nodes whose `kind()` is in [`target_node_types`].
    /// Default implementation returns no diagnostics.
    fn check(&self, _ctx: &NodeContext) -> Vec<Diagnostic> {
        Vec::new()
    }

    /// Check the file after the full traversal has completed.
    ///
    /// Called once per file for every registered rule. Use this for rules that
    /// require full-file context (e.g., unused variables, duplicate definitions).
    /// Default implementation returns no diagnostics.
    fn check_file(&self, _ctx: &FileContext) -> Vec<Diagnostic> {
        Vec::new()
    }
}
