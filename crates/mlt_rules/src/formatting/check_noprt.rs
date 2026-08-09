//! NOPRT check: remove unnecessary parentheses.

use super::*;

// ---------------------------------------------------------------------------
// NOPRT: Remove unnecessary parentheses
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// Detect `parenthesis` nodes that are unnecessary — e.g., `(x)` where
    /// x is a simple identifier or number, and the parenthesis is not a
    /// function call argument or condition.
    pub(crate) fn check_noprt(
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

#[cfg(test)]
mod tests {
    
    use crate::formatting::tests::{has_id, lint};

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
}
