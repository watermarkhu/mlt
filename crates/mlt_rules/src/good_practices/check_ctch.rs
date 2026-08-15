use super::*;

impl GoodPracticesEngine {
    /// CTCH: `catch` block is empty.
    pub(crate) fn check_ctch(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("CTCH") || node.kind() != "catch_clause" {
            return Vec::new();
        }

        // The catch clause's body is a `block` child. If it is empty
        // (no named children or only whitespace/comments), fire.
        let block = find_child_of_kind(node, "block");
        let is_empty = match block {
            Some(b) => {
                let mut cursor = b.walk();
                let has_statements = b
                    .children(&mut cursor)
                    .any(|c| c.kind() != "comment" && c.is_named());
                !has_statements
            }
            None => true,
        };

        if !is_empty {
            return Vec::new();
        }

        let pos = node.start_position();
        let text = node_text(node, source);
        let keyword_len = text.find(|c: char| c.is_whitespace()).unwrap_or(5);
        vec![Diagnostic {
            rule_id: "CTCH",
            message: "Best practice is for CATCH to be followed by an identifier that gets the error information."
                .to_string(),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.start_byte() + keyword_len,
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
    fn test_ctch_fires_on_empty_catch() {
        let source = "try\n    x = 1;\ncatch\nend\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let try_node = find_child_of_kind(root, "try_statement").unwrap();
        let catch_node = find_child_of_kind(try_node, "catch_clause").unwrap();
        let diags = eng.check_ctch(catch_node, source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "CTCH");
    }
}
