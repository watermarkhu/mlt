//! # Language Specification Errors Engine
//!
//! This module implements language specification error checks from MATLAB's Code Analyzer.
//! All checks are handled by a single file-level engine (`LanguageSpecEngine`) that performs
//! a full DFS traversal maintaining context stacks for parfor/spmd detection.
//!
//! ## Check IDs
//!
//! ### Parfor restrictions (PF* checks)
//!
//! | Check ID | Description                                      |
//! |----------|--------------------------------------------------|
//! | PFPF     | Nested parfor inside parfor                      |
//! | PFSPMD   | SPMD inside parfor                               |
//! | PFBRK    | break inside parfor                              |
//! | PFRTN    | return inside parfor                             |
//! | PFGLOB   | global declaration inside parfor                 |
//! | PFPERS   | persistent declaration inside parfor             |
//! | PFFORA   | Assignment to for-loop variable inside parfor    |
//! | PFXST    | Assignment to parfor loop variable               |
//! | PFNF     | Nested function call inside parfor               |
//!
//! ### SPMD restrictions (SP* checks)
//!
//! | Check ID | Description                                      |
//! |----------|--------------------------------------------------|
//! | SPNST    | parfor or spmd inside spmd                       |
//! | SPRET    | return/break/continue inside spmd                |
//! | SPGP     | global/persistent inside spmd                    |
//!
//! ### Class/OOP rules (MC* checks)
//!
//! | Check ID | Description                                      |
//! |----------|--------------------------------------------------|
//! | MCFIL    | Class name and file name don't match             |
//! | MCDIR    | Class name and @directory name don't match       |
//! | MCRED    | Property/event/enum name same as class name      |
//! | MCCBD    | Constructor not in class definition file         |
//! | MCS2I    | Setter must have exactly 2 inputs                |
//! | MCS1O    | Setter must have at most 1 output                |
//! | MCG1I    | Getter must have exactly 1 input                 |
//! | MCG1O    | Getter must have exactly 1 output                |
//! | MCEB     | Events defined in non-handle class               |
//! | MCANI    | Abstract property initialized                    |
//! | MCASC    | Abstract property in Sealed class                |
//! | MCSGA    | Set/get method in methods block with attributes  |
//! | MCSGP    | Set/get method refers to invalid property        |
//!
//! ### Function validation (FV* checks)
//!
//! | Check ID | Description                                      |
//! |----------|--------------------------------------------------|
//! | FVNST    | Arguments blocks in nested functions             |
//! | VTPOD    | Validation out of order (size, then class, then functions) |
//!
//! ### Other language spec checks
//!
//! | Check ID | Description                                      |
//! |----------|--------------------------------------------------|
//! | FCONV    | Variable name same as script name                |
//! | FCONF    | Local function name same as file name            |
//! | GPFST    | Global/persistent must precede first use         |
//! | GPNES    | Global/persistent must be in outermost function  |
//! | NPERS    | Persistent in script                             |
//! | ROWLN    | Matrix rows must be same length                  |
//! | FCNANS   | Function named 'ans'                             |
//! | CLANS    | Class named 'ans'                                |
//! | BRKFOR   | break outside loop                               |
//! | CONTFOR  | continue outside loop                            |
//! | IDXCOLND | END operator outside index expression            |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.LANGUAGE_SPEC_ENGINE]
//! severity = "error"
//! ```

use mlt_core::{Category, Config, Diagnostic, FileContext, Rule, Severity};
use serde::Deserialize;

use crate::analysis::metadata::{ClassMeta, FileMeta, FileType};
use crate::analysis::symbols::{DefKind, SymbolTable};

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for the language specification engine.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct LanguageSpecConfig {
    // Reserved for future rule-specific parameters.
}

// ---------------------------------------------------------------------------
// Context tracking for DFS
// ---------------------------------------------------------------------------

/// Tracks what kind of loop/parallel context we are in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopContext {
    /// Normal code (not inside any special parallel context).
    Normal,
    /// Inside a regular for/while loop.
    Loop,
    /// Inside a parfor loop.
    Parfor,
    /// Inside an spmd block.
    Spmd,
}

/// A frame on the context stack during DFS traversal.
#[derive(Debug, Clone)]
struct ContextFrame {
    /// The type of context this frame represents.
    context: LoopContext,
    /// The parfor loop variable name (only set for Parfor context).
    parfor_var: Option<String>,
    /// The for-loop iterator variable names active in this parfor (accumulated).
    for_vars_in_parfor: Vec<String>,
}

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// Language specification error engine.
///
/// Implements all language specification constraint checks as a single file-level
/// rule. Uses DFS traversal with a context stack to detect parfor/spmd/loop
/// violations, plus class metadata analysis for OOP rule checking.
pub struct LanguageSpecEngine {
    #[allow(dead_code)]
    config: LanguageSpecConfig,
}

impl LanguageSpecEngine {
    /// Factory constructor. Reads rule-specific params from config.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: LanguageSpecConfig = config.rule_params("LANGUAGE_SPEC_ENGINE");
        Box::new(Self {
            config: rule_config,
        })
    }
}

impl Rule for LanguageSpecEngine {
    fn id(&self) -> &'static str {
        "LANGUAGE_SPEC_ENGINE"
    }

    fn description(&self) -> &'static str {
        "Language specification constraint violation"
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn category(&self) -> Category {
        Category::LanguageSpecification
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        &[] // File-level only
    }

    fn has_file_check(&self) -> bool {
        true
    }

    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Build metadata and symbol table
        let meta = FileMeta::build(ctx.tree, ctx.source);
        let symbol_table = SymbolTable::build(ctx.tree, ctx.source);

        // 1. DFS traversal for parfor/spmd/loop context violations
        let root = ctx.tree.root_node();
        let mut context_stack = vec![ContextFrame {
            context: LoopContext::Normal,
            parfor_var: None,
            for_vars_in_parfor: Vec::new(),
        }];
        self.check_dfs(root, ctx.source, &mut context_stack, &mut diagnostics);

        // 2. Class/OOP checks
        if let Some(ref class) = meta.class {
            self.check_class_rules(class, &meta, ctx, &mut diagnostics);
        }

        // 3. Function validation checks
        self.check_function_validation(&meta, ctx, &mut diagnostics);

        // 4. Script-level checks
        self.check_script_rules(&meta, &symbol_table, ctx, &mut diagnostics);

        // 5. Global/persistent ordering checks
        self.check_global_persistent_rules(&symbol_table, ctx, &mut diagnostics);

        // 6. break/continue outside loop (top-level check)
        self.check_break_continue_outside_loop(root, ctx.source, &mut diagnostics);

        // 7. Matrix row length checks
        self.check_matrix_rows(root, ctx.source, &mut diagnostics);

        // 8. Function/class named 'ans'
        self.check_ans_naming(&meta, ctx, &mut diagnostics);

        // 9. END operator outside index expression
        self.check_end_operator(root, ctx.source, &mut diagnostics);

        // 10. Local function name same as file name
        self.check_local_function_name_conflict(&meta, ctx, &mut diagnostics);

        diagnostics
    }
}

// ---------------------------------------------------------------------------
// DFS traversal for parfor/spmd/loop context checks
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// DFS traversal to detect parfor/spmd context violations.
    fn check_dfs(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context_stack: &mut Vec<ContextFrame>,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let kind = node.kind();
        let current_context = context_stack.last().map(|f| f.context).unwrap_or(LoopContext::Normal);

        match kind {
            "for_statement" => {
                let is_parfor = self.is_parfor_node(node);

                if is_parfor {
                    // This is a parfor loop

                    // Check: PFPF — nested parfor inside parfor
                    if current_context == LoopContext::Parfor {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "PFPF",
                            message: "Nested parfor is not allowed inside a parfor loop"
                                .to_string(),
                            severity: Severity::Error,
                            byte_range: node.start_byte()..node.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }

                    // Check: SPNST — parfor inside spmd
                    if current_context == LoopContext::Spmd {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "SPNST",
                            message: "parfor is not allowed inside an spmd block".to_string(),
                            severity: Severity::Error,
                            byte_range: node.start_byte()..node.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }

                    // Extract the parfor loop variable
                    let parfor_var = self.extract_for_variable(node, source);

                    // Push parfor context and recurse
                    context_stack.push(ContextFrame {
                        context: LoopContext::Parfor,
                        parfor_var,
                        for_vars_in_parfor: Vec::new(),
                    });
                    self.visit_children(node, source, context_stack, diagnostics);
                    context_stack.pop();
                } else {
                    // Normal for loop
                    if current_context == LoopContext::Parfor {
                        // Track the for-loop variable inside parfor
                        if let Some(var) = self.extract_for_variable(node, source) {
                            if let Some(frame) = context_stack.last_mut() {
                                frame.for_vars_in_parfor.push(var);
                            }
                        }
                        // Stay in Parfor context
                        self.visit_children(node, source, context_stack, diagnostics);
                        // Remove the for variable from tracking
                        if let Some(frame) = context_stack.last_mut() {
                            frame.for_vars_in_parfor.pop();
                        }
                    } else {
                        // Push normal loop context
                        context_stack.push(ContextFrame {
                            context: LoopContext::Loop,
                            parfor_var: None,
                            for_vars_in_parfor: Vec::new(),
                        });
                        self.visit_children(node, source, context_stack, diagnostics);
                        context_stack.pop();
                    }
                }
                return;
            }

            "while_statement" => {
                if current_context == LoopContext::Parfor {
                    // Stay in Parfor context for while inside parfor
                    self.visit_children(node, source, context_stack, diagnostics);
                } else {
                    // Push normal loop context
                    context_stack.push(ContextFrame {
                        context: LoopContext::Loop,
                        parfor_var: None,
                        for_vars_in_parfor: Vec::new(),
                    });
                    self.visit_children(node, source, context_stack, diagnostics);
                    context_stack.pop();
                }
                return;
            }

            "spmd_statement" => {
                // Check: PFSPMD — spmd inside parfor
                if current_context == LoopContext::Parfor {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "PFSPMD",
                        message: "spmd is not allowed inside a parfor loop".to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }

                // Check: SPNST — spmd inside spmd
                if current_context == LoopContext::Spmd {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "SPNST",
                        message: "spmd is not allowed inside another spmd block".to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }

                // Push spmd context and recurse
                context_stack.push(ContextFrame {
                    context: LoopContext::Spmd,
                    parfor_var: None,
                    for_vars_in_parfor: Vec::new(),
                });
                self.visit_children(node, source, context_stack, diagnostics);
                context_stack.pop();
                return;
            }

            "break_statement" => {
                // Check: PFBRK — break inside parfor
                if current_context == LoopContext::Parfor {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "PFBRK",
                        message: "break is not allowed inside a parfor loop".to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
                // Check: SPRET — break inside spmd
                if current_context == LoopContext::Spmd {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "SPRET",
                        message: "break is not allowed inside an spmd block".to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }

            "continue_statement" if current_context == LoopContext::Spmd => {
                // Check: SPRET — continue inside spmd
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "SPRET",
                    message: "continue is not allowed inside an spmd block".to_string(),
                    severity: Severity::Error,
                    byte_range: node.start_byte()..node.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }

            "return_statement" => {
                // Check: PFRTN — return inside parfor
                if current_context == LoopContext::Parfor {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "PFRTN",
                        message: "return is not allowed inside a parfor loop".to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
                // Check: SPRET — return inside spmd
                if current_context == LoopContext::Spmd {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "SPRET",
                        message: "return is not allowed inside an spmd block".to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }

            "global_operator" => {
                // Check: PFGLOB — global inside parfor
                if current_context == LoopContext::Parfor {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "PFGLOB",
                        message: "global declarations are not allowed inside a parfor loop"
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
                // Check: SPGP — global inside spmd
                if current_context == LoopContext::Spmd {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "SPGP",
                        message: "global declarations are not allowed inside an spmd block"
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }

            "persistent_operator" => {
                // Check: PFPERS — persistent inside parfor
                if current_context == LoopContext::Parfor {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "PFPERS",
                        message: "persistent declarations are not allowed inside a parfor loop"
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
                // Check: SPGP — persistent inside spmd
                if current_context == LoopContext::Spmd {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "SPGP",
                        message: "persistent declarations are not allowed inside an spmd block"
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }

            "assignment" if current_context == LoopContext::Parfor => {
                // Check: PFXST — assignment to parfor loop variable
                // Check: PFFORA — assignment to for-loop variable inside parfor
                if let Some(lhs_name) = self.extract_assignment_lhs_name(node, source) {
                    // PFXST: assignment to the parfor loop variable itself
                    let parfor_var = context_stack
                        .iter()
                        .rev()
                        .find(|f| f.context == LoopContext::Parfor)
                        .and_then(|f| f.parfor_var.as_deref());
                    if let Some(pv) = parfor_var {
                        if lhs_name == pv {
                            let pos = node.start_position();
                            diagnostics.push(Diagnostic {
                                rule_id: "PFXST",
                                message: format!(
                                    "Assignment to parfor loop variable '{lhs_name}' is not allowed"
                                ),
                                severity: Severity::Error,
                                byte_range: node.start_byte()..node.end_byte(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        }
                    }

                    // PFFORA: assignment to a for-loop variable inside the parfor
                    let for_vars: Vec<String> = context_stack
                        .iter()
                        .rev()
                        .find(|f| f.context == LoopContext::Parfor)
                        .map(|f| f.for_vars_in_parfor.clone())
                        .unwrap_or_default();
                    if for_vars.contains(&lhs_name) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "PFFORA",
                            message: format!(
                                "Assignment to for-loop variable '{lhs_name}' inside parfor is not allowed"
                            ),
                            severity: Severity::Error,
                            byte_range: node.start_byte()..node.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            }

            "function_definition" => {
                // Check: PFNF — nested function definition inside parfor
                if current_context == LoopContext::Parfor {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "PFNF",
                        message: "Nested function definitions are not allowed inside a parfor loop"
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                    // Don't recurse into nested function_definitions from parfor context
                    return;
                }
                if current_context == LoopContext::Spmd {
                    // Don't recurse into function_definitions from spmd context
                    return;
                }
                // For Normal/Loop context, recurse normally into function body
                // (but reset context to Normal since function creates its own scope)
                context_stack.push(ContextFrame {
                    context: LoopContext::Normal,
                    parfor_var: None,
                    for_vars_in_parfor: Vec::new(),
                });
                self.visit_children(node, source, context_stack, diagnostics);
                context_stack.pop();
                return;
            }

            _ => {}
        }

        // Recurse into children
        self.visit_children(node, source, context_stack, diagnostics);
    }

    /// Visit all children of a node during DFS traversal.
    fn visit_children(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context_stack: &mut Vec<ContextFrame>,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.check_dfs(child, source, context_stack, diagnostics);
        }
    }

    /// Check if a `for_statement` node is actually a `parfor` loop.
    ///
    /// In tree-sitter-matlab, `parfor i = 1:10 ... end` is parsed as a
    /// `for_statement` with a child node of kind `"parfor"` (instead of `"for"`).
    fn is_parfor_node(&self, node: tree_sitter::Node) -> bool {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "parfor" {
                return true;
            }
            // Stop early once we hit the iterator (keyword comes before it)
            if child.kind() == "iterator" {
                break;
            }
        }
        false
    }

    /// Extract the loop variable from a `for_statement` (works for both for and parfor).
    fn extract_for_variable(&self, node: tree_sitter::Node, source: &str) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "iterator" {
                return self.extract_iterator_variable(child, source);
            }
        }
        None
    }

    /// Extract the variable name from an iterator node.
    fn extract_iterator_variable(
        &self,
        iter_node: tree_sitter::Node,
        source: &str,
    ) -> Option<String> {
        let mut cursor = iter_node.walk();
        for child in iter_node.children(&mut cursor) {
            if child.kind() == "identifier" {
                return Some(node_text(child, source).to_string());
            }
        }
        None
    }

    /// Extract the LHS variable name from a simple assignment.
    fn extract_assignment_lhs_name(
        &self,
        node: tree_sitter::Node,
        source: &str,
    ) -> Option<String> {
        let lhs = node.child_by_field_name("left")?;
        if lhs.kind() == "identifier" {
            Some(node_text(lhs, source).to_string())
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Class/OOP rule checks
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// Check all class/OOP related rules.
    fn check_class_rules(
        &self,
        class: &ClassMeta,
        meta: &FileMeta,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        self.check_mcfil(class, ctx, diagnostics);
        self.check_mcdir(class, ctx, diagnostics);
        self.check_mcred(class, diagnostics);
        self.check_mccbd(class, meta, ctx, diagnostics);
        self.check_setter_getter_signatures(class, meta, diagnostics);
        self.check_mceb(class, diagnostics);
        self.check_mcani(class, diagnostics);
        self.check_mcasc(class, diagnostics);
        self.check_mcsga(class, meta, diagnostics);
        self.check_mcsgp(class, meta, diagnostics);
    }

    /// MCFIL: Class name must match file name.
    fn check_mcfil(
        &self,
        class: &ClassMeta,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let file_stem = ctx
            .file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if !file_stem.is_empty() && file_stem != class.name {
            // Don't fire if the file is in a @directory (MCDIR handles that)
            let in_at_dir = ctx
                .file_path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .is_some_and(|dir| dir.starts_with('@'));
            if !in_at_dir {
                diagnostics.push(Diagnostic {
                    rule_id: "MCFIL",
                    message: format!(
                        "Class name '{}' does not match file name '{}'",
                        class.name, file_stem
                    ),
                    severity: Severity::Error,
                    byte_range: class.byte_range.clone(),
                    line: class.line,
                    column: 1,
                    fix: None,
                });
            }
        }
    }

    /// MCDIR: Class name must match @directory name.
    fn check_mcdir(
        &self,
        class: &ClassMeta,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if let Some(parent_dir) = ctx.file_path.parent() {
            if let Some(dir_name) = parent_dir.file_name().and_then(|n| n.to_str()) {
                if let Some(stripped) = dir_name.strip_prefix('@') {
                    if stripped != class.name {
                        diagnostics.push(Diagnostic {
                            rule_id: "MCDIR",
                            message: format!(
                                "Class name '{}' does not match @directory name '@{}'",
                                class.name, stripped
                            ),
                            severity: Severity::Error,
                            byte_range: class.byte_range.clone(),
                            line: class.line,
                            column: 1,
                            fix: None,
                        });
                    }
                }
            }
        }
    }

    /// MCRED: Property/event/enum name same as class name.
    fn check_mcred(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        // Check properties
        for pb in &class.properties_blocks {
            for prop in &pb.properties {
                if prop.name == class.name {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCRED",
                        message: format!(
                            "Property '{}' has the same name as the class",
                            prop.name
                        ),
                        severity: Severity::Error,
                        byte_range: prop.byte_range.clone(),
                        line: prop.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }

        // Check events
        for eb in &class.events_blocks {
            for event in &eb.events {
                if *event == class.name {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCRED",
                        message: format!(
                            "Event '{}' has the same name as the class",
                            event
                        ),
                        severity: Severity::Error,
                        byte_range: class.byte_range.clone(),
                        line: class.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }

        // Check enumeration members
        for enb in &class.enumeration_blocks {
            for member in &enb.members {
                if *member == class.name {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCRED",
                        message: format!(
                            "Enumeration member '{}' has the same name as the class",
                            member
                        ),
                        severity: Severity::Error,
                        byte_range: class.byte_range.clone(),
                        line: class.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }
    }

    /// MCCBD: Constructor must be defined in the class definition file.
    fn check_mccbd(
        &self,
        class: &ClassMeta,
        meta: &FileMeta,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // If we're in a @directory class, the constructor may be in a separate file.
        // Check if there's a constructor in the methods blocks.
        let in_at_dir = ctx
            .file_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .is_some_and(|dir| dir.starts_with('@'));

        if in_at_dir {
            // In @directory mode, skip this check — constructors can be in separate files.
            return;
        }

        // For single-file classes, check that a constructor is defined for non-abstract classes.
        // Actually, MCCBD checks that if a constructor IS defined, it should be in the class file.
        // This is mainly relevant for @folder classes where methods can be in separate files.
        // For single-file classes we verify the constructor is in a methods block (not local func).
        for func in &meta.local_functions {
            if func.name == class.name {
                diagnostics.push(Diagnostic {
                    rule_id: "MCCBD",
                    message: format!(
                        "Constructor '{}' should be defined inside a methods block, not as a local function",
                        class.name
                    ),
                    severity: Severity::Error,
                    byte_range: func.byte_range.clone(),
                    line: func.line,
                    column: 1,
                    fix: None,
                });
            }
        }
    }

    /// MCS2I, MCS1O, MCG1I, MCG1O: Setter/getter signature validation.
    fn check_setter_getter_signatures(
        &self,
        _class: &ClassMeta,
        meta: &FileMeta,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        for func in &meta.functions {
            if func.is_setter {
                // MCS2I: Setter must have exactly 2 inputs (obj, value)
                if func.inputs.len() != 2 {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCS2I",
                        message: format!(
                            "Setter '{}' must have exactly 2 input arguments (obj, value), found {}",
                            func.name,
                            func.inputs.len()
                        ),
                        severity: Severity::Error,
                        byte_range: func.byte_range.clone(),
                        line: func.line,
                        column: 1,
                        fix: None,
                    });
                }

                // MCS1O: Setter must have at most 1 output (obj)
                if func.outputs.len() > 1 {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCS1O",
                        message: format!(
                            "Setter '{}' must have at most 1 output argument, found {}",
                            func.name,
                            func.outputs.len()
                        ),
                        severity: Severity::Error,
                        byte_range: func.byte_range.clone(),
                        line: func.line,
                        column: 1,
                        fix: None,
                    });
                }
            }

            if func.is_getter {
                // MCG1I: Getter must have exactly 1 input (obj)
                if func.inputs.len() != 1 {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCG1I",
                        message: format!(
                            "Getter '{}' must have exactly 1 input argument (obj), found {}",
                            func.name,
                            func.inputs.len()
                        ),
                        severity: Severity::Error,
                        byte_range: func.byte_range.clone(),
                        line: func.line,
                        column: 1,
                        fix: None,
                    });
                }

                // MCG1O: Getter must have exactly 1 output
                if func.outputs.len() != 1 {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCG1O",
                        message: format!(
                            "Getter '{}' must have exactly 1 output argument, found {}",
                            func.name,
                            func.outputs.len()
                        ),
                        severity: Severity::Error,
                        byte_range: func.byte_range.clone(),
                        line: func.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }
    }

    /// MCEB: Events can only be defined in handle classes.
    fn check_mceb(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !class.events_blocks.is_empty() && !class.is_handle() {
            for eb in &class.events_blocks {
                if !eb.events.is_empty() {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCEB",
                        message: "Events can only be defined in classes that inherit from handle"
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: class.byte_range.clone(),
                        line: class.line,
                        column: 1,
                        fix: None,
                    });
                    break; // Only report once per class
                }
            }
        }
    }

    /// MCANI: Abstract property must not have a default value.
    fn check_mcani(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        for pb in &class.properties_blocks {
            let is_abstract = pb.attributes.iter().any(|a| a.name == "Abstract" && !a.negated);
            if is_abstract {
                for prop in &pb.properties {
                    if prop.has_default {
                        diagnostics.push(Diagnostic {
                            rule_id: "MCANI",
                            message: format!(
                                "Abstract property '{}' cannot have a default value",
                                prop.name
                            ),
                            severity: Severity::Error,
                            byte_range: prop.byte_range.clone(),
                            line: prop.line,
                            column: 1,
                            fix: None,
                        });
                    }
                }
            }
        }
    }

    /// MCASC: Abstract properties cannot be defined in Sealed classes.
    fn check_mcasc(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if class.is_sealed() {
            for pb in &class.properties_blocks {
                let is_abstract = pb.attributes.iter().any(|a| a.name == "Abstract" && !a.negated);
                if is_abstract {
                    for prop in &pb.properties {
                        diagnostics.push(Diagnostic {
                            rule_id: "MCASC",
                            message: format!(
                                "Abstract property '{}' cannot be defined in a Sealed class",
                                prop.name
                            ),
                            severity: Severity::Error,
                            byte_range: prop.byte_range.clone(),
                            line: prop.line,
                            column: 1,
                            fix: None,
                        });
                    }
                }
            }
        }
    }

    /// MCSGA: Set/get methods should not be in methods blocks with attributes.
    fn check_mcsga(
        &self,
        _class: &ClassMeta,
        meta: &FileMeta,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        for func in &meta.functions {
            if (func.is_setter || func.is_getter) && !func.method_attributes.is_empty() {
                // Check if the methods block has attributes other than Access
                let has_non_trivial_attrs = func.method_attributes.iter().any(|a| {
                    a.name != "Access"
                        || a.value
                            .as_deref()
                            .is_some_and(|v| v != "public" && v != "?")
                });
                if has_non_trivial_attrs {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCSGA",
                        message: format!(
                            "Set/get method '{}' should not be in a methods block with attributes",
                            func.name
                        ),
                        severity: Severity::Error,
                        byte_range: func.byte_range.clone(),
                        line: func.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }
    }

    /// MCSGP: Set/get method must refer to a valid property.
    fn check_mcsgp(
        &self,
        _class: &ClassMeta,
        meta: &FileMeta,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        for func in &meta.functions {
            if func.is_setter || func.is_getter {
                // Extract property name from "set.PropName" or "get.PropName"
                let prop_name = if func.is_setter {
                    func.name.strip_prefix("set.")
                } else {
                    func.name.strip_prefix("get.")
                };

                if let Some(prop_name) = prop_name {
                    if !meta.has_property(prop_name) {
                        diagnostics.push(Diagnostic {
                            rule_id: "MCSGP",
                            message: format!(
                                "Set/get method '{}' refers to non-existent property '{}'",
                                func.name, prop_name
                            ),
                            severity: Severity::Error,
                            byte_range: func.byte_range.clone(),
                            line: func.line,
                            column: 1,
                            fix: None,
                        });
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Function validation checks
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// Check function validation rules.
    fn check_function_validation(
        &self,
        meta: &FileMeta,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        self.check_fvnst(ctx, diagnostics);
        self.check_vtpod(meta, diagnostics);
    }

    /// FVNST: Arguments blocks in nested functions are not allowed.
    fn check_fvnst(&self, ctx: &FileContext, diagnostics: &mut Vec<Diagnostic>) {
        // Walk the tree to find nested function_definitions with arguments blocks
        let root = ctx.tree.root_node();
        self.find_nested_functions_with_args(root, 0, diagnostics);
    }

    /// Recursively find nested functions that have arguments blocks.
    fn find_nested_functions_with_args(
        &self,
        node: tree_sitter::Node,
        depth: usize,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "function_definition" {
                if depth > 0 {
                    // This is a nested function; check for arguments blocks
                    let mut inner_cursor = child.walk();
                    for inner in child.children(&mut inner_cursor) {
                        if inner.kind() == "arguments_statement" {
                            let pos = inner.start_position();
                            diagnostics.push(Diagnostic {
                                rule_id: "FVNST",
                                message: "Arguments blocks are not allowed in nested functions"
                                    .to_string(),
                                severity: Severity::Error,
                                byte_range: inner.start_byte()..inner.end_byte(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        }
                    }
                }
                // Recurse into the function definition at depth+1
                self.find_nested_functions_with_args(child, depth + 1, diagnostics);
            } else if child.kind() != "methods" && child.kind() != "class_definition" {
                // Continue recursing but don't increase depth for non-function nodes
                // Skip methods blocks (class methods are not nested functions)
                self.find_nested_functions_with_args(child, depth, diagnostics);
            }
        }
    }

    /// VTPOD: Validation order must be: size, class, then functions.
    ///
    /// Since the parser extracts validation components in source order,
    /// the ordering is inherently encoded in the struct. The real check
    /// requires inspecting the raw AST node order for edge cases.
    /// TODO: Implement detailed AST-level ordering check for edge cases.
    fn check_vtpod(&self, _meta: &FileMeta, _diagnostics: &mut Vec<Diagnostic>) {
        // Currently a no-op stub. The tree-sitter grammar enforces the
        // basic ordering. A future implementation will check for degenerate
        // cases where user annotations conflict with MATLAB's expected order.
    }
}

// ---------------------------------------------------------------------------
// Script-level checks
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// Check script-level rules.
    fn check_script_rules(
        &self,
        meta: &FileMeta,
        symbol_table: &SymbolTable,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if meta.file_type != FileType::Script {
            return;
        }

        // NPERS: Persistent declarations are not allowed in scripts
        self.check_npers(symbol_table, diagnostics);

        // FCONV: Variable name same as script name
        self.check_fconv(symbol_table, ctx, diagnostics);
    }

    /// NPERS: Persistent declarations in script files.
    fn check_npers(&self, symbol_table: &SymbolTable, diagnostics: &mut Vec<Diagnostic>) {
        let root_scope = symbol_table.root_scope();
        for def in &root_scope.defs {
            if def.kind == DefKind::Persistent {
                diagnostics.push(Diagnostic {
                    rule_id: "NPERS",
                    message: format!(
                        "persistent declaration of '{}' is not allowed in a script",
                        def.name
                    ),
                    severity: Severity::Error,
                    byte_range: def.byte_range.clone(),
                    line: def.line,
                    column: def.column,
                    fix: None,
                });
            }
        }
    }

    /// FCONV: Variable name same as script file name.
    fn check_fconv(
        &self,
        symbol_table: &SymbolTable,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let file_stem = ctx
            .file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if file_stem.is_empty() {
            return;
        }

        let root_scope = symbol_table.root_scope();
        for def in &root_scope.defs {
            if def.name == file_stem && def.kind == DefKind::Assignment {
                diagnostics.push(Diagnostic {
                    rule_id: "FCONV",
                    message: format!(
                        "Variable '{}' has the same name as the script file",
                        def.name
                    ),
                    severity: Severity::Error,
                    byte_range: def.byte_range.clone(),
                    line: def.line,
                    column: def.column,
                    fix: None,
                });
                break; // Only report first occurrence
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Global/persistent ordering checks
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// Check global/persistent ordering rules.
    fn check_global_persistent_rules(
        &self,
        symbol_table: &SymbolTable,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        self.check_gpfst(symbol_table, diagnostics);
        self.check_gpnes(ctx, diagnostics);
    }

    /// GPFST: Global/persistent must precede first use.
    fn check_gpfst(&self, symbol_table: &SymbolTable, diagnostics: &mut Vec<Diagnostic>) {
        for scope in &symbol_table.scopes {
            // Find all global/persistent declarations
            let gp_defs: Vec<&_> = scope
                .defs
                .iter()
                .filter(|d| d.kind == DefKind::Global || d.kind == DefKind::Persistent)
                .collect();

            for gp_def in &gp_defs {
                // Check if there are any uses of this variable before the declaration
                let first_use = scope
                    .uses
                    .iter()
                    .filter(|u| u.name == gp_def.name)
                    .min_by_key(|u| u.byte_range.start);

                if let Some(first_use) = first_use {
                    if first_use.byte_range.start < gp_def.byte_range.start {
                        diagnostics.push(Diagnostic {
                            rule_id: "GPFST",
                            message: format!(
                                "'{}' is used before its global/persistent declaration",
                                gp_def.name
                            ),
                            severity: Severity::Error,
                            byte_range: gp_def.byte_range.clone(),
                            line: gp_def.line,
                            column: gp_def.column,
                            fix: None,
                        });
                    }
                }

                // Also check if there are any assignment definitions before the declaration
                let prior_assignment = scope.defs.iter().find(|d| {
                    d.name == gp_def.name
                        && d.kind == DefKind::Assignment
                        && d.byte_range.start < gp_def.byte_range.start
                });
                if let Some(prior) = prior_assignment {
                    let _ = prior;
                    diagnostics.push(Diagnostic {
                        rule_id: "GPFST",
                        message: format!(
                            "'{}' is assigned before its global/persistent declaration",
                            gp_def.name
                        ),
                        severity: Severity::Error,
                        byte_range: gp_def.byte_range.clone(),
                        line: gp_def.line,
                        column: gp_def.column,
                        fix: None,
                    });
                }
            }
        }
    }

    /// GPNES: Global/persistent must be in outermost function (not nested).
    fn check_gpnes(&self, ctx: &FileContext, diagnostics: &mut Vec<Diagnostic>) {
        // Walk the tree to find nested functions with global/persistent
        let root = ctx.tree.root_node();
        self.find_gp_in_nested(root, 0, diagnostics);
    }

    /// Recursively find global/persistent in nested functions.
    fn find_gp_in_nested(
        &self,
        node: tree_sitter::Node,
        depth: usize,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "function_definition" {
                if depth > 0 {
                    // This is a nested function; check for global/persistent
                    self.find_gp_statements(child, diagnostics);
                }
                self.find_gp_in_nested(child, depth + 1, diagnostics);
            } else if child.kind() != "methods" && child.kind() != "class_definition" {
                self.find_gp_in_nested(child, depth, diagnostics);
            }
        }
    }

    /// Find global/persistent statements within a function definition.
    fn find_gp_statements(
        &self,
        func_node: tree_sitter::Node,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut cursor = func_node.walk();
        for child in func_node.children(&mut cursor) {
            match child.kind() {
                "global_operator" | "persistent_operator" => {
                    let pos = child.start_position();
                    let keyword = if child.kind() == "global_operator" {
                        "global"
                    } else {
                        "persistent"
                    };
                    diagnostics.push(Diagnostic {
                        rule_id: "GPNES",
                        message: format!(
                            "{keyword} declaration is not allowed in a nested function"
                        ),
                        severity: Severity::Error,
                        byte_range: child.start_byte()..child.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
                "block" => {
                    // Recurse into the block to find global/persistent
                    self.find_gp_statements(child, diagnostics);
                }
                _ => {}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Break/continue outside loop check
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// BRKFOR / CONTFOR: break/continue must be inside a loop.
    fn check_break_continue_outside_loop(
        &self,
        root: tree_sitter::Node,
        _source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        self.check_break_continue_dfs(root, false, diagnostics);
    }

    /// DFS to find break/continue outside loops.
    fn check_break_continue_dfs(
        &self,
        node: tree_sitter::Node,
        in_loop: bool,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        match node.kind() {
            "for_statement" | "while_statement" => {
                // Inside a loop now
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.check_break_continue_dfs(child, true, diagnostics);
                }
                return;
            }
            "function_definition" => {
                // Reset loop context for new function scope
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.check_break_continue_dfs(child, false, diagnostics);
                }
                return;
            }
            "break_statement" if !in_loop => {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "BRKFOR",
                    message: "break is not inside a for or while loop".to_string(),
                    severity: Severity::Error,
                    byte_range: node.start_byte()..node.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
            "continue_statement" if !in_loop => {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "CONTFOR",
                    message: "continue is not inside a for or while loop".to_string(),
                    severity: Severity::Error,
                    byte_range: node.start_byte()..node.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.check_break_continue_dfs(child, in_loop, diagnostics);
        }
    }
}

// ---------------------------------------------------------------------------
// Matrix row length checks
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// ROWLN: Matrix rows must be the same length.
    fn check_matrix_rows(
        &self,
        root: tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        self.check_matrix_rows_dfs(root, source, diagnostics);
    }

    /// DFS to find matrix nodes and check row consistency.
    fn check_matrix_rows_dfs(
        &self,
        node: tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "matrix" {
            self.validate_matrix_rows(node, source, diagnostics);
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.check_matrix_rows_dfs(child, source, diagnostics);
        }
    }

    /// Validate that all rows in a matrix have the same number of elements.
    fn validate_matrix_rows(
        &self,
        matrix_node: tree_sitter::Node,
        _source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut row_lengths: Vec<usize> = Vec::new();
        let mut cursor = matrix_node.walk();

        for child in matrix_node.children(&mut cursor) {
            if child.kind() == "row" {
                // Count the number of expression elements in this row
                let mut elem_count = 0;
                let mut inner_cursor = child.walk();
                for elem in child.children(&mut inner_cursor) {
                    let k = elem.kind();
                    // Skip punctuation/separators
                    if k != "," && k != ";" && k != "[" && k != "]" && k != " " && elem.is_named()
                    {
                        elem_count += 1;
                    }
                }
                row_lengths.push(elem_count);
            }
        }

        // Check if all rows have the same length (only if there are multiple rows)
        if row_lengths.len() > 1 {
            let first_len = row_lengths[0];
            if row_lengths.iter().any(|&len| len != first_len && first_len > 0 && len > 0) {
                let pos = matrix_node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "ROWLN",
                    message: "Matrix rows have inconsistent lengths".to_string(),
                    severity: Severity::Error,
                    byte_range: matrix_node.start_byte()..matrix_node.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Function/class 'ans' naming check
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// FCNANS / CLANS: Function or class named 'ans'.
    fn check_ans_naming(
        &self,
        meta: &FileMeta,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // CLANS: Class named 'ans'
        if let Some(ref class) = meta.class {
            if class.name == "ans" {
                diagnostics.push(Diagnostic {
                    rule_id: "CLANS",
                    message: "Class cannot be named 'ans'".to_string(),
                    severity: Severity::Error,
                    byte_range: class.byte_range.clone(),
                    line: class.line,
                    column: 1,
                    fix: None,
                });
            }
        }

        // FCNANS: Function named 'ans'
        for func in meta.functions.iter().chain(meta.local_functions.iter()) {
            if func.name == "ans" {
                diagnostics.push(Diagnostic {
                    rule_id: "FCNANS",
                    message: "Function cannot be named 'ans'".to_string(),
                    severity: Severity::Error,
                    byte_range: func.byte_range.clone(),
                    line: func.line,
                    column: 1,
                    fix: None,
                });
            }
        }

        // Also check by file name for the main function
        if meta.file_type == FileType::FunctionFile {
            let file_stem = ctx
                .file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            if file_stem == "ans" {
                if let Some(main) = meta.main_function() {
                    if main.name != "ans" {
                        // File is named 'ans' but function has a different name — still flag
                        diagnostics.push(Diagnostic {
                            rule_id: "FCNANS",
                            message: "Function file cannot be named 'ans.m'".to_string(),
                            severity: Severity::Error,
                            byte_range: main.byte_range.clone(),
                            line: main.line,
                            column: 1,
                            fix: None,
                        });
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// END operator outside index expression
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// IDXCOLND: END operator used outside of an indexing expression.
    fn check_end_operator(
        &self,
        root: tree_sitter::Node,
        _source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        self.check_end_dfs(root, false, diagnostics);
    }

    /// DFS to find `end` keywords used as values outside of indexing contexts.
    fn check_end_dfs(
        &self,
        node: tree_sitter::Node,
        in_index: bool,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        match node.kind() {
            "function_call" => {
                // The arguments of a function_call could be indexing context
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    // Arguments inside function_call are in indexing context
                    // (since function_call is also used for array indexing)
                    self.check_end_dfs(child, true, diagnostics);
                }
                return;
            }
            "cell_index" => {
                // Cell indexing — end is valid here
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.check_end_dfs(child, true, diagnostics);
                }
                return;
            }
            "end" | "end_operator" if !in_index => {
                // `end` used as a value (not as block terminator)
                let pos = node.start_position();
                // Only flag if this looks like an end used as a value expression,
                // not a block-closing end keyword
                let parent = node.parent();
                let is_block_end = parent
                    .map(|p| {
                        matches!(
                            p.kind(),
                            "function_definition"
                                | "if_statement"
                                | "for_statement"
                                | "while_statement"
                                | "switch_statement"
                                | "try_statement"
                                | "class_definition"
                                | "properties"
                                | "methods"
                                | "events"
                                | "enumeration"
                                | "parfor"
                                | "spmd_statement"
                                | "elseif_clause"
                                | "else_clause"
                                | "case_clause"
                                | "otherwise_clause"
                                | "catch_clause"
                        )
                    })
                    .unwrap_or(false);

                if !is_block_end {
                    diagnostics.push(Diagnostic {
                        rule_id: "IDXCOLND",
                        message: "END operator is only valid inside an index expression"
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.check_end_dfs(child, in_index, diagnostics);
        }
    }
}

// ---------------------------------------------------------------------------
// Local function name conflict
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// FCONF: Local function name same as file name.
    fn check_local_function_name_conflict(
        &self,
        meta: &FileMeta,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let file_stem = ctx
            .file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if file_stem.is_empty() {
            return;
        }

        for func in &meta.local_functions {
            if func.name == file_stem {
                diagnostics.push(Diagnostic {
                    rule_id: "FCONF",
                    message: format!(
                        "Local function '{}' has the same name as the file",
                        func.name
                    ),
                    severity: Severity::Error,
                    byte_range: func.byte_range.clone(),
                    line: func.line,
                    column: 1,
                    fix: None,
                });
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract the source text covered by a node.
fn node_text<'a>(node: tree_sitter::Node, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "LANGUAGE_SPEC_ENGINE",
    LanguageSpecEngine::from_config
));

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use mlt_core::FileContext;
    use std::path::Path;
    use tree_sitter::Parser;

    /// Parse MATLAB source and return the tree.
    fn parse_matlab(source: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        let language = tree_sitter_matlab::LANGUAGE;
        parser
            .set_language(&language.into())
            .expect("failed to set MATLAB language");
        parser.parse(source, None).expect("failed to parse")
    }

    /// Run the language spec engine on source code with a given file path.
    fn check_source(source: &str, file_path: &str) -> Vec<Diagnostic> {
        let config = Config::default();
        let rule = LanguageSpecEngine::from_config(&config);
        let tree = parse_matlab(source);
        let path = Path::new(file_path);
        let ctx = FileContext {
            tree: &tree,
            source,
            file_path: path,
        };
        rule.check_file(&ctx)
    }

    /// Filter diagnostics by rule_id.
    fn filter_by_id<'a>(diagnostics: &'a [Diagnostic], id: &str) -> Vec<&'a Diagnostic> {
        diagnostics.iter().filter(|d| d.rule_id == id).collect()
    }

    // ===== Parfor restriction tests =====

    #[test]
    fn test_pfpf_nested_parfor() {
        let source = "\
function foo()
    parfor i = 1:10
        parfor j = 1:5
            x = i + j;
        end
    end
end
";
        let diags = check_source(source, "foo.m");
        let pfpf = filter_by_id(&diags, "PFPF");
        assert!(!pfpf.is_empty(), "PFPF should fire for nested parfor");
    }

    #[test]
    fn test_pfspmd_spmd_in_parfor() {
        let source = "\
function foo()
    parfor i = 1:10
        spmd
            x = i;
        end
    end
end
";
        let diags = check_source(source, "foo.m");
        let pfspmd = filter_by_id(&diags, "PFSPMD");
        assert!(!pfspmd.is_empty(), "PFSPMD should fire for spmd inside parfor");
    }

    #[test]
    fn test_pfbrk_break_in_parfor() {
        let source = "\
function foo()
    parfor i = 1:10
        if i > 5
            break;
        end
    end
end
";
        let diags = check_source(source, "foo.m");
        let pfbrk = filter_by_id(&diags, "PFBRK");
        assert!(!pfbrk.is_empty(), "PFBRK should fire for break inside parfor");
    }

    #[test]
    fn test_pfrtn_return_in_parfor() {
        let source = "\
function foo()
    parfor i = 1:10
        return;
    end
end
";
        let diags = check_source(source, "foo.m");
        let pfrtn = filter_by_id(&diags, "PFRTN");
        assert!(!pfrtn.is_empty(), "PFRTN should fire for return inside parfor");
    }

    #[test]
    fn test_pfglob_global_in_parfor() {
        let source = "\
function foo()
    parfor i = 1:10
        global x;
    end
end
";
        let diags = check_source(source, "foo.m");
        let pfglob = filter_by_id(&diags, "PFGLOB");
        assert!(!pfglob.is_empty(), "PFGLOB should fire for global inside parfor");
    }

    #[test]
    fn test_pfpers_persistent_in_parfor() {
        let source = "\
function foo()
    parfor i = 1:10
        persistent x;
    end
end
";
        let diags = check_source(source, "foo.m");
        let pfpers = filter_by_id(&diags, "PFPERS");
        assert!(!pfpers.is_empty(), "PFPERS should fire for persistent inside parfor");
    }

    #[test]
    fn test_pfnf_nested_function_in_parfor() {
        let source = "\
function foo()
    parfor i = 1:10
        function y = helper(x)
            y = x + 1;
        end
    end
end
";
        let diags = check_source(source, "foo.m");
        let pfnf = filter_by_id(&diags, "PFNF");
        assert!(
            !pfnf.is_empty(),
            "PFNF should fire for nested function inside parfor"
        );
    }

    // ===== SPMD restriction tests =====

    #[test]
    fn test_spnst_parfor_in_spmd() {
        let source = "\
function foo()
    spmd
        parfor i = 1:10
            x = i;
        end
    end
end
";
        let diags = check_source(source, "foo.m");
        let spnst = filter_by_id(&diags, "SPNST");
        assert!(!spnst.is_empty(), "SPNST should fire for parfor inside spmd");
    }

    #[test]
    fn test_spnst_spmd_in_spmd() {
        let source = "\
function foo()
    spmd
        spmd
            x = 1;
        end
    end
end
";
        let diags = check_source(source, "foo.m");
        let spnst = filter_by_id(&diags, "SPNST");
        assert!(!spnst.is_empty(), "SPNST should fire for spmd inside spmd");
    }

    #[test]
    fn test_spret_return_in_spmd() {
        let source = "\
function foo()
    spmd
        return;
    end
end
";
        let diags = check_source(source, "foo.m");
        let spret = filter_by_id(&diags, "SPRET");
        assert!(!spret.is_empty(), "SPRET should fire for return inside spmd");
    }

    #[test]
    fn test_spgp_global_in_spmd() {
        let source = "\
function foo()
    spmd
        global x;
    end
end
";
        let diags = check_source(source, "foo.m");
        let spgp = filter_by_id(&diags, "SPGP");
        assert!(!spgp.is_empty(), "SPGP should fire for global inside spmd");
    }

    // ===== Class/OOP rule tests =====

    #[test]
    fn test_mcfil_class_file_name_mismatch() {
        let source = "\
classdef MyClass
    methods
    end
end
";
        // File is named "WrongName.m" but class is "MyClass"
        let diags = check_source(source, "WrongName.m");
        let mcfil = filter_by_id(&diags, "MCFIL");
        assert!(!mcfil.is_empty(), "MCFIL should fire when class name != file name");
    }

    #[test]
    fn test_mcfil_no_fire_when_match() {
        let source = "\
classdef MyClass
    methods
    end
end
";
        let diags = check_source(source, "MyClass.m");
        let mcfil = filter_by_id(&diags, "MCFIL");
        assert!(mcfil.is_empty(), "MCFIL should NOT fire when names match");
    }

    #[test]
    fn test_mcred_property_same_as_class() {
        let source = "\
classdef Foo
    properties
        Foo
    end
end
";
        let diags = check_source(source, "Foo.m");
        let mcred = filter_by_id(&diags, "MCRED");
        assert!(
            !mcred.is_empty(),
            "MCRED should fire when property name matches class name"
        );
    }

    #[test]
    fn test_mcs2i_setter_wrong_inputs() {
        let source = "\
classdef MyClass
    properties
        Value
    end

    methods
        function set.Value(obj)
            obj.Value = 0;
        end
    end
end
";
        let diags = check_source(source, "MyClass.m");
        let mcs2i = filter_by_id(&diags, "MCS2I");
        assert!(
            !mcs2i.is_empty(),
            "MCS2I should fire when setter has wrong number of inputs"
        );
    }

    #[test]
    fn test_mcg1i_getter_wrong_inputs() {
        let source = "\
classdef MyClass
    properties
        Value
    end

    methods
        function val = get.Value(obj, extra)
            val = obj.Value;
        end
    end
end
";
        let diags = check_source(source, "MyClass.m");
        let mcg1i = filter_by_id(&diags, "MCG1I");
        assert!(
            !mcg1i.is_empty(),
            "MCG1I should fire when getter has wrong number of inputs"
        );
    }

    #[test]
    fn test_mceb_events_in_non_handle() {
        let source = "\
classdef MyClass
    events
        DataChanged
    end
end
";
        let diags = check_source(source, "MyClass.m");
        let mceb = filter_by_id(&diags, "MCEB");
        assert!(
            !mceb.is_empty(),
            "MCEB should fire when events defined in non-handle class"
        );
    }

    #[test]
    fn test_mceb_no_fire_handle() {
        let source = "\
classdef MyClass < handle
    events
        DataChanged
    end
end
";
        let diags = check_source(source, "MyClass.m");
        let mceb = filter_by_id(&diags, "MCEB");
        assert!(
            mceb.is_empty(),
            "MCEB should NOT fire when events defined in handle class"
        );
    }

    #[test]
    fn test_mcani_abstract_property_with_default() {
        let source = "\
classdef MyClass
    properties (Abstract)
        Value = 42
    end
end
";
        let diags = check_source(source, "MyClass.m");
        let mcani = filter_by_id(&diags, "MCANI");
        assert!(
            !mcani.is_empty(),
            "MCANI should fire when abstract property has default"
        );
    }

    #[test]
    fn test_mcasc_abstract_in_sealed() {
        let source = "\
classdef (Sealed) MyClass
    properties (Abstract)
        Value
    end
end
";
        let diags = check_source(source, "MyClass.m");
        let mcasc = filter_by_id(&diags, "MCASC");
        assert!(
            !mcasc.is_empty(),
            "MCASC should fire when abstract property in sealed class"
        );
    }

    #[test]
    fn test_mcsgp_setter_invalid_property() {
        let source = "\
classdef MyClass
    properties
        Value
    end

    methods
        function set.NonExistent(obj, val)
            obj.NonExistent = val;
        end
    end
end
";
        let diags = check_source(source, "MyClass.m");
        let mcsgp = filter_by_id(&diags, "MCSGP");
        assert!(
            !mcsgp.is_empty(),
            "MCSGP should fire when setter refers to non-existent property"
        );
    }

    // ===== Function validation tests =====

    #[test]
    fn test_fvnst_arguments_in_nested() {
        let source = "\
function outer()
    x = inner(1);
    function y = inner(a)
        arguments
            a double
        end
        y = a + 1;
    end
end
";
        let diags = check_source(source, "outer.m");
        let fvnst = filter_by_id(&diags, "FVNST");
        assert!(
            !fvnst.is_empty(),
            "FVNST should fire when arguments block is in nested function"
        );
    }

    // ===== Script-level tests =====

    #[test]
    fn test_npers_persistent_in_script() {
        let source = "persistent x;\nx = 1;\n";
        let diags = check_source(source, "myscript.m");
        let npers = filter_by_id(&diags, "NPERS");
        assert!(
            !npers.is_empty(),
            "NPERS should fire for persistent in script"
        );
    }

    #[test]
    fn test_fconv_variable_same_as_script() {
        let source = "myscript = 42;\ndisp(myscript);\n";
        let diags = check_source(source, "myscript.m");
        let fconv = filter_by_id(&diags, "FCONV");
        assert!(
            !fconv.is_empty(),
            "FCONV should fire when variable name matches script file name"
        );
    }

    // ===== Other language spec tests =====

    #[test]
    fn test_fcnans_function_named_ans() {
        let source = "\
function y = ans(x)
    y = x;
end
";
        let diags = check_source(source, "ans.m");
        let fcnans = filter_by_id(&diags, "FCNANS");
        assert!(
            !fcnans.is_empty(),
            "FCNANS should fire for function named 'ans'"
        );
    }

    #[test]
    fn test_clans_class_named_ans() {
        let source = "\
classdef ans
end
";
        let diags = check_source(source, "ans.m");
        let clans = filter_by_id(&diags, "CLANS");
        assert!(!clans.is_empty(), "CLANS should fire for class named 'ans'");
    }

    #[test]
    fn test_fconf_local_function_same_as_file() {
        let source = "\
function y = main(x)
    y = helper(x);
end

function z = helper(x)
    z = x * 2;
end
";
        // File is named "helper.m" — the local function matches
        let diags = check_source(source, "helper.m");
        let fconf = filter_by_id(&diags, "FCONF");
        assert!(
            !fconf.is_empty(),
            "FCONF should fire when local function name matches file name"
        );
    }

    #[test]
    fn test_brkfor_break_outside_loop() {
        let source = "\
function foo()
    break;
end
";
        let diags = check_source(source, "foo.m");
        let brkfor = filter_by_id(&diags, "BRKFOR");
        assert!(
            !brkfor.is_empty(),
            "BRKFOR should fire for break outside loop"
        );
    }

    #[test]
    fn test_contfor_continue_outside_loop() {
        let source = "\
function foo()
    continue;
end
";
        let diags = check_source(source, "foo.m");
        let contfor = filter_by_id(&diags, "CONTFOR");
        assert!(
            !contfor.is_empty(),
            "CONTFOR should fire for continue outside loop"
        );
    }

    #[test]
    fn test_no_fire_break_inside_loop() {
        let source = "\
function foo()
    for i = 1:10
        if i > 5
            break;
        end
    end
end
";
        let diags = check_source(source, "foo.m");
        let brkfor = filter_by_id(&diags, "BRKFOR");
        assert!(
            brkfor.is_empty(),
            "BRKFOR should NOT fire for break inside a loop"
        );
    }

    #[test]
    fn test_no_parfor_violations_in_normal_code() {
        let source = "\
function foo()
    for i = 1:10
        global x;
        persistent y;
        break;
        return;
    end
end
";
        let diags = check_source(source, "foo.m");
        let pf_diags: Vec<_> = diags
            .iter()
            .filter(|d| d.rule_id.starts_with("PF"))
            .collect();
        assert!(
            pf_diags.is_empty(),
            "No PF* violations should fire in normal for-loop code, got {:?}",
            pf_diags.iter().map(|d| d.rule_id).collect::<Vec<_>>()
        );
    }
}
