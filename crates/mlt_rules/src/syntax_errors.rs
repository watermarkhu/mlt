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
//! | NOPAR2   | Missing closing bracket mid-file                         |
//! | EOLPAR   | Missing closing bracket at end of line                   |
//! | ENDPAR   | Missing closing bracket at end of file                   |
//! | ENDCT    | ERROR node suggesting missing END                        |
//! | ENDCT2   | An END might be missing after a block-opening keyword    |
//! | ENDCT3   | An END might be missing before a block-opening keyword   |
//! | ENDCT4   | A METHODS block or END might be missing before a function definition |
//! | EOFMI    | File ends with ERROR node (incomplete)                   |
//! | NOLHS    | Assignment with empty left side                          |
//! | BADCH    | Invalid control characters in source                     |
//! | BADSP    | Non-ASCII whitespace characters in source                |
//! | BADCT    | Unicode explicit directional formatting characters       |
//! | REDEF    | Same identifier used as both function name and variable  |
//! | SEPEXR   | Missing newline/semicolon between statements             |
//! | SBTMP    | Chaining outputs after parenthesis is not supported      |
//! | FVSYN    | Invalid function argument syntax                         |
//! | FVACI    | Name-value arguments in cell indexing not supported      |
//! | FVACS    | Quoted string used as name in name=value syntax          |
//! | FVAMI    | Name in name=value syntax is not a valid identifier      |
//! | UNSET    | Invalid use of operator on the left side of an assignment |
//! | LHROW    | Assignment left side cannot have multiple rows (';')      |
//! | RESWD    | Invalid use of a reserved word                           |
//! | SYNEND   | Invalid use for END operator                             |
//! | MCPLD    | Invalid property syntax                                  |
//! | BADNOT   | Using ~ to ignore a value is not permitted               |
//! | BADNOTLHS| Invalid use of logical not operator (~) on LHS           |
//! | STRIN    | A quoted character vector is unterminated                |
//! | DOUQT    | A double quoted string is unterminated                   |
//! | INBLK    | A block comment is unterminated at the end of the file   |
//! | BADFP    | Invalid floating-point constant (e.g., truncated `1.2.3`) |
//! | BADHBH   | Invalid digit in a hexadecimal literal                   |
//! | BADHBB   | Invalid digit in a binary literal                        |
//! | BADHBHT  | Hex literal has too many digits for its type suffix      |
//! | BADHBBT  | Binary literal has too many digits for its type suffix   |
//! | HEXTOOLONG | Hex literal has too many digits (max 16 without suffix) |
//! | BINARYTOOLONG | Binary literal has too many digits (max 64 without suffix) |
//! | VTPOD    | Specify validation in the following order: size, then class, then functions |
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

/// Reserved words that cannot be used as identifiers or standalone
/// expressions in MATLAB.
const RESERVED_KEYWORDS: &[&str] = &[
    "else", "end", "if", "for", "while", "switch", "case", "otherwise",
    "classdef", "function", "try", "catch", "break", "continue", "return",
    "persistent", "global", "parfor", "spmd", "properties", "methods",
    "events", "enumeration",
];

/// Keywords that open a block which must be closed with a matching `end`.
const BLOCK_OPENERS: &[&str] = &[
    "if", "for", "while", "switch", "try", "function", "classdef",
    "properties", "methods", "events", "enumeration",
];

/// Known MATLAB class names that must be specified before validation functions
/// in an arguments block.
const KNOWN_CLASS_NAMES: &[&str] = &[
    "double", "single", "int8", "int16", "int32", "int64",
    "uint8", "uint16", "uint32", "uint64",
    "char", "string", "logical", "cell", "struct", "function_handle",
    "table", "timetable", "categorical", "datetime", "duration",
];

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

    /// Scan source text for BADCT: Unicode explicit directional formatting
    /// characters, which MATLAB does not support.
    fn check_directional_formatting(&self, source: &str) -> Vec<Diagnostic> {
        if !self.is_check_enabled("BADCT") {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        let mut line = 1usize;
        let mut line_start = 0usize;

        for (i, ch) in source.char_indices() {
            if ch == '\n' {
                line += 1;
                line_start = i + 1;
                continue;
            }
            let cp = ch as u32;
            // U+202A..U+202E (LRE, RLE, PDF, LRO, RLO) and
            // U+2066..U+2069 (LRI, RLI, FSI, PDI) are directional formatting
            // characters that MATLAB rejects.
            if (0x202A..=0x202E).contains(&cp) || (0x2066..=0x2069).contains(&cp) {
                diagnostics.push(Diagnostic {
                    rule_id: "BADCT",
                    message: "Unicode explicit directional formatting characters are not supported."
                        .to_string(),
                    severity: Severity::Error,
                    byte_range: i..i + ch.len_utf8(),
                    line,
                    column: i - line_start + 1,
                    fix: None,
                });
            }
        }

        diagnostics
    }

    // -----------------------------------------------------------------------
    // Tree ERROR/MISSING node analysis (SYNER, NOPAR2, EOLPAR, ENDPAR, ENDCT, EOFMI)
    // -----------------------------------------------------------------------

    /// Walk the tree and collect diagnostics for ERROR and MISSING nodes.
    fn check_error_nodes(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let last = Self::last_descendant(root);
        self.walk_error_nodes(root, source, last, &mut diagnostics);

        // ENDPAR: Check if the file ends with a missing closing bracket.
        if self.is_check_enabled("ENDPAR") {
            if let Some(last) = last {
                let start = last.start_byte();
                let end = last.end_byte().max(start + 1);
                let text = &source[start..end.min(source.len())];
                let is_bracket_error =
                    last.is_error() && Self::looks_like_missing_bracket(text);
                let missing_kind = Self::missing_bracket_opener(last.kind());
                if is_bracket_error || (last.is_missing() && missing_kind.is_some()) {
                    let pos = last.start_position();
                    let bracket = Self::bracket_kinds(text)
                        .first()
                        .copied()
                        .or(missing_kind)
                        .unwrap_or("bracket");
                    diagnostics.push(Diagnostic {
                        rule_id: "ENDPAR",
                        message: format!(
                            "A {bracket} might be missing a closing {bracket}, causing invalid syntax at end of file."
                        ),
                        severity: Severity::Error,
                        byte_range: start..end,
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }

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
        last: Option<Node>,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.is_error() {
            let start = node.start_byte();
            let end = node.end_byte().max(start + 1);
            let pos = node.start_position();
            let text = &source[start..end.min(source.len())];

            // Classify the error node.
            let endct_variant = if self.is_check_enabled("ENDCT") {
                Self::classify_missing_end(&node, source)
            } else {
                None
            };

            // NOPAR2: Looks like a missing closing bracket.
            if Self::looks_like_missing_bracket(text) {
                let is_last = last.is_some_and(|l| l.id() == node.id());
                if self.is_check_enabled("NOPAR2") && !is_last {
                    let bracket = Self::bracket_kinds(text)
                        .first()
                        .copied()
                        .unwrap_or("bracket");
                    diagnostics.push(Diagnostic {
                        rule_id: "NOPAR2",
                        message: format!(
                            "A {bracket} might be missing a closing {bracket}, causing invalid syntax at {bracket} on line {}.",
                            pos.row + 1
                        ),
                        severity: Severity::Error,
                        byte_range: start..end,
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
            // ENDCT: Looks like a missing END.
            else if let Some((variant_id, variant_msg)) = endct_variant {
                if self.is_check_enabled(variant_id) {
                    diagnostics.push(Diagnostic {
                        rule_id: variant_id,
                        message: variant_msg,
                        severity: Severity::Error,
                        byte_range: start..end,
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                } else {
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
            } else if self.is_check_enabled("ENDCT") && Self::looks_like_missing_end(text, &node) {
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
            // FVSYN: Invalid function argument syntax. Fires for ERROR nodes
            // that appear inside a function call's argument list. Replaces
            // SYNER for these nodes (no double-fire).
            else if self.is_check_enabled("FVSYN")
                && Self::is_in_function_call_args(&node)
                && !text.contains('=')
            {
                diagnostics.push(Diagnostic {
                    rule_id: "FVSYN",
                    message: "Invalid function argument syntax".to_string(),
                    severity: Severity::Error,
                    byte_range: start..end,
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
            // SYNER: Generic syntax error.
            else if self.is_check_enabled("SYNER")
                && !(Self::is_in_function_call_args(&node) && text.contains('='))
            {
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
        } else if node.is_missing() {
            if let Some(bracket) = Self::missing_bracket_opener(node.kind()) {
                let is_last = last.is_some_and(|l| l.id() == node.id());
                if !is_last {
                    let start = node.start_byte();
                    let end = node.end_byte().max(start + 1);
                    let pos = node.start_position();
                    if Self::is_end_of_file(node, source) {
                        if self.is_check_enabled("ENDPAR") {
                            diagnostics.push(Diagnostic {
                                rule_id: "ENDPAR",
                                message: format!(
                                    "A {bracket} might be missing a closing {bracket}, causing invalid syntax at end of file."
                                ),
                                severity: Severity::Error,
                                byte_range: start..end,
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        }
                    } else if Self::is_line_ending(node, source) {
                        if self.is_check_enabled("EOLPAR") {
                            diagnostics.push(Diagnostic {
                                rule_id: "EOLPAR",
                                message: format!(
                                    "A {bracket} might be missing a closing {bracket}, causing invalid syntax at end of line."
                                ),
                                severity: Severity::Error,
                                byte_range: start..end,
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        }
                    } else if self.is_check_enabled("NOPAR2") {
                        diagnostics.push(Diagnostic {
                            rule_id: "NOPAR2",
                            message: format!(
                                "A {bracket} might be missing a closing {bracket}, causing invalid syntax at {bracket} on line {}.",
                                pos.row + 1
                            ),
                            severity: Severity::Error,
                            byte_range: start..end,
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            } else if self.is_check_enabled("SYNER") {
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
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_error_nodes(child, source, last, diagnostics);
        }
    }

    /// Heuristic: does this error text look like an unclosed bracket?
    fn looks_like_missing_bracket(text: &str) -> bool {
        let opens: usize = text.chars().filter(|&c| c == '(' || c == '[' || c == '{').count();
        let closes: usize = text.chars().filter(|&c| c == ')' || c == ']' || c == '}').count();
        opens > closes
    }

    /// Detect which bracket types are unbalanced (more opens than closes) in
    /// the given error text.
    fn bracket_kinds(text: &str) -> Vec<&str> {
        let mut kinds = Vec::new();
        for (open, close, kind) in [('(', ')', "("), ('[', ']', "["), ('{', '}', "{")] {
            let opens = text.chars().filter(|&c| c == open).count();
            let closes = text.chars().filter(|&c| c == close).count();
            if opens > closes {
                kinds.push(kind);
            }
        }
        kinds
    }

    /// Returns the matching opener for a missing closing bracket kind.
    fn missing_bracket_opener(kind: &str) -> Option<&'static str> {
        match kind {
            ")" => Some("("),
            "]" => Some("["),
            "}" => Some("{"),
            _ => None,
        }
    }

    /// True if only line terminators, whitespace, `;`, or `,` follow the node
    /// before end of file.
    fn is_end_of_file(node: Node, source: &str) -> bool {
        source[node.end_byte()..]
            .bytes()
            .all(|b| matches!(b, b'\n' | b'\r' | b' ' | b'\t' | b';' | b','))
    }

    /// True if the node is the last content on its line (only line terminators,
    /// whitespace, `;`, or `,` follow it up to the next newline).
    fn is_line_ending(node: Node, source: &str) -> bool {
        let rest = &source[node.end_byte()..];
        let line_end = rest.find('\n').unwrap_or(rest.len());
        rest[..line_end]
            .bytes()
            .all(|b| matches!(b, b'\r' | b' ' | b'\t' | b';' | b','))
    }

    /// Heuristic: does this error look like a missing `end`?
    fn looks_like_missing_end(text: &str, node: &Node) -> bool {
        // Check text for block openers without matching end.
        let lower = text.to_lowercase();
        let has_opener = BLOCK_OPENERS.iter().any(|kw| lower.contains(kw));
        let has_end = lower.contains("end");

        if has_opener && !has_end {
            return true;
        }

        // Check if previous sibling is a block statement.
        if let Some(prev) = node.prev_sibling() {
            let kind = prev.kind();
            if BLOCK_OPENERS.contains(&kind) {
                return true;
            }
        }

        false
    }

    /// True if the given ERROR node appears inside a function call's argument
    /// list: either directly inside an `arguments` node whose parent is a
    /// `function_call`, or as a sibling of such an `arguments` node within
    /// the call.
    fn is_in_function_call_args(node: &Node) -> bool {
        let mut current = node.parent();
        while let Some(ancestor) = current {
            match ancestor.kind() {
                "arguments" => {
                    return ancestor
                        .parent()
                        .is_some_and(|p| p.kind() == "function_call");
                }
                "function_call" => {
                    // The ERROR is a direct child of the call. It is within the
                    // argument list when the call also has an `arguments` child.
                    let mut cursor = ancestor.walk();
                    return ancestor
                        .children(&mut cursor)
                        .any(|c| c.kind() == "arguments");
                }
                _ => {}
            }
            current = ancestor.parent();
        }
        false
    }

    /// Classify a missing-END error node into a specific ENDCT variant.
    ///
    /// Returns the check ID and message for the first matching variant:
    /// ENDCT4 (classdef followed by a function definition without a
    /// METHODS block), ENDCT2 (a block opener appears in the error text or
    /// the previous sibling), or ENDCT3 (the error is followed by a sibling
    /// that starts with a block-opening keyword).
    fn classify_missing_end(node: &Node, source: &str) -> Option<(&'static str, String)> {
        let start = node.start_byte();
        let node_end = node.end_byte().min(source.len());
        let text = &source[start..node_end];
        let lower = text.to_lowercase();
        let line = node.start_position().row + 1;

        // ENDCT4: ERROR text contains "classdef", a function_definition
        // follows it as a sibling, and no "methods" appears between them.
        if lower.contains("classdef") {
            let mut sibling = node.next_sibling();
            while let Some(sib) = sibling {
                if sib.kind() == "function_definition" && sib.start_byte() > start {
                    let between = &source[node_end..sib.start_byte()];
                    if !between.to_lowercase().contains("methods") {
                        return Some((
                            "ENDCT4",
                            "A METHODS block or END might be missing before the function definition. This might be causing additional error messages.".to_string(),
                        ));
                    }
                    break;
                }
                sibling = sib.next_sibling();
            }
        }

        // ENDCT2: a block opener appears in the ERROR text or its previous
        // sibling, and the ERROR does not contain `end`.
        let has_end = lower.contains("end");
        if !has_end {
            let opener_in_text = BLOCK_OPENERS.iter().any(|kw| lower.contains(kw));
            let opener_in_prev = node
                .prev_sibling()
                .map(|prev| {
                    let prev_text = &source[prev.start_byte()..prev.end_byte()];
                    BLOCK_OPENERS.iter().any(|kw| prev_text.to_lowercase().contains(kw))
                })
                .unwrap_or(false);
            if opener_in_text || opener_in_prev {
                let raw_end = node.end_position().row;
                let end_line = if source[..node_end].ends_with('\n') {
                    raw_end
                } else {
                    raw_end + 1
                };
                let opener_line = if opener_in_text {
                    line
                } else {
                    node.prev_sibling()
                        .map(|p| p.start_position().row + 1)
                        .unwrap_or(line)
                };
                return Some((
                    "ENDCT2",
                    format!(
                        "An END might be missing (after line {end_line}), possibly matching line {opener_line}."
                    ),
                ));
            }
        }

        // ENDCT3: the ERROR is followed by a sibling that starts with a
        // block-opening keyword.
        let mut sibling = node.next_sibling();
        while let Some(sib) = sibling {
            let sib_text = &source[sib.start_byte()..sib.end_byte()];
            match sib_text.split_whitespace().next() {
                Some(first) if BLOCK_OPENERS.contains(&first.to_lowercase().as_str()) => {
                    let kw = text
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_uppercase();
                    let matched = first.to_uppercase();
                    return Some((
                        "ENDCT3",
                        format!(
                            "An END might be missing (before {kw} on line {line}), possibly matching {matched}."
                        ),
                    ));
                }
                Some(_) => break,
                None => {}
            }
            sibling = sib.next_sibling();
        }

        None
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
    // Assignment LHS validation (UNSET, LHROW)
    // -----------------------------------------------------------------------

    /// Check assignment left-hand sides for invalid operator targets (UNSET)
    /// and multi-row output lists (LHROW).
    fn check_assignment_lhs(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        self.walk_assignment_lhs(root, source, &mut diagnostics);
        diagnostics
    }

    /// Recursively walk assignment nodes and ERROR nodes to validate LHS.
    fn walk_assignment_lhs(&self, node: Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        const OPERATOR_LHS_KINDS: [&str; 5] = [
            "comparison_operator",
            "binary_operator",
            "boolean_operator",
            "unary_operator",
            "postfix_operator",
        ];

        let kind = node.kind();

        if kind == "assignment" {
            let lhs = node.child_by_field_name("left");
            if let Some(lhs) = lhs {
                let pos = lhs.start_position();

                // UNSET: operator expression on the left side of `=`.
                if self.is_check_enabled("UNSET") && OPERATOR_LHS_KINDS.contains(&lhs.kind()) {
                    diagnostics.push(Diagnostic {
                        rule_id: "UNSET",
                        message:
                            "Invalid use of operator on the left side of an assignment"
                                .to_string(),
                        severity: Severity::Error,
                        byte_range: lhs.byte_range(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }

                // LHROW: multi-output LHS containing a row separator (`;`).
                if self.is_check_enabled("LHROW") && lhs.kind() == "multioutput_variable" {
                    let lhs_text = &source[lhs.start_byte()..lhs.end_byte()];
                    if lhs_text.contains(';') {
                        diagnostics.push(Diagnostic {
                            rule_id: "LHROW",
                            message: "The left side of an assignment cannot have multiple rows (';')"
                                .to_string(),
                            severity: Severity::Error,
                            byte_range: lhs.byte_range(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            }
        }

        // UNSET fallback: the parser may emit an ERROR node (text starting
        // with `=`) as the sibling of a statement-level operator expression
        // instead of an assignment node (e.g., `x == 5 = 3;`).
        if self.is_check_enabled("UNSET") && node.is_error() {
            if let Some(prev) = node.prev_sibling() {
                if OPERATOR_LHS_KINDS.contains(&prev.kind()) {
                    let err_text = &source[node.start_byte()..node.end_byte()];
                    if err_text.trim_start().starts_with('=') {
                        let pos = prev.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "UNSET",
                            message: "Invalid use of operator on the left side of an assignment"
                                .to_string(),
                            severity: Severity::Error,
                            byte_range: prev.byte_range(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_assignment_lhs(child, source, diagnostics);
        }
    }

    // -----------------------------------------------------------------------
    // Reserved words and invalid ~ usage (RESWD, SYNEND, MCPLD, BADNOT,
    // BADNOTLHS)
    // -----------------------------------------------------------------------

    /// Check for reserved words used as identifiers (RESWD), invalid `end`
    /// usage (SYNEND), invalid property syntax (MCPLD), and invalid `~` usage
    /// (BADNOT, BADNOTLHS).
    fn check_reserved_and_not(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        self.walk_reserved_and_not(root, source, &mut diagnostics);
        diagnostics
    }

    /// Recursively walk the tree for reserved-word and `~` misuse.
    fn walk_reserved_and_not(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let kind = node.kind();

        if node.is_error() {
            let start = node.start_byte();
            let end = node.end_byte().max(start + 1);
            let pos = node.start_position();
            let text = &source[start..end.min(source.len())];

            // MCPLD: ERROR inside a property declaration in a properties block.
            if self.is_check_enabled("MCPLD") && Self::inside_property(node) {
                let name = Self::ancestor_of_kind(node, "property")
                    .and_then(|prop| Self::first_child_text(prop, "identifier", source))
                    .unwrap_or("(unknown)");
                diagnostics.push(Diagnostic {
                    rule_id: "MCPLD",
                    message: format!("Invalid property syntax at {name}"),
                    severity: Severity::Error,
                    byte_range: start..end,
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }

            // SYNEND takes priority over RESWD for `end`.
            if self.is_check_enabled("SYNEND") && Self::has_descendant_kind(node, "end_keyword")
            {
                diagnostics.push(Diagnostic {
                    rule_id: "SYNEND",
                    message: "Invalid use for END operator".to_string(),
                    severity: Severity::Error,
                    byte_range: start..end,
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            } else if self.is_check_enabled("RESWD") {
                if let Some(kw) = Self::contains_reserved_word(text) {
                    if kw != "end" {
                        diagnostics.push(Diagnostic {
                            rule_id: "RESWD",
                            message: "Invalid use of a reserved word.".to_string(),
                            severity: Severity::Error,
                            byte_range: start..end,
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            }

            // BADNOT: bare `~` statement.
            if self.is_check_enabled("BADNOT") && text.trim() == "~" {
                diagnostics.push(Diagnostic {
                    rule_id: "BADNOT",
                    message: "Using ~ to ignore a value is not permitted in this context."
                        .to_string(),
                    severity: Severity::Error,
                    byte_range: start..end,
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        }

        // BADNOT: `~` used as a value operator with a malformed operand.
        if kind == "not_operator"
            && self.is_check_enabled("BADNOT")
            && Self::has_error_or_missing_descendant(node)
        {
            let start = node.start_byte();
            let end = node.end_byte();
            let pos = node.start_position();
            diagnostics.push(Diagnostic {
                rule_id: "BADNOT",
                message: "Using ~ to ignore a value is not permitted in this context."
                    .to_string(),
                severity: Severity::Error,
                byte_range: start..end,
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            });
        }

        // BADNOTLHS: `~` adjacent to an output variable without a comma.
        if kind == "multioutput_variable" && self.is_check_enabled("BADNOTLHS") {
            let mut cursor = node.walk();
            let children: Vec<Node> = node.children(&mut cursor).collect();
            for (i, child) in children.iter().enumerate() {
                if child.kind() != "ignored_argument" {
                    continue;
                }
                let prev_is_id = i > 0 && children[i - 1].kind() == "identifier";
                let next_is_id = i + 1 < children.len() && children[i + 1].kind() == "identifier";
                if prev_is_id || next_is_id {
                    let pos = child.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "BADNOTLHS",
                        message: "Invalid use of logical not operator (~) on left side of an assignment. To use ~ to ignore function outputs, separate output variables with commas."
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: child.byte_range(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_reserved_and_not(child, source, diagnostics);
        }
    }

    /// Returns the first reserved keyword appearing as a whole word in `text`.
    fn contains_reserved_word(text: &str) -> Option<&'static str> {
        let lower = text.to_lowercase();
        RESERVED_KEYWORDS
            .iter()
            .find(|&&kw| Self::contains_whole_word(&lower, kw))
            .copied()
    }

    /// Whole-word substring check: `needle` must not be part of a larger word.
    fn contains_whole_word(haystack: &str, needle: &str) -> bool {
        let mut search_start = 0;
        while let Some(rel) = haystack[search_start..].find(needle) {
            let idx = search_start + rel;
            let prev = haystack[..idx].chars().next_back();
            let next = haystack[idx + needle.len()..].chars().next();
            let left_ok = prev.is_none_or(|c| !c.is_ascii_alphanumeric() && c != '_');
            let right_ok = next.is_none_or(|c| !c.is_ascii_alphanumeric() && c != '_');
            if left_ok && right_ok {
                return true;
            }
            search_start = idx + needle.len();
        }
        false
    }

    /// Returns true if `node` or any of its descendants has the given kind.
    fn has_descendant_kind(node: Node, kind: &str) -> bool {
        if node.kind() == kind {
            return true;
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if Self::has_descendant_kind(child, kind) {
                return true;
            }
        }
        false
    }

    /// Returns true if the node's subtree contains an ERROR or MISSING node.
    fn has_error_or_missing_descendant(node: Node) -> bool {
        if node.is_error() || node.is_missing() {
            return true;
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if Self::has_error_or_missing_descendant(child) {
                return true;
            }
        }
        false
    }

    /// Returns true if `node` is a descendant of a `property` node inside a
    /// `properties` block.
    fn inside_property(node: Node) -> bool {
        let mut current = node.parent();
        while let Some(n) = current {
            match n.kind() {
                "property" => return true,
                "properties" => return false,
                _ => {}
            }
            current = n.parent();
        }
        false
    }

    /// Returns the nearest ancestor of `node` with the given kind.
    fn ancestor_of_kind<'tree>(node: Node<'tree>, kind: &str) -> Option<Node<'tree>> {
        let mut current = node.parent();
        while let Some(n) = current {
            if n.kind() == kind {
                return Some(n);
            }
            current = n.parent();
        }
        None
    }

    /// Returns the text of the first child of `node` with the given kind.
    fn first_child_text<'tree>(
        node: Node<'tree>,
        kind: &str,
        source: &'tree str,
    ) -> Option<&'tree str> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == kind {
                return Some(&source[child.start_byte()..child.end_byte()]);
            }
        }
        None
    }

    // -----------------------------------------------------------------------
    // Unterminated strings and comments (STRIN, DOUQT, INBLK)
    // -----------------------------------------------------------------------

    /// Check for unterminated strings and block comments (STRIN, DOUQT, INBLK).
    fn check_unterminated(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut reported: HashSet<usize> = HashSet::new();

        if self.is_check_enabled("STRIN") || self.is_check_enabled("DOUQT") {
            Self::walk_unterminated_strings(root, source, &mut diagnostics, &mut reported);
        }

        if self.is_check_enabled("INBLK") {
            Self::walk_unterminated_comments(root, source, &mut diagnostics);
        }

        diagnostics
    }

    /// Recursively walk ERROR nodes for unterminated string markers.
    ///
    /// An unterminated single-quoted character vector (STRIN) appears either
    /// as an ERROR node whose text starts with an unmatched `'` (odd quote
    /// count, so a closed vector like `'abc'` never matches) or as an ERROR
    /// node with a direct `'` token child. A valid transpose (`A'`, `A.'`)
    /// parses as a `postfix_operator` and never produces an ERROR node. An
    /// unterminated double-quoted string (DOUQT) appears as an ERROR node
    /// with a direct `"` token child.
    fn walk_unterminated_strings(
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
        reported: &mut HashSet<usize>,
    ) {
        if node.is_error() {
            // STRIN: ERROR node whose text starts with an unmatched quote.
            let start = node.start_byte();
            let end = node.end_byte().min(source.len());
            let text = &source[start..end];
            if text.starts_with('\'')
                && text.matches('\'').count() % 2 == 1
                && !reported.contains(&start)
                && !Self::is_transpose_position(source, start)
            {
                let pos = node.start_position();
                reported.insert(start);
                diagnostics.push(Diagnostic {
                    rule_id: "STRIN",
                    message: "A quoted character vector is unterminated.".to_string(),
                    severity: Severity::Error,
                    byte_range: start..start + 1,
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }

            // STRIN/DOUQT: direct quote tokens inside the ERROR node.
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "'" {
                    let pos = child.start_byte();
                    if !reported.contains(&pos) && !Self::is_transpose_position(source, pos) {
                        let cpos = child.start_position();
                        reported.insert(pos);
                        diagnostics.push(Diagnostic {
                            rule_id: "STRIN",
                            message: "A quoted character vector is unterminated.".to_string(),
                            severity: Severity::Error,
                            byte_range: pos..pos + 1,
                            line: cpos.row + 1,
                            column: cpos.column + 1,
                            fix: None,
                        });
                    }
                } else if child.kind() == "\"" {
                    let pos = child.start_byte();
                    if !reported.contains(&pos) {
                        let cpos = child.start_position();
                        reported.insert(pos);
                        diagnostics.push(Diagnostic {
                            rule_id: "DOUQT",
                            message: "A double quoted string is unterminated.".to_string(),
                            severity: Severity::Error,
                            byte_range: pos..pos + 1,
                            line: cpos.row + 1,
                            column: cpos.column + 1,
                            fix: None,
                        });
                    }
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::walk_unterminated_strings(child, source, diagnostics, reported);
        }
    }

    /// Recursively walk comment nodes for unterminated block comments (INBLK).
    fn walk_unterminated_comments(
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "comment" {
            let text = &source[node.start_byte()..node.end_byte()];
            if text.starts_with("%{") && !text.contains("%}") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "INBLK",
                    message: "A block comment is unterminated at the end of the file."
                        .to_string(),
                    severity: Severity::Error,
                    byte_range: node.byte_range(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
            return;
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::walk_unterminated_comments(child, source, diagnostics);
        }
    }

    /// A `'` following a value (alphanumeric, `)`, `]`, `}`, `.`) is the
    /// transpose operator, not a string delimiter.
    fn is_transpose_position(source: &str, pos: usize) -> bool {
        match source[..pos].chars().next_back() {
            Some(c) => c.is_ascii_alphanumeric() || c == ')' || c == ']' || c == '}' || c == '.',
            None => false,
        }
    }

    // -----------------------------------------------------------------------
    // Number literal validation (BADFP, BADHBH, BADHBB, BADHBHT, BADHBBT,
    // HEXTOOLONG, BINARYTOOLONG)
    // -----------------------------------------------------------------------

    /// Walk the tree and validate number literals.
    ///
    /// Truncated literals (BADFP, BADHBH, BADHBB) are detected via the ERROR
    /// node that immediately follows a `number` node and holds the invalid
    /// remainder. Digit-count checks (BADHBHT, BADHBBT, HEXTOOLONG,
    /// BINARYTOOLONG) analyze the `number` node text directly.
    fn check_number_literals(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut numbers = Vec::new();
        let mut errors = Vec::new();
        Self::collect_numbers_and_errors(root, &mut numbers, &mut errors);
        errors.sort_by_key(|node| node.start_byte());

        for number in numbers {
            let start = number.start_byte();
            let end = number.end_byte();
            let text = &source[start..end];

            // Truncated literal checks: an ERROR node immediately following
            // the number holds the invalid remainder of the literal.
            if !Self::inside_error(number) {
                if let Ok(idx) = errors.binary_search_by_key(&end, |n| n.start_byte()) {
                    let err_text = &source[errors[idx].byte_range()];
                    if let Some(first) = err_text.chars().next() {
                        let literal_like = first.is_ascii_alphanumeric() || first == '_';
                        if (text.starts_with("0x") || text.starts_with("0X"))
                            && self.is_check_enabled("BADHBH")
                            && literal_like
                        {
                            diagnostics.push(Self::number_diagnostic(
                                "BADHBH",
                                "Invalid digit in hexadecimal literal. Supported hex digits are 0-9 and A-F. Supported type suffixes are u8,u16,u32,u64 and s8,s16,s32,s64.",
                                number,
                            ));
                        } else if (text.starts_with("0b") || text.starts_with("0B"))
                            && self.is_check_enabled("BADHBB")
                            && literal_like
                        {
                            diagnostics.push(Self::number_diagnostic(
                                "BADHBB",
                                "Invalid digit in binary literal. Supported binary digits are 0 and 1. Supported type suffixes are u8,u16,u32,u64 and s8,s16,s32,s64.",
                                number,
                            ));
                        } else if self.is_check_enabled("BADFP")
                            && (first == '.' || first.is_ascii_digit())
                        {
                            diagnostics.push(Self::number_diagnostic(
                                "BADFP",
                                "Invalid floating-point constant",
                                number,
                            ));
                        }
                    }
                }
            }

            // Digit-count checks for hex and binary literals.
            if let Some((is_hex, digit_count, suffix)) =
                Self::parse_hex_binary_literal(text)
            {
                match suffix {
                    Some(suffix) => {
                        if let Some(max) = Self::max_digits_for_suffix(suffix, is_hex) {
                            if digit_count > max {
                                let (rule_id, message) = if is_hex {
                                    (
                                        "BADHBHT",
                                        "Hexadecimal literal has too many digits for specified type suffix",
                                    )
                                } else {
                                    (
                                        "BADHBBT",
                                        "Binary literal has too many digits for specified type suffix",
                                    )
                                };
                                if self.is_check_enabled(rule_id) {
                                    diagnostics.push(Self::number_diagnostic(
                                        rule_id,
                                        message,
                                        number,
                                    ));
                                }
                            }
                        }
                    }
                    None => {
                        if is_hex && digit_count > 16 && self.is_check_enabled("HEXTOOLONG") {
                            diagnostics.push(Self::number_diagnostic(
                                "HEXTOOLONG",
                                "Hexadecimal literal has too many digits",
                                number,
                            ));
                        } else if !is_hex
                            && digit_count > 64
                            && self.is_check_enabled("BINARYTOOLONG")
                        {
                            diagnostics.push(Self::number_diagnostic(
                                "BINARYTOOLONG",
                                "Binary literal has too many digits",
                                number,
                            ));
                        }
                    }
                }
            }
        }

        diagnostics
    }

    /// Collect all `number` nodes and all ERROR nodes in document order.
    fn collect_numbers_and_errors<'a>(
        node: Node<'a>,
        numbers: &mut Vec<Node<'a>>,
        errors: &mut Vec<Node<'a>>,
    ) {
        if node.kind() == "number" {
            numbers.push(node);
        }
        if node.is_error() {
            errors.push(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_numbers_and_errors(child, numbers, errors);
        }
    }

    /// True if the node is contained within an ERROR node.
    fn inside_error(mut node: Node) -> bool {
        while let Some(parent) = node.parent() {
            if parent.is_error() {
                return true;
            }
            node = parent;
        }
        false
    }

    /// Parse a hex or binary literal into (is_hex, digit count, type suffix).
    fn parse_hex_binary_literal(text: &str) -> Option<(bool, usize, Option<&str>)> {
        let (is_hex, rest) = if text.starts_with("0x") || text.starts_with("0X") {
            (true, &text[2..])
        } else if text.starts_with("0b") || text.starts_with("0B") {
            (false, &text[2..])
        } else {
            return None;
        };

        if rest.is_empty() {
            return None;
        }

        let suffix = if rest.len() >= 3 && Self::suffix_bits(&rest[rest.len() - 3..]).is_some()
        {
            Some(&rest[rest.len() - 3..])
        } else if rest.len() >= 2 && Self::suffix_bits(&rest[rest.len() - 2..]).is_some() {
            Some(&rest[rest.len() - 2..])
        } else {
            None
        };

        let digits = match suffix {
            Some(s) => &rest[..rest.len() - s.len()],
            None => rest,
        };

        if digits.is_empty() {
            return None;
        }

        Some((is_hex, digits.chars().count(), suffix))
    }

    /// Bit width for a valid type suffix (`u8`/`s8` ... `u64`/`s64`), or None.
    fn suffix_bits(suffix: &str) -> Option<usize> {
        if suffix.len() < 2 {
            return None;
        }
        let kind = suffix.as_bytes()[0];
        let is_valid_kind = kind == b'u' || kind == b'U' || kind == b's' || kind == b'S';
        if !is_valid_kind {
            return None;
        }
        match &suffix[1..] {
            "8" => Some(8),
            "16" => Some(16),
            "32" => Some(32),
            "64" => Some(64),
            _ => None,
        }
    }

    /// Maximum number of digits a type suffix allows for a literal.
    fn max_digits_for_suffix(suffix: &str, is_hex: bool) -> Option<usize> {
        let bits = Self::suffix_bits(suffix)?;
        if is_hex {
            Some(bits / 4)
        } else {
            Some(bits)
        }
    }

    /// Build an Error-severity diagnostic pointing at a number literal node.
    fn number_diagnostic(rule_id: &'static str, message: &str, node: Node) -> Diagnostic {
        let pos = node.start_position();
        Diagnostic {
            rule_id,
            message: message.to_string(),
            severity: Severity::Error,
            byte_range: node.byte_range(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        }
    }

    // -----------------------------------------------------------------------
    // Call/argument syntax validation (SBTMP, FVACI, FVACS, FVAMI)
    // -----------------------------------------------------------------------

    /// Check call and name=value argument syntax (SBTMP, FVACI, FVACS, FVAMI).
    fn check_call_syntax(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        self.walk_call_syntax(root, source, &mut diagnostics);
        diagnostics
    }

    /// Recursive DFS visiting function calls and their argument lists.
    fn walk_call_syntax(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "function_call" {
            self.check_function_call(node, source, diagnostics);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_call_syntax(child, source, diagnostics);
        }
    }

    /// Check a single function_call node for SBTMP, FVACI, FVACS, and FVAMI.
    fn check_function_call(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // SBTMP: chaining outputs after parenthesis is not supported.
        if self.is_check_enabled("SBTMP") {
            if let Some(name) = node.child_by_field_name("name") {
                if name.kind() == "function_call" {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "SBTMP",
                        message: "Invalid array indexing or function call. Chaining outputs after parenthesis is not supported."
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: node.byte_range(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }

        let is_brace = (0..node.child_count())
            .any(|i| node.child(i).is_some_and(|c| c.kind() == "{"));
        let args = Self::arguments_child(node);

        // FVACI: name=value syntax in cell indexing (`{}` calls).
        if self.is_check_enabled("FVACI") && is_brace {
            if let Some(a) = args {
                if Self::has_eq_token(a) {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "FVACI",
                        message: "Use of name-value arguments in cell indexing is not supported."
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: node.byte_range(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }

        // FVACS/FVAMI: name=value with an invalid name in a call.
        if self.is_check_enabled("FVACS") || self.is_check_enabled("FVAMI") {
            self.check_name_value(node, args, source, diagnostics);
        }
    }

    /// Check a call's argument list for name=value syntax with an invalid name
    /// (FVACS for quoted names, FVAMI for non-identifier names).
    fn check_name_value(
        &self,
        call: Node,
        args: Option<Node>,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // Assignment shape: `f('Bad Name' = 1)` parses as an assignment whose
        // LHS is a function_call with a MISSING closing paren.
        let is_assignment_lhs = call.parent().is_some_and(|p| {
            p.kind() == "assignment"
                && p.child_by_field_name("left")
                    .is_some_and(|l| l.id() == call.id())
        });
        let has_missing_paren = (0..call.child_count()).any(|i| {
            call.child(i)
                .is_some_and(|c| c.is_missing() && c.kind() == ")")
        });
        if is_assignment_lhs && has_missing_paren {
            if let Some(name) = args.and_then(Self::last_named_child) {
                self.push_name_value_diagnostic(name, source, diagnostics);
            }
        }

        // ERROR shape: an ERROR node containing `=` inside the call's args.
        for e in Self::call_error_nodes(call) {
            let text = &source[e.start_byte()..e.end_byte()];
            if !text.contains('=') {
                continue;
            }
            let name = if Self::has_string_child(e) {
                e
            } else if let Some(last) = args.and_then(Self::last_named_child) {
                last
            } else {
                e
            };
            self.push_name_value_diagnostic(name, source, diagnostics);
        }

        // Direct `=` tokens inside arguments (name=value parses cleanly only
        // for valid identifier names).
        if let Some(a) = args {
            for i in 0..a.child_count() {
                if let Some(eq) = a.child(i) {
                    if eq.kind() != "=" {
                        continue;
                    }
                    if let Some(name) = eq.prev_named_sibling() {
                        self.push_name_value_diagnostic(name, source, diagnostics);
                    }
                }
            }
        }
    }

    /// Emit FVACS (quoted-string name) or FVAMI (non-identifier name) for a
    /// name=value argument whose name node is `name`.
    fn push_name_value_diagnostic(
        &self,
        name: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // A valid MATLAB identifier is a legitimate name=value name.
        if name.kind() == "identifier" {
            return;
        }
        let is_string = name.kind() == "string" || Self::has_string_child(name);
        let check_id = if is_string { "FVACS" } else { "FVAMI" };
        if !self.is_check_enabled(check_id) {
            return;
        }
        let name_text = &source[name.start_byte()..name.end_byte()];
        let pos = name.start_position();
        let message = if is_string {
            "Using a character vector or string as a name in name=value syntax is not supported. Remove the quotes around the name."
        } else {
            "Name in name-value argument syntax must be a valid MATLAB identifier."
        };
        diagnostics.push(Diagnostic {
            rule_id: check_id,
            message: format!("{message} (near '{name_text}')"),
            severity: Severity::Error,
            byte_range: name.byte_range(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        });
    }

    // -----------------------------------------------------------------------
    // Argument validation order (VTPOD)
    // -----------------------------------------------------------------------

    /// VTPOD: Validation must be specified in the order size, then class,
    /// then functions. Walks all arguments blocks and checks each property.
    fn check_validation_order(&self, root: Node, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        if !self.is_check_enabled("VTPOD") {
            return diagnostics;
        }
        self.walk_validation_order(root, source, &mut diagnostics);
        diagnostics
    }

    /// Recursive DFS that visits arguments_statement nodes.
    fn walk_validation_order(
        &self,
        node: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "arguments_statement" {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "property" {
                    self.check_property_validation_order(child, source, diagnostics);
                }
            }
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_validation_order(child, source, diagnostics);
        }
    }

    /// Check a single arguments property for out-of-order class specification.
    fn check_property_validation_order(
        &self,
        property: Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let name = property.child_by_field_name("name");
        let vf = Self::find_validation_functions(property);
        let mut class_id: Option<Node> = None;

        for i in 0..property.child_count() {
            let Some(child) = property.child(i) else {
                continue;
            };
            if child.kind() == "identifier" && Some(child) != name && class_id.is_none() {
                class_id = Some(child);
            }
        }

        // Pattern A: a class identifier appears after the validation functions.
        if let (Some(class_id), Some(vf)) = (class_id, vf) {
            if class_id.start_byte() > vf.start_byte() {
                self.push_vtpod(class_id, diagnostics);
                return;
            }
        }

        // Pattern B: a class name appears inside the validation functions.
        if class_id.is_none() {
            if let Some(vf) = vf {
                let mut cursor = vf.walk();
                for child in vf.children(&mut cursor) {
                    if child.kind() != "identifier" {
                        continue;
                    }
                    let text = &source[child.start_byte()..child.end_byte()];
                    if KNOWN_CLASS_NAMES.contains(&text) {
                        self.push_vtpod(child, diagnostics);
                    }
                }
            }
        }
    }

    /// Find the validation_functions node of a property, searching descendants
    /// so it is also found when nested inside an ERROR node.
    fn find_validation_functions(node: Node) -> Option<Node> {
        if node.kind() == "validation_functions" {
            return Some(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(found) = Self::find_validation_functions(child) {
                return Some(found);
            }
        }
        None
    }

    /// Emit a VTPOD diagnostic for a misplaced class node.
    fn push_vtpod(&self, node: Node, diagnostics: &mut Vec<Diagnostic>) {
        let pos = node.start_position();
        diagnostics.push(Diagnostic {
            rule_id: "VTPOD",
            message: "Specify validation in the following order: size, then class, then functions."
                .to_string(),
            severity: Severity::Error,
            byte_range: node.byte_range(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        });
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Find the `arguments` child of a function_call node.
    fn arguments_child(node: Node) -> Option<Node> {
        (0..node.child_count())
            .find_map(|i| node.child(i).filter(|c| c.kind() == "arguments"))
    }

    /// True if the node has a direct anonymous `=` token child.
    fn has_eq_token(node: Node) -> bool {
        (0..node.child_count())
            .any(|i| node.child(i).is_some_and(|c| c.kind() == "="))
    }

    /// True if the node has a direct `string` child.
    fn has_string_child(node: Node) -> bool {
        (0..node.child_count())
            .any(|i| node.child(i).is_some_and(|c| c.kind() == "string"))
    }

    /// Last named child of a node.
    fn last_named_child(node: Node) -> Option<Node> {
        let mut last = None;
        for i in 0..node.child_count() {
            if let Some(c) = node.child(i) {
                if c.is_named() {
                    last = Some(c);
                }
            }
        }
        last
    }

    /// Collect ERROR nodes that are direct children of the call or of its
    /// arguments node.
    fn call_error_nodes(call: Node) -> Vec<Node> {
        let mut errors = Vec::new();
        for i in 0..call.child_count() {
            if let Some(c) = call.child(i) {
                if c.is_error() {
                    errors.push(c);
                }
                if c.kind() == "arguments" {
                    for j in 0..c.child_count() {
                        if let Some(cc) = c.child(j) {
                            if cc.is_error() {
                                errors.push(cc);
                            }
                        }
                    }
                }
            }
        }
        errors
    }

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

        // 4. Unicode directional formatting characters (BADCT).
        diagnostics.extend(self.check_directional_formatting(source));

        // 5. Tree ERROR/MISSING node analysis (SYNER, NOPAR2, EOLPAR, ENDPAR, ENDCT, EOFMI).
        diagnostics.extend(self.check_error_nodes(root, source));

        // 6. Number literal validation (BADFP, BADHBH, BADHBB, BADHBHT,
        //     BADHBBT, HEXTOOLONG, BINARYTOOLONG).
        diagnostics.extend(self.check_number_literals(root, source));

        // 7. Unterminated strings and comments (STRIN, DOUQT, INBLK).
        diagnostics.extend(self.check_unterminated(root, source));

        // 8. File structure validation (CLIS, CLTWO, SOFOC, SEMFU).
        diagnostics.extend(self.check_file_structure(root, source));

        // 9. Function name validation (FNDOT, FNSWA).
        diagnostics.extend(self.check_function_names(root, source));

        // 10. Statement validation (NOLHS, SEPEXR, REDEF).
        diagnostics.extend(self.check_statements(root, source));

        // 11. Assignment LHS validation (UNSET, LHROW).
        diagnostics.extend(self.check_assignment_lhs(root, source));

        // 12. Reserved words and invalid ~ usage (RESWD, SYNEND, MCPLD,
        //     BADNOT, BADNOTLHS).
        diagnostics.extend(self.check_reserved_and_not(root, source));

        // 13. Call/argument syntax (SBTMP, FVACI, FVACS, FVAMI).
        diagnostics.extend(self.check_call_syntax(root, source));

        // 14. Argument validation order (VTPOD).
        diagnostics.extend(self.check_validation_order(root, source));

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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{has_id, lint_file};
    use mlt_core::Config;

    fn engine() -> Box<dyn Rule> {
        SyntaxErrorsEngine::from_config(&Config::default())
    }

    fn engine_with_disabled(checks: &[&str]) -> Box<dyn Rule> {
        let disabled = checks
            .iter()
            .map(|c| format!("\"{c}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let config = Config::from_toml(&format!(
            "[lint.rules.SYNTAX_ERRORS_ENGINE]\ndisabled_checks = [{disabled}]\n"
        ))
        .unwrap();
        SyntaxErrorsEngine::from_config(&config)
    }

    // -- BADNE: `!=` instead of `~=` ----------------------------------------

    #[test]
    fn badne_fires_on_bang_equals() {
        let diags = lint_file(&*engine(), "x = 1 != 2;\n");
        assert!(has_id(&diags, "BADNE"), "got: {diags:?}");
    }

    #[test]
    fn badne_ok_on_tilde_equals() {
        let diags = lint_file(&*engine(), "x = 1 ~= 2;\n");
        assert!(!has_id(&diags, "BADNE"), "got: {diags:?}");
    }

    // -- BADOT: `..` not part of `...` --------------------------------------

    #[test]
    fn badot_fires_on_double_dot() {
        let diags = lint_file(&*engine(), "x = 1..2;\n");
        assert!(has_id(&diags, "BADOT"), "got: {diags:?}");
    }

    #[test]
    fn badot_ok_on_line_continuation() {
        let diags = lint_file(&*engine(), "x = 1 + ...\n    2;\n");
        assert!(!has_id(&diags, "BADOT"), "got: {diags:?}");
    }

    // -- TWOCM: double comma ------------------------------------------------

    #[test]
    fn twocm_fires_on_double_comma() {
        let diags = lint_file(&*engine(), "x = [1,,2];\n");
        assert!(has_id(&diags, "TWOCM"), "got: {diags:?}");
    }

    #[test]
    fn twocm_ok_on_single_comma() {
        let diags = lint_file(&*engine(), "x = [1,2];\n");
        assert!(!has_id(&diags, "TWOCM"), "got: {diags:?}");
    }

    // -- BADCH: invalid control characters ----------------------------------

    #[test]
    fn badch_fires_on_control_char() {
        let diags = lint_file(&*engine(), "x = 1;\u{1}\n");
        assert!(has_id(&diags, "BADCH"), "got: {diags:?}");
    }

    #[test]
    fn badch_ok_on_normal_source() {
        let diags = lint_file(&*engine(), "x = 1;\n");
        assert!(!has_id(&diags, "BADCH"), "got: {diags:?}");
    }

    // -- BADSP: non-ASCII whitespace ----------------------------------------

    #[test]
    fn badsp_fires_on_non_ascii_space() {
        let diags = lint_file(&*engine(), "x = 1;\u{a0}\n");
        assert!(has_id(&diags, "BADSP"), "got: {diags:?}");
    }

    #[test]
    fn badsp_ok_on_regular_spaces() {
        let diags = lint_file(&*engine(), "x = 1;\n");
        assert!(!has_id(&diags, "BADSP"), "got: {diags:?}");
    }

    // -- BADCT: Unicode directional formatting characters --------------------

    #[test]
    fn badct_fires_on_directional_formatting() {
        let diags = lint_file(&*engine(), "x = 1;\u{202a}\n");
        assert!(has_id(&diags, "BADCT"), "got: {diags:?}");
    }

    #[test]
    fn badct_fires_on_rlo() {
        let diags = lint_file(&*engine(), "x = 1;\u{202e}\n");
        assert!(has_id(&diags, "BADCT"), "got: {diags:?}");
    }

    #[test]
    fn badct_ok_on_plain_source() {
        let diags = lint_file(&*engine(), "x = 1;\n");
        assert!(!has_id(&diags, "BADCT"), "got: {diags:?}");
    }

    #[test]
    fn badct_disabled_in_config_does_not_fire() {
        let engine = engine_with_disabled(&["BADCT"]);
        let diags = lint_file(&*engine, "x = 1;\u{202a}\n");
        assert!(!has_id(&diags, "BADCT"), "got: {diags:?}");
    }

    // -- SYNER: generic syntax error ----------------------------------------

    #[test]
    fn syner_fires_on_parse_error() {
        let diags = lint_file(&*engine(), "x = ;\n");
        assert!(has_id(&diags, "SYNER"), "got: {diags:?}");
    }

    #[test]
    fn syner_ok_on_valid_source() {
        let diags = lint_file(&*engine(), "x = 5;\ny = x + 1;\n");
        assert!(!has_id(&diags, "SYNER"), "got: {diags:?}");
    }

    // -- NOPAR2: missing closing bracket (mid-file) --------------------------

    #[test]
    fn nopar2_fires_on_unclosed_bracket_mid_file() {
        let diags = lint_file(&*engine(), "y = [1 2;\n");
        assert!(has_id(&diags, "NOPAR2"), "got: {diags:?}");
        assert!(!has_id(&diags, "EOLPAR"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDPAR"), "got: {diags:?}");
    }

    #[test]
    fn nopar2_fires_on_missing_bracket_not_at_line_end() {
        let diags = lint_file(&*engine(), "x = f(1;\ny = 2;\n");
        assert!(!has_id(&diags, "NOPAR2"), "got: {diags:?}");
        assert!(has_id(&diags, "EOLPAR"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDPAR"), "got: {diags:?}");
    }

    #[test]
    fn nopar2_ok_on_balanced_brackets() {
        let diags = lint_file(&*engine(), "y = [1 2];\n");
        assert!(!has_id(&diags, "NOPAR2"), "got: {diags:?}");
        assert!(!has_id(&diags, "EOLPAR"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDPAR"), "got: {diags:?}");
    }

    // -- EOLPAR: missing closing bracket at end of line ----------------------

    #[test]
    fn eolpar_fires_on_missing_paren_at_line_end() {
        let diags = lint_file(&*engine(), "function foo()\n    x = f(1;\n    y = 2;\nend\n");
        assert!(has_id(&diags, "EOLPAR"), "got: {diags:?}");
        assert!(!has_id(&diags, "NOPAR2"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDPAR"), "got: {diags:?}");
    }

    #[test]
    fn eolpar_fires_on_top_level_missing_paren_at_line_end() {
        let diags = lint_file(&*engine(), "x = f(1;\ny = 2;\n");
        assert!(has_id(&diags, "EOLPAR"), "got: {diags:?}");
        assert!(!has_id(&diags, "NOPAR2"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDPAR"), "got: {diags:?}");
    }

    #[test]
    fn eolpar_ok_on_balanced_parens() {
        let diags = lint_file(&*engine(), "x = f(1);\n");
        assert!(!has_id(&diags, "EOLPAR"), "got: {diags:?}");
    }

    // -- ENDPAR: missing closing bracket at end of file ----------------------

    #[test]
    fn endpar_fires_on_missing_paren_at_eof() {
        let diags = lint_file(&*engine(), "x = f(1 + 2;\n");
        assert!(has_id(&diags, "ENDPAR"), "got: {diags:?}");
        assert!(!has_id(&diags, "NOPAR2"), "got: {diags:?}");
        assert!(!has_id(&diags, "EOLPAR"), "got: {diags:?}");
    }

    #[test]
    fn endpar_fires_on_missing_bracket_at_eof_no_newline() {
        let diags = lint_file(&*engine(), "x = f(1 + 2;");
        assert!(has_id(&diags, "ENDPAR"), "got: {diags:?}");
    }

    #[test]
    fn endpar_ok_on_complete_file() {
        let diags = lint_file(&*engine(), "x = [1, 2];\n");
        assert!(!has_id(&diags, "ENDPAR"), "got: {diags:?}");
    }

    // -- Missing bracket variants: must fire / must not fire -----------------

    #[test]
    fn missing_bracket_variants_must_fire_example() {
        let diags = lint_file(
            &*engine(),
            "x = f(1;\nfunction foo()\n    x = f(1;\n    y = 2;\nend\nx = f(1 + 2;\n",
        );
        assert!(has_id(&diags, "EOLPAR"), "got: {diags:?}");
        assert!(has_id(&diags, "ENDPAR"), "got: {diags:?}");
    }

    #[test]
    fn missing_bracket_variants_must_not_fire_example() {
        let diags = lint_file(&*engine(), "x = f(1);\nx = [1, 2];\nx = f(g(1));\n");
        assert!(!has_id(&diags, "NOPAR2"), "got: {diags:?}");
        assert!(!has_id(&diags, "EOLPAR"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDPAR"), "got: {diags:?}");
    }

    #[test]
    fn eolpar_cooccurs_with_eofmi() {
        let diags = lint_file(&*engine(), "x = f(1;\ny = 1; 2");
        assert!(has_id(&diags, "EOLPAR"), "got: {diags:?}");
        assert!(has_id(&diags, "EOFMI"), "got: {diags:?}");
    }

    #[test]
    fn missing_bracket_variants_respect_disabled_checks() {
        let engine = engine_with_disabled(&["NOPAR2"]);
        let diags = lint_file(&*engine, "y = [1 2;\n");
        assert!(!has_id(&diags, "NOPAR2"), "got: {diags:?}");

        let engine = engine_with_disabled(&["EOLPAR"]);
        let diags = lint_file(&*engine, "x = f(1;\ny = 2;\n");
        assert!(!has_id(&diags, "EOLPAR"), "got: {diags:?}");

        let engine = engine_with_disabled(&["ENDPAR"]);
        let diags = lint_file(&*engine, "x = f(1 + 2;\n");
        assert!(!has_id(&diags, "ENDPAR"), "got: {diags:?}");
    }

    // -- ENDCT family: possible missing `end` -------------------------------

    #[test]
    fn endct2_fires_on_unterminated_if() {
        let diags = lint_file(&*engine(), "if x > 0\n    y = 1;\n");
        assert!(has_id(&diags, "ENDCT2"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT"), "got: {diags:?}");
    }

    #[test]
    fn endct2_fires_on_unterminated_loops() {
        let diags = lint_file(&*engine(), "for i = 1:10\n    disp(i);\n");
        assert!(has_id(&diags, "ENDCT2"), "got: {diags:?}");
        let diags = lint_file(&*engine(), "while x\n    y = 1;\n");
        assert!(has_id(&diags, "ENDCT2"), "got: {diags:?}");
        let diags = lint_file(&*engine(), "switch x\n    case 1\n        y = 1;\n");
        assert!(has_id(&diags, "ENDCT2"), "got: {diags:?}");
    }

    #[test]
    fn endct2_ok_on_terminated_if() {
        let diags = lint_file(&*engine(), "if x > 0\n    y = 1;\nend\n");
        assert!(!has_id(&diags, "ENDCT2"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT"), "got: {diags:?}");
    }

    #[test]
    fn endct3_fires_on_stray_else_followed_by_opener() {
        let diags = lint_file(&*engine(), "else\nif x\n");
        assert!(has_id(&diags, "ENDCT3"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT"), "got: {diags:?}");
    }

    #[test]
    fn endct3_ok_without_following_opener() {
        let diags = lint_file(&*engine(), "if x\n    y = 1;\nend\nelse\n");
        assert!(!has_id(&diags, "ENDCT3"), "got: {diags:?}");
    }

    #[test]
    fn endct4_fires_on_classdef_without_methods() {
        let diags = lint_file(&*engine(), "classdef Foo\n  function f()\n  end\nend\n");
        assert!(has_id(&diags, "ENDCT4"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT"), "got: {diags:?}");
    }

    #[test]
    fn endct4_fires_on_missing_methods_block() {
        let diags = lint_file(
            &*engine(),
            "function f()\n    if x\n        y = 1;\nclassdef Foo\n    function f()\n    end\nend\n",
        );
        assert!(has_id(&diags, "ENDCT4"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT2"), "got: {diags:?}");
    }

    #[test]
    fn endct_family_ok_on_valid_code() {
        let diags = lint_file(
            &*engine(),
            "function f()\n    if x\n        y = 1;\n    end\nend\nclassdef Foo\n    methods\n        function f()\n            x = 1;\n        end\n    end\nend\n",
        );
        assert!(!has_id(&diags, "ENDCT"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT2"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT3"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT4"), "got: {diags:?}");
    }

    #[test]
    fn endct_variant_disabled_falls_back_to_generic() {
        let engine = engine_with_disabled(&["ENDCT2"]);
        let diags = lint_file(&*engine, "if x > 0\n    y = 1;\n");
        assert!(has_id(&diags, "ENDCT"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT2"), "got: {diags:?}");
    }

    #[test]
    fn endct3_disabled_falls_back_to_generic() {
        let engine = engine_with_disabled(&["ENDCT3"]);
        let diags = lint_file(&*engine, "else\nif x\n");
        assert!(has_id(&diags, "ENDCT"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT3"), "got: {diags:?}");
    }

    #[test]
    fn endct_disabled_suppresses_variants() {
        let engine = engine_with_disabled(&["ENDCT"]);
        let diags = lint_file(&*engine, "if x > 0\n    y = 1;\nelse\nif x\nclassdef Foo\n  function f()\n  end\nend\n");
        assert!(!has_id(&diags, "ENDCT"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT2"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT3"), "got: {diags:?}");
        assert!(!has_id(&diags, "ENDCT4"), "got: {diags:?}");
    }

    // -- EOFMI: file ends with ERROR node -----------------------------------

    #[test]
    fn eofmi_fires_on_incomplete_tail() {
        let diags = lint_file(&*engine(), "x = 1; 2");
        assert!(has_id(&diags, "EOFMI"), "got: {diags:?}");
    }

    #[test]
    fn eofmi_ok_on_complete_file() {
        let diags = lint_file(&*engine(), "x = 5;\n");
        assert!(!has_id(&diags, "EOFMI"), "got: {diags:?}");
    }

    // -- SEPEXR: missing separator between statements -----------------------

    #[test]
    fn sepexr_fires_on_same_line_statements() {
        let diags = lint_file(&*engine(), "x = 1; 2");
        assert!(has_id(&diags, "SEPEXR"), "got: {diags:?}");
    }

    #[test]
    fn sepexr_ok_on_separate_lines() {
        let diags = lint_file(&*engine(), "x = 1;\ny = 2;\n");
        assert!(!has_id(&diags, "SEPEXR"), "got: {diags:?}");
    }

    // -- NOLHS: assignment with empty left side -----------------------------

    #[test]
    fn nolhs_fires_on_empty_lhs() {
        let diags = lint_file(&*engine(), "= 5;\n");
        assert!(has_id(&diags, "NOLHS"), "got: {diags:?}");
    }

    #[test]
    fn nolhs_ok_on_normal_assignment() {
        let diags = lint_file(&*engine(), "x = 5;\n");
        assert!(!has_id(&diags, "NOLHS"), "got: {diags:?}");
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
        let diags = lint_file(
            &*engine(),
            "classdef Foo\nend\nfunction f()\nend\nx = 5;\n",
        );
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

    // -- FNDOT: dotted function name outside methods ------------------------

    #[test]
    fn fndot_fires_on_dotted_function_name() {
        let diags = lint_file(&*engine(), "function y = foo.bar()\n    y = 1;\nend\n");
        assert!(has_id(&diags, "FNDOT"), "got: {diags:?}");
    }

    #[test]
    fn fndot_ok_on_simple_function_name() {
        let diags = lint_file(&*engine(), "function y = foo()\n    y = 1;\nend\n");
        assert!(!has_id(&diags, "FNDOT"), "got: {diags:?}");
    }

    // -- FNSWA: function name starts with non-alphabetic --------------------

    #[test]
    fn fnswa_fires_on_underscore_name() {
        let diags = lint_file(&*engine(), "function y = _foo()\n    y = 1;\nend\n");
        assert!(has_id(&diags, "FNSWA"), "got: {diags:?}");
    }

    #[test]
    fn fnswa_ok_on_alphabetic_name() {
        let diags = lint_file(&*engine(), "function y = foo()\n    y = 1;\nend\n");
        assert!(!has_id(&diags, "FNSWA"), "got: {diags:?}");
    }

    // -- REDEF: identifier as both function name and variable ---------------

    #[test]
    fn redef_fires_on_function_and_variable() {
        let diags = lint_file(&*engine(), "function foo()\n    foo = 1;\nend\n");
        assert!(has_id(&diags, "REDEF"), "got: {diags:?}");
    }

    #[test]
    fn redef_ok_on_distinct_names() {
        let diags = lint_file(&*engine(), "function foo()\n    x = 1;\nend\n");
        assert!(!has_id(&diags, "REDEF"), "got: {diags:?}");
    }

    // -- disabled_checks config ---------------------------------------------

    #[test]
    fn disabled_checks_turn_off_checks() {
        let engine = engine_with_disabled(&["BADNE", "TWOCM"]);
        let diags = lint_file(&*engine, "x = 1 != 2; y = [1,,2];\n");
        assert!(!has_id(&diags, "BADNE"), "got: {diags:?}");
        assert!(!has_id(&diags, "TWOCM"), "got: {diags:?}");
    }

    // -- UNSET: operator on the left side of an assignment ------------------

    #[test]
    fn unset_fires_on_comparison_lhs() {
        let diags = lint_file(&*engine(), "x == 5 = 3;\n");
        assert!(has_id(&diags, "UNSET"), "got: {diags:?}");
    }

    #[test]
    fn unset_fires_on_binary_lhs() {
        let diags = lint_file(&*engine(), "x + 1 = 2;\n");
        assert!(has_id(&diags, "UNSET"), "got: {diags:?}");
    }

    #[test]
    fn unset_fires_on_unary_lhs() {
        let diags = lint_file(&*engine(), "-x = 3;\n");
        assert!(has_id(&diags, "UNSET"), "got: {diags:?}");
    }

    #[test]
    fn unset_ok_on_valid_assignment() {
        let diags = lint_file(&*engine(), "x = 5;\n");
        assert!(!has_id(&diags, "UNSET"), "got: {diags:?}");
    }

    #[test]
    fn unset_ok_on_operator_rhs() {
        let diags = lint_file(&*engine(), "x = x + 1;\n");
        assert!(!has_id(&diags, "UNSET"), "got: {diags:?}");
    }

    #[test]
    fn unset_ok_on_indexed_lhs() {
        let diags = lint_file(&*engine(), "x(1) = 5;\n");
        assert!(!has_id(&diags, "UNSET"), "got: {diags:?}");
    }

    #[test]
    fn unset_ok_on_field_lhs() {
        let diags = lint_file(&*engine(), "obj.field = 5;\n");
        assert!(!has_id(&diags, "UNSET"), "got: {diags:?}");
    }

    // -- LHROW: assignment LHS with multiple rows ---------------------------

    #[test]
    fn lhrow_fires_on_semicolon_rows() {
        let diags = lint_file(&*engine(), "[a;b] = f();\n");
        assert!(has_id(&diags, "LHROW"), "got: {diags:?}");
    }

    #[test]
    fn lhrow_fires_on_semicolon_rows_with_spaces() {
        let diags = lint_file(&*engine(), "[a; b] = f();\n");
        assert!(has_id(&diags, "LHROW"), "got: {diags:?}");
    }

    #[test]
    fn lhrow_ok_on_comma_list() {
        let diags = lint_file(&*engine(), "[a, b] = f();\n");
        assert!(!has_id(&diags, "LHROW"), "got: {diags:?}");
    }

    #[test]
    fn lhrow_ok_on_single_output() {
        let diags = lint_file(&*engine(), "x = f();\n");
        assert!(!has_id(&diags, "LHROW"), "got: {diags:?}");
    }

    // -- UNSET/LHROW disabled via config ------------------------------------

    #[test]
    fn disabled_checks_turn_off_unset_and_lhrow() {
        let engine = engine_with_disabled(&["UNSET", "LHROW"]);
        let diags = lint_file(&*engine, "x == 5 = 3;\n[a;b] = f();\n");
        assert!(!has_id(&diags, "UNSET"), "got: {diags:?}");
        assert!(!has_id(&diags, "LHROW"), "got: {diags:?}");
    }

    // -- RESWD: reserved word used as identifier ----------------------------

    #[test]
    fn reswd_fires_on_else_statement() {
        let diags = lint_file(&*engine(), "else\n");
        assert!(has_id(&diags, "RESWD"), "got: {diags:?}");
    }

    #[test]
    fn reswd_fires_on_reserved_word_in_assignment() {
        let diags = lint_file(&*engine(), "y = for;\n");
        assert!(has_id(&diags, "RESWD"), "got: {diags:?}");
    }

    #[test]
    fn reswd_fires_on_case_statement() {
        let diags = lint_file(&*engine(), "case = 5;\n");
        assert!(has_id(&diags, "RESWD"), "got: {diags:?}");
    }

    #[test]
    fn reswd_ok_on_valid_identifiers_and_blocks() {
        let diags = lint_file(
            &*engine(),
            "foo_else = 5;\nfor x = 1:10\nend\nswitch x\ncase 1\ny = 1;\nend\n",
        );
        assert!(!has_id(&diags, "RESWD"), "got: {diags:?}");
    }

    // -- SYNEND: invalid use of END -----------------------------------------

    #[test]
    fn synend_fires_on_end_assignment_lhs() {
        let diags = lint_file(&*engine(), "end = 5;\n");
        assert!(has_id(&diags, "SYNEND"), "got: {diags:?}");
    }

    #[test]
    fn synend_fires_on_end_as_value() {
        let diags = lint_file(&*engine(), "y = end;\n");
        assert!(has_id(&diags, "SYNEND"), "got: {diags:?}");
    }

    #[test]
    fn synend_ok_on_end_indexing() {
        let diags = lint_file(&*engine(), "x(end) = 5;\n");
        assert!(!has_id(&diags, "SYNEND"), "got: {diags:?}");
    }

    #[test]
    fn synend_ok_on_block_end() {
        let diags = lint_file(&*engine(), "for x = 1:10\nend\n");
        assert!(!has_id(&diags, "SYNEND"), "got: {diags:?}");
    }

    #[test]
    fn synend_takes_priority_over_reswd() {
        let diags = lint_file(&*engine(), "y = end;\n");
        assert!(has_id(&diags, "SYNEND"), "got: {diags:?}");
        assert!(!has_id(&diags, "RESWD"), "got: {diags:?}");
    }

    // -- MCPLD: invalid property syntax -------------------------------------

    #[test]
    fn mcpld_fires_on_double_equals_property() {
        let diags = lint_file(
            &*engine(),
            "classdef Foo\nproperties\nx = 1 = 2\nend\nend\n",
        );
        assert!(has_id(&diags, "MCPLD"), "got: {diags:?}");
    }

    #[test]
    fn mcpld_ok_on_valid_properties() {
        let diags = lint_file(
            &*engine(),
            "classdef Foo\nproperties\nx = 1;\nend\nend\n",
        );
        assert!(!has_id(&diags, "MCPLD"), "got: {diags:?}");
    }

    #[test]
    fn mcpld_ok_outside_properties() {
        let diags = lint_file(&*engine(), "x = 1 = 2;\n");
        assert!(!has_id(&diags, "MCPLD"), "got: {diags:?}");
    }

    // -- BADNOT: ~ misuse ----------------------------------------------------

    #[test]
    fn badnot_fires_on_tilde_equals_expression() {
        let diags = lint_file(&*engine(), "x = ~ = 5;\n");
        assert!(has_id(&diags, "BADNOT"), "got: {diags:?}");
    }

    #[test]
    fn badnot_fires_on_bare_tilde_statement() {
        let diags = lint_file(&*engine(), "~\n");
        assert!(has_id(&diags, "BADNOT"), "got: {diags:?}");
    }

    #[test]
    fn badnot_ok_on_logical_not() {
        let diags = lint_file(&*engine(), "x = ~y;\n");
        assert!(!has_id(&diags, "BADNOT"), "got: {diags:?}");
    }

    #[test]
    fn badnot_ok_on_ignored_assignment_lhs() {
        let diags = lint_file(&*engine(), "~ = 5;\n");
        assert!(!has_id(&diags, "BADNOT"), "got: {diags:?}");
    }

    // -- BADNOTLHS: ~ adjacent to output without comma ----------------------

    #[test]
    fn badnotlhs_fires_on_adjacent_tilde() {
        let diags = lint_file(&*engine(), "[x ~ y] = f();\n");
        assert!(has_id(&diags, "BADNOTLHS"), "got: {diags:?}");
    }

    #[test]
    fn badnotlhs_ok_on_comma_separated() {
        let diags = lint_file(&*engine(), "[~, y] = f();\n");
        assert!(!has_id(&diags, "BADNOTLHS"), "got: {diags:?}");
    }

    #[test]
    fn badnotlhs_ok_on_solo_tilde() {
        let diags = lint_file(&*engine(), "[~] = f();\n");
        assert!(!has_id(&diags, "BADNOTLHS"), "got: {diags:?}");
    }

    #[test]
    fn badnotlhs_ok_on_fully_comma_separated() {
        let diags = lint_file(&*engine(), "[x, ~, y] = f();\n");
        assert!(!has_id(&diags, "BADNOTLHS"), "got: {diags:?}");
    }

    // -- G3 checks disabled via config --------------------------------------

    #[test]
    fn disabled_checks_turn_off_g3_checks() {
        let engine = engine_with_disabled(&[
            "RESWD",
            "SYNEND",
            "MCPLD",
            "BADNOT",
            "BADNOTLHS",
        ]);
        let diags = lint_file(
            &*engine,
            "else\nend = 5;\nclassdef Foo\nproperties\nx = 1 = 2\nend\nend\nx = ~ = 5;\n[x ~ y] = f();\n",
        );
        assert!(!has_id(&diags, "RESWD"), "got: {diags:?}");
        assert!(!has_id(&diags, "SYNEND"), "got: {diags:?}");
        assert!(!has_id(&diags, "MCPLD"), "got: {diags:?}");
        assert!(!has_id(&diags, "BADNOT"), "got: {diags:?}");
        assert!(!has_id(&diags, "BADNOTLHS"), "got: {diags:?}");
    }

    // -- STRIN: unterminated single-quoted character vector -----------------

    #[test]
    fn strin_fires_on_unterminated_char_vector() {
        let diags = lint_file(&*engine(), "x = 'abc;\n");
        assert!(has_id(&diags, "STRIN"), "got: {diags:?}");
    }

    #[test]
    fn strin_fires_on_unterminated_at_eof() {
        let diags = lint_file(&*engine(), "x = 'abc");
        assert!(has_id(&diags, "STRIN"), "got: {diags:?}");
    }

    #[test]
    fn strin_fires_on_unterminated_with_escaped_quote() {
        let diags = lint_file(&*engine(), "x = 'don''t stop\n");
        assert!(has_id(&diags, "STRIN"), "got: {diags:?}");
    }

    #[test]
    fn strin_ok_on_closed_char_vector() {
        let diags = lint_file(&*engine(), "x = 'abc';\n");
        assert!(!has_id(&diags, "STRIN"), "got: {diags:?}");
    }

    #[test]
    fn strin_ok_on_escaped_quote() {
        let diags = lint_file(&*engine(), "x = 'it''s ok';\n");
        assert!(!has_id(&diags, "STRIN"), "got: {diags:?}");
    }

    #[test]
    fn strin_ok_on_transpose() {
        let diags = lint_file(&*engine(), "x = A';\n");
        assert!(!has_id(&diags, "STRIN"), "got: {diags:?}");
    }

    #[test]
    fn strin_ok_on_elementwise_transpose() {
        let diags = lint_file(&*engine(), "x = A.';\n");
        assert!(!has_id(&diags, "STRIN"), "got: {diags:?}");
    }

    #[test]
    fn strin_ok_on_transpose_of_expression() {
        let diags = lint_file(&*engine(), "x = (1:5)';\n");
        assert!(!has_id(&diags, "STRIN"), "got: {diags:?}");
    }

    #[test]
    fn strin_ok_on_matrix_transpose() {
        let diags = lint_file(&*engine(), "x = [1 2; 3 4]';\n");
        assert!(!has_id(&diags, "STRIN"), "got: {diags:?}");
    }

    #[test]
    fn strin_ok_on_transpose_without_semicolon() {
        let diags = lint_file(&*engine(), "x = A'\n");
        assert!(!has_id(&diags, "STRIN"), "got: {diags:?}");
    }

    #[test]
    fn strin_ok_on_adjacent_strings() {
        let diags = lint_file(&*engine(), "x = 'abc' 'def';\n");
        assert!(!has_id(&diags, "STRIN"), "got: {diags:?}");
    }

    // -- DOUQT: unterminated double-quoted string ---------------------------

    #[test]
    fn douqt_fires_on_unterminated_string() {
        let diags = lint_file(&*engine(), "x = \"abc;\n");
        assert!(has_id(&diags, "DOUQT"), "got: {diags:?}");
    }

    #[test]
    fn douqt_fires_on_unterminated_at_eof() {
        let diags = lint_file(&*engine(), "x = \"abc");
        assert!(has_id(&diags, "DOUQT"), "got: {diags:?}");
    }

    #[test]
    fn douqt_fires_on_empty_unterminated() {
        let diags = lint_file(&*engine(), "s = \"\n");
        assert!(has_id(&diags, "DOUQT"), "got: {diags:?}");
    }

    #[test]
    fn douqt_ok_on_closed_string() {
        let diags = lint_file(&*engine(), "x = \"abc\";\n");
        assert!(!has_id(&diags, "DOUQT"), "got: {diags:?}");
    }

    #[test]
    fn douqt_ok_on_multiline_string() {
        let diags = lint_file(&*engine(), "x = \"multi\nline\";\n");
        assert!(!has_id(&diags, "DOUQT"), "got: {diags:?}");
    }

    #[test]
    fn douqt_ok_on_concatenated_strings() {
        let diags = lint_file(&*engine(), "x = \"a\" + \"b\";\n");
        assert!(!has_id(&diags, "DOUQT"), "got: {diags:?}");
    }

    // -- INBLK: unterminated block comment ----------------------------------

    #[test]
    fn inblk_fires_on_unterminated_block_comment() {
        let diags = lint_file(&*engine(), "%{\nunterminated\n");
        assert!(has_id(&diags, "INBLK"), "got: {diags:?}");
    }

    #[test]
    fn inblk_fires_on_unterminated_block_comment_at_eof() {
        let diags = lint_file(&*engine(), "%{ unterminated");
        assert!(has_id(&diags, "INBLK"), "got: {diags:?}");
    }

    #[test]
    fn inblk_ok_on_closed_block_comment() {
        let diags = lint_file(&*engine(), "%{\nclosed\n%}\n");
        assert!(!has_id(&diags, "INBLK"), "got: {diags:?}");
    }

    #[test]
    fn inblk_ok_on_single_line_block_comment() {
        let diags = lint_file(&*engine(), "%{ block %}\n");
        assert!(!has_id(&diags, "INBLK"), "got: {diags:?}");
    }

    #[test]
    fn inblk_ok_on_regular_comment() {
        let diags = lint_file(&*engine(), "x = 1; % regular comment\n");
        assert!(!has_id(&diags, "INBLK"), "got: {diags:?}");
    }

    // -- STRIN/DOUQT/INBLK disabled via config ------------------------------

    #[test]
    fn disabled_checks_turn_off_unterminated_checks() {
        let engine = engine_with_disabled(&["STRIN", "DOUQT", "INBLK"]);
        let diags = lint_file(&*engine, "x = 'abc;\nx = \"abc;\n%{\nunterminated\n");
        assert!(!has_id(&diags, "STRIN"), "got: {diags:?}");
        assert!(!has_id(&diags, "DOUQT"), "got: {diags:?}");
        assert!(!has_id(&diags, "INBLK"), "got: {diags:?}");
    }

    // -- BADFP: invalid floating-point constant ------------------------------

    #[test]
    fn badfp_fires_on_truncated_float() {
        let diags = lint_file(&*engine(), "x = 1.2.3;\n");
        assert!(has_id(&diags, "BADFP"), "got: {diags:?}");
    }

    #[test]
    fn badfp_ok_on_valid_float() {
        let diags = lint_file(&*engine(), "x = 1.2;\n");
        assert!(!has_id(&diags, "BADFP"), "got: {diags:?}");
    }

    #[test]
    fn badfp_ok_on_whitespace_separated_numbers() {
        let diags = lint_file(&*engine(), "x = 1 2;\n");
        assert!(!has_id(&diags, "BADFP"), "got: {diags:?}");
    }

    #[test]
    fn badfp_reports_number_byte_range() {
        let diags = lint_file(&*engine(), "x = 1.2.3;\n");
        let d = diags.iter().find(|d| d.rule_id == "BADFP").expect("BADFP fired");
        assert_eq!(d.byte_range, 4..7);
    }

    // -- BADHBH: invalid digit in hexadecimal literal ------------------------

    #[test]
    fn badhbh_fires_on_invalid_hex_digit() {
        let diags = lint_file(&*engine(), "x = 0xFFG;\n");
        assert!(has_id(&diags, "BADHBH"), "got: {diags:?}");
    }

    #[test]
    fn badhbh_fires_on_invalid_hex_suffix() {
        let diags = lint_file(&*engine(), "x = 0xFFu;\n");
        assert!(has_id(&diags, "BADHBH"), "got: {diags:?}");
    }

    #[test]
    fn badhbh_ok_on_valid_hex() {
        let diags = lint_file(&*engine(), "x = 0xFF;\n");
        assert!(!has_id(&diags, "BADHBH"), "got: {diags:?}");
    }

    #[test]
    fn badhbh_ok_on_unrelated_error() {
        let diags = lint_file(&*engine(), "x = 0xFF);\n");
        assert!(!has_id(&diags, "BADHBH"), "got: {diags:?}");
    }

    // -- BADHBB: invalid digit in binary literal -----------------------------

    #[test]
    fn badhbb_fires_on_invalid_binary_digit() {
        let diags = lint_file(&*engine(), "x = 0b102;\n");
        assert!(has_id(&diags, "BADHBB"), "got: {diags:?}");
    }

    #[test]
    fn badhbb_ok_on_valid_binary() {
        let diags = lint_file(&*engine(), "x = 0b10;\n");
        assert!(!has_id(&diags, "BADHBB"), "got: {diags:?}");
    }

    // -- BADHBHT: hex literal too long for its type suffix -------------------

    #[test]
    fn badhbht_fires_on_too_many_hex_digits() {
        let diags = lint_file(&*engine(), "x = 0xFFFFu8;\n");
        assert!(has_id(&diags, "BADHBHT"), "got: {diags:?}");
    }

    #[test]
    fn badhbht_ok_on_exact_fit() {
        let diags = lint_file(&*engine(), "x = 0xFFu8;\n");
        assert!(!has_id(&diags, "BADHBHT"), "got: {diags:?}");
    }

    #[test]
    fn badhbht_ok_on_larger_suffix() {
        let diags = lint_file(&*engine(), "x = 0xFFFFu16;\n");
        assert!(!has_id(&diags, "BADHBHT"), "got: {diags:?}");
    }

    // -- BADHBBT: binary literal too long for its type suffix ----------------

    #[test]
    fn badhbbt_fires_on_too_many_binary_digits() {
        let diags = lint_file(&*engine(), "x = 0b111111111u8;\n");
        assert!(has_id(&diags, "BADHBBT"), "got: {diags:?}");
    }

    #[test]
    fn badhbbt_ok_on_exact_fit() {
        let diags = lint_file(&*engine(), "x = 0b11111111u8;\n");
        assert!(!has_id(&diags, "BADHBBT"), "got: {diags:?}");
    }

    #[test]
    fn badhbbt_ok_on_larger_suffix() {
        let diags = lint_file(&*engine(), "x = 0b111111111u16;\n");
        assert!(!has_id(&diags, "BADHBBT"), "got: {diags:?}");
    }

    // -- HEXTOOLONG: unsuffixed hex literal with too many digits -------------

    #[test]
    fn hextoolong_fires_on_more_than_16_hex_digits() {
        let diags = lint_file(&*engine(), "x = 0xFFFFFFFFFFFFFFFFFFF;\n");
        assert!(has_id(&diags, "HEXTOOLONG"), "got: {diags:?}");
    }

    #[test]
    fn hextoolong_ok_on_16_hex_digits() {
        let diags = lint_file(&*engine(), "x = 0xFFFFFFFFFFFFFFFF;\n");
        assert!(!has_id(&diags, "HEXTOOLONG"), "got: {diags:?}");
    }

    // -- BINARYTOOLONG: unsuffixed binary literal with too many digits -------

    #[test]
    fn binarytoolong_fires_on_more_than_64_bits() {
        let src = format!("x = 0b{};\n", "1".repeat(65));
        let diags = lint_file(&*engine(), &src);
        assert!(has_id(&diags, "BINARYTOOLONG"), "got: {diags:?}");
    }

    #[test]
    fn binarytoolong_ok_on_64_bits() {
        let src = format!("x = 0b{};\n", "1".repeat(64));
        let diags = lint_file(&*engine(), &src);
        assert!(!has_id(&diags, "BINARYTOOLONG"), "got: {diags:?}");
    }

    // -- number literal checks disabled via config ---------------------------

    #[test]
    fn disabled_checks_turn_off_number_literal_checks() {
        let engine = engine_with_disabled(&[
            "BADFP",
            "BADHBH",
            "BADHBB",
            "BADHBHT",
            "BADHBBT",
            "HEXTOOLONG",
            "BINARYTOOLONG",
        ]);
        let src = format!(
            "x = 1.2.3;\nx = 0xFFG;\nx = 0b102;\nx = 0xFFFFFFFFFFFFFFFFFFFF;\nx = 0b{};\nx = 0xFFFFu8;\nx = 0b111111111u8;\n",
            "1".repeat(65)
        );
        let diags = lint_file(&*engine, &src);
        assert!(!has_id(&diags, "BADFP"), "got: {diags:?}");
        assert!(!has_id(&diags, "BADHBH"), "got: {diags:?}");
        assert!(!has_id(&diags, "BADHBB"), "got: {diags:?}");
        assert!(!has_id(&diags, "BADHBHT"), "got: {diags:?}");
        assert!(!has_id(&diags, "BADHBBT"), "got: {diags:?}");
        assert!(!has_id(&diags, "HEXTOOLONG"), "got: {diags:?}");
        assert!(!has_id(&diags, "BINARYTOOLONG"), "got: {diags:?}");
    }

    // -- SBTMP: chained call result indexing --------------------------------

    #[test]
    fn sbtmp_fires_on_chained_call() {
        let diags = lint_file(&*engine(), "x = f()(1);\n");
        assert!(has_id(&diags, "SBTMP"), "got: {diags:?}");
    }

    #[test]
    fn sbtmp_fires_on_chained_call_with_args() {
        let diags = lint_file(&*engine(), "x = f(1)(2);\n");
        assert!(has_id(&diags, "SBTMP"), "got: {diags:?}");
    }

    #[test]
    fn sbtmp_ok_on_plain_call() {
        let diags = lint_file(&*engine(), "x = f(1);\n");
        assert!(!has_id(&diags, "SBTMP"), "got: {diags:?}");
    }

    #[test]
    fn sbtmp_ok_on_cell_index() {
        let diags = lint_file(&*engine(), "x = y{1};\n");
        assert!(!has_id(&diags, "SBTMP"), "got: {diags:?}");
    }

    #[test]
    fn sbtmp_ok_on_method_call() {
        let diags = lint_file(&*engine(), "obj.method();\n");
        assert!(!has_id(&diags, "SBTMP"), "got: {diags:?}");
    }

    // -- FVSYN: invalid function argument syntax ----------------------------

    #[test]
    fn fvsyn_fires_on_missing_comma() {
        let diags = lint_file(&*engine(), "f(1 2);\n");
        assert!(has_id(&diags, "FVSYN"), "got: {diags:?}");
    }

    #[test]
    fn fvsyn_replaces_syner_in_args() {
        let diags = lint_file(&*engine(), "f(1 2);\n");
        assert!(has_id(&diags, "FVSYN"), "got: {diags:?}");
        assert!(!has_id(&diags, "SYNER"), "got: {diags:?}");
    }

    #[test]
    fn fvsyn_ok_on_comma_separated_args() {
        let diags = lint_file(&*engine(), "f(1, 2);\n");
        assert!(!has_id(&diags, "FVSYN"), "got: {diags:?}");
    }

    #[test]
    fn fvsyn_ok_on_name_value_args() {
        let diags = lint_file(&*engine(), "f(Name = 1);\n");
        assert!(!has_id(&diags, "FVSYN"), "got: {diags:?}");
    }

    // -- FVACI: name=value in cell indexing ---------------------------------

    #[test]
    fn fvaci_fires_on_name_value_cell_index() {
        let diags = lint_file(&*engine(), "c{key = 'Name'} = 5;\n");
        assert!(has_id(&diags, "FVACI"), "got: {diags:?}");
    }

    #[test]
    fn fvaci_ok_on_comma_cell_index() {
        let diags = lint_file(&*engine(), "c{key, 'Name'} = 5;\n");
        assert!(!has_id(&diags, "FVACI"), "got: {diags:?}");
    }

    #[test]
    fn fvaci_ok_on_scalar_cell_index() {
        let diags = lint_file(&*engine(), "x = y{1};\n");
        assert!(!has_id(&diags, "FVACI"), "got: {diags:?}");
    }

    // -- FVACS: quoted string as name in name=value -------------------------

    #[test]
    fn fvacs_fires_on_quoted_name() {
        let diags = lint_file(&*engine(), "f('Bad Name' = 1);\n");
        assert!(has_id(&diags, "FVACS"), "got: {diags:?}");
    }

    #[test]
    fn fvacs_fires_on_quoted_name_no_space() {
        let diags = lint_file(&*engine(), "f('Bad Name'=1);\n");
        assert!(has_id(&diags, "FVACS"), "got: {diags:?}");
    }

    #[test]
    fn fvacs_ok_on_valid_identifier_name() {
        let diags = lint_file(&*engine(), "f(Name = 1);\n");
        assert!(!has_id(&diags, "FVACS"), "got: {diags:?}");
    }

    #[test]
    fn fvacs_ok_on_no_equals() {
        let diags = lint_file(&*engine(), "f(Name, 1);\n");
        assert!(!has_id(&diags, "FVACS"), "got: {diags:?}");
    }

    // -- FVAMI: name not a valid identifier in name=value -------------------

    #[test]
    fn fvami_fires_on_number_name() {
        let diags = lint_file(&*engine(), "f(123 = 1);\n");
        assert!(has_id(&diags, "FVAMI"), "got: {diags:?}");
    }

    #[test]
    fn fvami_fires_on_number_name_no_space() {
        let diags = lint_file(&*engine(), "f(123=1);\n");
        assert!(has_id(&diags, "FVAMI"), "got: {diags:?}");
    }

    #[test]
    fn fvami_ok_on_valid_identifier_name() {
        let diags = lint_file(&*engine(), "f(Name = 1);\n");
        assert!(!has_id(&diags, "FVAMI"), "got: {diags:?}");
    }

    #[test]
    fn fvami_ok_on_plain_args() {
        let diags = lint_file(&*engine(), "f(1, 2);\n");
        assert!(!has_id(&diags, "FVAMI"), "got: {diags:?}");
    }

    // -- call syntax checks disabled via config -----------------------------

    #[test]
    fn disabled_checks_turn_off_call_syntax_checks() {
        let engine = engine_with_disabled(&["SBTMP", "FVSYN", "FVACI", "FVACS", "FVAMI"]);
        let src = "x = f()(1);\nf(1 2);\nc{key = 'Name'} = 5;\nf('Bad Name' = 1);\nf(123 = 1);\n";
        let diags = lint_file(&*engine, src);
        assert!(!has_id(&diags, "SBTMP"), "got: {diags:?}");
        assert!(!has_id(&diags, "FVSYN"), "got: {diags:?}");
        assert!(!has_id(&diags, "FVACI"), "got: {diags:?}");
        assert!(!has_id(&diags, "FVACS"), "got: {diags:?}");
        assert!(!has_id(&diags, "FVAMI"), "got: {diags:?}");
    }

    // -- VTPOD: validation order must be size, then class, then functions ---

    #[test]
    fn vtpod_fires_on_class_inside_braces_with_dims() {
        let src = "function f(x)\n    arguments\n        x (1,1) {mustBeReal, double}\n    end\nend\n";
        let diags = lint_file(&*engine(), src);
        assert!(has_id(&diags, "VTPOD"), "got: {diags:?}");
    }

    #[test]
    fn vtpod_fires_on_class_inside_braces_no_dims() {
        let src = "function f(x)\n    arguments\n        x {mustBeReal, double}\n    end\nend\n";
        let diags = lint_file(&*engine(), src);
        assert!(has_id(&diags, "VTPOD"), "got: {diags:?}");
    }

    #[test]
    fn vtpod_fires_on_class_as_sole_validator() {
        let src = "function f(x)\n    arguments\n        x {double}\n    end\nend\n";
        let diags = lint_file(&*engine(), src);
        assert!(has_id(&diags, "VTPOD"), "got: {diags:?}");
    }

    #[test]
    fn vtpod_fires_on_class_after_validation_functions() {
        let src = "function f(x)\n    arguments\n        x {mustBeReal} double\n    end\nend\n";
        let diags = lint_file(&*engine(), src);
        assert!(has_id(&diags, "VTPOD"), "got: {diags:?}");
    }

    #[test]
    fn vtpod_ok_on_correct_order() {
        let src = "function f(x)\n    arguments\n        x (1,1) double {mustBeReal}\n        y (1,1) double\n        z {mustBePositive}\n        w (1,1) double {mustBeReal} = 1\n    end\nend\n";
        let diags = lint_file(&*engine(), src);
        assert!(!has_id(&diags, "VTPOD"), "got: {diags:?}");
    }

    #[test]
    fn vtpod_ok_on_class_without_dims() {
        let src = "function f(x)\n    arguments\n        x double {mustBePositive}\n    end\nend\n";
        let diags = lint_file(&*engine(), src);
        assert!(!has_id(&diags, "VTPOD"), "got: {diags:?}");
    }

    #[test]
    fn vtpod_ok_on_class_properties_block() {
        let src = "classdef Foo\n    properties\n        x {mustBeReal, double}\n    end\nend\n";
        let diags = lint_file(&*engine(), src);
        assert!(!has_id(&diags, "VTPOD"), "got: {diags:?}");
    }

    #[test]
    fn vtpod_disabled_via_config() {
        let engine = engine_with_disabled(&["VTPOD"]);
        let src = "function f(x)\n    arguments\n        x (1,1) {mustBeReal, double}\n    end\nend\n";
        let diags = lint_file(&*engine, src);
        assert!(!has_id(&diags, "VTPOD"), "got: {diags:?}");
    }
}
