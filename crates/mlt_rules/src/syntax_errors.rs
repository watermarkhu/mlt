//! # Syntax Errors: Parser-Level and Semantic Syntax Validation
//!
//! This module implements syntax error checks that detect issues tree-sitter flags
//! as ERROR/MISSING nodes, as well as semantic syntax issues the parser cannot catch
//! (invalid operators, bad file names, structural violations).
//!
//! ## Check IDs
//!
//! | Check ID | Description                                              |
//! |----------|----------------------------------------------------------|
//! | SYNER    | ERROR node detected in parse tree                        |
//! | BDFIL    | File name doesn't follow MATLAB naming rules             |
//! | BADNE    | Source contains `!=` (MATLAB uses `~=`)                  |
//! | BADOT    | Source contains `..` not part of `...`                   |
//! | TWOCM    | Source contains `,,`                                     |
//! | CLIS     | class_definition in a script file                        |
//! | CLTWO    | Multiple class_definition nodes in one file              |
//! | SOFOC    | Statements outside a class_definition in a class file    |
//! | SEMFU    | File has only empty statements (only `;` and whitespace) |
//! | FNDOT    | Function name contains dots outside class methods block  |
//! | FNSWA    | Function name doesn't start with alphabetic character    |
//! | NOPAR    | ERROR node looks like a missing closing bracket          |
//! | ENDCT    | ERROR node suggesting missing END                        |
//! | EOFMI    | File ends with ERROR node (incomplete)                   |
//! | NOLHS    | Assignment with empty left side                          |
//! | BADCH    | Invalid control characters in source                     |
//! | BADSP    | Non-ASCII whitespace characters in source                |
//! | REDEF    | Same identifier used as both function name and variable  |
//! | SEPEXR   | Missing newline/semicolon between statements             |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.SYNTAX_ERRORS_ENGINE]
//! disabled_checks = []
//! ```

use std::collections::HashSet;
use std::ops::Range;

use mlt_core::{Category, Config, Diagnostic, FileContext, Rule, Severity};
use serde::Deserialize;
use tree_sitter::Node;

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for syntax error checks.
///
/// Deserialized from the `[lint.rules.SYNTAX_ERRORS_ENGINE]` section in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SyntaxErrorsConfig {
    /// List of individual check IDs to disable (e.g., `["BADOT", "TWOCM"]`).
    #[serde(default)]
    pub disabled_checks: Vec<String>,
}

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// Syntax Errors Engine: a file-level rule that performs parser-level and
/// semantic syntax validation in a single pass.
pub struct SyntaxErrorsEngine {
    /// Set of disabled check IDs for fast lookup.
    disabled: HashSet<String>,
}

impl SyntaxErrorsEngine {
    /// Factory constructor. Reads rule-specific params from config.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: SyntaxErrorsConfig = config.rule_params("SYNTAX_ERRORS_ENGINE");
        let disabled: HashSet<String> = rule_config.disabled_checks.into_iter().collect();
        Box::new(Self { disabled })
    }

    /// Returns true if the given check ID is enabled.
    fn is_check_enabled(&self, check_id: &str) -> bool {
        !self.disabled.contains(check_id)
    }

    // -----------------------------------------------------------------------
    // Source text scanning (BADNE, BADOT, TWOCM, BADCH, BADSP)
    // -----------------------------------------------------------------------

    /// Collect byte ranges of all comment and string nodes to skip during
    /// source-level scanning.
    fn collect_skip_ranges(node: Node, ranges: &mut Vec<Range<usize>>) {
        if node.kind() == "comment" || node.kind() == "string" {
            ranges.push(node.byte_range());
            return;
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_skip_ranges(child, ranges);
        }
    }

    /// Check if a byte position is inside a skip range.
    fn in_skip_range(pos: usize, ranges: &[Range<usize>]) -> bool {
        ranges.iter().any(|r| r.contains(&pos))
    }

    /// Scan source text for BADNE (`!=`), BADOT (`..` not `...`), TWOCM (`,,`),
    /// BADCH (invalid control chars), and BADSP (non-ASCII whitespace).
    fn scan_source_text(
        &self,
        source: &str,
        skip_ranges: &[Range<usize>],
    ) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let bytes = source.as_bytes();
        let len = bytes.len();

        // Track line/column for diagnostics.
        let mut line = 1usize;
        let mut line_start = 0usize;

        let mut i = 0;
        while i < len {
            let b = bytes[i];

            // Track newlines.
            if b == b'\n' {
                line += 1;
                line_start = i + 1;
                i += 1;
                continue;
            }

            // BADCH: Invalid control characters (< 0x20 except \t, \n, \r).
            if self.is_check_enabled("BADCH")
                && b < 0x20
                && b != b'\t'
                && b != b'\n'
                && b != b'\r'
                && !Self::in_skip_range(i, skip_ranges)
            {
                diagnostics.push(Diagnostic {
                    rule_id: "BADCH",
                    message: format!(
                        "Invalid control character (0x{:02X}) in source",
                        b
                    ),
                    severity: Severity::Error,
                    byte_range: i..i + 1,
                    line,
                    column: i - line_start + 1,
                    fix: None,
                });
                i += 1;
                continue;
            }

            // BADSP: Non-ASCII whitespace detection.
            // Check for multi-byte UTF-8 sequences that are whitespace.
            if self.is_check_enabled("BADSP") && b > 0x7F && !Self::in_skip_range(i, skip_ranges) {
                // Decode the UTF-8 character at this position.
                if let Some(ch) = source[i..].chars().next() {
                    if ch.is_whitespace() && ch != ' ' && ch != '\t' && ch != '\n' && ch != '\r' {
                        let ch_len = ch.len_utf8();
                        diagnostics.push(Diagnostic {
                            rule_id: "BADSP",
                            message: format!(
                                "Non-ASCII whitespace character (U+{:04X}) in source",
                                ch as u32
                            ),
                            severity: Severity::Error,
                            byte_range: i..i + ch_len,
                            line,
                            column: i - line_start + 1,
                            fix: None,
                        });
                        i += ch_len;
                        continue;
                    }
                }
            }

            // BADNE: `!=` (MATLAB uses `~=`).
            if self.is_check_enabled("BADNE")
                && b == b'!'
                && i + 1 < len
                && bytes[i + 1] == b'='
                && !Self::in_skip_range(i, skip_ranges)
            {
                diagnostics.push(Diagnostic {
                    rule_id: "BADNE",
                    message: "Use '~=' instead of '!=' for not-equal in MATLAB".to_string(),
                    severity: Severity::Error,
                    byte_range: i..i + 2,
                    line,
                    column: i - line_start + 1,
                    fix: Some(mlt_core::Fix::new(i..i + 2, "~=")),
                });
                i += 2;
                continue;
            }

            // BADOT: `..` not part of `...` (line continuation).
            if self.is_check_enabled("BADOT")
                && b == b'.'
                && i + 1 < len
                && bytes[i + 1] == b'.'
                && !Self::in_skip_range(i, skip_ranges)
            {
                // Check if this is part of `...` (line continuation).
                let is_ellipsis = i + 2 < len && bytes[i + 2] == b'.';
                if !is_ellipsis {
                    diagnostics.push(Diagnostic {
                        rule_id: "BADOT",
                        message: "Invalid '..' operator; did you mean '...' (line continuation)?"
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: i..i + 2,
                        line,
                        column: i - line_start + 1,
                        fix: None,
                    });
                    i += 2;
                    continue;
                } else {
                    // Skip the whole `...`.
                    i += 3;
                    continue;
                }
            }

            // TWOCM: `,,` (double comma).
            if self.is_check_enabled("TWOCM")
                && b == b','
                && i + 1 < len
                && bytes[i + 1] == b','
                && !Self::in_skip_range(i, skip_ranges)
            {
                diagnostics.push(Diagnostic {
                    rule_id: "TWOCM",
                    message: "Double comma ',,'; possible typo".to_string(),
                    severity: Severity::Error,
                    byte_range: i..i + 2,
                    line,
                    column: i - line_start + 1,
                    fix: None,
                });
                i += 2;
                continue;
            }

            // Advance past multi-byte UTF-8 characters.
            if b > 0x7F {
                if let Some(ch) = source[i..].chars().next() {
                    i += ch.len_utf8();
                    continue;
                }
            }

            i += 1;
        }

        diagnostics
    }

    // -----------------------------------------------------------------------
    // Tree ERROR/MISSING node analysis (SYNER, NOPAR, ENDCT, EOFMI)
    // -----------------------------------------------------------------------

    /// Walk the tree and collect diagnostics for ERROR and MISSING nodes.
    fn check_error_nodes(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        self.walk_error_nodes(root, source, &mut diagnostics);

        // EOFMI: Check if file ends with ERROR node.
        if self.is_check_enabled("EOFMI") {
            if let Some(last) = Self::last_descendant(root) {
                if last.is_error() || last.is_missing() {
                    let start = last.start_byte();
                    let end = last.end_byte().max(start + 1);
                    let pos = last.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "EOFMI",
                        message: "File ends with an incomplete or erroneous construct"
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: start..end,
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }

        diagnostics
    }

    /// Recursive DFS to find ERROR/MISSING nodes and classify them.
    fn walk_error_nodes(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.is_error() {
            let start = node.start_byte();
            let end = node.end_byte().max(start + 1);
            let pos = node.start_position();
            let text = &source[start..end.min(source.len())];

            // Classify the error node.
            // NOPAR: Looks like a missing closing bracket.
            if self.is_check_enabled("NOPAR") && Self::looks_like_missing_bracket(text) {
                diagnostics.push(Diagnostic {
                    rule_id: "NOPAR",
                    message: "Possible missing closing bracket/parenthesis".to_string(),
                    severity: Severity::Error,
                    byte_range: start..end,
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
            // ENDCT: Looks like a missing END.
            else if self.is_check_enabled("ENDCT") && Self::looks_like_missing_end(text, &node) {
                diagnostics.push(Diagnostic {
                    rule_id: "ENDCT",
                    message: "Possible missing 'end' keyword".to_string(),
                    severity: Severity::Error,
                    byte_range: start..end,
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
            // SYNER: Generic syntax error.
            else if self.is_check_enabled("SYNER") {
                let snippet: String = text.chars().take(40).collect();
                let msg = if snippet.is_empty() {
                    "Syntax error".to_string()
                } else {
                    format!("Syntax error near '{snippet}'")
                };
                diagnostics.push(Diagnostic {
                    rule_id: "SYNER",
                    message: msg,
                    severity: Severity::Error,
                    byte_range: start..end,
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        } else if node.is_missing() && self.is_check_enabled("SYNER") {
            let start = node.start_byte();
            let end = node.end_byte().max(start + 1);
            let pos = node.start_position();
            diagnostics.push(Diagnostic {
                rule_id: "SYNER",
                message: format!("Missing expected '{}'", node.kind()),
                severity: Severity::Error,
                byte_range: start..end,
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            });
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_error_nodes(child, source, diagnostics);
        }
    }

    /// Heuristic: does this error text look like an unclosed bracket?
    fn looks_like_missing_bracket(text: &str) -> bool {
        let opens: usize = text.chars().filter(|&c| c == '(' || c == '[' || c == '{').count();
        let closes: usize = text.chars().filter(|&c| c == ')' || c == ']' || c == '}').count();
        opens > closes
    }

    /// Heuristic: does this error look like a missing `end`?
    fn looks_like_missing_end(text: &str, node: &Node) -> bool {
        // If the error node is preceded by block-opening keywords.
        let block_openers = [
            "if", "for", "while", "switch", "try", "function", "classdef",
            "properties", "methods", "events", "enumeration",
        ];
        // Check text for block openers without matching end.
        let lower = text.to_lowercase();
        let has_opener = block_openers.iter().any(|kw| lower.contains(kw));
        let has_end = lower.contains("end");

        if has_opener && !has_end {
            return true;
        }

        // Check if previous sibling is a block statement.
        if let Some(prev) = node.prev_sibling() {
            let kind = prev.kind();
            if block_openers.contains(&kind) {
                return true;
            }
        }

        false
    }

    // -----------------------------------------------------------------------
    // File structure validation (BDFIL, CLIS, CLTWO, SOFOC, SEMFU)
    // -----------------------------------------------------------------------

    /// Validate file name against MATLAB naming rules.
    fn check_file_name(&self, ctx: &FileContext) -> Vec<Diagnostic> {
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

    /// Check for class/script structure issues (CLIS, CLTWO, SOFOC, SEMFU).
    fn check_file_structure(&self, root: Node, source: &str) -> Vec<Diagnostic> {
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
                message:
                    "Statements outside class definition in a class file are not allowed"
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
            let only_empty = source
                .lines()
                .all(|line| {
                    let trimmed = line.trim();
                    trimmed.is_empty()
                        || trimmed == ";"
                        || trimmed.starts_with('%')
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

    // -----------------------------------------------------------------------
    // Function name validation (FNDOT, FNSWA)
    // -----------------------------------------------------------------------

    /// Check function names for issues (FNDOT, FNSWA).
    fn check_function_names(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        self.walk_function_names(root, source, &mut diagnostics, false);
        diagnostics
    }

    /// Recursively walk to find function_definition nodes and validate names.
    fn walk_function_names(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
        in_methods_block: bool,
    ) {
        let kind = node.kind();
        let is_methods = kind == "methods";

        if kind == "function_definition" {
            // Find the function name node.
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = &source[name_node.start_byte()..name_node.end_byte()];
                let pos = name_node.start_position();

                // FNSWA: Function name doesn't start with alphabetic character.
                if self.is_check_enabled("FNSWA")
                    && !name.starts_with(|c: char| c.is_ascii_alphabetic())
                {
                    diagnostics.push(Diagnostic {
                        rule_id: "FNSWA",
                        message: format!(
                            "Function name '{}' must start with an alphabetic character",
                            name
                        ),
                        severity: Severity::Error,
                        byte_range: name_node.byte_range(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }

                // FNDOT: Function name contains dots but is not in a class methods block.
                if self.is_check_enabled("FNDOT") && name.contains('.') && !in_methods_block {
                    diagnostics.push(Diagnostic {
                        rule_id: "FNDOT",
                        message: format!(
                            "Function name '{}' contains dots but is not in a class methods block",
                            name
                        ),
                        severity: Severity::Error,
                        byte_range: name_node.byte_range(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_function_names(
                child,
                source,
                diagnostics,
                in_methods_block || is_methods,
            );
        }
    }

    // -----------------------------------------------------------------------
    // Statement validation (NOLHS, SEPEXR, REDEF)
    // -----------------------------------------------------------------------

    /// Check statements for issues (NOLHS, SEPEXR, REDEF).
    fn check_statements(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Collect function names and variable assignments for REDEF.
        let mut function_names: HashSet<String> = HashSet::new();
        let mut variable_names: HashSet<String> = HashSet::new();
        let mut redef_reported: HashSet<String> = HashSet::new();

        self.walk_statements(
            root,
            source,
            &mut diagnostics,
            &mut function_names,
            &mut variable_names,
            &mut redef_reported,
        );

        diagnostics
    }

    /// Recursively walk statements for validation.
    fn walk_statements(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
        function_names: &mut HashSet<String>,
        variable_names: &mut HashSet<String>,
        redef_reported: &mut HashSet<String>,
    ) {
        let kind = node.kind();

        // NOLHS: Assignment with empty left side.
        if self.is_check_enabled("NOLHS") && kind == "assignment" {
            if let Some(lhs) = node.child_by_field_name("left") {
                let lhs_text = &source[lhs.start_byte()..lhs.end_byte()];
                if lhs_text.trim().is_empty() {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "NOLHS",
                        message: "Assignment with empty left-hand side".to_string(),
                        severity: Severity::Error,
                        byte_range: node.byte_range(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }

        // REDEF: Track function definitions and variable assignments.
        if kind == "function_definition" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = source[name_node.start_byte()..name_node.end_byte()].to_string();
                if self.is_check_enabled("REDEF")
                    && variable_names.contains(&name)
                    && !redef_reported.contains(&name)
                {
                    let pos = name_node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "REDEF",
                        message: format!(
                            "Identifier '{}' is used as both a function name and a variable",
                            name
                        ),
                        severity: Severity::Error,
                        byte_range: name_node.byte_range(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                    redef_reported.insert(name.clone());
                }
                function_names.insert(name);
            }
        }

        if kind == "assignment" {
            if let Some(lhs) = node.child_by_field_name("left") {
                let lhs_kind = lhs.kind();
                if lhs_kind == "identifier" {
                    let name = source[lhs.start_byte()..lhs.end_byte()].to_string();
                    if self.is_check_enabled("REDEF")
                        && function_names.contains(&name)
                        && !redef_reported.contains(&name)
                    {
                        let pos = lhs.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "REDEF",
                            message: format!(
                                "Identifier '{}' is used as both a function name and a variable",
                                name
                            ),
                            severity: Severity::Error,
                            byte_range: lhs.byte_range(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                        redef_reported.insert(name.clone());
                    }
                    variable_names.insert(name);
                }
            }
        }

        // SEPEXR: Missing separator between statements.
        // Check consecutive children of block/source_file that are statements
        // on the same line without separator.
        if self.is_check_enabled("SEPEXR") && (kind == "source_file" || kind == "block") {
            let mut cursor = node.walk();
            let children: Vec<Node> = node.children(&mut cursor).collect();

            for pair in children.windows(2) {
                let prev = pair[0];
                let curr = pair[1];

                // Skip error nodes and comments.
                if prev.is_error() || curr.is_error() {
                    continue;
                }
                if prev.kind() == "comment" || curr.kind() == "comment" {
                    continue;
                }

                // Check if they're on the same line with no separator.
                let prev_end = prev.end_position();
                let curr_start = curr.start_position();

                if prev_end.row == curr_start.row {
                    // Check the text between them for a separator (; or ,).
                    let between_start = prev.end_byte();
                    let between_end = curr.start_byte();
                    if between_start < between_end {
                        let between = &source[between_start..between_end];
                        let has_separator =
                            between.contains(';') || between.contains(',');
                        if !has_separator {
                            let pos = curr.start_position();
                            diagnostics.push(Diagnostic {
                                rule_id: "SEPEXR",
                                message:
                                    "Missing semicolon or newline between statements"
                                        .to_string(),
                                severity: Severity::Error,
                                byte_range: curr.byte_range(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: Some(mlt_core::Fix::insert(between_start, ";")),
                            });
                        }
                    }
                }
            }
        }

        // Recurse into children.
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_statements(
                child,
                source,
                diagnostics,
                function_names,
                variable_names,
                redef_reported,
            );
        }
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Find the last (rightmost, deepest) descendant node in a tree.
    fn last_descendant(node: Node) -> Option<Node> {
        let child_count = node.child_count();
        if child_count == 0 {
            return Some(node);
        }
        let last_child = node.child(child_count - 1)?;
        Self::last_descendant(last_child)
    }
}

impl Rule for SyntaxErrorsEngine {
    fn id(&self) -> &'static str {
        "SYNTAX_ERRORS_ENGINE"
    }

    fn description(&self) -> &'static str {
        "Parser-level and semantic syntax validation"
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn category(&self) -> Category {
        Category::SyntaxErrors
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        // File-level only — no node subscriptions.
        &[]
    }

    fn can_be_disabled(&self) -> bool {
        true
    }

    fn has_file_check(&self) -> bool {
        true
    }

    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let root = ctx.tree.root_node();
        let source = ctx.source;

        // 1. File name validation (BDFIL).
        diagnostics.extend(self.check_file_name(ctx));

        // 2. Collect skip ranges (comments and strings) for source scanning.
        let mut skip_ranges = Vec::new();
        Self::collect_skip_ranges(root, &mut skip_ranges);

        // 3. Source text scanning (BADNE, BADOT, TWOCM, BADCH, BADSP).
        diagnostics.extend(self.scan_source_text(source, &skip_ranges));

        // 4. Tree ERROR/MISSING node analysis (SYNER, NOPAR, ENDCT, EOFMI).
        diagnostics.extend(self.check_error_nodes(root, source));

        // 5. File structure validation (CLIS, CLTWO, SOFOC, SEMFU).
        diagnostics.extend(self.check_file_structure(root, source));

        // 6. Function name validation (FNDOT, FNSWA).
        diagnostics.extend(self.check_function_names(root, source));

        // 7. Statement validation (NOLHS, SEPEXR, REDEF).
        diagnostics.extend(self.check_statements(root, source));

        diagnostics
    }
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "SYNTAX_ERRORS_ENGINE",
    SyntaxErrorsEngine::from_config
));
