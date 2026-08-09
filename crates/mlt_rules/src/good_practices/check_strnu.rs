use super::*;

impl GoodPracticesEngine {
    /// STRNU: Use `str2double` instead of `str2num`.
    pub(crate) fn check_strnu(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("STRNU") || node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match get_function_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };

        if func_name != "str2num" {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "STRNU",
            message: "Use str2double() instead of str2num(); str2num uses eval internally"
                .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
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
    fn test_strnu_fires_on_str2num() {
        let source = "x = str2num('123');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        // Find the function_call inside the assignment.
        let assignment = find_child_of_kind(root, "assignment").unwrap();
        let mut cursor = assignment.walk();
        let fc = assignment
            .children(&mut cursor)
            .find(|c| c.kind() == "function_call");
        if let Some(fc) = fc {
            let diags = eng.check_strnu(fc, source);
            assert_eq!(diags.len(), 1);
            assert_eq!(diags[0].rule_id, "STRNU");
        }
    }
}
