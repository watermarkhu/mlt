//! `check_file_structure` — one check of the Syntax Errors engine.

use super::*;

impl SyntaxErrorsEngine {
    pub(crate) fn check_file_structure(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut cursor = root.walk();

        let mut class_count = 0usize;
        let mut has_non_class_statements = false;
        let mut has_function_def = false;
        let mut has_any_real_content = false;

        for child in root.children(&mut cursor) {
            let kind = child.kind();
            match kind {
                "class_definition" => class_count += 1,
                "function_definition" => {
                    has_function_def = true;
                    has_any_real_content = true;
                }
                "comment" => {} // Comments don't count as content.
                "expression_statement" => {
                    // Check if it's just a semicolon.
                    let text = &source[child.start_byte()..child.end_byte()];
                    let trimmed = text.trim();
                    if trimmed == ";" {
                        // Empty statement — doesn't count as real content.
                    } else {
                        has_non_class_statements = true;
                        has_any_real_content = true;
                    }
                }
                _ if !child.is_error() && kind != "\n" => {
                    has_non_class_statements = true;
                    has_any_real_content = true;
                }
                _ => {}
            }
        }

        // CLTWO: Multiple class definitions in one file.
        if self.is_check_enabled("CLTWO") && class_count > 1 {
            diagnostics.push(Diagnostic {
                rule_id: "CLTWO",
                message: format!(
                    "File contains {class_count} class definitions; only one is allowed per file"
                ),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // CLIS: Class definition in a script file (non-function statements exist).
        if self.is_check_enabled("CLIS")
            && class_count > 0
            && has_non_class_statements
            && !has_function_def
        {
            diagnostics.push(Diagnostic {
                rule_id: "CLIS",
                message: "Class definition in a script file; statements exist outside the class"
                    .to_string(),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // SOFOC: Statements outside a class definition in a class file.
        if self.is_check_enabled("SOFOC")
            && class_count > 0
            && has_non_class_statements
            && has_function_def
        {
            diagnostics.push(Diagnostic {
                rule_id: "SOFOC",
                message: "Statements outside class definition in a class file are not allowed"
                    .to_string(),
                severity: Severity::Error,
                byte_range: 0..source.len().min(1),
                line: 1,
                column: 1,
                fix: None,
            });
        }

        // SEMFU: File has only empty statements (only `;` and whitespace).
        if self.is_check_enabled("SEMFU")
            && !has_any_real_content
            && class_count == 0
            && !source.trim().is_empty()
        {
            // Verify the file truly only has semicolons, whitespace, and comments.
            let only_empty = source.lines().all(|line| {
                let trimmed = line.trim();
                trimmed.is_empty() || trimmed == ";" || trimmed.starts_with('%')
            });
            if only_empty {
                diagnostics.push(Diagnostic {
                    rule_id: "SEMFU",
                    message: "File contains only empty statements (semicolons and whitespace)"
                        .to_string(),
                    severity: Severity::Error,
                    byte_range: 0..source.len().min(1),
                    line: 1,
                    column: 1,
                    fix: None,
                });
            }
        }

        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{has_id, lint_file};
    use mlt_core::Config;

    fn engine() -> Box<dyn Rule> {
        SyntaxErrorsEngine::from_config(&Config::default())
    }

    // -- CLIS: class definition in a script file ----------------------------

    #[test]
    fn clis_fires_on_class_with_script_statements() {
        let diags = lint_file(&*engine(), "classdef Foo\nend\nx = 5;\n");
        assert!(has_id(&diags, "CLIS"), "got: {diags:?}");
    }

    #[test]
    fn clis_ok_on_plain_class_file() {
        let diags = lint_file(&*engine(), "classdef Foo\nend\n");
        assert!(!has_id(&diags, "CLIS"), "got: {diags:?}");
    }

    // -- CLTWO: multiple class definitions ----------------------------------

    #[test]
    fn cltwo_fires_on_two_classes() {
        let diags = lint_file(&*engine(), "classdef Foo\nend\nclassdef Bar\nend\n");
        assert!(has_id(&diags, "CLTWO"), "got: {diags:?}");
    }

    #[test]
    fn cltwo_ok_on_single_class() {
        let diags = lint_file(&*engine(), "classdef Foo\nend\n");
        assert!(!has_id(&diags, "CLTWO"), "got: {diags:?}");
    }

    // -- SOFOC: statements outside class in class file ----------------------

    #[test]
    fn sofoc_fires_on_statements_outside_class() {
        let diags = lint_file(&*engine(), "classdef Foo\nend\nfunction f()\nend\nx = 5;\n");
        assert!(has_id(&diags, "SOFOC"), "got: {diags:?}");
    }

    #[test]
    fn sofoc_ok_on_class_with_methods() {
        let diags = lint_file(
            &*engine(),
            "classdef Foo\nmethods\nfunction f()\nend\nend\nend\n",
        );
        assert!(!has_id(&diags, "SOFOC"), "got: {diags:?}");
    }

    // -- SEMFU: file with only empty statements -----------------------------

    #[test]
    fn semfu_fires_on_comment_only_file() {
        let diags = lint_file(&*engine(), "% just a comment\n");
        assert!(has_id(&diags, "SEMFU"), "got: {diags:?}");
    }

    #[test]
    fn semfu_ok_on_real_content() {
        let diags = lint_file(&*engine(), "x = 5;\n");
        assert!(!has_id(&diags, "SEMFU"), "got: {diags:?}");
    }
}
