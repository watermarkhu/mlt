use super::*;

impl BugsEngine {
    /// MULCC: Multiple conditions that could be simplified.
    ///
    /// Flags patterns like `a && a` or `a || a` (duplicate conditions).
    pub(crate) fn check_duplicate_conditions(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if node.kind() != "boolean_operator" {
            return Vec::new();
        }

        let lhs = match node.child(0) {
            Some(c) => c,
            None => return Vec::new(),
        };
        let rhs = match node.child(2) {
            Some(c) => c,
            None => return Vec::new(),
        };

        let lhs_text = node_text(lhs, source).trim().to_string();
        let rhs_text = node_text(rhs, source).trim().to_string();

        if lhs_text == rhs_text {
            let pos = node.start_position();
            vec![Diagnostic {
                rule_id: "MULCC",
                message: format!(
                    "Duplicate condition '{lhs_text}' could be simplified"
                ),
                severity: Severity::Error,
                byte_range: node.start_byte()..node.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: Some(Fix::new(
                    node.start_byte()..node.end_byte(),
                    lhs_text,
                )),
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

    // -- MULCC ---------------------------------------------------------------

    #[test]
    fn mulcc_fires_on_duplicate_conditions() {
        let src = "if (a && a)\n    x = 1;\nend\n";
        let diags = node_diags(src);
        assert!(has_id(&diags, "MULCC"), "got: {diags:?}");
    }

    #[test]
    fn mulcc_no_fire_on_distinct_conditions() {
        let src = "if (a && b)\n    x = 1;\nend\n";
        let diags = node_diags(src);
        assert!(!has_id(&diags, "MULCC"), "got: {diags:?}");
    }
}
