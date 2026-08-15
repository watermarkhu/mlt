use super::*;

impl GoodPracticesEngine {
    /// EVLCS / EVLDOT / EVLEQ / EVLSYS / EVLSEQVAR / EVLDUAL: `eval`/`evalin` usage.
    pub(crate) fn check_eval(&self, node: Node, source: &str) -> Vec<Diagnostic> {
        if node.kind() != "function_call" && node.kind() != "command" {
            return Vec::new();
        }

        let func_name = match node.kind() {
            "function_call" => get_function_call_name(node, source),
            "command" => get_command_name(node, source),
            _ => None,
        };

        let func_name = match func_name {
            Some(n) => n,
            None => return Vec::new(),
        };

        let pos = node.start_position();

        // EVLDUAL: evalin
        if func_name == "evalin" && self.is_check_enabled("EVLDUAL") {
            return vec![Diagnostic {
                rule_id: "EVLDUAL",
                message: "This use of 'eval' is unnecessary and can be removed. Call the evaluated function directly using parentheses. For example, use 'load(filename)' instead of 'eval(['load ' filename])'.".to_string(),
                severity: Severity::Warning,
                byte_range: node.start_byte()..node.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            }];
        }

        if func_name != "eval" {
            return Vec::new();
        }

        // Determine which eval sub-check to fire based on argument patterns.
        let args_text = get_arguments_text(node, source);

        let (check_id, message) = if !self.is_check_enabled("EVLCS") {
            return Vec::new();
        } else {
            classify_eval_usage(&args_text, &self.config)
        };

        vec![Diagnostic {
            rule_id: check_id,
            message: message.to_string(),
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
    fn test_evlcs_fires_on_eval() {
        let source = "eval('x = 1');\n";
        let tree = parse(source);
        let root = tree.root_node();
        let eng = engine();

        let fc = find_child_of_kind(root, "function_call").unwrap();
        let diags = eng.check_eval(fc, source);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].rule_id.starts_with("EVL"));
    }
}
