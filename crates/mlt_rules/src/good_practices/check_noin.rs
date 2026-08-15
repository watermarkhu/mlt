use super::*;

impl GoodPracticesEngine {
    /// NOIN: Function has no input validation (no arguments block).
    pub(crate) fn check_noin(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("NOIN") {
            return Vec::new();
        }

        let meta = FileMeta::build(tree, source);
        let mut diagnostics = Vec::new();

        for func in meta.functions.iter().chain(meta.local_functions.iter()) {
            // Skip constructors, setters, getters, and functions with no inputs.
            if func.is_constructor || func.is_setter || func.is_getter {
                continue;
            }
            // Skip abstract methods.
            if func.is_abstract {
                continue;
            }
            // Only flag if the function has inputs but no arguments block.
            if !func.inputs.is_empty() && !func.has_arguments_block {
                diagnostics.push(Diagnostic {
                    rule_id: "NOIN",
                    message: format!(
                        "Method {} should either be a static method or have at least one input argument.",
                        func.name
                    ),
                    severity: Severity::Info,
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
