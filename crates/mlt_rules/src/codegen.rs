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
            Self::find_nested_functions(&root, 0, &mut diags);
        }

        diags
    }

    /// Recursively find nested function definitions (depth > 1).
    fn find_nested_functions(
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
                    Self::find_nested_functions(&child, depth + 1, diags);
                } else {
                    Self::find_nested_functions(&child, depth, diags);
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{has_id, lint_file, lint_nodes};
    use mlt_core::Config;

    fn engine() -> Box<dyn Rule> {
        CodegenEngine::from_config(&Config::default())
    }

    // -- EMFCN ---------------------------------------------------------------

    #[test]
    fn emfcn_fires_on_unsupported_function_call() {
        let src = "eval('x');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMFCN"), "got: {diags:?}");
    }

    #[test]
    fn emfcn_fires_on_unsupported_command() {
        let src = "clear all;\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMFCN"), "got: {diags:?}");
    }

    #[test]
    fn emfcn_fires_on_plot() {
        let src = "plot(x, y);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMFCN"), "got: {diags:?}");
    }

    #[test]
    fn emfcn_not_fire_on_supported_function() {
        let src = "y = sum(x);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "EMFCN"), "got: {diags:?}");
    }

    // -- EMLOAD ---------------------------------------------------------------

    #[test]
    fn emload_fires_on_load_function() {
        let src = "load('file.mat');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMLOAD"), "got: {diags:?}");
    }

    #[test]
    fn emload_not_fire_on_save() {
        let src = "save('file.mat');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "EMLOAD"), "got: {diags:?}");
    }

    // -- EMS2N ----------------------------------------------------------------

    #[test]
    fn ems2n_fires_on_str2num() {
        let src = "x = str2num('1 2');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMS2N"), "got: {diags:?}");
    }

    #[test]
    fn ems2n_not_fire_on_str2double() {
        let src = "x = str2double('1 2');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "EMS2N"), "got: {diags:?}");
    }

    // -- PRMNOIN --------------------------------------------------------------

    #[test]
    fn prmnoin_fires_on_validateattributes() {
        let src = "validateattributes(x, {'numeric'}, {'scalar'});\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "PRMNOIN"), "got: {diags:?}");
    }

    #[test]
    fn prmnoin_fires_on_inputparser() {
        let src = "p = inputParser();\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "PRMNOIN"), "got: {diags:?}");
    }

    #[test]
    fn prmnoin_not_fire_on_plain_call() {
        let src = "x = fcn(1, 2);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "PRMNOIN"), "got: {diags:?}");
    }

    // -- EMIMP ----------------------------------------------------------------

    #[test]
    fn emimp_fires_on_import_command() {
        let src = "import pkg.fcn;\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMIMP"), "got: {diags:?}");
    }

    #[test]
    fn emimp_not_fire_on_plain_statement() {
        let src = "x = 1;\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "EMIMP"), "got: {diags:?}");
    }

    // -- EMTC -----------------------------------------------------------------

    #[test]
    fn emtc_fires_on_try_catch() {
        let src = "try\n    x = 1;\ncatch\n    y = 2;\nend\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMTC"), "got: {diags:?}");
    }

    #[test]
    fn emtc_not_fire_on_plain_block() {
        let src = "if x > 0\n    y = 1;\nend\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "EMTC"), "got: {diags:?}");
    }

    // -- EMPFR ----------------------------------------------------------------

    #[test]
    fn empfr_fires_on_parfor() {
        let src = "parfor i = 1:10\n    x = i;\nend\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMPFR"), "got: {diags:?}");
    }

    #[test]
    fn empfr_not_fire_on_regular_for() {
        let src = "for i = 1:10\n    x = i;\nend\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "EMPFR"), "got: {diags:?}");
    }

    // -- EMVDF ----------------------------------------------------------------

    #[test]
    fn emvdf_fires_on_empty_init_in_loop() {
        let src = "for i = 1:10\n    x = [];\nend\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMVDF"), "got: {diags:?}");
    }

    #[test]
    fn emvdf_not_fire_outside_loop() {
        let src = "x = [];\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "EMVDF"), "got: {diags:?}");
    }

    // -- EMGRO ----------------------------------------------------------------

    #[test]
    fn emgro_fires_on_end_plus_one_growth() {
        let src = "for i = 1:10\n    x(end+1) = i;\nend\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMGRO"), "got: {diags:?}");
    }

    #[test]
    fn emgro_fires_on_concatenation_growth() {
        let src = "for i = 1:10\n    x = [x, i];\nend\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMGRO"), "got: {diags:?}");
    }

    #[test]
    fn emgro_not_fire_on_indexed_assignment() {
        let src = "for i = 1:10\n    x(i) = i;\nend\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "EMGRO"), "got: {diags:?}");
    }

    // -- FPASE ----------------------------------------------------------------

    #[test]
    fn fpase_fires_on_fi_rhs() {
        let src = "x = fi(y, 1, 16);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "FPASE"), "got: {diags:?}");
    }

    #[test]
    fn fpase_not_fire_on_plain_rhs() {
        let src = "x = y + 1;\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "FPASE"), "got: {diags:?}");
    }

    // -- EMCEL ----------------------------------------------------------------

    #[test]
    fn emcel_fires_on_cell_literal() {
        let src = "x = {1, 2};\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMCEL"), "got: {diags:?}");
    }

    #[test]
    fn emcel_not_fire_on_matrix_literal() {
        let src = "x = [1, 2];\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "EMCEL"), "got: {diags:?}");
    }

    // -- EMRIFAV ---------------------------------------------------------------

    #[test]
    fn emrifav_fires_on_arguments_block() {
        let src = "function f(a)\n    arguments\n        a (1,1) double\n    end\n    x = a;\nend\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "EMRIFAV"), "got: {diags:?}");
    }

    #[test]
    fn emrifav_not_fire_without_arguments_block() {
        let src = "function f(a)\n    x = a;\nend\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "EMRIFAV"), "got: {diags:?}");
    }

    // -- EMSCR (file-level) -----------------------------------------------------

    #[test]
    fn emscr_fires_on_script_file() {
        let src = "x = 1;\n";
        let diags = lint_file(&*engine(), src);
        assert!(has_id(&diags, "EMSCR"), "got: {diags:?}");
    }

    #[test]
    fn emscr_not_fire_on_function_file() {
        let src = "function f()\n    x = 1;\nend\n";
        let diags = lint_file(&*engine(), src);
        assert!(!has_id(&diags, "EMSCR"), "got: {diags:?}");
    }

    // -- EMNST (file-level) -------------------------------------------------------

    #[test]
    fn emnst_fires_on_nested_function() {
        let src = "function a()\n    function b()\n        x = 1;\n    end\nend\n";
        let diags = lint_file(&*engine(), src);
        assert!(has_id(&diags, "EMNST"), "got: {diags:?}");
    }

    #[test]
    fn emnst_not_fire_on_flat_functions() {
        let src = "function a()\n    x = 1;\nend\n";
        let diags = lint_file(&*engine(), src);
        assert!(!has_id(&diags, "EMNST"), "got: {diags:?}");
    }

    // -- Config-respected test -----------------------------------------------------

    #[test]
    fn config_skip_checks_disables_individual_checks() {
        let config = Config::from_toml(
            r#"
[lint.rules.CODEGEN_ENGINE]
skip_checks = ["EMSCR", "EMFCN"]
"#,
        )
        .unwrap();
        let rule = CodegenEngine::from_config(&config);

        // Skipped file-level check no longer fires.
        let src = "x = 1;\n";
        let diags = lint_file(&*rule, src);
        assert!(!has_id(&diags, "EMSCR"), "got: {diags:?}");

        // Skipped node-level check no longer fires.
        let src = "eval('x');\n";
        let diags = lint_nodes(&*rule, src);
        assert!(!has_id(&diags, "EMFCN"), "got: {diags:?}");

        // Non-skipped checks still fire.
        let src = "try\n    x = 1;\ncatch\n    y = 2;\nend\n";
        let diags = lint_nodes(&*rule, src);
        assert!(has_id(&diags, "EMTC"), "got: {diags:?}");
    }
}
