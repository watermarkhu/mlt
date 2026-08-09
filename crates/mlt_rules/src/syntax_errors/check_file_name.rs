//! `check_file_name` — one check of the Syntax Errors engine.

use super::*;

impl SyntaxErrorsEngine {
        pub(crate) fn check_file_name(&self, ctx: &FileContext) -> Vec<Diagnostic> {
            let mut diagnostics = Vec::new();

            if !self.is_check_enabled("BDFIL") {
                return diagnostics;
            }

            if let Some(stem) = ctx.file_path.file_stem() {
                let name = stem.to_string_lossy();

                let valid = !name.is_empty()
                    && name.len() <= 63
                    && name.starts_with(|c: char| c.is_ascii_alphabetic())
                    && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');

                if !valid {
                    let reason = if name.is_empty() {
                        "file name is empty".to_string()
                    } else if name.len() > 63 {
                        format!("file name exceeds 63 characters ({} chars)", name.len())
                    } else if !name.starts_with(|c: char| c.is_ascii_alphabetic()) {
                        "file name must start with a letter".to_string()
                    } else {
                        "file name contains invalid characters (only alphanumeric and underscore allowed)".to_string()
                    };
                    diagnostics.push(Diagnostic {
                        rule_id: "BDFIL",
                        message: format!("Invalid MATLAB file name '{}': {}", name, reason),
                        severity: Severity::Error,
                        byte_range: 0..ctx.source.len().min(1),
                        line: 1,
                        column: 1,
                        fix: None,
                    });
                }
            }

            diagnostics
        }

}
