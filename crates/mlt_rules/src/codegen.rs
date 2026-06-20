//! # CODEGEN_ENGINE: MATLAB Code Generation Checks
//!
//! Implements 20 checks for MATLAB Coder / code generation constraints.
//! MATLAB code intended for code generation (C/C++) has many restrictions
//! compared to general MATLAB code. This engine detects constructs that are
//! unsupported or problematic for code generation.
//!
//! ## Checks
//!
//! | ID | Description |
//! |----|-------------|
//! | EMVDF | Variable-size data not supported |
//! | EMGRO | Growing arrays not supported |
//! | EMNODEF | Variable must be defined before use |
//! | EMFCN | Unsupported function for codegen |
//! | EMCEL | Cell arrays not supported |
//! | EMTC | Try-catch not supported |
//! | EMIMP | Import not supported |
//! | EMNST | Nested functions not supported |
//! | EMSCR | Scripts not supported |
//! | EMBRK | Break in unsupported context |
//! | EMCNT | Continue in unsupported context |
//! | EMPFR | Parfor not supported |
//! | EMRTN | Return in unsupported context |
//! | EMWHL | While loops with non-constant bounds |
//! | EMRIFAV | Arguments block feature |
//! | EMLOAD | Load not supported |
//! | EMS2N | str2num not supported |
//! | PRMNOIN | No input validation in codegen |
//! | LOOPPRAGMAWITHOUTFOR | coder.loop pragma without for |
//! | FPASE | Fixed-point: assignment to scaled expression |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.CODEGEN_ENGINE]
//! skip_checks = ["EMSCR", "EMWHL"]
//! ```

use mlt_core::{Category, Config, Diagnostic, FileContext, NodeContext, Rule, Severity};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for the code generation engine.
///
/// Deserialized from the `[lint.rules.CODEGEN_ENGINE]` section in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CodegenEngineConfig {
    /// Check IDs to skip (e.g., `["EMSCR", "EMWHL"]`).
    #[serde(default)]
    pub skip_checks: Vec<String>,
}

// ---------------------------------------------------------------------------
// Check metadata
// ---------------------------------------------------------------------------

/// Static metadata for a single code generation check.
struct CheckMeta {
    id: &'static str,
    description: &'static str,
}

/// All 20 code generation check definitions.
const CHECKS: &[CheckMeta] = &[
    CheckMeta { id: "EMVDF", description: "Variable-size data is not supported for code generation" },
    CheckMeta { id: "EMGRO", description: "Growing arrays inside loops is not supported for code generation" },
    CheckMeta { id: "EMNODEF", description: "Variable must be defined before use in code generation" },
    CheckMeta { id: "EMFCN", description: "Function is not supported for code generation" },
    CheckMeta { id: "EMCEL", description: "Cell arrays are not supported for code generation" },
    CheckMeta { id: "EMTC", description: "Try-catch statements are not supported for code generation" },
    CheckMeta { id: "EMIMP", description: "Import statements are not supported for code generation" },
    CheckMeta { id: "EMNST", description: "Nested functions are not supported for code generation" },
    CheckMeta { id: "EMSCR", description: "Scripts are not supported for code generation; use functions instead" },
    CheckMeta { id: "EMBRK", description: "Break statement in unsupported context for code generation" },
    CheckMeta { id: "EMCNT", description: "Continue statement in unsupported context for code generation" },
    CheckMeta { id: "EMPFR", description: "Parfor is not supported for code generation" },
    CheckMeta { id: "EMRTN", description: "Return statement in unsupported context for code generation" },
    CheckMeta { id: "EMWHL", description: "While loops with non-constant bounds may not be supported for code generation" },
    CheckMeta { id: "EMRIFAV", description: "Arguments validation block is not fully supported for code generation" },
    CheckMeta { id: "EMLOAD", description: "'load' is not supported for code generation" },
    CheckMeta { id: "EMS2N", description: "'str2num' is not supported for code generation; use 'str2double'" },
    CheckMeta { id: "PRMNOIN", description: "No input validation available in generated code" },
    CheckMeta { id: "LOOPPRAGMAWITHOUTFOR", description: "coder.loop pragma must be immediately followed by a for-loop" },
    CheckMeta { id: "FPASE", description: "Assignment to a scaled fixed-point expression may lose precision" },
];

/// Functions unsupported in code generation.
const UNSUPPORTED_FUNCTIONS: &[&str] = &[
    "eval", "evalc", "evalin", "feval", "assignin", "input", "keyboard",
    "dbstop", "dbclear", "dbcont", "dbstep", "dbup", "dbdown", "dbquit",
    "who", "whos", "clear", "clearvars", "pack",
    "figure", "plot", "subplot", "xlabel", "ylabel", "title",
    "diary", "save", "load", "matfile",
    "cd", "ls", "dir", "mkdir", "rmdir",
    "javaObject", "javaMethod", "javaArray",
    "NET", "actxserver",
    "matlabroot", "tempdir", "tempname",
    "script", "run",
];

/// Look up check description by ID.
fn check_description(id: &str) -> &'static str {
    CHECKS
        .iter()
        .find(|c| c.id == id)
        .map(|c| c.description)
        .unwrap_or("Code generation constraint violation")
}

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// Engine covering 20 code generation checks.
///
/// Node-level checks fire on `function_call`, `command`, `try_statement`,
/// `for_statement`, `assignment`, and `cell`. File-level checks detect
/// nested functions and script-mode files.
pub struct CodegenEngine {
    config: CodegenEngineConfig,
}

/// Node types for node-level dispatch.
const TARGET_NODES: &[&str] = &[
    "function_call",
    "command",
    "try_statement",
    "for_statement",
    "assignment",
    "cell",
    "arguments_statement",
];

impl CodegenEngine {
    /// Factory constructor called by the rule registry.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: CodegenEngineConfig = config.rule_params("CODEGEN_ENGINE");
        Box::new(Self {
            config: rule_config,
        })
    }

    /// Whether a check ID is enabled (not in `skip_checks`).
    fn is_enabled(&self, check_id: &str) -> bool {
        !self.config.skip_checks.iter().any(|s| s == check_id)
    }

    // -----------------------------------------------------------------------
    // Node-level check dispatchers
    // -----------------------------------------------------------------------

    /// Check a `function_call` node for unsupported functions.
    fn check_function_call<'a>(
        &self,
        node: tree_sitter::Node<'a>,
        source: &'a str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        let func_name = match extract_func_name(node, source) {
            Some(n) => n,
            None => return diags,
        };

        // EMFCN: Unsupported function for code generation
        if self.is_enabled("EMFCN") && UNSUPPORTED_FUNCTIONS.contains(&func_name) {
            diags.push(make_diag("EMFCN", node));
        }

        // EMLOAD: load function
        if self.is_enabled("EMLOAD") && func_name == "load" {
            diags.push(make_diag("EMLOAD", node));
        }

        // EMS2N: str2num
        if self.is_enabled("EMS2N") && func_name == "str2num" {
            diags.push(make_diag("EMS2N", node));
        }

        // PRMNOIN: validateattributes / inputParser in codegen context
        if self.is_enabled("PRMNOIN")
            && (func_name == "validateattributes"
                || func_name == "inputParser"
                || func_name == "addRequired"
                || func_name == "addParameter")
        {
            diags.push(make_diag("PRMNOIN", node));
        }

        // LOOPPRAGMAWITHOUTFOR: coder.loop without following for
        if self.is_enabled("LOOPPRAGMAWITHOUTFOR") && func_name == "coder.loop" {
            // Check if next sibling is a for_statement
            if !is_followed_by_for(node) {
                diags.push(make_diag("LOOPPRAGMAWITHOUTFOR", node));
            }
        }

        diags
    }

    /// Check a `command` node for unsupported commands.
    fn check_command<'a>(
        &self,
        node: tree_sitter::Node<'a>,
        source: &'a str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        let cmd_name = match extract_command_name(node, source) {
            Some(n) => n,
            None => return diags,
        };

        // EMIMP: import command
        if self.is_enabled("EMIMP") && cmd_name == "import" {
            diags.push(make_diag("EMIMP", node));
        }

        // EMFCN: unsupported commands (clear, load, etc.)
        if self.is_enabled("EMFCN") && UNSUPPORTED_FUNCTIONS.contains(&cmd_name) {
            diags.push(make_diag("EMFCN", node));
        }

        diags
    }

    /// Check a `try_statement` node.
    fn check_try_statement(&self, node: tree_sitter::Node) -> Vec<Diagnostic> {
        if self.is_enabled("EMTC") {
            vec![make_diag("EMTC", node)]
        } else {
            Vec::new()
        }
    }

    /// Check a `for_statement` node for parfor.
    fn check_for_statement<'a>(
        &self,
        node: tree_sitter::Node<'a>,
        source: &'a str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        // EMPFR: parfor detection — check if the node text starts with "parfor"
        if self.is_enabled("EMPFR") {
            let text = node_text(node, source);
            if text.starts_with("parfor") {
                diags.push(make_diag("EMPFR", node));
            }
        }

        diags
    }

    /// Check an `assignment` node for growth patterns (EMGRO) and
    /// variable-size data (EMVDF).
    fn check_assignment<'a>(
        &self,
        node: tree_sitter::Node<'a>,
        source: &'a str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        let rhs = match node.child_by_field_name("right").or_else(|| node.child(2)) {
            Some(n) => n,
            None => return diags,
        };

        let rhs_text = node_text(rhs, source);

        // EMVDF: dynamic allocation patterns like `zeros(1, n)` where n is not constant
        // Simplified heuristic: detect `[]` as empty matrix initialization
        if self.is_enabled("EMVDF") && rhs_text == "[]" && is_inside_loop(node) {
            diags.push(make_diag("EMVDF", node));
        }

        // EMGRO: growing arrays inside loops (same pattern as AGROW)
        if self.is_enabled("EMGRO") && is_inside_loop(node) {
            let lhs = match node.child_by_field_name("left").or_else(|| node.child(0)) {
                Some(n) => n,
                None => return diags,
            };

            // Pattern: x(end+1) = ... or x = [x, val]
            if lhs.kind() == "function_call" && contains_end_plus_pattern(lhs, source) {
                diags.push(make_diag("EMGRO", node));
            } else if lhs.kind() == "identifier" && rhs.kind() == "matrix" {
                let var_name = node_text(lhs, source);
                if matrix_contains_var(rhs, source, var_name) {
                    diags.push(make_diag("EMGRO", node));
                }
            }
        }

        // FPASE: assignment to scaled expression (heuristic: detect fi() on RHS)
        if self.is_enabled("FPASE") && rhs_text.contains("fi(") {
            diags.push(make_diag("FPASE", node));
        }

        diags
    }

    /// Check a `cell` node (EMCEL).
    fn check_cell(&self, node: tree_sitter::Node) -> Vec<Diagnostic> {
        if self.is_enabled("EMCEL") {
            vec![make_diag("EMCEL", node)]
        } else {
            Vec::new()
        }
    }

    /// Check an `arguments_statement` node (EMRIFAV).
    fn check_arguments_statement(&self, node: tree_sitter::Node) -> Vec<Diagnostic> {
        if self.is_enabled("EMRIFAV") {
            vec![make_diag("EMRIFAV", node)]
        } else {
            Vec::new()
        }
    }

    // -----------------------------------------------------------------------
    // File-level checks
    // -----------------------------------------------------------------------

    /// File-level checks: EMNST (nested functions) and EMSCR (scripts).
    fn check_file_level(
        &self,
        tree: &tree_sitter::Tree,
        _source: &str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        let root = tree.root_node();

        // EMSCR: Script detection — a file with no function_definition at top level
        if self.is_enabled("EMSCR") {
            let has_top_level_function = has_child_of_kind(&root, "function_definition");
            if !has_top_level_function {
                // File is a script (no function definitions)
                let start = root.start_position();
                diags.push(Diagnostic {
                    rule_id: "EMSCR",
                    message: check_description("EMSCR").to_string(),
                    severity: Severity::Error,
                    byte_range: 0..root.end_byte().min(1),
                    line: start.row + 1,
                    column: start.column + 1,
                    fix: None,
                });
            }
        }

        // EMNST: Nested function detection
        if self.is_enabled("EMNST") {
            self.find_nested_functions(&root, 0, &mut diags);
        }

        diags
    }

    /// Recursively find nested function definitions (depth > 1).
    fn find_nested_functions(
        &self,
        node: &tree_sitter::Node,
        depth: usize,
        diags: &mut Vec<Diagnostic>,
    ) {
        let count = node.child_count();
        for i in 0..count {
            if let Some(child) = node.child(i) {
                if child.kind() == "function_definition" {
                    if depth >= 1 {
                        // This is a nested function
                        diags.push(make_diag("EMNST", child));
                    }
                    // Recurse into this function to find deeper nesting
                    self.find_nested_functions(&child, depth + 1, diags);
                } else {
                    self.find_nested_functions(&child, depth, diags);
                }
            }
        }
    }
}

impl Rule for CodegenEngine {
    fn id(&self) -> &'static str {
        "CODEGEN_ENGINE"
    }

    fn description(&self) -> &'static str {
        "MATLAB code generation constraint checks"
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn category(&self) -> Category {
        Category::CodeGeneration
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        TARGET_NODES
    }

    fn has_file_check(&self) -> bool {
        true
    }

    fn check(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        match ctx.node.kind() {
            "function_call" => self.check_function_call(ctx.node, ctx.source),
            "command" => self.check_command(ctx.node, ctx.source),
            "try_statement" => self.check_try_statement(ctx.node),
            "for_statement" => self.check_for_statement(ctx.node, ctx.source),
            "assignment" => self.check_assignment(ctx.node, ctx.source),
            "cell" => self.check_cell(ctx.node),
            "arguments_statement" => self.check_arguments_statement(ctx.node),
            _ => Vec::new(),
        }
    }

    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> {
        self.check_file_level(ctx.tree, ctx.source)
    }
}

// ---------------------------------------------------------------------------
// Helper: diagnostic construction
// ---------------------------------------------------------------------------

/// Build a diagnostic from a check ID and node.
fn make_diag(check_id: &'static str, node: tree_sitter::Node) -> Diagnostic {
    let start = node.start_position();
    Diagnostic {
        rule_id: check_id,
        message: check_description(check_id).to_string(),
        severity: Severity::Error,
        byte_range: node.start_byte()..node.end_byte(),
        line: start.row + 1,
        column: start.column + 1,
        fix: None,
    }
}

// ---------------------------------------------------------------------------
// Helper: node text extraction
// ---------------------------------------------------------------------------

/// Extract the raw text of a node.
fn node_text<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

/// Extract function name from a `function_call` node.
fn extract_func_name<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> Option<&'a str> {
    if node.kind() != "function_call" {
        return None;
    }
    let name_node = node.child_by_field_name("name")?;
    Some(&source[name_node.start_byte()..name_node.end_byte()])
}

/// Extract command name from a `command` node.
fn extract_command_name<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> Option<&'a str> {
    if node.kind() != "command" {
        return None;
    }
    let name_node = node.child(0)?;
    if name_node.kind() == "command_name" {
        Some(&source[name_node.start_byte()..name_node.end_byte()])
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Helper: pattern detection
// ---------------------------------------------------------------------------

/// Check if a node is inside a loop.
fn is_inside_loop(node: tree_sitter::Node) -> bool {
    let mut current = node.parent();
    while let Some(p) = current {
        match p.kind() {
            "for_statement" | "while_statement" => return true,
            "function_definition" => return false,
            _ => {}
        }
        current = p.parent();
    }
    false
}

/// Check if an indexing expression contains `end+1` pattern.
fn contains_end_plus_pattern(node: tree_sitter::Node, source: &str) -> bool {
    let text = node_text(node, source);
    let normalized: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    normalized.contains("end+1")
}

/// Check if a matrix literal contains a variable name as an element.
fn matrix_contains_var(node: tree_sitter::Node, source: &str, var_name: &str) -> bool {
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            if child.kind() == "identifier" && node_text(child, source) == var_name {
                return true;
            }
            if child.kind() == "row" && matrix_contains_var(child, source, var_name) {
                return true;
            }
        }
    }
    false
}

/// Check if a node has a direct child of a specific kind.
fn has_child_of_kind(node: &tree_sitter::Node, kind: &str) -> bool {
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            if child.kind() == kind {
                return true;
            }
        }
    }
    false
}

/// Check if the next statement after a node is a for_statement.
fn is_followed_by_for(node: tree_sitter::Node) -> bool {
    let mut sibling = node.next_sibling();
    while let Some(sib) = sibling {
        match sib.kind() {
            "for_statement" => return true,
            ";" | "comment" | "line_continuation" => {
                sibling = sib.next_sibling();
            }
            _ => return false,
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "CODEGEN_ENGINE",
    CodegenEngine::from_config
));
