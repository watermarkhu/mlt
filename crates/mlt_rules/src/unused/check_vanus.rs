//! VANUS check: value assigned to `ans` is unused.

use super::*;

impl UnusedEngine {
    /// Run VANUS check: value assigned to `ans` is unused.
    pub(crate) fn check_vanus(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("VANUS") {
            return;
        }

        for scope in &table.scopes {
            for def in &scope.defs {
                if def.kind != DefKind::Assignment {
                    continue;
                }
                if def.name != "ans" {
                    continue;
                }
                if !scope.is_used("ans") {
                    diagnostics.push(Diagnostic {
                        rule_id: "VANUS",
                        message: "Value assigned to 'ans' is unused".to_string(),
                        severity: Severity::Info,
                        byte_range: def.byte_range.clone(),
                        line: def.line,
                        column: def.column,
                        fix: None,
                    });
                }
            }
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

    // -- VANUS: value assigned to `ans` unused ------------------------------

    #[test]
    fn vanus_fires_on_unused_ans_assignment() {
        let diags = lint_file(&*engine(), "function foo()\n    ans = 5;\nend\n");
        assert!(has_id(&diags, "VANUS"), "got: {diags:?}");
    }

    #[test]
    fn vanus_ok_when_ans_used() {
        let diags = lint_file(&*engine(), "function foo()\n    ans = 5;\n    disp(ans);\nend\n");
        assert!(!has_id(&diags, "VANUS"), "got: {diags:?}");
    }

}
