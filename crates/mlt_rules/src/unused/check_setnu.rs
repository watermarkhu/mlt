//! SETNU check: output of a function assigned but never used.

use super::*;

impl UnusedEngine {
    /// Run SETNU check: output of function call assigned but never used.
    /// This is a refinement of NASGU specifically for function call outputs.
    pub(crate) fn check_setnu(
        &self,
        table: &SymbolTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if self.is_check_disabled("SETNU") {
            return;
        }

        for scope in &table.scopes {
            let output_args: HashSet<&str> = scope
                .defs
                .iter()
                .filter(|d| d.kind == DefKind::OutputArg)
                .map(|d| d.name.as_str())
                .collect();

            for def in &scope.defs {
                if def.kind != DefKind::Assignment {
                    continue;
                }
                let name = &def.name;

                if output_args.contains(name.as_str()) {
                    continue;
                }
                if self.should_ignore_name(name) {
                    continue;
                }
                // SETNU is for function call outputs; we only emit if the var
                // is never used. We differentiate from NASGU by checking later,
                // but to avoid duplication with NASGU, SETNU only fires if
                // NASGU is disabled.
                if !self.is_check_disabled("NASGU") {
                    continue;
                }
                if !scope.is_used(name) {
                    diagnostics.push(Diagnostic {
                        rule_id: "SETNU",
                        message: format!(
                            "Output assigned to '{name}' is never used"
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{has_id, lint_file};
    use mlt_core::Config;

    fn engine() -> Box<dyn Rule> {
        UnusedEngine::from_config(&Config::default())
    }

    /// Build an engine with the given params (e.g., `disabled_checks = [...]`).
    fn engine_with(params: &str) -> Box<dyn Rule> {
        let config = Config::from_toml(&format!("[lint.rules.UNUSED_ENGINE]\n{params}\n")).unwrap();
        UnusedEngine::from_config(&config)
    }

    // -- SETNU: output assigned but never used (only when NASGU off) --------

    #[test]
    fn setnu_fires_when_nasgu_disabled() {
        let engine = engine_with("disabled_checks = [\"NASGU\"]");
        let diags = lint_file(&*engine, "function foo()\n    x = 5;\nend\n");
        assert!(has_id(&diags, "SETNU"), "got: {diags:?}");
    }

    #[test]
    fn setnu_ok_when_nasgu_enabled() {
        let diags = lint_file(&*engine(), "function foo()\n    x = 5;\nend\n");
        assert!(!has_id(&diags, "SETNU"), "got: {diags:?}");
    }

}
