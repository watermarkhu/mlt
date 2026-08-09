use super::*;

impl GoodPracticesEngine {
    /// UNRPWR: Power of negative base may produce complex result.
    pub(crate) fn check_unrpwr(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("UNRPWR") || node.kind() != "binary_operator" {
            return Vec::new();
        }

        let text = node_text(node, source);
        if !text.contains('^') {
            return Vec::new();
        }

        // Check if the left operand is a negative literal or unary negation.
        let left = node.child_by_field_name("left").or_else(|| node.child(0));
        let has_negative_base = left
            .map(|l| {
                if l.kind() == "unary_operator" {
                    let lt = node_text(l, source);
                    return lt.starts_with('-');
                }
                if l.kind() == "number" {
                    let lt = node_text(l, source);
                    return lt.starts_with('-');
                }
                false
            })
            .unwrap_or(false);

        if !has_negative_base {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "UNRPWR",
            message: "Power of a negative base may produce a complex result; use parentheses to clarify intent".to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }
}
