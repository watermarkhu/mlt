use super::*;

impl GoodPracticesEngine {
    /// WNTAG / ERTAG: `warning`/`error` called without a message ID.
    ///
    /// A message ID is a string argument of the form `'comp:tag'` (contains `:`).
    pub(crate) fn check_warning_error_tag(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match get_function_call_name(node, source) {
            Some(n) => n,
            None => return Vec::new(),
        };

        let (check_id, label) = match func_name {
            "warning" if self.is_check_enabled("WNTAG") => ("WNTAG", "warning"),
            "error" if self.is_check_enabled("ERTAG") => ("ERTAG", "error"),
            _ => return Vec::new(),
        };

        // Check first argument: it should be a string containing ':' (message ID).
        let args_node = find_child_of_kind(node, "arguments");
        let first_arg = args_node.and_then(|a| first_named_child(a));
        let has_msg_id = first_arg
            .map(|arg| {
                let text = node_text(arg, source);
                // String literals are quoted; check for ':' inside quotes.
                (arg.kind() == "string" || arg.kind() == "string_content")
                    && text.contains(':')
            })
            .unwrap_or(false);

        if has_msg_id {
            return Vec::new();
        }

        let pos = node.start_position();
        vec![Diagnostic {
            rule_id: check_id,
            message: format!(
                "{label}() called without a message identifier; use {label}('component:id', ...)"
            ),
            severity: Severity::Warning,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }]
    }
}
