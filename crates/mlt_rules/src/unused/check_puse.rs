//! PUSE check: global/persistent variable declared but never used.

use super::*;

impl UnusedEngine {
    /// Run PUSE check: global/persistent variables declared but not used.
    pub(crate) fn check_puse(&self, table: &SymbolTable, diagnostics: &mut Vec<Diagnostic>) {
        if self.is_check_disabled("PUSE") {
            return;
        }

        for scope in &table.scopes {
            for def in &scope.defs {
                if !matches!(def.kind, DefKind::Global | DefKind::Persistent) {
                    continue;
                }
                let name = &def.name;

                if self.should_ignore_name(name) {
                    continue;
                }

                if !scope.is_used(name) {
                    diagnostics.push(Diagnostic {
                        rule_id: "PUSE",
                        message: "Persistent variable might be unused.".to_string(),
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

    // -- PUSE: global/persistent declared but not used ----------------------

    #[test]
    fn puse_fires_on_unused_global() {
        let diags = lint_file(&*engine(), "function foo()\n    global g;\nend\n");
        assert!(has_id(&diags, "PUSE"), "got: {diags:?}");
    }

    #[test]
    fn puse_ok_when_global_used() {
        let diags = lint_file(
            &*engine(),
            "function foo()\n    global g;\n    disp(g);\nend\n",
        );
        assert!(!has_id(&diags, "PUSE"), "got: {diags:?}");
    }
}
