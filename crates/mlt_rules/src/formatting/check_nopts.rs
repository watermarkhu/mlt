//! NOPTS check: add parentheses around condition in if/while.

use super::*;

// ---------------------------------------------------------------------------
// NOPTS: Add parentheses around condition in if/while
// ---------------------------------------------------------------------------

impl FormattingEngine {
    /// For `if_statement` and `while_statement`, check if the condition
    /// expression is wrapped in a `parenthesis` node. If so, suggest
    /// removing the outer parens since MATLAB does not require them.
    pub(crate) fn check_nopts(&self, node: Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        let condition = match node.kind() {
            "if_statement" | "while_statement" => node.child_by_field_name("condition"),
            _ => None,
        };

        let Some(cond) = condition else {
            return;
        };

        if cond.kind() == "parenthesis" {
            let pos = cond.start_position();
            // The fix removes the outer parentheses, keeping the inner content.
            let inner_text = inner_paren_text(cond, source);
            diagnostics.push(Diagnostic {
                rule_id: "NOPTS",
                message: "Add a semicolon after the statement to hide the output (in a script)."
                    .to_string(),
                severity: Severity::Info,
                byte_range: cond.start_byte()..cond.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: Some(Fix::new(cond.start_byte()..cond.end_byte(), inner_text)),
            });
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::formatting::tests::{has_id, lint};

    // -- NOPTS ---------------------------------------------------------------

    #[test]
    fn nopts_parenthesized_condition() {
        let source = "if (x > 0)\n    y = 1;\nend\n";
        let diags = lint(source);
        assert!(has_id(&diags, "NOPTS"), "got: {diags:?}");
    }

    #[test]
    fn nopts_unparenthesized_condition() {
        let source = "if x > 0\n    y = 1;\nend\n";
        let diags = lint(source);
        assert!(!has_id(&diags, "NOPTS"), "got: {diags:?}");
    }
}
