//! NOCOMMA check: use commas to separate elements in a row.

use super::*;

impl FormattingEngine {
    /// In `matrix` or `cell` nodes, check if `row` children have elements
    /// separated by spaces instead of commas. Look for adjacent expression
    /// nodes without a `,` between them.
    pub(crate) fn check_nocomma(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let child_count = node.child_count();
        for i in 0..child_count {
            let Some(child) = node.child(i) else {
                continue;
            };
            if child.kind() == "row" {
                self.check_row_commas(child, source, diagnostics);
            }
        }
    }

    /// Check a single `row` node for missing commas between elements.
    fn check_row_commas(
        &self,
        row: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // Collect expression children (skip punctuation like "[", "]", ",", ";").
        let mut prev_expr: Option<Node> = None;
        let child_count = row.child_count();

        for i in 0..child_count {
            let Some(child) = row.child(i) else {
                continue;
            };
            let kind = child.kind();

            // Reset tracking on comma or semicolon. A zero-width comma token
            // is inserted by the grammar when an element separator is missing.
            if kind == "," {
                if child.start_byte() == child.end_byte() {
                    let pos = child.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "NOCOMMA",
                        message: "Use commas to separate elements in a row".to_string(),
                        severity: Severity::Info,
                        byte_range: child.start_byte()..child.start_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: Some(Fix::insert(child.start_byte(), ", ")),
                    });
                }
                prev_expr = None;
                continue;
            }
            if kind == ";" {
                prev_expr = None;
                continue;
            }

            // Skip non-expression tokens (brackets, whitespace nodes).
            if is_punctuation(kind) {
                continue;
            }

            // If we have a previous expression and no comma was found between,
            // check whether there was only whitespace separating them.
            if let Some(prev) = prev_expr {
                let gap = &source[prev.end_byte()..child.start_byte()];
                if !gap.is_empty() && gap.chars().all(|c| c == ' ' || c == '\t') {
                    let pos = child.start_position();
                    let insert_pos = prev.end_byte();
                    diagnostics.push(Diagnostic {
                        rule_id: "NOCOMMA",
                        message: "Use commas to separate elements in a row".to_string(),
                        severity: Severity::Info,
                        byte_range: prev.end_byte()..child.start_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: Some(Fix::new(insert_pos..insert_pos + 1, ", ")),
                    });
                }
            }
            prev_expr = Some(child);
        }
    }
}

#[cfg(test)]
mod tests {
    
    use crate::formatting::tests::{has_id, lint};

    // -- NOCOMMA -------------------------------------------------------------

    #[test]
    fn nocomma_space_separated_row() {
        let source = "x = [1 2 3];\n";
        let diags = lint(source);
        assert!(has_id(&diags, "NOCOMMA"), "got: {diags:?}");
    }

    #[test]
    fn nocomma_comma_separated_row() {
        let source = "x = [1, 2, 3];\n";
        let diags = lint(source);
        assert!(!has_id(&diags, "NOCOMMA"), "got: {diags:?}");
    }

    #[test]
    fn nocomma_cell_space_separated() {
        let source = "c = {1 2};\n";
        let diags = lint(source);
        assert!(has_id(&diags, "NOCOMMA"), "got: {diags:?}");
    }
}
