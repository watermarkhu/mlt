//! MANU check: method defined but never called.

use super::*;

impl UnusedEngine {
    /// Run MANU check: method defined but never called within the file.
    pub(crate) fn check_manu(&self, table: &SymbolTable, diagnostics: &mut Vec<Diagnostic>) {
        if self.is_check_disabled("MANU") {
            return;
        }

        // Collect all uses across all scopes.
        let all_uses: HashSet<&str> = table
            .scopes
            .iter()
            .flat_map(|s| s.uses.iter().map(|u| u.name.as_str()))
            .collect();

        for scope in &table.scopes {
            if scope.kind != ScopeKind::Method {
                continue;
            }
            let name = &scope.name;
            if name.is_empty() {
                continue;
            }
            if self.should_ignore_name(name) {
                continue;
            }
            if !all_uses.contains(name.as_str()) {
                diagnostics.push(Diagnostic {
                    rule_id: "MANU",
                    message: "Input argument might be unused. Consider replacing the argument with ~, or make this method Static instead.".to_string(),
                    severity: Severity::Warning,
                    byte_range: scope.byte_range.clone(),
                    line: scope.line,
                    column: 1,
                    fix: None,
                });
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

    // -- MANU: method defined but never called ------------------------------

    #[test]
    fn manu_fires_on_uncalled_method() {
        let diags = lint_file(
            &*engine(),
            "classdef Foo\n    methods\n        function y = bar()\n            y = 1;\n        end\n    end\nend\n",
        );
        assert!(has_id(&diags, "MANU"), "got: {diags:?}");
    }

    #[test]
    fn manu_ok_when_method_called() {
        let diags = lint_file(
            &*engine(),
            "classdef Foo\n    methods\n        function y = bar()\n            y = baz();\n        end\n        function y = baz()\n            y = bar();\n        end\n    end\nend\n",
        );
        assert!(!has_id(&diags, "MANU"), "got: {diags:?}");
    }
}
