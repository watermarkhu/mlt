//! NOEFF/EQEFF checks: statements with no effect.

use super::*;

impl UnusedEngine {
    /// Run NOEFF/EQEFF checks: statements with no effect.
    pub(crate) fn check_noeff_eqeff(&self, ctx: &FileContext, diagnostics: &mut Vec<Diagnostic>) {
        let noeff_disabled = self.is_check_disabled("NOEFF");
        let eqeff_disabled = self.is_check_disabled("EQEFF");

        if noeff_disabled && eqeff_disabled {
            return;
        }

        // Walk the tree looking for expression nodes at statement level.
        walk_for_no_effect(
            ctx.tree.root_node(),
            diagnostics,
            noeff_disabled,
            eqeff_disabled,
        );
    }
}

// ---------------------------------------------------------------------------
// No-effect statement walker (free function to satisfy clippy)
// ---------------------------------------------------------------------------

/// Recursively walk tree to find statement-level expressions with no effect.
pub(crate) fn walk_for_no_effect(
    node: Node,
    diagnostics: &mut Vec<Diagnostic>,
    noeff_disabled: bool,
    eqeff_disabled: bool,
) {
    // Check if this node is at statement level (parent is block or source_file).
    let is_statement_level = node
        .parent()
        .map(|p| STATEMENT_PARENTS.contains(&p.kind()))
        .unwrap_or(false);

    if is_statement_level {
        let kind = node.kind();

        // EQEFF: comparison operator at statement level.
        if !eqeff_disabled && kind == COMPARISON_NODE {
            let pos = node.start_position();
            diagnostics.push(Diagnostic {
                rule_id: "EQEFF",
                message: "Comparison has no effect (result is not used)".to_string(),
                severity: Severity::Warning,
                byte_range: node.start_byte()..node.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            });
        }
        // NOEFF: other expression nodes at statement level that are not
        // assignments, function_calls, or commands.
        else if !noeff_disabled && NO_EFFECT_EXPR_NODES.contains(&kind) {
            let pos = node.start_position();
            diagnostics.push(Diagnostic {
                rule_id: "NOEFF",
                message: "Statement has no effect (expression result is discarded)".to_string(),
                severity: Severity::Warning,
                byte_range: node.start_byte()..node.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            });
        }
    }

    // Recurse into children.
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            walk_for_no_effect(child, diagnostics, noeff_disabled, eqeff_disabled);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{has_id, lint_file};
    use mlt_core::Config;

    fn engine() -> Box<dyn Rule> {
        UnusedEngine::from_config(&Config::default())
    }

    // -- NOEFF: statement with no effect ------------------------------------

    #[test]
    fn noeff_fires_on_discarded_expression() {
        let diags = lint_file(&*engine(), "function foo()\n    1 + 2;\nend\n");
        assert!(has_id(&diags, "NOEFF"), "got: {diags:?}");
    }

    #[test]
    fn noeff_ok_on_assignment() {
        let diags = lint_file(&*engine(), "function foo()\n    x = 1 + 2;\nend\n");
        assert!(!has_id(&diags, "NOEFF"), "got: {diags:?}");
    }

    // -- EQEFF: comparison with no effect -----------------------------------

    #[test]
    fn eqeff_fires_on_discarded_comparison() {
        let diags = lint_file(&*engine(), "function foo()\n    a == b;\nend\n");
        assert!(has_id(&diags, "EQEFF"), "got: {diags:?}");
    }

    #[test]
    fn eqeff_ok_when_result_used() {
        let diags = lint_file(
            &*engine(),
            "function foo(a, b)\n    x = (a == b);\n    disp(x);\nend\n",
        );
        assert!(!has_id(&diags, "EQEFF"), "got: {diags:?}");
    }
}
