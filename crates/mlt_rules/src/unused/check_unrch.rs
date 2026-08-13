//! UNRCH check: unreachable code after return/break/continue.

use super::*;

impl UnusedEngine {
    /// Run UNRCH check: unreachable code after return/break/continue.
    pub(crate) fn check_unrch(&self, ctx: &FileContext, diagnostics: &mut Vec<Diagnostic>) {
        if self.is_check_disabled("UNRCH") {
            return;
        }

        let spans = find_unreachable(ctx.tree, ctx.source);
        for span in spans {
            diagnostics.push(Diagnostic {
                rule_id: "UNRCH",
                message: format!("Unreachable code after '{}'", span.cause),
                severity: Severity::Warning,
                byte_range: span.byte_range,
                line: span.line,
                column: span.column,
                fix: None,
            });
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

    // -- UNRCH: unreachable code --------------------------------------------

    #[test]
    fn unrch_fires_after_return() {
        let diags = lint_file(&*engine(), "function foo()\n    return;\n    x = 1;\nend\n");
        assert!(has_id(&diags, "UNRCH"), "got: {diags:?}");
    }

    #[test]
    fn unrch_ok_without_terminator() {
        let diags = lint_file(
            &*engine(),
            "function foo()\n    x = 1;\n    disp(x);\nend\n",
        );
        assert!(!has_id(&diags, "UNRCH"), "got: {diags:?}");
    }
}
