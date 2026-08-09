use super::*;

impl BugsEngine {
    /// DEFSIZE: `size(x) == [m n]` instead of `isequal(size(x), [m n])`.
    pub(crate) fn check_size_comparison(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if node.kind() != "comparison_operator" {
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

        let op = find_operator_text(node, source);
        if op != "==" && op != "~=" {
            return Vec::new();
        }

        // Check for `size(x) == [m n]` or `[m n] == size(x)`.
        let (call_node, array_node) =
            if lhs.kind() == "function_call" && rhs.kind() == "matrix" {
                (lhs, rhs)
            } else if rhs.kind() == "function_call" && lhs.kind() == "matrix" {
                (rhs, lhs)
            } else {
                return Vec::new();
            };

        let func_name = match extract_call_name(call_node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };

        if func_name != "size" {
            return Vec::new();
        }

        let call_text = node_text(call_node, source);
        let array_text = node_text(array_node, source);
        let pos = node.start_position();

        let replacement = if op == "==" {
            format!("isequal({call_text}, {array_text})")
        } else {
            format!("~isequal({call_text}, {array_text})")
        };

        vec![Diagnostic {
            rule_id: "DEFSIZE",
            message: format!(
                "Comparing size() output with '{op}' may fail for arrays; use isequal() instead"
            ),
            severity: Severity::Error,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: Some(Fix::new(
                node.start_byte()..node.end_byte(),
                replacement,
            )),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::has_id;

    // -- DEFSIZE -------------------------------------------------------------

    #[test]
    fn defsize_fires_on_size_eq_matrix() {
        let src = "y = size(x) == [1 2];\n";
        let diags = node_diags(src);
        assert!(has_id(&diags, "DEFSIZE"), "got: {diags:?}");
    }

    #[test]
    fn defsize_no_fire_on_size_eq_scalar() {
        let src = "y = size(x) == 1;\n";
        let diags = node_diags(src);
        assert!(!has_id(&diags, "DEFSIZE"), "got: {diags:?}");
    }
}
