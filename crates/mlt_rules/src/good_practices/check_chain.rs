use super::*;

impl GoodPracticesEngine {
    /// CHAIN: Method chaining on one line (field_expression chains).
    pub(crate) fn check_chain(&self, node: Node, _source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("CHAIN") || node.kind() != "function_call" {
            return Vec::new();
        }

        // Check if the function_call's name child is a field_expression with deep nesting.
        let name_node = match node.child_by_field_name("name") {
            Some(n) => n,
            None => return Vec::new(),
        };

        if name_node.kind() != "field_expression" {
            return Vec::new();
        }

        let depth = field_chain_depth(name_node);
        if depth < 4 {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "CHAIN",
            message: format!(
                "Method chain depth is {depth}; consider breaking into intermediate variables"
            ),
            severity: Severity::Info,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }
}
