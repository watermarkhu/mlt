use super::*;

impl BugsEngine {
    /// LOGEMP: `length(x) == 0` should be `isempty(x)`.
    pub(crate) fn check_length_empty(&self, node: Node, source: &str) -> Vec<Diagnostic> {
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
        if op != "==" {
            return Vec::new();
        }

        // Check for `length(x) == 0` or `0 == length(x)`.
        let (call_node, zero_node) =
            if lhs.kind() == "function_call" && node_text(rhs, source).trim() == "0" {
                (lhs, rhs)
            } else if rhs.kind() == "function_call" && node_text(lhs, source).trim() == "0" {
                (rhs, lhs)
            } else {
                return Vec::new();
            };

        let func_name = match extract_call_name(call_node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };

        if func_name != "length" && func_name != "numel" {
            return Vec::new();
        }

        // Extract the argument to length().
        let arg_text = extract_first_arg_text(call_node, source).unwrap_or("x");

        let pos = node.start_position();
        let _ = zero_node; // suppress unused warning
        vec![Diagnostic {
            rule_id: "LOGEMP",
            message: format!("Use 'isempty({arg_text})' instead of '{func_name}({arg_text}) == 0'"),
            severity: Severity::Error,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: Some(Fix::new(
                node.start_byte()..node.end_byte(),
                format!("isempty({arg_text})"),
            )),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::has_id;

    // -- LOGEMP --------------------------------------------------------------

    #[test]
    fn logemp_fires_on_length_eq_zero() {
        let src = "y = length(x) == 0;\n";
        let diags = node_diags(src);
        assert!(has_id(&diags, "LOGEMP"), "got: {diags:?}");
    }

    #[test]
    fn logemp_no_fire_on_nonzero_length() {
        let src = "y = length(x) > 0;\n";
        let diags = node_diags(src);
        assert!(!has_id(&diags, "LOGEMP"), "got: {diags:?}");
    }
}
