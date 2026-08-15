use super::*;

impl BugsEngine {
    /// STCUL: `strcmpi` called with arguments that are already the same case.
    pub(crate) fn check_strcmpi_same_case(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match extract_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };

        if func_name != "strcmpi" {
            return Vec::new();
        }

        let args = collect_call_args(node, source);
        if args.len() < 2 {
            return Vec::new();
        }

        // Check if both arguments are string literals with the same case.
        let a = args[0].trim().trim_matches('\'').trim_matches('"');
        let b = args[1].trim().trim_matches('\'').trim_matches('"');

        // Only flag if both are string literals AND have the same case.
        let a_is_literal = args[0].trim().starts_with('\'') || args[0].trim().starts_with('"');
        let b_is_literal = args[1].trim().starts_with('\'') || args[1].trim().starts_with('"');

        if a_is_literal && b_is_literal && a == b {
            let pos = node.start_position();
            return vec![Diagnostic {
                rule_id: "STCUL",
                message: "The comparison will likely fail due to case mismatch.".to_string(),
                severity: Severity::Error,
                byte_range: node.start_byte()..node.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            }];
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::has_id;

    // -- STCUL ---------------------------------------------------------------

    #[test]
    fn stcul_fires_on_same_case_args() {
        let src = "strcmpi('abc', 'abc');\n";
        let diags = node_diags(src);
        assert!(has_id(&diags, "STCUL"), "got: {diags:?}");
    }

    #[test]
    fn stcul_no_fire_on_different_case_args() {
        let src = "strcmpi('abc', 'ABC');\n";
        let diags = node_diags(src);
        assert!(!has_id(&diags, "STCUL"), "got: {diags:?}");
    }
}
