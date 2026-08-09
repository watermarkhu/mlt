use super::*;

impl GoodPracticesEngine {
    /// DISPLAY: Overriding `display` is discouraged (use `disp` instead).
    pub(crate) fn check_display_override(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("DISPLAY") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let mut diagnostics = Vec::new();

        for func in &meta.functions {
            if func.is_method && func.name == "display" {
                diagnostics.push(Diagnostic {
                    rule_id: "DISPLAY",
                    message: "Overriding display() is discouraged; override disp() instead"
                        .to_string(),
                    severity: Severity::Warning,
                    byte_range: func.byte_range.clone(),
                    line: func.line,
                    column: 1,
                    fix: None,
                });
            }
        }

        diagnostics
    }
}
