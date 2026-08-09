//! NUSED/INUSA/INUSD checks: input arguments that are never used.

use super::*;

impl UnusedEngine {
    /// Run NUSED/INUSA/INUSD checks: input arguments that are never used.
    pub(crate) fn check_unused_inputs(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let nused_disabled = self.is_check_disabled("NUSED");
        let inusa_disabled = self.is_check_disabled("INUSA");
        let inusd_disabled = self.is_check_disabled("INUSD");

        if nused_disabled && inusa_disabled && inusd_disabled {
            return;
        }

        for scope in &table.scopes {
            // Only check function/method scopes (not scripts or lambdas).
            if !matches!(
                scope.kind,
                ScopeKind::Function
                    | ScopeKind::LocalFunction
                    | ScopeKind::NestedFunction
                    | ScopeKind::Method
            ) {
                continue;
            }

            for def in &scope.defs {
                if def.kind != DefKind::InputArg {
                    continue;
                }
                let name = &def.name;

                if self.should_ignore_name(name) {
                    continue;
                }

                if !scope.is_used(name) {
                    // NUSED: general "input arg not used"
                    if !nused_disabled {
                        diagnostics.push(Diagnostic {
                            rule_id: "NUSED",
                            message: format!(
                                "Input argument '{name}' is defined but never used"
                            ),
                            severity: Severity::Warning,
                            byte_range: def.byte_range.clone(),
                            line: def.line,
                            column: def.column,
                            fix: None,
                        });
                    }

                    // INUSA: input argument not used in function (same as NUSED
                    // but different check ID for compatibility).
                    if !inusa_disabled {
                        diagnostics.push(Diagnostic {
                            rule_id: "INUSA",
                            message: format!(
                                "Input argument '{name}' is not used in function '{}'",
                                scope.name
                            ),
                            severity: Severity::Warning,
                            byte_range: def.byte_range.clone(),
                            line: def.line,
                            column: def.column,
                            fix: None,
                        });
                    }

                    // INUSD: input argument could be removed.
                    if !inusd_disabled {
                        diagnostics.push(Diagnostic {
                            rule_id: "INUSD",
                            message: format!(
                                "Input argument '{name}' is defined but could be removed"
                            ),
                            severity: Severity::Warning,
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

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{has_id, lint_file};
    use mlt_core::Config;

    fn engine() -> Box<dyn Rule> {
        UnusedEngine::from_config(&Config::default())
    }

    // -- NUSED / INUSA / INUSD: unused input arguments ----------------------

    #[test]
    fn nused_fires_on_unused_input() {
        let diags = lint_file(&*engine(), "function foo(x)\nend\n");
        assert!(has_id(&diags, "NUSED"), "got: {diags:?}");
    }

    #[test]
    fn nused_ok_when_input_used() {
        let diags = lint_file(&*engine(), "function foo(x)\n    disp(x);\nend\n");
        assert!(!has_id(&diags, "NUSED"), "got: {diags:?}");
    }

    #[test]
    fn inusa_fires_on_unused_input() {
        let diags = lint_file(&*engine(), "function foo(x)\nend\n");
        assert!(has_id(&diags, "INUSA"), "got: {diags:?}");
    }

    #[test]
    fn inusa_ok_when_input_used() {
        let diags = lint_file(&*engine(), "function foo(x)\n    disp(x);\nend\n");
        assert!(!has_id(&diags, "INUSA"), "got: {diags:?}");
    }

    #[test]
    fn inusd_fires_on_unused_input() {
        let diags = lint_file(&*engine(), "function foo(x)\nend\n");
        assert!(has_id(&diags, "INUSD"), "got: {diags:?}");
    }

    #[test]
    fn inusd_ok_when_input_used() {
        let diags = lint_file(&*engine(), "function foo(x)\n    disp(x);\nend\n");
        assert!(!has_id(&diags, "INUSD"), "got: {diags:?}");
    }

}
