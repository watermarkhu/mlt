use super::*;

impl BugsEngine {
    /// STRCMPCSTR: `strcmp` with single-character string comparison.
    pub(crate) fn check_strcmp_char(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match extract_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };

        if func_name != "strcmp" {
            return Vec::new();
        }

        let args = collect_call_args(node, source);
        if args.len() < 2 {
            return Vec::new();
        }

        // Check if either argument is a single-character string literal.
        let has_single_char = args.iter().any(|arg| {
            let trimmed = arg.trim();
            (trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() == 3)
                || (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() == 3)
        });

        if has_single_char {
            let pos = node.start_position();
            vec![Diagnostic {
                rule_id: "STRCMPCSTR",
                message: "strcmp used with single-character string; consider using '==' for char comparison".to_string(),
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

    // -- STRCMPCSTR ----------------------------------------------------------

    #[test]
    fn strcmpcstr_fires_on_single_char_arg() {
        let src = "strcmp('a', 'b');\n";
        let diags = node_diags(src);
        assert!(has_id(&diags, "STRCMPCSTR"), "got: {diags:?}");
    }

    #[test]
    fn strcmpcstr_no_fire_on_multi_char_args() {
        let src = "strcmp('ab', 'cd');\n";
        let diags = node_diags(src);
        assert!(!has_id(&diags, "STRCMPCSTR"), "got: {diags:?}");
    }
}
