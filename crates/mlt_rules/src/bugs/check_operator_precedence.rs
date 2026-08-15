use super::*;

impl BugsEngine {
    /// MOCUP: Operator precedence issue — mixed `&`/`|` with comparison operators
    /// without explicit parentheses.
    pub(crate) fn check_operator_precedence(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if node.kind() != "binary_operator" {
            return Vec::new();
        }

        let op = find_operator_text(node, source);
        if op != "&" && op != "|" {
            return Vec::new();
        }

        // Check if a child is a comparison_operator without parentheses.
        let has_unparenthesized_cmp = node
            .child(0)
            .is_some_and(|c| c.kind() == "comparison_operator")
            || node
                .child(2)
                .is_some_and(|c| c.kind() == "comparison_operator");

        // Check if there's a mix of & and | at the same level.
        let has_mixed = node.child(0).is_some_and(|c| {
            if c.kind() == "binary_operator" {
                let child_op = find_operator_text(c, source);
                (op == "&" && child_op == "|") || (op == "|" && child_op == "&")
            } else {
                false
            }
        });

        if has_unparenthesized_cmp || has_mixed {
            let pos = node.start_position();
            vec![Diagnostic {
                rule_id: "MOCUP",
                message: "Variable VAR_NAME may be cleared before the cleanup function that references VAR_NAME executes, resulting in an undefined variable error.".to_string(),
                severity: Severity::Error,
                byte_range: node.start_byte()..node.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            }]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::has_id;

    // -- MOCUP ---------------------------------------------------------------

    #[test]
    fn mocup_fires_on_unparenthesized_comparison() {
        let src = "y = a & b | c;\n";
        let diags = node_diags(src);
        assert!(has_id(&diags, "MOCUP"), "got: {diags:?}");
    }

    #[test]
    fn mocup_no_fire_without_mix() {
        let src = "y = a + b * c;\n";
        let diags = node_diags(src);
        assert!(!has_id(&diags, "MOCUP"), "got: {diags:?}");
    }
}
