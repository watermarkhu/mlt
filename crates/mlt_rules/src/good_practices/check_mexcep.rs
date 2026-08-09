use super::*;

impl GoodPracticesEngine {
    /// MEXCEP: `catch` clause without an exception variable.
    pub(crate) fn check_mexcep(&self, node: Node, _source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("MEXCEP") || node.kind() != "catch_clause" {
            return Vec::new();
        }

        // If the catch clause has an identifier child directly (the exception var),
        // it is captured. Otherwise, fire.
        let has_exception_var = has_child_of_kind(node, "identifier");
        if has_exception_var {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "MEXCEP",
            message: "Catch clause has no exception variable; use 'catch ME' to capture the error"
                .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.start_byte() + "catch".len(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mexcep_fires_without_variable() {
        let source = "try\n    x = 1;\ncatch\n    disp('err');\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let try_node = find_child_of_kind(root, "try_statement").unwrap();
        let catch_node = find_child_of_kind(try_node, "catch_clause").unwrap();
        let diags = eng.check_mexcep(catch_node, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "MEXCEP");
    }
}
