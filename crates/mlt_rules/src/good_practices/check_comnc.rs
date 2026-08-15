use super::*;

impl GoodPracticesEngine {
    /// COMNC: Comment lacks space after `%`.
    pub(crate) fn check_comnc(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("COMNC") || node.kind() != "comment" {
            return Vec::new();
        }

        let text = node_text(node, source);

        // Ignore block comments (%{ ... %}) and pragma comments (%#...).
        if text.starts_with("%{") || text.starts_with("%#") || text.starts_with("%%") {
            return Vec::new();
        }

        // Check if `%` is followed by a non-space, non-empty character.
        let after_pct = text.strip_prefix('%').unwrap_or("");
        if after_pct.is_empty() || after_pct.starts_with(' ') || after_pct.starts_with('\t') {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: "COMNC",
            message: "Comment with percent (%) following comma acts as a row separator. Replace the comma with a semicolon to make the row separation clearer. Alternatively, replace the percent (%) with an ellipsis (...) to add a comment inside a row.".to_string(),
            severity: Severity::Info,
            byte_range: node.start_byte()..node.start_byte() + 2,
            line: pos.row + 1,
            column: pos.column + 1,
            fix: Some(mlt_core::Fix::new(
                node.start_byte()..node.start_byte() + 1,
                "% ",
            )),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comnc_fires_on_no_space() {
        let source = "%comment without space\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comment = find_child_of_kind(root, "comment").unwrap();
        let diags = eng.check_comnc(comment, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "COMNC");
    }

    #[test]
    fn test_comnc_silent_with_space() {
        let source = "% comment with space\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let comment = find_child_of_kind(root, "comment").unwrap();
        let diags = eng.check_comnc(comment, source);
        assert!(diags.is_empty());
    }
}
