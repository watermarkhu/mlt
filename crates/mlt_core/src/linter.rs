use std::path::Path;

use tree_sitter::Parser;

use crate::diagnostic::Diagnostic;
use crate::registry::RuleRegistry;
use crate::rule::{FileContext, NodeContext};

// ---------------------------------------------------------------------------
// Linter
// ---------------------------------------------------------------------------

/// The core lint engine. Performs a single-pass, depth-first traversal of the
/// tree-sitter syntax tree and dispatches nodes to matching rules.
///
/// # Performance
///
/// - Uses `TreeCursor` for stack-based, allocation-free traversal.
/// - Looks up rules via the `RuleRegistry`'s O(1) node-type index.
/// - Each node is visited exactly once; rules share the traversal.
/// - After traversal, `check_file()` is called once per rule for file-level analysis.
///
/// # Severity Overrides
///
/// Diagnostics emitted by rules carry the rule's default severity. The linter
/// stamps the effective severity from the registry (which resolves config
/// overrides) onto each diagnostic before returning.
pub struct Linter {
    registry: RuleRegistry,
    parser: Parser,
}

/// Errors that can occur during linting.
#[derive(Debug, thiserror::Error)]
pub enum LintError {
    #[error("failed to parse source: tree-sitter returned no tree")]
    ParseFailed,
}

impl Linter {
    /// Create a new `Linter` with the given rule registry.
    ///
    /// Initializes the tree-sitter parser with the MATLAB language grammar.
    pub fn new(registry: RuleRegistry) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_matlab::LANGUAGE.into())
            .expect("failed to load tree-sitter-matlab grammar");

        Self { registry, parser }
    }

    /// Lint a source file, returning all diagnostics sorted by position.
    ///
    /// This is the main entry point. It:
    /// 1. Parses `source` into a tree-sitter `Tree`.
    /// 2. Performs a single DFS traversal, dispatching nodes to subscribed rules.
    /// 3. Calls `check_file()` on all rules for file-level analysis.
    /// 4. Stamps effective severity from config overrides.
    /// 5. Returns diagnostics sorted by (line, column).
    pub fn lint(&mut self, source: &str, file_path: &Path) -> Result<Vec<Diagnostic>, LintError> {
        // Step 1: Parse
        let tree = self
            .parser
            .parse(source, None)
            .ok_or(LintError::ParseFailed)?;

        // Step 2: Single-pass DFS traversal using TreeCursor
        let mut diagnostics = self.traverse(&tree, source, file_path);

        // Step 3: File-level checks (only on rules that implement check_file)
        let file_ctx = FileContext {
            tree: &tree,
            source,
            file_path,
        };
        for &idx in self.registry.file_check_rules() {
            let rule = self.registry.get_rule(idx);
            let mut file_diags = rule.check_file(&file_ctx);
            // Stamp effective severity on file-level diagnostics.
            let effective_severity = self.registry.effective_severity(idx);
            for diag in &mut file_diags {
                diag.severity = effective_severity;
            }
            diagnostics.extend(file_diags);
        }

        // Step 4: Sort by position for deterministic output
        diagnostics.sort_by(|a, b| a.line.cmp(&b.line).then(a.column.cmp(&b.column)));

        Ok(diagnostics)
    }

    /// Perform the single-pass depth-first traversal.
    ///
    /// Uses `TreeCursor` which is the most efficient way to traverse a
    /// tree-sitter tree — it avoids allocating `Node` objects and uses an
    /// internal stack.
    fn traverse(
        &self,
        tree: &tree_sitter::Tree,
        source: &str,
        file_path: &Path,
    ) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut cursor = tree.walk();

        // Depth-first traversal using TreeCursor's goto_* methods.
        // We visit every node exactly once.
        loop {
            let node = cursor.node();
            let node_kind = node.kind();

            // O(1) lookup: are there rules interested in this node type?
            let rule_indices = self.registry.rules_for_node_type(node_kind);
            if !rule_indices.is_empty() {
                let node_ctx = NodeContext {
                    node,
                    source,
                    file_path,
                };

                for &idx in rule_indices {
                    let rule = self.registry.get_rule(idx);
                    let mut rule_diags = rule.check(&node_ctx);
                    // Stamp effective severity from config onto each diagnostic.
                    let effective_severity = self.registry.effective_severity(idx);
                    for diag in &mut rule_diags {
                        diag.severity = effective_severity;
                    }
                    diagnostics.extend(rule_diags);
                }
            }

            // DFS navigation: try child first, then sibling, then backtrack.
            if cursor.goto_first_child() {
                continue;
            }
            if cursor.goto_next_sibling() {
                continue;
            }

            // Backtrack up the tree until we find an ancestor with a next sibling.
            loop {
                if !cursor.goto_parent() {
                    // We've returned to the root — traversal complete.
                    return diagnostics;
                }
                if cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
}
