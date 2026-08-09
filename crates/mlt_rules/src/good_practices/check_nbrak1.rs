use super::*;

impl GoodPracticesEngine {
    /// NBRAK1: Unnecessary parentheses around a scalar expression.
    pub(crate) fn check_nbrak1(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("NBRAK1") || node.kind() != "parenthesized_expression" {
            return Vec::new();
        }

        // Check if the inner content is a single scalar (number or identifier).
        let inner = first_named_child(node);
        let is_single_scalar = inner
            .map(|n| {
                let kind = n.kind();
                (kind == "number" || kind == "identifier")
                    && n.next_named_sibling().is_none()
            })
            .unwrap_or(false);

        if !is_single_scalar {
            return Vec::new();
        }

        // Suppress when the parent requires parens (function_call args, etc.).
        if let Some(parent) = node.parent() {
            let pk = parent.kind();
            if pk == "function_call" || pk == "arguments" || pk == "function_arguments" {
                return Vec::new();
            }
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "NBRAK1",
            message: "Unnecessary parentheses around scalar expression".to_string(),
            severity: Severity::Info,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: inner.map(|n| {
                mlt_core::Fix::new(
                    node.start_byte()..node.end_byte(),
                    node_text(n, source),
                )
            }),
        }]
    }
}
