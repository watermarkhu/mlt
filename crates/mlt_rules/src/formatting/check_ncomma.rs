//! NCOMMA check: use comma to separate input arguments.

use super::*;

// ---------------------------------------------------------------------------
// NCOMMA: Use comma to separate input arguments
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// For `function_call` arguments, check if args are separated by spaces
    /// instead of commas.
    pub(crate) fn check_ncomma(&self, node: Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        let Some(args_node) = find_arguments_child(node) else {
            return;
        };

        // Walk argument children looking for adjacent expressions without commas.
        let mut prev_expr: Option<Node> = None;
        let arg_count = args_node.child_count();

        for i in 0..arg_count {
            let Some(child) = args_node.child(i) else {
                continue;
            };
            let kind = child.kind();

            // Reset on comma. A zero-width comma token is inserted by the
            // grammar when an argument separator is missing.
            if kind == "," {
                if child.start_byte() == child.end_byte() {
                    let pos = child.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "NCOMMA",
                        message: "Use comma to separate input arguments".to_string(),
                        severity: Severity::Info,
                        byte_range: child.start_byte()..child.start_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: Some(Fix::insert(child.start_byte(), ", ")),
                    });
                }
                prev_expr = None;
                continue;
            }

            // Skip parentheses and other punctuation.
            if is_punctuation(kind) {
                continue;
            }

            if let Some(prev) = prev_expr {
                let gap = &source[prev.end_byte()..child.start_byte()];
                if !gap.is_empty() && gap.chars().all(|c| c == ' ' || c == '\t') {
                    let pos = child.start_position();
                    let insert_pos = prev.end_byte();
                    diagnostics.push(Diagnostic {
                        rule_id: "NCOMMA",
                        message: "Use comma to separate input arguments".to_string(),
                        severity: Severity::Info,
                        byte_range: prev.end_byte()..child.start_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: Some(Fix::new(insert_pos..insert_pos + 1, ", ")),
                    });
                }
            }
            prev_expr = Some(child);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::formatting::tests::{has_id, lint};

    // -- NCOMMA --------------------------------------------------------------
    //
    // Note: space-separated function arguments (e.g. `foo(a b)`) parse as
    // syntax errors in tree-sitter-matlab, so NCOMMA is not reachable on that
    // input; it is reported by the syntax-errors engine instead. The check is
    // verified here only for the well-formed case (no false positive).

    #[test]
    fn ncomma_args_comma_separated() {
        let source = "foo(a, b);\n";
        let diags = lint(source);
        assert!(!has_id(&diags, "NCOMMA"), "got: {diags:?}");
    }
}
