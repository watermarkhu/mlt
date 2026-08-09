//! PRTCAL check: consider using command syntax instead of function syntax.

use super::*;

// ---------------------------------------------------------------------------
// PRTCAL: Consider using command syntax
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// For `function_call` nodes at statement level with only string literal
    /// arguments, suggest command syntax instead. For example:
    /// `disp('hello')` could be `disp hello`.
    pub(crate) fn check_prtcal(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // Only fire at statement level.
        let is_statement_level = node
            .parent()
            .map(|p| STATEMENT_PARENTS.contains(&p.kind()))
            .unwrap_or(false);
        if !is_statement_level {
            return;
        }

        // Must have an arguments node with only string/identifier children.
        let Some(args_node) = find_arguments_child(node) else {
            return;
        };

        let mut arg_texts = Vec::new();
        let mut all_strings = true;
        let arg_count = args_node.child_count();
        let mut expr_count = 0;

        for i in 0..arg_count {
            let Some(arg) = args_node.child(i) else {
                continue;
            };
            let kind = arg.kind();
            // Skip punctuation (parens, commas).
            if is_punctuation(kind) || kind == "," {
                continue;
            }
            expr_count += 1;
            if kind == "string" {
                // Extract string content without quotes.
                let text = &source[arg.start_byte()..arg.end_byte()];
                let unquoted = text
                    .trim_start_matches('\'')
                    .trim_end_matches('\'')
                    .trim_start_matches('"')
                    .trim_end_matches('"');
                // Command syntax only works for simple strings without spaces.
                if unquoted.contains(' ') || unquoted.contains('\'') {
                    all_strings = false;
                    break;
                }
                arg_texts.push(unquoted.to_string());
            } else {
                all_strings = false;
                break;
            }
        }

        // Must have at least one argument, and all must be simple strings.
        if expr_count == 0 || !all_strings {
            return;
        }

        let func_name = node
            .child_by_field_name("name")
            .map(|n| &source[n.start_byte()..n.end_byte()]);
        let Some(name) = func_name else {
            return;
        };

        let pos = node.start_position();
        let command_form = format!("{} {}", name, arg_texts.join(" "));
        diagnostics.push(Diagnostic {
            rule_id: "PRTCAL",
            message: format!(
                "Consider using command syntax: '{}'",
                command_form
            ),
            severity: Severity::Info,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: Some(Fix::new(
                node.start_byte()..node.end_byte(),
                command_form,
            )),
        });
    }
}

#[cfg(test)]
mod tests {
    
    use crate::formatting::tests::{has_id, lint};

    // -- PRTCAL --------------------------------------------------------------

    #[test]
    fn prtcal_simple_string_call() {
        let source = "disp('hello');\n";
        let diags = lint(source);
        assert!(has_id(&diags, "PRTCAL"), "got: {diags:?}");
    }

    #[test]
    fn prtcal_non_string_argument() {
        let source = "disp(123);\n";
        let diags = lint(source);
        assert!(!has_id(&diags, "PRTCAL"), "got: {diags:?}");
    }
}
