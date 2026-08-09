use super::*;

impl GoodPracticesEngine {
    /// COMFS/SEMFS: a top-level `,` (COMFS) or `;` (SEMFS) in a file that also
    /// contains a `function_definition` makes the file a script, so all
    /// functions in it become local functions.
    pub(crate) fn check_comfs_semfs(&self, tree: &tree_sitter::Tree, _source: &str) -> Vec<Diagnostic> {
        let root = tree.root_node();
        if root.kind() != "source_file" {
            return Vec::new();
        }

        let mut cursor = root.walk();
        let children: Vec<Node> = root.children(&mut cursor).collect();
        if !children.iter().any(|c| c.kind() == "function_definition") {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        for child in children {
            let (check_id, message) = match child.kind() {
                "," if self.is_check_enabled("COMFS") => (
                    "COMFS",
                    "This comma makes the file a script. Therefore, all functions in the file are local functions.",
                ),
                ";" if self.is_check_enabled("SEMFS") => (
                    "SEMFS",
                    "This semicolon makes the file a script. Therefore, all functions in the file are local functions.",
                ),
                _ => continue,
            };
            let pos = child.start_position();
            diagnostics.push(Diagnostic {
                rule_id: check_id,
                message: message.to_string(),
                severity: Severity::Warning,
                byte_range: child.start_byte()..child.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            });
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comfs_semfs_fires_with_function_definition() {
        let source = "x = 1,\ny = 2;\nfunction f()\nend\n";
        let tree = parse(source);
        let eng = engine();

        let diags = eng.check_comfs_semfs(&tree, source);
        assert!(diags.iter().any(|d| d.rule_id == "COMFS"));
        assert!(diags.iter().any(|d| d.rule_id == "SEMFS"));
    }

    #[test]
    fn test_comfs_semfs_silent_without_function_definition() {
        let source = "x = 1,\ny = 2;\n";
        let tree = parse(source);
        let eng = engine();

        let diags = eng.check_comfs_semfs(&tree, source);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_comfs_semfs_silent_in_function_file() {
        let source = "function f()\n  x = 1;\nend\n";
        let tree = parse(source);
        let eng = engine();

        let diags = eng.check_comfs_semfs(&tree, source);
        assert!(diags.is_empty());
    }
}
