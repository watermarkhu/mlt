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
//! ### Parfor variable, slicing, and reduction checks
//!
//! | Check ID | Description                                      |
//! |----------|--------------------------------------------------|
//! | FPFORP   | fprintf writing to a file opened read-only       |
//! | FWFORP   | fwrite writing to a file opened read-only        |
//! | PFANON   | Sliced output variable used in an anonymous function |
//! | PFANSLP  | 'ans' as parfor loop variable                    |
//! | PFANSNS  | 'ans' as for loop variable inside parfor         |
//! | PFCEL    | Function does not support cell arrays            |
//! | PFCTXT   | Sliced variable indexed outside its defining for loop |
//! | PFEVC    | EVALIN/ASSIGNIN('caller') invalid inside parfor  |
//! | PFFRNG   | Nested for range must be positive constants      |
//! | PFFSUB   | Indexing a nested for loop variable              |
//! | PFINCR   | Different reduction functions on the same variable |
//! | PFINPT   | 'inputname' not supported in parfor              |
//! | PFLD     | 'load' must assign to an output in parfor        |
//! | PFMLTI   | Nested for loop variable assigned in the body    |
//! | PFNACK   | narginchk/nargoutchk cannot be used in parfor    |
//! | PFNAIO   | nargin/nargout require a function argument       |
//! | PFNAR    | Subtracting a reduction variable from expressions |
//! | PFRFH    | Reduction function must be a name or broadcast variable |
//! | PFRNG    | Parfor range must be increasing consecutive integers |
//! | PFSLO    | Variable indexed with parfor var is not a sliced output |
//! | PFSLRD   | Sliced indexing combined with non-indexed reads  |
//! | PFSLW    | Sliced accesses must all use the same subscripts |
//! | PFSV     | SAVE requires '-fromstruct' in parfor            |
//! | PFUNK    | Parfor cannot run due to the way a variable is used |
//! | PFUTMP   | Temporary variable must be set before it is used |
//! | PFUTVR   | Variable intended as reduction but uninitialized |
//! | PFVARS   | Parfor loop contains too many variables          |
//! | PFVSUB   | Indexing the parfor loop variable                |
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
//! | MABSEAC  | Instance properties/methods illegal in Sealed+Abstract classes |
//! | MABSEAM  | A method cannot be both Abstract and Sealed      |
//! | MCAPP    | Private property cannot be Abstract              |
//! | MCCBS    | Superclass constructor is not a declared superclass name |
//! | MCCBU    | Superclass constructor called after object use   |
//! | MCCMC    | Constructor for superclass can only be called once |
//! | MCCSOP   | Unable to modify Constant property               |
//! | MCGSA    | Set/get method tries to access an abstract property |
//! | MCMIO    | Method has too many inputs or outputs            |
//! | MCMSP    | Private method cannot be Abstract                |
//! | MCMTP    | TestParameterDefinition methods must be Static   |
//! | MCPIN    | Property initialized to instance of the class itself |
//! | MCPSG    | Set or get method must be fully defined in the class file |
//! | MCSCC    | Superclass constructor call needs a matching subclass constructor name |
//! | MCSCF    | Superclass constructor must be assigned to the first output |
//! | MCSCM    | Superclass method call needs a matching method name |
//! | MCSCN    | Method tries to set a constant property         |
//! | MCSCO    | Superclass constructor must use the first output argument |
//! | MCSCT    | Superclass constructor call must not be conditionalized |
//! | MCSMO    | Multiple outputs from superclass initialization unsupported |
//! | MCSWA    | Sealed class cannot specify allowed subclasses  |
//! | MTAGS3   | Access attribute conflicts with SetAccess/GetAccess |
//! | MTMAT    | Attribute can only be set once                  |
//! | MWKCL    | WeakHandle property must have a class validation |
//! | MWKCT    | WeakHandle and Constant attributes conflict      |
//! | MWKREF   | WeakHandle and Dependent attributes conflict     |
//!
//! ### Function validation (FV* checks)
//!
//! | Check ID | Description                                      |
//! |----------|--------------------------------------------------|
//! | FVNST    | Arguments blocks in nested functions             |
//! | VTPOD    | Validation out of order (size, then class, then functions) |
//! | FVAPN    | Move name=value name-value arguments to the end  |
//! | FVATF    | Attribute values in arguments blocks must be logical constants |
//! | FVBTN    | Use of this function is not supported in arguments blocks |
//! | FVDAN    | Same name as name-value structure and positional |
//! | FVDAP    | Positional argument can only be declared once    |
//! | FVDNF    | Name-value argument can only be declared once    |
//! | FVDREP   | Multiple Repeating arguments blocks not supported |
//! | FVIDV    | Validation/default on ignored arguments unsupported |
//! | FVIOA    | Both 'Input' and 'Output' attributes on one block |
//! | FVMCL    | Multiple name-value structures using .? syntax   |
//! | FVNDE    | Default values illegal for class-name name-value |
//! | FVNIV    | Variable is not an input to the function        |
//! | FVNREP   | Name-value arguments in Repeating block unsupported |
//! | FVNSC    | Calling nested functions in arguments blocks     |
//! | FVNVL    | Validation illegal for class-name name-value    |
//! | FVOBI    | Declare input blocks before output blocks       |
//! | FVOCON   | Output validation only uses arg or literals     |
//! | FVOND    | Name-value arguments in default values unsupported |
//! | FVONV    | Name-value arguments without dotted name in validation |
//! | FVOOD    | Default value for output argument unsupported   |
//! | FVOOI    | Ignored arguments in output block unsupported   |
//! | FVOON    | Name-value argument as output unsupported       |
//! | FVORDI   | Ignored inputs after Repeating or name-value    |
//! | FVORDN   | Positional arguments before name-value arguments |
//! | FVORDO   | Repeating outputs after required outputs        |
//! | FVORDP   | Positional order: required, optional, repeating |
//! | FVORM    | Multiple repeating output arguments unsupported |
//! | FVOVREP  | varargout only in Repeating output block        |
//! | FVREPD   | Defaults in Repeating block unsupported         |
//! | FVREPO   | Repeating input block with varargin has no other args |
//! | FVSOR    | Input block matches function line order         |
//! | FVSORO   | Output block matches function line order        |
//! | FVUBD    | Argument referenced before declared             |
//! | FVVCON   | Input validation only uses prior positionals    |
//! | FVVIN    | Validation function must use the argument       |
//! | FVVREP   | varargin only inside repeating input block      |
//! | TINVALDIM| Each dimension must be nonnegative integer or colon |
//! | TTOOFEWDIMS | Specify at least two dimensions for size    |
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
//! ### SPMD transparency, message, and construction checks (C4 group)
//!
//! | Check ID | Description                                      |
//! |----------|--------------------------------------------------|
//! | CTOINE   | Use of constructed object as input to constructor is not supported |
//! | CTORO    | Class constructors must be declared with at least one output argument |
//! | ERTXT    | Specify an error message with the message identifier |
//! | MHERIT   | Deriving from a built-in MATLAB class is not supported |
//! | NCHKOS   | NARGINCHK does not return any values             |
//! | SPBFN    | Non-transparent workspace access inside spmd     |
//! | SPBRK    | break/continue not fully contained in spmd (subsumed by SPRET) |
//! | SPDEC    | SPMD worker bounds must be nonnegative integers  |
//! | SPDEC3   | SPMD can only specify lower and upper worker bounds |
//! | SPEVC    | EVALIN/ASSIGNIN('caller') invalid inside spmd    |
//! | SPLD     | Assign the output of LOAD in spmd blocks         |
//! | SPNF     | Nested function call inside spmd                 |
//! | SPSV     | SAVE requires '-fromstruct' inside spmd          |
//! | SPWHOS   | who/whos without '-file' invalid inside spmd     |
//! | USESWNS  | Variable must be explicitly defined before first use |
//! | WTXT     | Specify a warning message with the message identifier |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.LANGUAGE_SPEC_ENGINE]
//! severity = "error"
//! ```

use std::collections::{HashMap, HashSet};
use std::ops::Range;

use mlt_core::{Category, Config, Diagnostic, FileContext, Rule, Severity};
use serde::Deserialize;

use crate::analysis::metadata::{
    ArgValidation, AttributeMeta, ClassMeta, FileMeta, FileType, FunctionMeta,
};
use crate::analysis::symbols::{DefKind, SymbolTable};

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for the language specification engine.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct LanguageSpecConfig {
    /// Check IDs to skip (e.g., `["MABSEAC", "MTAGS3"]`).
    #[serde(default)]
    pub disabled_checks: Vec<String>,
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
    /// Per-parfor analysis state for the parfor variable/slicing/reduction checks.
    /// Only populated for `LoopContext::Parfor` frames.
    parfor_analysis: ParforAnalysis,
}

/// Tracks a nested `for` loop inside a parfor body.
#[derive(Debug, Clone)]
struct NestedForInfo {
    /// The nested loop variable name.
    var: String,
    /// Byte range of the nested `for_statement` node.
    for_start: usize,
    /// End byte of the nested `for_statement` node.
    for_end: usize,
    /// Byte range of the nested loop's range expression.
    range_start: usize,
    /// End byte of the nested loop's range expression.
    range_end: usize,
    /// Whether the range is a row of positive constant numbers.
    range_is_const_pos: bool,
}

/// Per-parfor analysis state collected during the DFS traversal.
///
/// Aggregates variable classification (sliced outputs, temporaries, reductions),
/// indexed accesses, and file-handle tracking so that body-level parfor checks
/// can be finalized when the parfor frame is popped.
#[derive(Debug, Clone, Default)]
struct ParforAnalysis {
    /// The parfor loop variable name.
    parfor_var: String,
    /// Byte range of the parfor `for_statement` node.
    parfor_start: usize,
    /// End byte of the parfor `for_statement` node.
    parfor_end: usize,
    /// Nested `for` loops inside the parfor body.
    nested_fors: Vec<NestedForInfo>,
    /// Sliced output assignments `v(i,...) = ...`: var -> (arguments text, start, end).
    sliced_lhs: HashMap<String, Vec<(String, usize, usize)>>,
    /// Indexed reads `v(...)` used as a value: var -> (arguments text, start, end).
    indexed_reads: HashMap<String, Vec<(String, usize, usize)>>,
    /// Plain (temporary) assignments: var -> assignment start bytes.
    temp_assigns: HashMap<String, Vec<usize>>,
    /// Whole (non-indexed) reads of a variable: var -> read start bytes.
    whole_reads: HashMap<String, Vec<usize>>,
    /// Proper reductions `v = v op expr`: var -> (operator, assignment start).
    reductions: HashMap<String, Vec<(String, usize)>>,
    /// Functional reductions `v = f(v, ...)`: var -> (callee, assignment start).
    func_reductions: HashMap<String, Vec<(String, usize)>>,
    /// `expr - v` patterns in assignment RHSs: (var, assignment start).
    sub_right: Vec<(String, usize)>,
    /// Variables that appear on both sides of an assignment in the body.
    both_sides: HashSet<String>,
    /// Anonymous functions in the body: (start, referenced variable names).
    lambdas: Vec<(usize, Vec<String>)>,
    /// File handles opened read-only via `fid = fopen(...)`.
    read_only_handles: HashSet<String>,
    /// All variable names observed in the parfor body.
    var_names: HashSet<String>,
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
        let nested_functions = self.collect_nested_function_names(root, ctx.source);
        let mut context_stack = vec![ContextFrame {
            context: LoopContext::Normal,
            parfor_var: None,
            for_vars_in_parfor: Vec::new(),
            parfor_analysis: ParforAnalysis::default(),
        }];
        self.check_dfs(
            root,
            ctx.source,
            &mut context_stack,
            &nested_functions,
            &mut diagnostics,
        );

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
        nested_functions: &HashSet<String>,
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

                    // C1 checks on the parfor header (loop variable name and range)
                    self.check_parfor_header(node, source, diagnostics);

                    // Push parfor context and recurse
                    context_stack.push(ContextFrame {
                        context: LoopContext::Parfor,
                        parfor_var,
                        for_vars_in_parfor: Vec::new(),
                        parfor_analysis: ParforAnalysis {
                            parfor_var: self.extract_for_variable(node, source).unwrap_or_default(),
                            parfor_start: node.start_byte(),
                            parfor_end: node.end_byte(),
                            ..Default::default()
                        },
                    });
                    self.visit_children(node, source, context_stack, nested_functions, diagnostics);
                    if let Some(frame) = context_stack.last() {
                        self.finalize_parfor_analysis(&frame.parfor_analysis, node, source, diagnostics);
                    }
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
                        // C1 checks for nested for loops inside parfor
                        self.check_nested_for_in_parfor(node, source, context_stack, diagnostics);
                        // Stay in Parfor context
                        self.visit_children(node, source, context_stack, nested_functions, diagnostics);
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
                            parfor_analysis: ParforAnalysis::default(),
                        });
                        self.visit_children(node, source, context_stack, nested_functions, diagnostics);
                        context_stack.pop();
                    }
                }
                return;
            }

            "while_statement" => {
                if current_context == LoopContext::Parfor {
                    // Stay in Parfor context for while inside parfor
                    self.visit_children(node, source, context_stack, nested_functions, diagnostics);
                } else {
                    // Push normal loop context
                    context_stack.push(ContextFrame {
                        context: LoopContext::Loop,
                        parfor_var: None,
                        for_vars_in_parfor: Vec::new(),
                        parfor_analysis: ParforAnalysis::default(),
                    });
                    self.visit_children(node, source, context_stack, nested_functions, diagnostics);
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

                // Check: SPDEC / SPDEC3 — spmd worker bounds
                self.check_spmd_worker_bounds(node, source, diagnostics);

                // Push spmd context and recurse
                context_stack.push(ContextFrame {
                    context: LoopContext::Spmd,
                    parfor_var: None,
                    for_vars_in_parfor: Vec::new(),
                    parfor_analysis: ParforAnalysis::default(),
                });
                self.visit_children(node, source, context_stack, nested_functions, diagnostics);
                context_stack.pop();
                return;
            }

            "break_statement" => {
                // SPBRK (break/continue whose loop is not fully contained in the spmd
                // block) is subsumed by SPRET, which fires for every break/continue
                // inside an spmd block.
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

                // C1 checks: variable classification, reductions, nested loop variables
                self.check_parfor_assignment(node, source, context_stack, diagnostics);
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
                    parfor_analysis: ParforAnalysis::default(),
                });
                self.visit_children(node, source, context_stack, nested_functions, diagnostics);
                context_stack.pop();
                return;
            }

            "function_call" => {
                // SPMD transparency checks (only inside spmd blocks)
                if current_context == LoopContext::Spmd {
                    self.check_spmd_function_call(node, source, nested_functions, diagnostics);
                }
                // C1 checks (only inside parfor blocks)
                if current_context == LoopContext::Parfor {
                    self.check_parfor_function_call(node, source, context_stack, diagnostics);
                }
                // Message-specification checks (any context)
                self.check_error_warning_message(node, source, diagnostics);
                // NCHKOS: narginchk/nargoutchk used as a value
                self.check_nchkos_output_use(node, source, diagnostics);
                // Recurse into arguments (nested calls may contain checks)
                self.visit_children(node, source, context_stack, nested_functions, diagnostics);
                return;
            }

            "command" => {
                // Command-form calls inside spmd blocks
                if current_context == LoopContext::Spmd {
                    self.check_spmd_command(node, source, diagnostics);
                }
                // C1 checks (only inside parfor blocks)
                if current_context == LoopContext::Parfor {
                    self.check_parfor_command(node, source, diagnostics);
                }
                self.visit_children(node, source, context_stack, nested_functions, diagnostics);
                return;
            }

            "identifier" if current_context == LoopContext::Parfor => {
                // C1 checks: record whole (non-indexed) variable reads
                self.check_parfor_identifier(node, source, context_stack);
            }

            "lambda" if current_context == LoopContext::Parfor => {
                // C1 checks: record anonymous-function references (PFANON)
                self.check_parfor_lambda(node, source, context_stack);
            }

            _ => {}
        }

        // Recurse into children
        self.visit_children(node, source, context_stack, nested_functions, diagnostics);
    }

    /// Visit all children of a node during DFS traversal.
    fn visit_children(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context_stack: &mut Vec<ContextFrame>,
        nested_functions: &HashSet<String>,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.check_dfs(child, source, context_stack, nested_functions, diagnostics);
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
// Parfor variable / slicing / reduction checks (C1 group)
// ---------------------------------------------------------------------------

/// Functions that do not accept cell array inputs (PFCEL).
const PFCEL_NO_CELL_FUNCTIONS: &[&str] = &[
    "sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh",
    "exp", "log", "log10", "log2", "sqrt", "abs", "sign", "floor", "ceil",
    "round", "fix", "mod", "rem",
];

/// Reduction functions supported by parfor functional reductions (PFRFH).
const PFRFH_REDUCTION_FUNCTIONS: &[&str] = &[
    "plus", "minus", "times", "mtimes", "max", "min", "or", "and", "xor",
    "horzcat", "vertcat", "hypot", "union", "intersect", "setxor",
    "setdiff", "unique", "strcat",
];

/// Maximum number of variables a parfor loop may contain (PFVARS).
const PARFOR_VAR_LIMIT: usize = 1024;

impl LanguageSpecEngine {
    /// Check the parfor header: loop variable name (PFANSLP) and range (PFRNG).
    fn check_parfor_header(
        &self,
        node: tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // PFANSLP: 'ans' as the parfor loop variable
        if let Some(var) = self.extract_for_variable(node, source) {
            if var == "ans" {
                self.push_diag(
                    node,
                    "PFANSLP",
                    "'ans' is not supported as a parfor loop variable",
                    diagnostics,
                );
            }
        }

        // PFRNG: the parfor range must be increasing consecutive integers
        if let Some(range_node) = self.parfor_range_node(node) {
            let mut named = 0usize;
            let mut step_is_one = true;
            let mut cursor = range_node.walk();
            for child in range_node.children(&mut cursor) {
                if child.is_named() {
                    named += 1;
                    if named == 2 {
                        step_is_one = node_text(child, source).trim() == "1";
                    }
                }
            }
            if named >= 3 && !step_is_one {
                self.push_diag(
                    range_node,
                    "PFRNG",
                    "The range of a PARFOR statement must be increasing consecutive integers",
                    diagnostics,
                );
            }
        }
    }

    /// Handle a nested `for` loop inside a parfor body (PFANSNS) and record it
    /// for the PFCTXT/PFFRNG body-level checks.
    fn check_nested_for_in_parfor(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context_stack: &mut [ContextFrame],
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let Some(var) = self.extract_for_variable(node, source) else {
            return;
        };

        // PFANSNS: 'ans' as a for loop variable inside a parfor
        if var == "ans" {
            self.push_diag(
                node,
                "PFANSNS",
                "'ans' is not supported as a for loop variable in parfor loops",
                diagnostics,
            );
        }

        let (range_start, range_end, is_const_pos) = self.nested_for_range_info(node, source);
        if let Some(analysis) = current_parfor_analysis_mut(context_stack) {
            analysis.nested_fors.push(NestedForInfo {
                var: var.clone(),
                for_start: node.start_byte(),
                for_end: node.end_byte(),
                range_start,
                range_end,
                range_is_const_pos: is_const_pos,
            });
            analysis.var_names.insert(var);
        }
    }

    /// Record an assignment inside a parfor body: variable classification
    /// (temporary / sliced / reduction), PFMLTI, PFNAR candidates, and
    /// read-only file-handle tracking for FPFORP/FWFORP.
    fn check_parfor_assignment(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context_stack: &mut [ContextFrame],
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let Some(lhs_node) = node.child_by_field_name("left") else {
            return;
        };
        if lhs_node.kind() != "identifier" {
            // Indexed LHS (`x(i) = ...`) is recorded by the function_call arm.
            return;
        }
        let lhs_name = node_text(lhs_node, source).to_string();
        let rhs = node.child_by_field_name("right");

        let Some(analysis) = current_parfor_analysis_mut(context_stack) else {
            return;
        };
        let nested_for_vars: Vec<String> =
            analysis.nested_fors.iter().map(|nf| nf.var.clone()).collect();

        // PFMLTI: the nested for loop variable must not be assigned in the body
        if nested_for_vars.contains(&lhs_name) {
            self.push_diag(
                node,
                "PFMLTI",
                &format!(
                    "The nested for loop variable '{lhs_name}' must not be assigned \
                     other than by its for statement"
                ),
                diagnostics,
            );
        }

        analysis.var_names.insert(lhs_name.clone());

        // Classify the assignment: proper reduction, functional reduction, or temp.
        let mut reduction_op: Option<String> = None;
        let mut func_reduction_callee: Option<String> = None;
        if let Some(rhs_node) = rhs {
            match rhs_node.kind() {
                "binary_operator" => {
                    let left = rhs_node.child(0);
                    if let Some(l) = left {
                        if l.kind() == "identifier" && node_text(l, source) == lhs_name {
                            reduction_op = Some(operator_text(rhs_node, source));
                        }
                    }
                }
                "function_call" => {
                    let first = argument_nodes(rhs_node).first().copied();
                    if let Some(a) = first {
                        if a.kind() == "identifier" && node_text(a, source) == lhs_name {
                            if let Some(callee) = callee_name(rhs_node, source) {
                                func_reduction_callee = Some(callee);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        match (reduction_op, func_reduction_callee) {
            (Some(op), _) => {
                analysis
                    .reductions
                    .entry(lhs_name.clone())
                    .or_default()
                    .push((op, node.start_byte()));
            }
            (None, Some(callee)) => {
                analysis
                    .func_reductions
                    .entry(lhs_name.clone())
                    .or_default()
                    .push((callee, node.start_byte()));
            }
            (None, None) => {
                analysis
                    .temp_assigns
                    .entry(lhs_name.clone())
                    .or_default()
                    .push(node.start_byte());
            }
        }

        if let Some(rhs_node) = rhs {
            // Track variables that appear on both sides of an assignment.
            if contains_identifier(rhs_node, source, &lhs_name) {
                analysis.both_sides.insert(lhs_name.clone());
            }

            // PFNAR candidate: `expr - reduction_var`
            if rhs_node.kind() == "binary_operator" {
                let op = operator_text(rhs_node, source);
                if op == "-" {
                    let right = rhs_node.child(2);
                    if let Some(r) = right {
                        if r.kind() == "identifier" && node_text(r, source) == lhs_name {
                            analysis.sub_right.push((lhs_name.clone(), node.start_byte()));
                        }
                    }
                }
            }

            // FPFORP/FWFORP: track file handles opened read-only.
            if rhs_node.kind() == "function_call"
                && callee_name(rhs_node, source).as_deref() == Some("fopen")
                && fopen_is_read_only(rhs_node, source)
            {
                analysis.read_only_handles.insert(lhs_name.clone());
            }
        }
    }

    /// Handle a function call inside a parfor body: banned functions (PFEVC,
    /// PFINPT, PFNACK, PFNAIO, PFLD, PFSV, FPFORP, FWFORP, PFCEL), loop-variable
    /// indexing (PFVSUB, PFFSUB), and indexed-access recording for the
    /// sliced-variable checks.
    fn check_parfor_function_call(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context_stack: &mut [ContextFrame],
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let Some(name) = callee_name(node, source) else {
            return;
        };

        // Snapshot the parfor state needed for the immediate checks.
        let snapshot = context_stack
            .iter()
            .rev()
            .find(|f| f.context == LoopContext::Parfor)
            .map(|f| {
                (
                    f.parfor_analysis.parfor_var.clone(),
                    f.parfor_analysis
                        .nested_fors
                        .iter()
                        .map(|nf| nf.var.clone())
                        .collect::<Vec<String>>(),
                    f.parfor_analysis.read_only_handles.clone(),
                )
            });
        let Some((parfor_var, nested_for_vars, read_only_handles)) = snapshot else {
            return;
        };

        let is_assign_lhs = node
            .parent()
            .map(|p| {
                p.kind() == "assignment"
                    && p.child_by_field_name("left")
                        .map(|l| l.id() == node.id())
                        .unwrap_or(false)
            })
            .unwrap_or(false);

        match name.as_str() {
            "evalin" | "assignin" => {
                if first_string_argument(node, source)
                    .map(|s| s.eq_ignore_ascii_case("caller"))
                    .unwrap_or(false)
                {
                    self.push_diag(
                        node,
                        "PFEVC",
                        "EVALIN('caller') and ASSIGNIN('caller') are invalid inside a PARFOR loop",
                        diagnostics,
                    );
                }
            }
            "inputname" => {
                self.push_diag(
                    node,
                    "PFINPT",
                    "'inputname' is not supported in parfor loops",
                    diagnostics,
                );
            }
            "narginchk" | "nargoutchk" => {
                self.push_diag(
                    node,
                    "PFNACK",
                    &format!("'{name}' cannot be used in parfor loops"),
                    diagnostics,
                );
            }
            "nargin" | "nargout" => {
                if argument_nodes(node).is_empty() {
                    self.push_diag(
                        node,
                        "PFNAIO",
                        &format!("'{name}' requires a function argument in parfor loops"),
                        diagnostics,
                    );
                }
            }
            "load" => {
                if !is_assignment_rhs(node) {
                    self.push_diag(
                        node,
                        "PFLD",
                        "'load' must assign to an output variable in parfor loops",
                        diagnostics,
                    );
                }
            }
            "save" => {
                if !has_string_argument(node, source, "-fromstruct") {
                    self.push_diag(
                        node,
                        "PFSV",
                        "SAVE cannot be called in a PARFOR loop without the '-fromstruct' option",
                        diagnostics,
                    );
                }
            }
            "fprintf" => {
                if self.fwrite_handle_is_read_only(node, source, &read_only_handles) {
                    self.push_diag(
                        node,
                        "FPFORP",
                        "'fprintf' is writing to a file opened with read permission only. \
                         Open using 'fopen(...,''W'',...)'.",
                        diagnostics,
                    );
                }
            }
            "fwrite" => {
                if self.fwrite_handle_is_read_only(node, source, &read_only_handles) {
                    self.push_diag(
                        node,
                        "FWFORP",
                        "'fwrite' is writing to a file opened with read permission only.",
                        diagnostics,
                    );
                }
            }
            _ => {
                // PFCEL: the function does not support cell arrays
                if PFCEL_NO_CELL_FUNCTIONS.contains(&name.as_str()) {
                    for (i, arg) in argument_nodes(node).iter().enumerate() {
                        if is_cell_array_arg(*arg, source) {
                            self.push_diag(
                                node,
                                "PFCEL",
                                &format!(
                                    "The function '{name}' does not support cell arrays \
                                     (argument {}).",
                                    i + 1
                                ),
                                diagnostics,
                            );
                            break;
                        }
                    }
                }
            }
        }

        // Loop-variable indexing and indexed-access recording.
        if name == parfor_var {
            self.push_diag(
                node,
                "PFVSUB",
                &format!(
                    "Indexing the parfor loop variable '{name}' is not supported \
                     in parfor loops"
                ),
                diagnostics,
            );
        } else if nested_for_vars.contains(&name) {
            self.push_diag(
                node,
                "PFFSUB",
                &format!(
                    "Indexing the nested for loop variable '{name}' is not supported \
                     in parfor loops"
                ),
                diagnostics,
            );
        } else if !node_is_inside_lambda(node) {
            if let Some(analysis) = current_parfor_analysis_mut(context_stack) {
                let args = arguments_text(node, source);
                analysis.var_names.insert(name.clone());
                if is_assign_lhs {
                    analysis
                        .sliced_lhs
                        .entry(name)
                        .or_default()
                        .push((args, node.start_byte(), node.end_byte()));
                } else {
                    analysis
                        .indexed_reads
                        .entry(name)
                        .or_default()
                        .push((args, node.start_byte(), node.end_byte()));
                }
            }
        }
    }

    /// Handle a command-syntax statement inside a parfor body (PFLD, PFSV, PFNAIO).
    fn check_parfor_command(
        &self,
        node: tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() != "command_name" {
                continue;
            }
            let name = node_text(child, source);
            match name {
                "load" => {
                    self.push_diag(
                        node,
                        "PFLD",
                        "'load' must assign to an output variable in parfor loops",
                        diagnostics,
                    );
                }
                "save" => {
                    self.push_diag(
                        node,
                        "PFSV",
                        "SAVE cannot be called in a PARFOR loop without the '-fromstruct' \
                         option",
                        diagnostics,
                    );
                }
                "nargin" | "nargout" => {
                    self.push_diag(
                        node,
                        "PFNAIO",
                        &format!("'{name}' requires a function argument in parfor loops"),
                        diagnostics,
                    );
                }
                _ => {}
            }
        }
    }

    /// Record a whole (non-indexed) read of a variable inside a parfor body.
    fn check_parfor_identifier(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context_stack: &mut [ContextFrame],
    ) {
        if let Some(parent) = node.parent() {
            match parent.kind() {
                // Callee of a function call — recorded by the function_call arm.
                "function_call" => return,
                "field_expression" => {
                    if parent
                        .child_by_field_name("field")
                        .map(|f| f.id() == node.id())
                        .unwrap_or(false)
                    {
                        return;
                    }
                }
                "assignment" => {
                    if parent
                        .child_by_field_name("left")
                        .map(|l| l.id() == node.id())
                        .unwrap_or(false)
                    {
                        return;
                    }
                }
                "iterator" => return,
                "global_operator" | "persistent_operator" => return,
                "arguments" | "lambda_arguments" => {
                    if let Some(gp) = parent.parent() {
                        if gp.kind() == "lambda" {
                            return;
                        }
                    }
                }
                _ => {}
            }
        }
        if node_is_inside_lambda(node) {
            return;
        }
        let name = node_text(node, source).to_string();
        if let Some(analysis) = current_parfor_analysis_mut(context_stack) {
            analysis
                .whole_reads
                .entry(name.clone())
                .or_default()
                .push(node.start_byte());
            analysis.var_names.insert(name);
        }
    }

    /// Record an anonymous function in a parfor body for PFANON.
    fn check_parfor_lambda(
        &self,
        node: tree_sitter::Node,
        source: &str,
        context_stack: &mut [ContextFrame],
    ) {
        let mut params = HashSet::new();
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "arguments" || child.kind() == "lambda_arguments" {
                collect_identifiers(child, source, &mut params);
            }
        }
        let mut refs = HashSet::new();
        let mut c2 = node.walk();
        for child in node.children(&mut c2) {
            if child.kind() == "arguments" || child.kind() == "lambda_arguments" {
                continue;
            }
            collect_lambda_refs(child, source, &params, &mut refs);
        }
        if let Some(analysis) = current_parfor_analysis_mut(context_stack) {
            analysis
                .lambdas
                .push((node.start_byte(), refs.into_iter().collect()));
        }
    }

    /// Run the body-level parfor checks once the whole parfor body has been
    /// visited: PFSLO, PFSLRD, PFSLW, PFINCR, PFNAR, PFRFH, PFUTMP, PFUTVR,
    /// PFUNK, PFVARS, PFANON, PFCTXT, PFFRNG.
    fn finalize_parfor_analysis(
        &self,
        analysis: &ParforAnalysis,
        parfor_node: tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let predefined = self.predefined_vars_before(parfor_node, source);
        let parfor_var = analysis.parfor_var.clone();
        let mut reported_utvr = HashSet::new();

        // PFINCR: different reduction functions with the same reduction variable
        for (var, ops) in &analysis.reductions {
            let mut fns: Vec<String> = ops.iter().map(|(op, _)| op.clone()).collect();
            if let Some(frs) = analysis.func_reductions.get(var) {
                fns.extend(frs.iter().map(|(c, _)| c.clone()));
            }
            fns.sort();
            fns.dedup();
            if fns.len() > 1 {
                let start = ops.iter().map(|(_, s)| *s).min().unwrap_or(0);
                self.push_diag_bytes(
                    source,
                    start,
                    start + 1,
                    "PFINCR",
                    &format!(
                        "Using different reduction functions with the same reduction \
                         variable '{var}' is not supported in parfor loops"
                    ),
                    diagnostics,
                );
            }
        }

        // PFNAR: subtracting a reduction variable from expressions
        for (var, start) in &analysis.sub_right {
            if analysis.both_sides.contains(var) {
                self.push_diag_bytes(
                    source,
                    *start,
                    *start + 1,
                    "PFNAR",
                    &format!(
                        "Subtracting reduction variable '{var}' from expressions is not \
                         supported in parfor loops"
                    ),
                    diagnostics,
                );
            }
        }

        // PFRFH: the functional reduction must be a function name or broadcast variable
        for frs in analysis.func_reductions.values() {
            for (callee, start) in frs {
                let known = PFRFH_REDUCTION_FUNCTIONS.contains(&callee.as_str());
                let is_plain_ident = callee.chars().all(|c| c.is_alphanumeric() || c == '_');
                let is_broadcast = !analysis.temp_assigns.contains_key(callee);
                if !known && (!is_plain_ident || !is_broadcast) {
                    self.push_diag_bytes(
                        source,
                        *start,
                        *start + 1,
                        "PFRFH",
                        "The PARFOR reduction function must be a function name or a \
                         broadcast variable",
                        diagnostics,
                    );
                }
            }
        }

        // PFUTMP: temporary variable used before it is set in the body
        for (var, set_starts) in &analysis.temp_assigns {
            if predefined.contains(var) {
                continue;
            }
            let first_set = set_starts.iter().min().copied();
            let mut read_starts: Vec<usize> = Vec::new();
            if let Some(rs) = analysis.whole_reads.get(var) {
                read_starts.extend(rs.iter().copied());
            }
            if let Some(rs) = analysis.indexed_reads.get(var) {
                read_starts.extend(rs.iter().map(|(_, s, _)| *s));
            }
            if let (Some(fs), Some(fr)) = (first_set, read_starts.into_iter().min()) {
                if fr < fs {
                    self.push_diag_bytes(
                        source,
                        fr,
                        fr + 1,
                        "PFUTMP",
                        &format!(
                            "Temporary variable '{var}' must be set inside the parfor loop \
                             before it is used"
                        ),
                        diagnostics,
                    );
                }
            }
        }

        // PFUTVR: variable may have been intended as a reduction but is uninitialized
        for (var, ops) in analysis
            .reductions
            .iter()
            .chain(analysis.func_reductions.iter())
        {
            if predefined.contains(var) {
                continue;
            }
            let first_red = ops.iter().map(|(_, s)| *s).min().unwrap_or(0);
            let prior_set = analysis
                .temp_assigns
                .get(var)
                .map(|v| v.iter().any(|s| *s < first_red))
                .unwrap_or(false)
                || analysis
                    .sliced_lhs
                    .get(var)
                    .map(|v| v.iter().any(|(_, s, _)| *s < first_red))
                    .unwrap_or(false);
            if !prior_set && reported_utvr.insert(var.clone()) {
                self.push_diag_bytes(
                    source,
                    first_red,
                    first_red + 1,
                    "PFUTVR",
                    &format!(
                        "Variable '{var}' may have been intended as a reduction variable, \
                         but is an uninitialized temporary"
                    ),
                    diagnostics,
                );
            }
        }

        // PFSLO: indexed with the parfor loop variable but not a sliced output
        for (var, reads) in &analysis.indexed_reads {
            if analysis.temp_assigns.contains_key(var) && !analysis.sliced_lhs.contains_key(var)
            {
                if let Some(first) = reads
                    .iter()
                    .filter(|(args, _, _)| args_tokens(args).contains(&parfor_var))
                    .map(|(_, s, _)| *s)
                    .min()
                {
                    self.push_diag_bytes(
                        source,
                        first,
                        first + 1,
                        "PFSLO",
                        &format!(
                            "Variable '{var}' is indexed using the parfor loop variable, \
                             but it is not a valid sliced output variable"
                        ),
                        diagnostics,
                    );
                }
            }
        }

        // PFSLRD: sliced output variable read without slicing
        for var in analysis.sliced_lhs.keys() {
            if let Some(wrs) = analysis.whole_reads.get(var) {
                if let Some(first) = wrs.iter().min().copied() {
                    self.push_diag_bytes(
                        source,
                        first,
                        first + 1,
                        "PFSLRD",
                        &format!(
                            "Invalid combination of sliced indexing and non-indexed reads \
                             of the sliced output variable '{var}'"
                        ),
                    diagnostics,
                    );
                }
            }
        }

        // PFSLW: multiple sliced accesses with different subscripts
        for (var, writes) in &analysis.sliced_lhs {
            let mut sigs: Vec<String> = writes.iter().map(|(args, _, _)| args.clone()).collect();
            if let Some(reads) = analysis.indexed_reads.get(var) {
                sigs.extend(reads.iter().map(|(args, _, _)| args.clone()));
            }
            sigs.sort();
            sigs.dedup();
            if sigs.len() > 1 {
                let first = writes.iter().map(|(_, s, _)| *s).min().unwrap_or(0);
                self.push_diag_bytes(
                    source,
                    first,
                    first + 1,
                    "PFSLW",
                    &format!(
                        "Multiple sliced accesses to variable '{var}' must all use the same \
                         list of subscripts"
                    ),
                    diagnostics,
                );
            }
        }

        // PFUNK: variable used both as a sliced output and as a temporary
        for var in analysis.sliced_lhs.keys() {
            if let Some(temps) = analysis.temp_assigns.get(var) {
                let start = temps.iter().min().copied().unwrap_or(0);
                self.push_diag_bytes(
                    source,
                    start,
                    start + 1,
                    "PFUNK",
                    &format!(
                        "The PARFOR loop cannot run due to the way variable '{var}' is used"
                    ),
                    diagnostics,
                );
            }
        }

        // PFANON: sliced output variable used inside an anonymous function
        for (start, refs) in &analysis.lambdas {
            if refs.iter().any(|r| analysis.sliced_lhs.contains_key(r)) {
                self.push_diag_bytes(
                    source,
                    *start,
                    *start + 1,
                    "PFANON",
                    "Using a sliced output variable in an anonymous function is not \
                     supported in parfor loops",
                    diagnostics,
                );
            }
        }

        // PFCTXT: a sliced variable indexed with a nested for loop variable must be
        // inside the for loop that defines its range.
        let mut accesses: Vec<(String, String, usize, usize)> = Vec::new();
        for (var, writes) in &analysis.sliced_lhs {
            for (args, start, end) in writes {
                accesses.push((var.clone(), args.clone(), *start, *end));
            }
        }
        for (var, reads) in &analysis.indexed_reads {
            if analysis.sliced_lhs.contains_key(var) {
                for (args, start, end) in reads {
                    accesses.push((var.clone(), args.clone(), *start, *end));
                }
            }
        }
        for (var, args, start, end) in accesses {
            let tokens = args_tokens(&args);
            for nf in &analysis.nested_fors {
                if tokens.contains(&nf.var)
                    && !(start >= nf.for_start && start <= nf.for_end)
                {
                    self.push_diag_bytes(
                        source,
                        start,
                        end,
                        "PFCTXT",
                        &format!(
                            "When indexing the sliced variable '{var}' with the nested for \
                             loop variable '{}', the sliced variable must be inside the for \
                             loop that defines its range",
                            nf.var
                        ),
                        diagnostics,
                    );
                }
            }
        }

        // PFFRNG: nested for loop range must be a row of positive constants when it
        // indexes a sliced variable.
        for nf in &analysis.nested_fors {
            let indexes_sliced = analysis
                .sliced_lhs
                .iter()
                .any(|(_, writes)| {
                    writes
                        .iter()
                        .any(|(args, _, _)| args_tokens(args).contains(&nf.var))
                })
                || analysis
                    .indexed_reads
                    .iter()
                    .any(|(v, reads)| {
                        analysis.sliced_lhs.contains_key(v)
                            && reads
                                .iter()
                                .any(|(args, _, _)| args_tokens(args).contains(&nf.var))
                    });
            if indexes_sliced && !nf.range_is_const_pos {
                self.push_diag_bytes(
                    source,
                    nf.range_start,
                    nf.range_end,
                    "PFFRNG",
                    "When indexing a sliced variable with a nested for loop variable, the \
                     range must be a row vector of positive constant numbers",
                    diagnostics,
                );
            }
        }

        // PFVARS: too many variables in the parfor loop
        let count = analysis.var_names.len() + usize::from(!analysis.parfor_var.is_empty());
        if count > PARFOR_VAR_LIMIT {
            self.push_diag_bytes(
                source,
                analysis.parfor_start,
                analysis.parfor_end,
                "PFVARS",
                "Parfor loop contains too many variables",
                diagnostics,
            );
        }
    }

    /// Determine whether `fprintf`/`fwrite` writes to a file handle opened
    /// read-only (via a prior `fid = fopen(...)` or an inline `fopen(...)`).
    fn fwrite_handle_is_read_only(
        &self,
        node: tree_sitter::Node,
        source: &str,
        read_only_handles: &HashSet<String>,
    ) -> bool {
        let Some(first_arg) = argument_nodes(node).first().copied() else {
            return false;
        };
        match first_arg.kind() {
            "identifier" => read_only_handles.contains(node_text(first_arg, source)),
            "function_call" => {
                callee_name(first_arg, source).as_deref() == Some("fopen")
                    && fopen_is_read_only(first_arg, source)
            }
            _ => false,
        }
    }

    /// Collect the variables that are predefined before the parfor loop starts
    /// (function inputs/outputs, global/persistent declarations, and assignments
    /// in the enclosing scope before the parfor).
    fn predefined_vars_before(
        &self,
        node: tree_sitter::Node,
        source: &str,
    ) -> HashSet<String> {
        let mut predefined = HashSet::new();
        let stop = node.start_byte();
        let mut func: Option<tree_sitter::Node> = None;
        let mut top: tree_sitter::Node = node;
        let mut cur = node.parent();
        while let Some(p) = cur {
            if p.kind() == "function_definition" {
                func = Some(p);
                break;
            }
            top = p;
            cur = p.parent();
        }
        if let Some(f) = func {
            let mut cursor = f.walk();
            for child in f.children(&mut cursor) {
                match child.kind() {
                    "function_arguments" | "function_output" => {
                        collect_identifiers(child, source, &mut predefined);
                    }
                    "block" => {
                        collect_assigns_before(child, stop, source, &mut predefined);
                    }
                    _ => {}
                }
            }
        } else {
            collect_assigns_before(top, stop, source, &mut predefined);
        }
        predefined
    }

    /// Find the `range` node of a for/parfor statement's iterator, if any.
    fn parfor_range_node<'a>(
        &self,
        node: tree_sitter::Node<'a>,
    ) -> Option<tree_sitter::Node<'a>> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "iterator" {
                let mut inner = child.walk();
                for c in child.children(&mut inner) {
                    if c.kind() == "range" {
                        return Some(c);
                    }
                }
            }
        }
        None
    }

    /// Return the byte range of a nested for loop's range expression and whether
    /// it consists of positive constant numbers.
    fn nested_for_range_info(
        &self,
        node: tree_sitter::Node,
        source: &str,
    ) -> (usize, usize, bool) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "iterator" {
                let mut inner = child.walk();
                for c in child.children(&mut inner) {
                    if c.kind() == "range" {
                        return (
                            c.start_byte(),
                            c.end_byte(),
                            range_is_positive_constants(c, source),
                        );
                    }
                }
            }
        }
        (node.start_byte(), node.start_byte(), false)
    }

    /// Push a diagnostic at a byte range (used by the body-level finalize checks).
    fn push_diag_bytes(
        &self,
        source: &str,
        start: usize,
        end: usize,
        rule_id: &'static str,
        message: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled(rule_id) {
            return;
        }
        let start = start.min(source.len());
        let end = end.min(source.len());
        let line = source[..start].bytes().filter(|&b| b == b'\n').count() + 1;
        let column = source[..start]
            .bytes()
            .rev()
            .take_while(|&b| b != b'\n')
            .count()
            + 1;
        diagnostics.push(Diagnostic {
            rule_id,
            message: message.to_string(),
            severity: Severity::Error,
            byte_range: start..end,
            line,
            column,
            fix: None,
        });
    }
}

/// Get the innermost active parfor frame's analysis state, if any.
fn current_parfor_analysis_mut(
    context_stack: &mut [ContextFrame],
) -> Option<&mut ParforAnalysis> {
    context_stack
        .iter_mut()
        .rev()
        .find(|f| f.context == LoopContext::Parfor)
        .map(|f| &mut f.parfor_analysis)
}

/// Whether `node` is nested inside a `lambda` node.
fn node_is_inside_lambda(node: tree_sitter::Node) -> bool {
    let mut cur = node.parent();
    while let Some(p) = cur {
        if p.kind() == "lambda" {
            return true;
        }
        cur = p.parent();
    }
    false
}

/// Extract the operator text of a binary/boolean/comparison operator node.
fn operator_text(node: tree_sitter::Node, source: &str) -> String {
    if let Some(op) = node.child_by_field_name("operator") {
        return node_text(op, source).trim().to_string();
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if !child.is_named() {
            let t = node_text(child, source).trim();
            if !t.is_empty() && t != "(" && t != ")" && t != "," {
                return t.to_string();
            }
        }
    }
    String::new()
}

/// Extract the text of the `arguments` child of a function call.
fn arguments_text(node: tree_sitter::Node, source: &str) -> String {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "arguments" {
            return node_text(child, source).to_string();
        }
    }
    String::new()
}

/// Tokenize an arguments text into identifier-like tokens.
fn args_tokens(args: &str) -> Vec<String> {
    args.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Whether `name` occurs as an identifier anywhere inside `node`.
fn contains_identifier(node: tree_sitter::Node, source: &str, name: &str) -> bool {
    if node.kind() == "identifier" {
        return node_text(node, source) == name;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if contains_identifier(child, source, name) {
            return true;
        }
    }
    false
}

/// Whether an `fopen` call opens the file read-only.
fn fopen_is_read_only(call: tree_sitter::Node, source: &str) -> bool {
    let args = argument_nodes(call);
    match args.get(1) {
        Some(mode_arg) => {
            let mode = node_text(*mode_arg, source).trim();
            let mode = mode.trim_matches('\'').trim_matches('"');
            fopen_mode_is_read_only(mode)
        }
        // fopen(filename) defaults to read permission.
        None => true,
    }
}

/// Whether an `fopen` mode string grants read permission only.
fn fopen_mode_is_read_only(mode: &str) -> bool {
    let m = mode.to_ascii_lowercase();
    !m.contains('w') && !m.contains('a') && !m.contains('+') && m.contains('r')
}

/// Whether a function argument is a cell array (literal `{...}` or `cell(...)`).
fn is_cell_array_arg(arg: tree_sitter::Node, source: &str) -> bool {
    match arg.kind() {
        "cell" => true,
        "function_call" => callee_name(arg, source).as_deref() == Some("cell"),
        _ => false,
    }
}

/// Whether a `range` node consists entirely of positive constant numbers.
fn range_is_positive_constants(range_node: tree_sitter::Node, source: &str) -> bool {
    let mut found = 0;
    let mut all_pos = true;
    let mut cursor = range_node.walk();
    for child in range_node.children(&mut cursor) {
        if child.is_named() {
            found += 1;
            if child.kind() == "number" {
                if let Ok(n) = node_text(child, source).parse::<i64>() {
                    if n <= 0 {
                        all_pos = false;
                    }
                } else {
                    all_pos = false;
                }
            } else {
                all_pos = false;
            }
        }
    }
    found > 0 && all_pos
}

/// Collect all identifier texts inside `node`.
fn collect_identifiers(node: tree_sitter::Node, source: &str, out: &mut HashSet<String>) {
    if node.kind() == "identifier" {
        out.insert(node_text(node, source).to_string());
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_identifiers(child, source, out);
    }
}

/// Collect identifiers referenced inside a lambda body, excluding its parameters.
fn collect_lambda_refs(
    node: tree_sitter::Node,
    source: &str,
    params: &HashSet<String>,
    out: &mut HashSet<String>,
) {
    if node.kind() == "identifier" {
        let name = node_text(node, source).to_string();
        if !params.contains(&name) {
            out.insert(name);
        }
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_lambda_refs(child, source, params, out);
    }
}

/// Collect assignment LHS names and global/persistent names in nodes that start
/// before `stop` bytes (used to find variables predefined before a parfor loop).
fn collect_assigns_before(
    node: tree_sitter::Node,
    stop: usize,
    source: &str,
    out: &mut HashSet<String>,
) {
    if node.start_byte() >= stop {
        return;
    }
    match node.kind() {
        "assignment" => {
            if let Some(lhs) = node.child_by_field_name("left") {
                if lhs.kind() == "identifier" {
                    out.insert(node_text(lhs, source).to_string());
                }
            }
        }
        "global_operator" | "persistent_operator" => {
            collect_identifiers(node, source, out);
        }
        "function_definition" => return,
        _ => {}
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_assigns_before(child, stop, source, out);
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
        self.check_ctoro(class, diagnostics);
        self.check_mherit(class, diagnostics);
        self.check_ctoine(class, ctx, diagnostics);
        self.check_mabseac(class, diagnostics);
        self.check_mcswa(class, diagnostics);
        self.check_mtmat(class, diagnostics);
        self.check_mtags3(class, diagnostics);
        self.check_mabseam(class, diagnostics);
        self.check_mcmsp(class, diagnostics);
        self.check_mcapp(class, diagnostics);
        self.check_mcgsa(class, diagnostics);
        self.check_mcpsg(class, diagnostics);
        self.check_mccsop(class, ctx, diagnostics);
        self.check_mcscn(class, ctx, diagnostics);
        self.check_mwkcl(class, diagnostics);
        self.check_mwkct(class, diagnostics);
        self.check_mwkref(class, diagnostics);
        self.check_superclass_rules(class, ctx, diagnostics);
        self.check_mcmio(class, diagnostics);
        self.check_mcmtp(class, diagnostics);
        self.check_mcpin(class, diagnostics);
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
// C2 checks: class/method attribute, constructor/superclass, and signature rules
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// Check whether a specific sub-check ID is enabled.
    fn is_check_enabled(&self, check_id: &str) -> bool {
        !self
            .config
            .disabled_checks
            .iter()
            .any(|id| id == check_id)
    }

    /// MABSEAC: instance properties/methods are illegal in Sealed+Abstract classes.
    fn check_mabseac(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !(self.is_check_enabled("MABSEAC") && class.is_sealed() && class.is_abstract()) {
            return;
        }
        for pb in &class.properties_blocks {
            let is_constant = pb.attributes.iter().any(|a| a.name == "Constant" && !a.negated);
            if is_constant {
                continue;
            }
            for prop in &pb.properties {
                diagnostics.push(Diagnostic {
                    rule_id: "MABSEAC",
                    message:
                        "Instance properties and methods are illegal in classes that are both Sealed and Abstract"
                            .to_string(),
                    severity: Severity::Error,
                    byte_range: prop.byte_range.clone(),
                    line: prop.line,
                    column: 1,
                    fix: None,
                });
            }
        }
        for mb in &class.methods_blocks {
            let is_static = mb.attributes.iter().any(|a| a.name == "Static" && !a.negated);
            if is_static {
                continue;
            }
            for method in &mb.methods {
                diagnostics.push(Diagnostic {
                    rule_id: "MABSEAC",
                    message:
                        "Instance properties and methods are illegal in classes that are both Sealed and Abstract"
                            .to_string(),
                    severity: Severity::Error,
                    byte_range: method.byte_range.clone(),
                    line: method.line,
                    column: 1,
                    fix: None,
                });
            }
        }
    }

    /// MCSWA: a Sealed class cannot specify allowed subclasses.
    fn check_mcswa(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MCSWA") || !class.is_sealed() {
            return;
        }
        if class
            .attributes
            .iter()
            .any(|a| a.name == "AllowedSubclasses" && !a.negated)
        {
            diagnostics.push(Diagnostic {
                rule_id: "MCSWA",
                message: "A sealed class cannot specify allowed subclasses".to_string(),
                severity: Severity::Error,
                byte_range: class.byte_range.clone(),
                line: class.line,
                column: 1,
                fix: None,
            });
        }
    }

    /// MTMAT: an attribute can only be set once within a single attribute list.
    fn check_mtmat(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MTMAT") {
            return;
        }
        Self::check_duplicate_attributes(
            &class.attributes,
            class.byte_range.clone(),
            class.line,
            diagnostics,
        );
        for pb in &class.properties_blocks {
            Self::check_duplicate_attributes(
                &pb.attributes,
                class.byte_range.clone(),
                class.line,
                diagnostics,
            );
        }
        for mb in &class.methods_blocks {
            Self::check_duplicate_attributes(
                &mb.attributes,
                class.byte_range.clone(),
                class.line,
                diagnostics,
            );
        }
    }

    /// Report MTMAT for duplicate attribute names in a single attribute list.
    fn check_duplicate_attributes(
        attrs: &[AttributeMeta],
        byte_range: std::ops::Range<usize>,
        line: usize,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut seen: Vec<&str> = Vec::new();
        for attr in attrs {
            if seen.contains(&attr.name.as_str()) {
                diagnostics.push(Diagnostic {
                    rule_id: "MTMAT",
                    message: format!("Attribute '{}' can only be set once", attr.name),
                    severity: Severity::Error,
                    byte_range: byte_range.clone(),
                    line,
                    column: 1,
                    fix: None,
                });
            } else {
                seen.push(&attr.name);
            }
        }
    }

    /// MTAGS3: the Access attribute cannot be combined with SetAccess/GetAccess.
    fn check_mtags3(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MTAGS3") {
            return;
        }
        for pb in &class.properties_blocks {
            if Self::block_has_access_conflict(&pb.attributes) {
                diagnostics.push(Diagnostic {
                    rule_id: "MTAGS3",
                    message: "Cannot use the Access attribute when using the SetAccess or GetAccess attribute"
                        .to_string(),
                    severity: Severity::Error,
                    byte_range: class.byte_range.clone(),
                    line: class.line,
                    column: 1,
                    fix: None,
                });
            }
        }
        for mb in &class.methods_blocks {
            if Self::block_has_access_conflict(&mb.attributes) {
                diagnostics.push(Diagnostic {
                    rule_id: "MTAGS3",
                    message: "Cannot use the Access attribute when using the SetAccess or GetAccess attribute"
                        .to_string(),
                    severity: Severity::Error,
                    byte_range: class.byte_range.clone(),
                    line: class.line,
                    column: 1,
                    fix: None,
                });
            }
        }
    }

    /// Whether an attribute list contains Access together with SetAccess or GetAccess.
    fn block_has_access_conflict(attrs: &[AttributeMeta]) -> bool {
        let has = |name: &str| attrs.iter().any(|a| a.name == name && !a.negated);
        has("Access") && (has("SetAccess") || has("GetAccess"))
    }

    /// MABSEAM: a method cannot be both Abstract and Sealed.
    fn check_mabseam(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MABSEAM") {
            return;
        }
        for mb in &class.methods_blocks {
            let is_abstract = mb.attributes.iter().any(|a| a.name == "Abstract" && !a.negated);
            let is_sealed = mb.attributes.iter().any(|a| a.name == "Sealed" && !a.negated);
            if is_abstract && is_sealed {
                for method in &mb.methods {
                    diagnostics.push(Diagnostic {
                        rule_id: "MABSEAM",
                        message: "A method cannot be both Abstract and Sealed".to_string(),
                        severity: Severity::Error,
                        byte_range: method.byte_range.clone(),
                        line: method.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }
    }

    /// MCMSP: a private method cannot be Abstract.
    fn check_mcmsp(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MCMSP") {
            return;
        }
        for mb in &class.methods_blocks {
            let is_abstract = mb.attributes.iter().any(|a| a.name == "Abstract" && !a.negated);
            let is_private = mb
                .attributes
                .iter()
                .any(|a| a.name == "Access" && !a.negated && a.value.as_deref() == Some("private"));
            if is_abstract && is_private {
                for method in &mb.methods {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCMSP",
                        message: "Private method cannot be Abstract".to_string(),
                        severity: Severity::Error,
                        byte_range: method.byte_range.clone(),
                        line: method.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }
    }

    /// MCAPP: a private property cannot be Abstract.
    fn check_mcapp(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MCAPP") {
            return;
        }
        for pb in &class.properties_blocks {
            let is_abstract = pb.attributes.iter().any(|a| a.name == "Abstract" && !a.negated);
            let is_private = pb
                .attributes
                .iter()
                .any(|a| a.name == "Access" && !a.negated && a.value.as_deref() == Some("private"));
            if is_abstract && is_private {
                for prop in &pb.properties {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCAPP",
                        message: "Private property cannot be Abstract".to_string(),
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

    /// MCGSA: a set/get method cannot target an abstract property.
    fn check_mcgsa(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MCGSA") {
            return;
        }
        let abstract_props: Vec<&str> = class
            .properties_blocks
            .iter()
            .filter(|pb| pb.attributes.iter().any(|a| a.name == "Abstract" && !a.negated))
            .flat_map(|pb| pb.properties.iter().map(|p| p.name.as_str()))
            .collect();
        if abstract_props.is_empty() {
            return;
        }
        for mb in &class.methods_blocks {
            for method in &mb.methods {
                if method.is_setter || method.is_getter {
                    let prop_name = method
                        .name
                        .strip_prefix("set.")
                        .or_else(|| method.name.strip_prefix("get."));
                    if let Some(pn) = prop_name {
                        if abstract_props.contains(&pn) {
                            diagnostics.push(Diagnostic {
                                rule_id: "MCGSA",
                                message: format!(
                                    "Method '{}' tries to set or get an abstract property",
                                    method.name
                                ),
                                severity: Severity::Error,
                                byte_range: method.byte_range.clone(),
                                line: method.line,
                                column: 1,
                                fix: None,
                            });
                        }
                    }
                }
            }
        }
    }

    /// MCPSG: set/get methods must be fully defined in the class definition file.
    fn check_mcpsg(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MCPSG") {
            return;
        }
        for mb in &class.methods_blocks {
            let block_abstract = mb.attributes.iter().any(|a| a.name == "Abstract" && !a.negated);
            for method in &mb.methods {
                if (method.is_setter || method.is_getter) && (method.is_abstract || block_abstract) {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCPSG",
                        message: "Set or get method must be fully defined in the class definition file"
                            .to_string(),
                        severity: Severity::Error,
                        byte_range: method.byte_range.clone(),
                        line: method.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }
    }

    /// MCCSOP: the constructor cannot modify Constant properties.
    fn check_mccsop(
        &self,
        class: &ClassMeta,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("MCCSOP") {
            return;
        }
        let root = ctx.tree.root_node();
        let constructor = self.find_function_definition_by_name(root, &class.name, ctx.source);
        if let Some(node) = constructor {
            Self::walk_constant_assignments(node, class, ctx.source, true, diagnostics);
        }
    }

    /// MCSCN: any method that sets a Constant property.
    fn check_mcscn(
        &self,
        class: &ClassMeta,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("MCSCN") {
            return;
        }
        let functions = self.collect_c2_function_definitions(ctx.tree.root_node(), ctx.source);
        for (node, _) in functions {
            Self::walk_constant_assignments(node, class, ctx.source, false, diagnostics);
        }
    }

    /// Walk a function body for assignments to Constant properties.
    fn walk_constant_assignments(
        node: tree_sitter::Node,
        class: &ClassMeta,
        source: &str,
        is_constructor: bool,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let method_name = node
            .child_by_field_name("name")
            .map(|n| node_text(n, source).to_string())
            .unwrap_or_default();
        Self::walk_constant_assignments_dfs(
            node,
            class,
            source,
            is_constructor,
            &method_name,
            diagnostics,
        );
    }

    /// DFS helper for [`Self::walk_constant_assignments`].
    fn walk_constant_assignments_dfs(
        node: tree_sitter::Node,
        class: &ClassMeta,
        source: &str,
        is_constructor: bool,
        method_name: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "assignment" {
            if let Some(prop_name) = assignment_field_property(node, source) {
                let is_constant = class.properties_blocks.iter().any(|pb| {
                    pb.attributes.iter().any(|a| a.name == "Constant" && !a.negated)
                        && pb.properties.iter().any(|p| p.name == prop_name)
                });
                if is_constant {
                    let pos = node.start_position();
                    let (rule_id, message) = if is_constructor {
                        (
                            "MCCSOP",
                            format!("Unable to modify Constant property '{prop_name}'"),
                        )
                    } else {
                        (
                            "MCSCN",
                            format!("Method '{method_name}' tries to set a constant property"),
                        )
                    };
                    diagnostics.push(Diagnostic {
                        rule_id,
                        message,
                        severity: Severity::Error,
                        byte_range: node.start_byte()..node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::walk_constant_assignments_dfs(
                child,
                class,
                source,
                is_constructor,
                method_name,
                diagnostics,
            );
        }
    }

    /// MWKCL: a WeakHandle property must have a class validation (type constraint).
    fn check_mwkcl(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MWKCL") {
            return;
        }
        for pb in &class.properties_blocks {
            let is_weak = pb.attributes.iter().any(|a| a.name == "WeakHandle" && !a.negated);
            if !is_weak {
                continue;
            }
            for prop in &pb.properties {
                if prop.type_constraint.is_none() {
                    diagnostics.push(Diagnostic {
                        rule_id: "MWKCL",
                        message: format!(
                            "A WeakHandle property must restrict its type using a class validation: '{}'",
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

    /// MWKCT: WeakHandle and Constant attributes cannot be combined.
    fn check_mwkct(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MWKCT") {
            return;
        }
        for pb in &class.properties_blocks {
            let is_weak = pb.attributes.iter().any(|a| a.name == "WeakHandle" && !a.negated);
            let is_constant = pb.attributes.iter().any(|a| a.name == "Constant" && !a.negated);
            if is_weak && is_constant {
                diagnostics.push(Diagnostic {
                    rule_id: "MWKCT",
                    message:
                        "Specifying both WeakHandle and Constant attributes on the same property is not supported"
                            .to_string(),
                    severity: Severity::Error,
                    byte_range: class.byte_range.clone(),
                    line: class.line,
                    column: 1,
                    fix: None,
                });
            }
        }
    }

    /// MWKREF: WeakHandle and Dependent attributes cannot be combined.
    fn check_mwkref(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MWKREF") {
            return;
        }
        for pb in &class.properties_blocks {
            let is_weak = pb.attributes.iter().any(|a| a.name == "WeakHandle" && !a.negated);
            let is_dependent = pb.attributes.iter().any(|a| a.name == "Dependent" && !a.negated);
            if is_weak && is_dependent {
                diagnostics.push(Diagnostic {
                    rule_id: "MWKREF",
                    message: "Specifying both WeakHandle and Dependent attributes is invalid"
                        .to_string(),
                    severity: Severity::Error,
                    byte_range: class.byte_range.clone(),
                    line: class.line,
                    column: 1,
                    fix: None,
                });
            }
        }
    }

    /// Group E: constructor/superclass call validation.
    fn check_superclass_rules(
        &self,
        class: &ClassMeta,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let functions = self.collect_c2_function_definitions(ctx.tree.root_node(), ctx.source);
        for (func_node, mname) in functions {
            let outputs = function_output_names(func_node, ctx.source);
            let first_output = outputs.first().cloned();
            let is_constructor = mname == class.name;
            let calls = self.collect_superclass_calls(func_node, ctx.source);
            let mut constructor_calls_seen = 0usize;
            for (call_node, caller, super_name) in calls {
                if is_constructor && Some(caller.as_str()) == first_output.as_deref() {
                    // Superclass constructor call.
                    if constructor_calls_seen == 0 {
                        constructor_calls_seen += 1;
                        if self.is_check_enabled("MCCBU") {
                            Self::check_super_after_object_use(
                                func_node,
                                call_node,
                                &caller,
                                ctx.source,
                                diagnostics,
                            );
                        }
                    } else if self.is_check_enabled("MCCMC") {
                        let pos = call_node.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "MCCMC",
                            message: "Constructor for superclass can only be called once".to_string(),
                            severity: Severity::Error,
                            byte_range: call_node.start_byte()..call_node.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                    if self.is_check_enabled("MCCBS")
                        && !class.superclasses.iter().any(|s| s == &super_name)
                    {
                        let pos = call_node.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "MCCBS",
                            message: format!(
                                "A superclass constructor is being called, but '{super_name}' is not a declared superclass name"
                            ),
                            severity: Severity::Error,
                            byte_range: call_node.start_byte()..call_node.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                    if self.is_check_enabled("MCSCT")
                        && Self::super_call_is_conditional_or_expression(call_node)
                    {
                        let pos = call_node.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "MCSCT",
                            message:
                                "Superclass constructor call must not be conditionalized or be part of another expression"
                                    .to_string(),
                            severity: Severity::Error,
                            byte_range: call_node.start_byte()..call_node.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                    let lhs = call_node
                        .parent()
                        .filter(|p| p.kind() == "assignment")
                        .and_then(|p| p.child_by_field_name("left"));
                    match lhs {
                        Some(lhs) if lhs.kind() == "multioutput_variable" => {
                            if self.is_check_enabled("MCSMO") {
                                let pos = call_node.start_position();
                                diagnostics.push(Diagnostic {
                                    rule_id: "MCSMO",
                                    message: "Returning multiple outputs from a superclass object initialization is not supported"
                                        .to_string(),
                                    severity: Severity::Error,
                                    byte_range: call_node.start_byte()..call_node.end_byte(),
                                    line: pos.row + 1,
                                    column: pos.column + 1,
                                    fix: None,
                                });
                            }
                        }
                        Some(lhs) if lhs.kind() == "identifier" => {
                            let lhs_text = node_text(lhs, ctx.source);
                            if self.is_check_enabled("MCSCF")
                                && Some(lhs_text) != first_output.as_deref()
                            {
                                let pos = call_node.start_position();
                                diagnostics.push(Diagnostic {
                                    rule_id: "MCSCF",
                                    message:
                                        "A superclass constructor must be assigned to the first constructor output argument"
                                            .to_string(),
                                    severity: Severity::Error,
                                    byte_range: call_node.start_byte()..call_node.end_byte(),
                                    line: pos.row + 1,
                                    column: pos.column + 1,
                                    fix: None,
                                });
                            }
                        }
                        _ => {
                            if self.is_check_enabled("MCSCF") {
                                let pos = call_node.start_position();
                                diagnostics.push(Diagnostic {
                                    rule_id: "MCSCF",
                                    message:
                                        "A superclass constructor must be assigned to the first constructor output argument"
                                            .to_string(),
                                    severity: Severity::Error,
                                    byte_range: call_node.start_byte()..call_node.end_byte(),
                                    line: pos.row + 1,
                                    column: pos.column + 1,
                                    fix: None,
                                });
                            }
                        }
                    }
                } else if is_constructor {
                    // Inside the constructor but not using the first output argument.
                    if outputs.iter().any(|o| o == &caller) {
                        if self.is_check_enabled("MCSCO") {
                            let pos = call_node.start_position();
                            diagnostics.push(Diagnostic {
                                rule_id: "MCSCO",
                                message: "A superclass constructor must be called using the first constructor output argument"
                                    .to_string(),
                                severity: Severity::Error,
                                byte_range: call_node.start_byte()..call_node.end_byte(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        }
                    } else if caller != mname && self.is_check_enabled("MCSCM") {
                        let pos = call_node.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "MCSCM",
                            message:
                                "To call a superclass method, the method name must match the subclass method name"
                                    .to_string(),
                            severity: Severity::Error,
                            byte_range: call_node.start_byte()..call_node.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                } else if Some(caller.as_str()) == first_output.as_deref() {
                    // Superclass constructor call from a non-constructor method.
                    if self.is_check_enabled("MCSCC") {
                        let pos = call_node.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "MCSCC",
                            message: "To call the superclass constructor, the subclass constructor name must match the subclass name"
                                .to_string(),
                            severity: Severity::Error,
                            byte_range: call_node.start_byte()..call_node.end_byte(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                } else if caller != mname && self.is_check_enabled("MCSCM") {
                    let pos = call_node.start_position();
                    diagnostics.push(Diagnostic {
                        rule_id: "MCSCM",
                        message:
                            "To call a superclass method, the method name must match the subclass method name"
                                .to_string(),
                        severity: Severity::Error,
                        byte_range: call_node.start_byte()..call_node.end_byte(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }
    }

    /// MCCBU: report a superclass constructor call that follows a use of the object.
    fn check_super_after_object_use(
        func_node: tree_sitter::Node,
        call_node: tree_sitter::Node,
        caller: &str,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let Some(block) = find_child_kind(func_node, "block") else {
            return;
        };
        let mut cur = call_node;
        let stmt = loop {
            match cur.parent() {
                Some(p) if p == block => break Some(cur),
                Some(p) => cur = p,
                None => break None,
            }
        };
        let Some(stmt) = stmt else {
            return;
        };
        let mut prior_use = false;
        let mut cursor = block.walk();
        for child in block.children(&mut cursor) {
            if child == stmt {
                break;
            }
            if child.is_named() && node_contains_identifier(child, caller, source) {
                prior_use = true;
                break;
            }
        }
        if prior_use {
            let pos = call_node.start_position();
            diagnostics.push(Diagnostic {
                rule_id: "MCCBU",
                message: "This superclass constructor is called after a use of the constructed object"
                    .to_string(),
                severity: Severity::Error,
                byte_range: call_node.start_byte()..call_node.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            });
        }
    }

    /// MCSCT: whether a super call is conditionalized or part of another expression.
    fn super_call_is_conditional_or_expression(call_node: tree_sitter::Node) -> bool {
        let parent = call_node.parent();
        let part_of_expression = match parent {
            Some(p) if p.kind() == "assignment" => p
                .child_by_field_name("right")
                .map(|r| r.id() != call_node.id())
                .unwrap_or(true),
            Some(p) => p.kind() != "assignment",
            None => true,
        };
        if part_of_expression {
            return true;
        }
        let mut cur = call_node;
        while let Some(p) = cur.parent() {
            if p.kind() == "function_definition" {
                break;
            }
            if matches!(
                p.kind(),
                "if_statement"
                    | "for_statement"
                    | "while_statement"
                    | "switch_statement"
                    | "try_statement"
                    | "spmd_statement"
            ) {
                return true;
            }
            cur = p;
        }
        false
    }

    /// MCMIO: a method has too many inputs or outputs (limit 64 each).
    fn check_mcmio(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MCMIO") {
            return;
        }
        for mb in &class.methods_blocks {
            for method in &mb.methods {
                let n_inputs = method.inputs.iter().filter(|s| s.as_str() != "~").count();
                let n_outputs = method.outputs.iter().filter(|s| s.as_str() != "~").count();
                if n_inputs > 64 || n_outputs > 64 {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCMIO",
                        message: format!(
                            "Method '{}' has too many inputs or outputs ({} inputs, {} outputs)",
                            method.name, n_inputs, n_outputs
                        ),
                        severity: Severity::Error,
                        byte_range: method.byte_range.clone(),
                        line: method.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }
    }

    /// MCMTP: TestParameterDefinition methods must be Static.
    fn check_mcmtp(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MCMTP") {
            return;
        }
        for mb in &class.methods_blocks {
            let is_static = mb.attributes.iter().any(|a| a.name == "Static" && !a.negated);
            if is_static {
                continue;
            }
            for method in &mb.methods {
                if method.name == "TestParameterDefinition" {
                    diagnostics.push(Diagnostic {
                        rule_id: "MCMTP",
                        message: "TestParameterDefinition methods must be Static".to_string(),
                        severity: Severity::Error,
                        byte_range: method.byte_range.clone(),
                        line: method.line,
                        column: 1,
                        fix: None,
                    });
                }
            }
        }
    }

    /// MCPIN: a property cannot be initialized to an instance of the class itself.
    fn check_mcpin(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("MCPIN") {
            return;
        }
        for pb in &class.properties_blocks {
            for prop in &pb.properties {
                if let Some(ref default_value) = prop.default_value {
                    if contains_self_constructor_call(default_value, &class.name) {
                        diagnostics.push(Diagnostic {
                            rule_id: "MCPIN",
                            message: format!(
                                "Unable to initialize class property '{}' to an instance of the class itself",
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

    /// Find a `function_definition` node by name in a tree.
    fn find_function_definition_by_name<'a>(
        &self,
        root: tree_sitter::Node<'a>,
        name: &str,
        source: &str,
    ) -> Option<tree_sitter::Node<'a>> {
        let mut stack = vec![root];
        while let Some(n) = stack.pop() {
            if n.kind() == "function_definition" {
                let fname = n
                    .child_by_field_name("name")
                    .map(|f| node_text(f, source))
                    .unwrap_or("");
                if fname == name {
                    return Some(n);
                }
            }
            let mut cursor = n.walk();
            for child in n.children(&mut cursor) {
                stack.push(child);
            }
        }
        None
    }

    /// Collect all `function_definition` nodes in a tree with their names.
    fn collect_c2_function_definitions<'a>(
        &self,
        root: tree_sitter::Node<'a>,
        source: &str,
    ) -> Vec<(tree_sitter::Node<'a>, String)> {
        let mut out = Vec::new();
        let mut stack = vec![root];
        while let Some(n) = stack.pop() {
            if n.kind() == "function_definition" {
                let name = n
                    .child_by_field_name("name")
                    .map(|f| node_text(f, source).to_string())
                    .unwrap_or_default();
                out.push((n, name));
            }
            let mut cursor = n.walk();
            for child in n.children(&mut cursor) {
                stack.push(child);
            }
        }
        out
    }

    /// Collect superclass calls (`X@Super(...)`) within a function node.
    fn collect_superclass_calls<'a>(
        &self,
        func_node: tree_sitter::Node<'a>,
        source: &str,
    ) -> Vec<(tree_sitter::Node<'a>, String, String)> {
        let mut out = Vec::new();
        let mut stack = vec![func_node];
        while let Some(n) = stack.pop() {
            if n.kind() == "function_call" && find_child_kind(n, "superclass").is_some() {
                let caller = {
                    let mut caller_cursor = n.walk();
                    let mut caller_name = String::new();
                    for child in n.children(&mut caller_cursor) {
                        if child.kind() == "identifier" {
                            caller_name = node_text(child, source).to_string();
                            break;
                        }
                    }
                    caller_name
                };
                let super_name = find_child_kind(n, "superclass")
                    .map(|s| node_text(s, source).trim().to_string())
                    .unwrap_or_default();
                out.push((n, caller, super_name));
            }
            let mut cursor = n.walk();
            for child in n.children(&mut cursor) {
                stack.push(child);
            }
        }
        out.reverse();
        out
    }
}

/// Find the first child of a node with the given kind.
fn find_child_kind<'a>(node: tree_sitter::Node<'a>, kind: &str) -> Option<tree_sitter::Node<'a>> {
    let mut cursor = node.walk();
    let mut result = None;
    for child in node.children(&mut cursor) {
        if child.kind() == kind {
            result = Some(child);
            break;
        }
    }
    result
}

/// Collect the output argument names of a function (excluding `~`).
fn function_output_names(node: tree_sitter::Node, source: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "function_output" {
            collect_identifiers_recursive(child, source, &mut out);
        }
    }
    out
}

/// Collect all identifier names within a node.
fn collect_identifiers_recursive(node: tree_sitter::Node, source: &str, out: &mut Vec<String>) {
    if node.kind() == "identifier" {
        out.push(node_text(node, source).to_string());
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_identifiers_recursive(child, source, out);
    }
}

/// Whether a node subtree references the given identifier name.
fn node_contains_identifier(node: tree_sitter::Node, name: &str, source: &str) -> bool {
    if node.kind() == "identifier" && node_text(node, source) == name {
        return true;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if node_contains_identifier(child, name, source) {
            return true;
        }
    }
    false
}

/// Get the property name of a `field_expression` assignment LHS (e.g., `obj.x` → `x`).
fn assignment_field_property(node: tree_sitter::Node, source: &str) -> Option<String> {
    let lhs = node.child_by_field_name("left")?;
    if lhs.kind() != "field_expression" {
        return None;
    }
    let last = lhs.child(lhs.child_count() - 1)?;
    if last.kind() == "identifier" {
        Some(node_text(last, source).to_string())
    } else {
        None
    }
}

/// Whether a property default value creates an instance of `class_name`.
fn contains_self_constructor_call(default_value: &str, class_name: &str) -> bool {
    let needle = format!("{class_name}(");
    let mut start = 0;
    while let Some(idx) = default_value[start..].find(&needle) {
        let abs = start + idx;
        let prev_is_ident = abs
            .checked_sub(1)
            .and_then(|i| default_value.as_bytes().get(i))
            .copied()
            .map(|b| b.is_ascii_alphanumeric() || b == b'_')
            .unwrap_or(false);
        if !prev_is_ident {
            return true;
        }
        start = abs + 1;
    }
    false
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

        // Walk the tree to collect top-level function definitions (matched to
        // metadata by byte range) and the names of nested functions.
        let mut function_nodes = Vec::new();
        let mut nested_function_names = Vec::new();
        Self::collect_function_definitions(
            ctx.tree.root_node(),
            0,
            ctx.source,
            &mut function_nodes,
            &mut nested_function_names,
        );

        for function_node in function_nodes {
            let func_meta = meta
                .functions
                .iter()
                .chain(meta.local_functions.iter())
                .find(|f| {
                    f.byte_range.start == function_node.start_byte()
                        && f.byte_range.end == function_node.end_byte()
                });
            let Some(func_meta) = func_meta else {
                continue;
            };

            let blocks = extract_arguments_blocks_grouped(function_node, ctx.source);
            if blocks.is_empty() {
                continue;
            }

            self.check_fv_structure(&blocks, ctx.source, diagnostics);
            self.check_fv_ordering(func_meta, &blocks, ctx.source, diagnostics);
            self.check_fv_validation(&blocks, &nested_function_names, ctx.source, diagnostics);
        }
    }

    /// Collect top-level `function_definition` nodes (depth 0) and the names
    /// of nested functions (depth >= 1).
    fn collect_function_definitions<'a>(
        node: tree_sitter::Node<'a>,
        depth: usize,
        source: &str,
        out_nodes: &mut Vec<tree_sitter::Node<'a>>,
        out_nested_names: &mut Vec<String>,
    ) {
        if node.kind() == "function_definition" {
            if depth > 0 {
                if let Some(name) = node.child_by_field_name("name") {
                    out_nested_names.push(node_text(name, source).to_string());
                }
            } else {
                out_nodes.push(node);
            }
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                Self::collect_function_definitions(
                    child,
                    depth + 1,
                    source,
                    out_nodes,
                    out_nested_names,
                );
            }
            return;
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_function_definitions(child, depth, source, out_nodes, out_nested_names);
        }
    }

    /// FVNST: Arguments blocks in nested functions are not allowed.
    fn check_fvnst(&self, ctx: &FileContext, diagnostics: &mut Vec<Diagnostic>) {
        // Walk the tree to find nested function_definitions with arguments blocks
        let root = ctx.tree.root_node();
        Self::find_nested_functions_with_args(root, 0, diagnostics);
    }

    /// Recursively find nested functions that have arguments blocks.
    fn find_nested_functions_with_args(
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
                Self::find_nested_functions_with_args(child, depth + 1, diagnostics);
            } else if child.kind() != "methods" && child.kind() != "class_definition" {
                // Continue recursing but don't increase depth for non-function nodes
                // Skip methods blocks (class methods are not nested functions)
                Self::find_nested_functions_with_args(child, depth, diagnostics);
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
// Arguments block structure and validation checks (FV* / TIN* / TTOO* checks)
// ---------------------------------------------------------------------------

/// The role of an `arguments ... end` block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockRole {
    /// Explicit `(Input)` block.
    Input,
    /// Explicit `(Output)` block.
    Output,
    /// Block with the `Repeating` attribute.
    Repeating,
    /// No attributes — implicitly an input block.
    Default,
}

impl BlockRole {
    /// Whether this block declares input arguments.
    fn is_input(self) -> bool {
        matches!(self, BlockRole::Input | BlockRole::Default)
    }

    /// Whether this block declares output arguments.
    fn is_output(self) -> bool {
        matches!(self, BlockRole::Output)
    }

    /// Whether this is a repeating block.
    fn is_repeating(self) -> bool {
        matches!(self, BlockRole::Repeating)
    }
}

/// Grouped metadata for a single `arguments ... end` block.
#[derive(Debug, Clone)]
struct ArgumentsBlockMeta<'a> {
    /// The raw `arguments_statement` AST node.
    node: tree_sitter::Node<'a>,
    /// Attribute identifiers declared on the block.
    attributes: Vec<String>,
    /// The role derived from the block attributes.
    role: BlockRole,
    /// Per-argument validation entries in declaration order.
    args: Vec<ArgValidation>,
    /// Byte range of the block.
    byte_range: Range<usize>,
}

impl ArgumentsBlockMeta<'_> {
    /// Whether this block declares repeating output arguments.
    fn is_output_repeating(&self) -> bool {
        self.role.is_repeating()
            && (self.attributes.iter().any(|a| a == "Output")
                || self.args.iter().any(|a| a.name == "varargout"))
    }

    /// Whether this block declares repeating input arguments.
    fn is_input_repeating(&self) -> bool {
        self.role.is_repeating() && !self.is_output_repeating()
    }
}

/// Extract all `arguments ... end` blocks from a function definition,
/// preserving block grouping, attributes, and declaration order.
fn extract_arguments_blocks_grouped<'a>(
    func_node: tree_sitter::Node<'a>,
    source: &str,
) -> Vec<ArgumentsBlockMeta<'a>> {
    let mut blocks = Vec::new();
    let mut cursor = func_node.walk();
    for child in func_node.children(&mut cursor) {
        if child.kind() != "arguments_statement" {
            continue;
        }
        let attributes = block_attributes(child, source);
        let role = block_role(&attributes);
        let args = block_arg_validations(child, source);
        blocks.push(ArgumentsBlockMeta {
            node: child,
            attributes,
            role,
            args,
            byte_range: child.start_byte()..child.end_byte(),
        });
    }
    blocks
}

/// Collect the attribute identifiers declared on an arguments block.
fn block_attributes(block_node: tree_sitter::Node, source: &str) -> Vec<String> {
    let mut attrs = Vec::new();
    let mut cursor = block_node.walk();
    for child in block_node.children(&mut cursor) {
        if child.kind() == "attributes" {
            let mut inner = child.walk();
            for attr in child.children(&mut inner) {
                if attr.kind() == "identifier" {
                    attrs.push(node_text(attr, source).to_string());
                }
            }
            break;
        }
    }
    attrs
}

/// Derive the role of an arguments block from its attribute identifiers.
fn block_role(attributes: &[String]) -> BlockRole {
    if attributes.iter().any(|a| a == "Repeating") {
        BlockRole::Repeating
    } else if attributes.iter().any(|a| a == "Output") {
        BlockRole::Output
    } else if attributes.iter().any(|a| a == "Input") {
        BlockRole::Input
    } else {
        BlockRole::Default
    }
}

/// Extract per-argument validation entries from an arguments block.
fn block_arg_validations(block_node: tree_sitter::Node, source: &str) -> Vec<ArgValidation> {
    let mut validations = Vec::new();
    let mut cursor = block_node.walk();
    for child in block_node.children(&mut cursor) {
        if child.kind() == "property" {
            if let Some(validation) = extract_block_arg_validation(child, source) {
                validations.push(validation);
            }
        }
    }
    validations
}

/// Extract a single argument validation entry from an arguments block property.
fn extract_block_arg_validation(
    prop_node: tree_sitter::Node,
    source: &str,
) -> Option<ArgValidation> {
    let name = if prop_has_ignored(prop_node) {
        "~".to_string()
    } else if prop_has_name_value(prop_node) {
        prop_name_value_text(prop_node, source)
    } else {
        prop_plain_name(prop_node, source)?
    };

    let dimensions = find_child_kind(prop_node, "dimensions")
        .map(|n| node_text(n, source).to_string());

    let type_constraint = extract_block_type_constraint(prop_node, source);

    let validators = find_child_kind(prop_node, "validation_functions")
        .map(|vf| {
            let mut v = Vec::new();
            let mut vc = vf.walk();
            for vchild in vf.children(&mut vc) {
                if vchild.kind() == "identifier" {
                    v.push(node_text(vchild, source).to_string());
                }
            }
            v
        })
        .unwrap_or_default();

    let default_value = find_child_kind(prop_node, "default_value").map(|dv| {
        let text = node_text(dv, source).trim();
        let text = text.strip_prefix('=').unwrap_or(text).trim().to_string();
        text
    });

    Some(ArgValidation {
        name,
        dimensions,
        type_constraint,
        validators,
        default_value,
    })
}

/// Extract the type constraint (class name) of an arguments block property.
fn extract_block_type_constraint(
    prop_node: tree_sitter::Node,
    source: &str,
) -> Option<String> {
    let mut cursor = prop_node.walk();
    let mut past_name = false;
    let mut past_dims = false;
    for child in prop_node.children(&mut cursor) {
        match child.kind() {
            "identifier" | "property_name" => {
                if !past_name {
                    past_name = true;
                    continue;
                }
                if !past_dims {
                    continue;
                }
                return Some(node_text(child, source).to_string());
            }
            "ignored_argument" => {
                past_name = true;
            }
            "dimensions" => {
                past_name = true;
                past_dims = true;
            }
            "validation_functions" | "default_value" | "=" => {
                return None;
            }
            _ => {}
        }
    }
    None
}

/// Whether the property is an ignored argument (`~`).
fn prop_has_ignored(prop: tree_sitter::Node) -> bool {
    find_child_kind(prop, "ignored_argument").is_some()
}

/// Whether the property is a name-value argument (`struct.field`).
fn prop_has_name_value(prop: tree_sitter::Node) -> bool {
    find_child_kind(prop, "property_name").is_some()
}

/// The full dotted text of a name-value property.
fn prop_name_value_text(prop: tree_sitter::Node, source: &str) -> String {
    find_child_kind(prop, "property_name")
        .map(|n| node_text(n, source).to_string())
        .unwrap_or_default()
}

/// The struct name of a name-value property (before the first dot).
fn prop_name_value_struct(prop: tree_sitter::Node, source: &str) -> Option<String> {
    let pn = find_child_kind(prop, "property_name")?;
    find_child_kind(pn, "identifier").map(|n| node_text(n, source).to_string())
}

/// The field name of a name-value property (after the last dot).
fn prop_name_value_field(prop: tree_sitter::Node, source: &str) -> Option<String> {
    let pn = find_child_kind(prop, "property_name")?;
    let mut last = None;
    let mut cursor = pn.walk();
    for child in pn.children(&mut cursor) {
        if child.kind() == "identifier" {
            last = Some(node_text(child, source).to_string());
        }
    }
    last
}

/// The plain identifier name of a positional property.
fn prop_plain_name(prop: tree_sitter::Node, source: &str) -> Option<String> {
    let name = prop
        .child_by_field_name("name")
        .or_else(|| find_child_kind(prop, "identifier"))?;
    Some(node_text(name, source).to_string())
}

/// Whether the property declares a default value.
fn prop_has_default(prop: tree_sitter::Node) -> bool {
    find_child_kind(prop, "default_value").is_some()
}

/// Whether the property declares validation functions.
fn prop_has_validation(prop: tree_sitter::Node) -> bool {
    find_child_kind(prop, "validation_functions").is_some()
}

/// Whether the property declares a size constraint.
fn prop_has_dimensions(prop: tree_sitter::Node) -> bool {
    find_child_kind(prop, "dimensions").is_some()
}

/// Collect the `property` and `class_property` children of an arguments block.
fn block_properties(block_node: tree_sitter::Node) -> Vec<tree_sitter::Node> {
    let mut props = Vec::new();
    let mut cursor = block_node.walk();
    for child in block_node.children(&mut cursor) {
        if child.kind() == "property" || child.kind() == "class_property" {
            props.push(child);
        }
    }
    props
}

/// Whether an arguments block declares any attribute value (must be logical constants).
fn attributes_have_value(block_node: tree_sitter::Node) -> bool {
    let mut cursor = block_node.walk();
    for child in block_node.children(&mut cursor) {
        if child.kind() != "attributes" {
            continue;
        }
        let mut inner = child.walk();
        for attr in child.children(&mut inner) {
            if attr.kind() == "=" || attr.is_error() || attr.is_missing() {
                return true;
            }
        }
        return false;
    }
    false
}

/// A reference made inside a validation function call.
#[derive(Debug, Clone)]
enum CallReference {
    /// A plain identifier reference.
    Identifier(String),
    /// A dotted field reference such as `opts.Name`.
    Field,
}

/// A validation function call extracted from an arguments block property.
#[derive(Debug, Clone)]
struct ValidatorCall {
    /// References to arguments made inside the call.
    references: Vec<CallReference>,
}

/// Extract all validation function calls from a property.
fn prop_validator_calls(prop: tree_sitter::Node, source: &str) -> Vec<ValidatorCall> {
    let mut calls = Vec::new();
    let Some(vf) = find_child_kind(prop, "validation_functions") else {
        return calls;
    };
    let mut cursor = vf.walk();
    for child in vf.children(&mut cursor) {
        if child.kind() == "function_call" {
            if let Some(call) = extract_validator_call(child, source) {
                calls.push(call);
            }
        }
    }
    calls
}

/// Extract a single validation function call node.
fn extract_validator_call(call_node: tree_sitter::Node, source: &str) -> Option<ValidatorCall> {
    let mut references = Vec::new();
    if let Some(args_node) = find_child_kind(call_node, "arguments") {
        let mut cursor = args_node.walk();
        for arg in args_node.children(&mut cursor) {
            match arg.kind() {
                "identifier" => {
                    references.push(CallReference::Identifier(node_text(arg, source).to_string()));
                }
                "field_expression" => {
                    references.push(CallReference::Field);
                }
                _ => {}
            }
        }
    }
    Some(ValidatorCall { references })
}

/// Collect all `function_call` nodes within a subtree.
fn collect_function_calls<'a>(
    node: tree_sitter::Node<'a>,
    out: &mut Vec<tree_sitter::Node<'a>>,
) {
    if node.kind() == "function_call" {
        out.push(node);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_function_calls(child, out);
    }
}

/// Whether a default value expression references a name-value structure.
fn default_refs_name_value(node: tree_sitter::Node, source: &str, nv_structs: &[String]) -> bool {
    if node.kind() == "field_expression" {
        if let Some(base) = find_child_kind(node, "identifier") {
            if nv_structs.iter().any(|s| s == node_text(base, source)) {
                return true;
            }
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if default_refs_name_value(child, source, nv_structs) {
            return true;
        }
    }
    false
}

/// Build the declaration sequence of an arguments block group.
///
/// Consecutive name-value struct names are deduplicated: multiple fields of
/// the same structure map to a single function-line argument.
fn block_declaration_sequence(
    blocks: &[ArgumentsBlockMeta],
    source: &str,
    outputs: bool,
) -> Vec<String> {
    let mut sequence = Vec::new();
    for block in blocks {
        let relevant = if outputs {
            block.role.is_output() || block.is_output_repeating()
        } else {
            block.role.is_input() || block.is_input_repeating()
        };
        if !relevant {
            continue;
        }
        for prop in block_properties(block.node) {
            let name = if prop_has_ignored(prop) {
                Some("~".to_string())
            } else if prop_has_name_value(prop) {
                prop_name_value_struct(prop, source)
            } else {
                prop_plain_name(prop, source)
            };
            let Some(name) = name else {
                continue;
            };
            if sequence.last() == Some(&name) {
                continue;
            }
            sequence.push(name);
        }
    }
    sequence
}

/// Whether `sequence` is a prefix of `full`.
fn is_prefix_of(sequence: &[String], full: &[String]) -> bool {
    sequence.len() <= full.len() && sequence.iter().zip(full.iter()).all(|(a, b)| a == b)
}

/// Whether a name is a known MATLAB class name.
fn is_known_class_name(name: &str) -> bool {
    matches!(
        name,
        "double" | "single" | "logical" | "char" | "string" | "cell" | "struct"
            | "function_handle" | "int8" | "int16" | "int32" | "int64" | "uint8"
            | "uint16" | "uint32" | "uint64" | "table" | "datetime" | "duration"
            | "categorical" | "calendarDuration"
    )
}

/// Functions that are not supported inside arguments blocks.
const BANNED_ARGUMENTS_FUNCTIONS: &[&str] = &[
    "eval",
    "evalin",
    "assignin",
    "inputname",
    "nargin",
    "nargout",
    "narginchk",
    "nargoutchk",
];

impl LanguageSpecEngine {
    /// Block-level structural checks for arguments blocks.
    fn check_fv_structure(
        &self,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        self.check_fvdrep(blocks, diagnostics);
        self.check_fvioa(blocks, diagnostics);
        self.check_fvrepo(blocks, diagnostics);
        self.check_fvvrep(blocks, diagnostics);
        self.check_fvovrep(blocks, diagnostics);
        self.check_fvnrep_fvrepd(blocks, source, diagnostics);
        self.check_fvood_fvooi_fvoon(blocks, source, diagnostics);
        self.check_fvorm(blocks, source, diagnostics);
        self.check_fvobi(blocks, diagnostics);
        self.check_fvatf(blocks, source, diagnostics);
        self.check_fvmcl(blocks, source, diagnostics);
        self.check_fvnde_fvnvl(blocks, source, diagnostics);
    }

    /// Push a diagnostic at the start of an arguments block.
    fn push_block_diag(
        &self,
        block: &ArgumentsBlockMeta,
        rule_id: &'static str,
        message: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let pos = block.node.start_position();
        diagnostics.push(Diagnostic {
            rule_id,
            message: message.to_string(),
            severity: Severity::Error,
            byte_range: block.byte_range.clone(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        });
    }

    /// FVDREP: Multiple Repeating arguments blocks are not supported.
    fn check_fvdrep(&self, blocks: &[ArgumentsBlockMeta], diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("FVDREP") {
            return;
        }
        let mut seen_repeating = false;
        for block in blocks {
            if !block.role.is_repeating() {
                continue;
            }
            if seen_repeating {
                self.push_block_diag(
                    block,
                    "FVDREP",
                    "Multiple Repeating arguments blocks are not supported.",
                    diagnostics,
                );
            }
            seen_repeating = true;
        }
    }

    /// FVIOA: Both 'Input' and 'Output' attributes on one block are not supported.
    fn check_fvioa(&self, blocks: &[ArgumentsBlockMeta], diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("FVIOA") {
            return;
        }
        for block in blocks {
            let has_input = block.attributes.iter().any(|a| a == "Input");
            let has_output = block.attributes.iter().any(|a| a == "Output");
            if has_input && has_output {
                self.push_block_diag(
                    block,
                    "FVIOA",
                    "Specifying both 'Input' and 'Output' attributes on the same arguments block is not supported.",
                    diagnostics,
                );
            }
        }
    }

    /// FVREPO: A repeating input block with varargin must not have other arguments.
    fn check_fvrepo(&self, blocks: &[ArgumentsBlockMeta], diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("FVREPO") {
            return;
        }
        for block in blocks {
            if !block.is_input_repeating() {
                continue;
            }
            let has_varargin = block.args.iter().any(|a| a.name == "varargin");
            if has_varargin && block.args.len() > 1 {
                self.push_block_diag(
                    block,
                    "FVREPO",
                    "Repeating input arguments block containing varargin must not have other arguments.",
                    diagnostics,
                );
            }
        }
    }

    /// FVVREP: varargin can only be used inside a repeating input arguments block.
    fn check_fvvrep(&self, blocks: &[ArgumentsBlockMeta], diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("FVVREP") {
            return;
        }
        for block in blocks {
            if block.role.is_repeating() {
                continue;
            }
            for arg in &block.args {
                if arg.name == "varargin" {
                    self.push_block_diag(
                        block,
                        "FVVREP",
                        "varargin can only be used inside repeating input arguments block.",
                        diagnostics,
                    );
                }
            }
        }
    }

    /// FVOVREP: varargout can only be used inside a repeating output arguments block.
    fn check_fvovrep(&self, blocks: &[ArgumentsBlockMeta], diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("FVOVREP") {
            return;
        }
        for block in blocks {
            if block.role.is_repeating() {
                continue;
            }
            for arg in &block.args {
                if arg.name == "varargout" {
                    self.push_block_diag(
                        block,
                        "FVOVREP",
                        "Output argument varargout can only be used inside a Repeating output arguments block.",
                        diagnostics,
                    );
                }
            }
        }
    }

    /// FVNREP / FVREPD: name-value arguments and default values are not
    /// supported in a Repeating arguments block.
    fn check_fvnrep_fvrepd(
        &self,
        blocks: &[ArgumentsBlockMeta],
        _source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let nrep = self.is_check_enabled("FVNREP");
        let repd = self.is_check_enabled("FVREPD");
        if !nrep && !repd {
            return;
        }
        for block in blocks {
            if !block.role.is_repeating() {
                continue;
            }
            for prop in block_properties(block.node) {
                if nrep && prop_has_name_value(prop) {
                    self.push_diag(
                        prop,
                        "FVNREP",
                        "Name-value arguments are not supported in a Repeating arguments block.",
                        diagnostics,
                    );
                }
                if repd && prop_has_default(prop) {
                    self.push_diag(
                        prop,
                        "FVREPD",
                        "Default values are not supported in a Repeating arguments block.",
                        diagnostics,
                    );
                }
            }
        }
    }

    /// FVOOD / FVOOI / FVOON: output arguments blocks cannot declare defaults,
    /// ignored arguments, or name-value arguments.
    fn check_fvood_fvooi_fvoon(
        &self,
        blocks: &[ArgumentsBlockMeta],
        _source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let ood = self.is_check_enabled("FVOOD");
        let ooi = self.is_check_enabled("FVOOI");
        let oon = self.is_check_enabled("FVOON");
        if !ood && !ooi && !oon {
            return;
        }
        for block in blocks.iter().filter(|b| b.role.is_output()) {
            for prop in block_properties(block.node) {
                if ood && prop_has_default(prop) {
                    self.push_diag(
                        prop,
                        "FVOOD",
                        "Specifying a default value for an output argument is not supported.",
                        diagnostics,
                    );
                }
                if ooi && prop_has_ignored(prop) {
                    self.push_diag(
                        prop,
                        "FVOOI",
                        "Use of ignored arguments in output arguments block is not supported.",
                        diagnostics,
                    );
                }
                if oon && prop_has_name_value(prop) {
                    self.push_diag(
                        prop,
                        "FVOON",
                        "Using name-value argument as output argument is not supported.",
                        diagnostics,
                    );
                }
            }
        }
    }

    /// FVORM: Declaring multiple repeating output arguments is not supported.
    fn check_fvorm(
        &self,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVORM") {
            return;
        }
        let mut seen_varargout = false;
        for block in blocks {
            for prop in block_properties(block.node) {
                let is_varargout = prop_plain_name(prop, source)
                    .as_deref()
                    .map(|n| n == "varargout")
                    .unwrap_or(false);
                if !is_varargout {
                    continue;
                }
                if seen_varargout {
                    self.push_diag(
                        prop,
                        "FVORM",
                        "Declaring multiple repeating output arguments is not supported.",
                        diagnostics,
                    );
                }
                seen_varargout = true;
            }
        }
    }

    /// FVOBI: Declare all input argument blocks before all output arguments blocks.
    fn check_fvobi(&self, blocks: &[ArgumentsBlockMeta], diagnostics: &mut Vec<Diagnostic>) {
        if !self.is_check_enabled("FVOBI") {
            return;
        }
        let mut saw_output = false;
        for block in blocks {
            if block.role.is_output() || block.is_output_repeating() {
                saw_output = true;
            } else if saw_output && (block.role.is_input() || block.is_input_repeating()) {
                self.push_block_diag(
                    block,
                    "FVOBI",
                    "Declare all input argument blocks before all output arguments blocks.",
                    diagnostics,
                );
            }
        }
    }

    /// FVATF: Attribute values in arguments blocks must be logical constants.
    fn check_fvatf(
        &self,
        blocks: &[ArgumentsBlockMeta],
        _source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVATF") {
            return;
        }
        for block in blocks {
            if attributes_have_value(block.node) {
                self.push_block_diag(
                    block,
                    "FVATF",
                    "Attribute values in arguments blocks must be logical constants.",
                    diagnostics,
                );
            }
        }
    }

    /// FVMCL: `.?ClassName` can only be used for one name-value structure.
    fn check_fvmcl(
        &self,
        blocks: &[ArgumentsBlockMeta],
        _source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVMCL") {
            return;
        }
        let mut seen_class_property = false;
        for block in blocks {
            for prop in block_properties(block.node) {
                if prop.kind() != "class_property" {
                    continue;
                }
                if seen_class_property {
                    self.push_diag(
                        prop,
                        "FVMCL",
                        "Specifying multiple name-value structures using .? syntax and a class name is not supported.",
                        diagnostics,
                    );
                }
                seen_class_property = true;
            }
        }
    }

    /// FVNDE / FVNVL: name-value arguments using a class name cannot have
    /// default values or validation functions.
    fn check_fvnde_fvnvl(
        &self,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let nde = self.is_check_enabled("FVNDE");
        let nvl = self.is_check_enabled("FVNVL");
        if !nde && !nvl {
            return;
        }
        for block in blocks {
            for prop in block_properties(block.node) {
                if !prop_has_name_value(prop) {
                    continue;
                }
                let uses_class_name = prop_name_value_field(prop, source)
                    .as_deref()
                    .map(is_known_class_name)
                    .unwrap_or(false);
                if !uses_class_name {
                    continue;
                }
                if nde && prop_has_default(prop) {
                    self.push_diag(
                        prop,
                        "FVNDE",
                        "When specifying name-value arguments using a class name, it is illegal to specify default values for the arguments.",
                        diagnostics,
                    );
                }
                if nvl && prop_has_validation(prop) {
                    self.push_diag(
                        prop,
                        "FVNVL",
                        "When specifying name-value arguments using a class name, it is illegal to specify validation for the arguments.",
                        diagnostics,
                    );
                }
            }
        }
    }
}

impl LanguageSpecEngine {
    /// Ordering, duplication, and consistency checks for arguments blocks.
    fn check_fv_ordering(
        &self,
        func_meta: &FunctionMeta,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let input_props: Vec<tree_sitter::Node> = blocks
            .iter()
            .filter(|b| b.role.is_input() || b.is_input_repeating())
            .flat_map(|b| block_properties(b.node))
            .collect();

        self.check_fvdap(&input_props, source, diagnostics);
        self.check_fvdnf(&input_props, source, diagnostics);
        self.check_fvdan(&input_props, source, diagnostics);
        self.check_fvniv(func_meta, &input_props, source, diagnostics);
        self.check_fvordi(blocks, source, diagnostics);
        self.check_fvordn(&input_props, source, diagnostics);
        self.check_fvapn(&input_props, source, diagnostics);
        self.check_fvordp(blocks, source, diagnostics);
        self.check_fvordo(blocks, source, diagnostics);
        self.check_fvidv(blocks, source, diagnostics);
        self.check_fvsor(func_meta, blocks, source, diagnostics);
        self.check_fvsoro(func_meta, blocks, source, diagnostics);
    }

    /// FVDAP: Positional argument can only be declared once.
    fn check_fvdap(
        &self,
        input_props: &[tree_sitter::Node],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVDAP") {
            return;
        }
        let mut seen: Vec<String> = Vec::new();
        for prop in input_props {
            if prop_has_ignored(*prop) || prop_has_name_value(*prop) {
                continue;
            }
            let Some(name) = prop_plain_name(*prop, source) else {
                continue;
            };
            if seen.contains(&name) {
                self.push_diag(
                    *prop,
                    "FVDAP",
                    "Positional argument can only be declared once.",
                    diagnostics,
                );
            } else {
                seen.push(name);
            }
        }
    }

    /// FVDNF: Name-value argument can only be declared once.
    fn check_fvdnf(
        &self,
        input_props: &[tree_sitter::Node],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVDNF") {
            return;
        }
        let mut seen: Vec<String> = Vec::new();
        for prop in input_props {
            if !prop_has_name_value(*prop) {
                continue;
            }
            let name = prop_name_value_text(*prop, source);
            if seen.contains(&name) {
                self.push_diag(
                    *prop,
                    "FVDNF",
                    "Name-value argument can only be declared once.",
                    diagnostics,
                );
            } else {
                seen.push(name);
            }
        }
    }

    /// FVDAN: Same name as both a name-value structure and a positional argument.
    fn check_fvdan(
        &self,
        input_props: &[tree_sitter::Node],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVDAN") {
            return;
        }
        let positional_names: Vec<String> = input_props
            .iter()
            .filter_map(|p| {
                if prop_has_ignored(*p) || prop_has_name_value(*p) {
                    None
                } else {
                    prop_plain_name(*p, source)
                }
            })
            .collect();
        let mut reported: Vec<String> = Vec::new();
        for prop in input_props {
            if !prop_has_name_value(*prop) {
                continue;
            }
            let Some(struct_name) = prop_name_value_struct(*prop, source) else {
                continue;
            };
            if positional_names.contains(&struct_name) && !reported.contains(&struct_name) {
                reported.push(struct_name);
                self.push_diag(
                    *prop,
                    "FVDAN",
                    "Using the same name as both a name-value argument structure and as a positional argument is not supported.",
                    diagnostics,
                );
            }
        }
    }

    /// FVNIV: A declared argument must be an input to the function.
    fn check_fvniv(
        &self,
        func_meta: &FunctionMeta,
        input_props: &[tree_sitter::Node],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVNIV") {
            return;
        }
        for prop in input_props {
            if prop_has_ignored(*prop) {
                continue;
            }
            let declared = if prop_has_name_value(*prop) {
                prop_name_value_struct(*prop, source)
            } else {
                prop_plain_name(*prop, source)
            };
            let Some(name) = declared else {
                continue;
            };
            if name == "varargin" || name == "varargout" {
                continue;
            }
            if !func_meta.inputs.iter().any(|i| i == &name) {
                self.push_diag(
                    *prop,
                    "FVNIV",
                    "This variable is not an input to the function and cannot be used in an arguments block.",
                    diagnostics,
                );
            }
        }
    }

    /// FVORDI: Ignored input arguments are not allowed after a Repeating
    /// arguments block or name-value arguments.
    fn check_fvordi(
        &self,
        blocks: &[ArgumentsBlockMeta],
        _source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVORDI") {
            return;
        }
        let mut saw_name_value = false;
        let mut saw_repeating = false;
        for block in blocks {
            if !(block.role.is_input() || block.is_input_repeating()) {
                continue;
            }
            if block.is_input_repeating() {
                saw_repeating = true;
            }
            for prop in block_properties(block.node) {
                if prop_has_ignored(prop) {
                    if saw_name_value || saw_repeating {
                        self.push_diag(
                            prop,
                            "FVORDI",
                            "Ignored input arguments are not allowed after a Repeating arguments block or name-value arguments.",
                            diagnostics,
                        );
                    }
                } else if prop_has_name_value(prop) {
                    saw_name_value = true;
                }
            }
        }
    }

    /// FVORDN: Positional arguments must be defined before name-value arguments.
    fn check_fvordn(
        &self,
        input_props: &[tree_sitter::Node],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVORDN") {
            return;
        }
        for (index, prop) in input_props.iter().enumerate() {
            if !self.fv_is_positional_prop(*prop, source) {
                continue;
            }
            let has_earlier_name_value = input_props[..index]
                .iter()
                .any(|p| prop_has_name_value(*p));
            if has_earlier_name_value {
                self.push_diag(
                    *prop,
                    "FVORDN",
                    "Positional arguments must be defined before name-value arguments.",
                    diagnostics,
                );
            }
        }
    }

    /// FVAPN: name-value arguments using the name=value syntax must come at the end.
    fn check_fvapn(
        &self,
        input_props: &[tree_sitter::Node],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVAPN") {
            return;
        }
        for (index, prop) in input_props.iter().enumerate() {
            if !prop_has_name_value(*prop) || !prop_has_default(*prop) {
                continue;
            }
            let later_positional = input_props[index + 1..]
                .iter()
                .any(|p| self.fv_is_positional_prop(*p, source));
            if later_positional {
                self.push_diag(
                    *prop,
                    "FVAPN",
                    "Move name-value arguments that use the name=value syntax to the end of the argument list.",
                    diagnostics,
                );
            }
        }
    }

    /// Whether a property is a positional input argument (not ignored, not a
    /// name-value argument, and not `varargin`).
    fn fv_is_positional_prop(&self, prop: tree_sitter::Node, source: &str) -> bool {
        !prop_has_ignored(prop)
            && !prop_has_name_value(prop)
            && prop_plain_name(prop, source)
                .map(|n| n != "varargin")
                .unwrap_or(false)
    }

    /// FVORDP: Positional arguments must be ordered required, optional, repeating.
    fn check_fvordp(
        &self,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVORDP") {
            return;
        }
        let mut saw_optional = false;
        let mut saw_repeating = false;
        for block in blocks {
            if !(block.role.is_input() || block.is_input_repeating()) {
                continue;
            }
            let block_repeating = block.is_input_repeating();
            for prop in block_properties(block.node) {
                if prop_has_ignored(prop) || prop_has_name_value(prop) {
                    continue;
                }
                let Some(name) = prop_plain_name(prop, source) else {
                    continue;
                };
                if name == "varargin" || block_repeating {
                    saw_repeating = true;
                    continue;
                }
                let is_optional = prop_has_default(prop);
                if saw_repeating || (!is_optional && saw_optional) {
                    self.push_diag(
                        prop,
                        "FVORDP",
                        "Positional arguments must be defined in the following order: required, optional, and repeating.",
                        diagnostics,
                    );
                }
                if is_optional {
                    saw_optional = true;
                }
            }
        }
    }

    /// FVORDO: Repeating output arguments must be defined after required output arguments.
    fn check_fvordo(
        &self,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVORDO") {
            return;
        }
        let mut saw_repeating_output = false;
        for block in blocks {
            if !(block.role.is_output() || block.is_output_repeating()) {
                continue;
            }
            let block_repeating_output = block.is_output_repeating();
            for prop in block_properties(block.node) {
                if prop_has_ignored(prop) || prop_has_name_value(prop) {
                    continue;
                }
                let Some(name) = prop_plain_name(prop, source) else {
                    continue;
                };
                if name == "varargout" || block_repeating_output {
                    saw_repeating_output = true;
                    continue;
                }
                if saw_repeating_output {
                    self.push_diag(
                        prop,
                        "FVORDO",
                        "Repeating output arguments must be defined after required output arguments.",
                        diagnostics,
                    );
                }
            }
        }
    }

    /// FVIDV: Validation or default values on ignored arguments are not supported.
    fn check_fvidv(
        &self,
        blocks: &[ArgumentsBlockMeta],
        _source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVIDV") {
            return;
        }
        for block in blocks {
            for prop in block_properties(block.node) {
                if !prop_has_ignored(prop) {
                    continue;
                }
                let has_spec = prop_has_dimensions(prop)
                    || prop_has_validation(prop)
                    || prop_has_default(prop)
                    || find_child_kind(prop, "identifier").is_some();
                if has_spec {
                    self.push_diag(
                        prop,
                        "FVIDV",
                        "Specifying validation or default value for ignored arguments is not supported.",
                        diagnostics,
                    );
                }
            }
        }
    }

    /// FVSOR: Input arguments block declarations must match the function line.
    fn check_fvsor(
        &self,
        func_meta: &FunctionMeta,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVSOR") {
            return;
        }
        let sequence = block_declaration_sequence(blocks, source, false);
        if sequence.is_empty() {
            return;
        }
        if !is_prefix_of(&sequence, &func_meta.inputs) {
            if let Some(block) = blocks
                .iter()
                .find(|b| b.role.is_input() || b.is_input_repeating())
            {
                self.push_block_diag(
                    block,
                    "FVSOR",
                    "Input arguments block declarations and the function line must contain the same input arguments in the same order, including ignored arguments.",
                    diagnostics,
                );
            }
        }
    }

    /// FVSORO: Output arguments block declarations must match the function line.
    fn check_fvsoro(
        &self,
        func_meta: &FunctionMeta,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVSORO") {
            return;
        }
        let sequence = block_declaration_sequence(blocks, source, true);
        if sequence.is_empty() {
            return;
        }
        if !is_prefix_of(&sequence, &func_meta.outputs) {
            if let Some(block) = blocks
                .iter()
                .find(|b| b.role.is_output() || b.is_output_repeating())
            {
                self.push_block_diag(
                    block,
                    "FVSORO",
                    "Output arguments block declarations and the function line must contain the same output arguments in the same order.",
                    diagnostics,
                );
            }
        }
    }
}

impl LanguageSpecEngine {
    /// Validation function and size constraint checks for arguments blocks.
    fn check_fv_validation(
        &self,
        blocks: &[ArgumentsBlockMeta],
        nested_function_names: &[String],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut nv_structs: Vec<String> = Vec::new();
        let mut nv_fields: Vec<String> = Vec::new();
        for block in blocks {
            for prop in block_properties(block.node) {
                if !prop_has_name_value(prop) {
                    continue;
                }
                if let Some(struct_name) = prop_name_value_struct(prop, source) {
                    if !nv_structs.contains(&struct_name) {
                        nv_structs.push(struct_name);
                    }
                }
                if let Some(field) = prop_name_value_field(prop, source) {
                    if !nv_fields.contains(&field) {
                        nv_fields.push(field);
                    }
                }
            }
        }

        self.check_fvtinvaldim_ttoofewdims(blocks, source, diagnostics);
        self.check_fvbtn(blocks, source, diagnostics);
        self.check_fvnsc(blocks, nested_function_names, source, diagnostics);
        self.check_fvond(blocks, &nv_structs, source, diagnostics);
        self.check_fvonv(blocks, &nv_fields, source, diagnostics);
        self.check_fvvin_fvvcon_fvubd(blocks, source, diagnostics);
        self.check_fvvin_fvocon(blocks, source, diagnostics);
    }

    /// TINVALDIM / TTOOFEWDIMS: size constraints must have at least two
    /// nonnegative integer (or colon) dimensions.
    fn check_fvtinvaldim_ttoofewdims(
        &self,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let invalid = self.is_check_enabled("TINVALDIM");
        let too_few = self.is_check_enabled("TTOOFEWDIMS");
        if !invalid && !too_few {
            return;
        }
        for block in blocks {
            for prop in block_properties(block.node) {
                let Some(dims) = find_child_kind(prop, "dimensions") else {
                    continue;
                };
                let mut entries = Vec::new();
                let mut cursor = dims.walk();
                for child in dims.children(&mut cursor) {
                    match child.kind() {
                        "(" | ")" | "," => {}
                        _ => entries.push(child),
                    }
                }
                if too_few && entries.len() < 2 {
                    self.push_diag(
                        dims,
                        "TTOOFEWDIMS",
                        "Specify at least two dimensions for size.",
                        diagnostics,
                    );
                }
                if invalid {
                    for entry in &entries {
                        let valid = match entry.kind() {
                            "spread_operator" => true,
                            "number" => node_text(*entry, source)
                                .parse::<f64>()
                                .map(|v| v >= 0.0 && v.fract() == 0.0)
                                .unwrap_or(false),
                            _ => false,
                        };
                        if !valid {
                            self.push_diag(
                                dims,
                                "TINVALDIM",
                                "Each dimension must be a nonnegative integer number or a colon.",
                                diagnostics,
                            );
                            break;
                        }
                    }
                }
            }
        }
    }

    /// FVBTN: Banned functions are not supported in arguments blocks.
    fn check_fvbtn(
        &self,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVBTN") {
            return;
        }
        for block in blocks {
            for prop in block_properties(block.node) {
                let mut calls = Vec::new();
                collect_function_calls(prop, &mut calls);
                for call in calls {
                    let Some(callee) = callee_name(call, source) else {
                        continue;
                    };
                    if BANNED_ARGUMENTS_FUNCTIONS.iter().any(|f| *f == callee) {
                        self.push_diag(
                            call,
                            "FVBTN",
                            "Use of this function is not supported in arguments blocks.",
                            diagnostics,
                        );
                    }
                }
            }
        }
    }

    /// FVNSC: Calling nested functions is not supported in arguments blocks.
    fn check_fvnsc(
        &self,
        blocks: &[ArgumentsBlockMeta],
        nested_function_names: &[String],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVNSC") {
            return;
        }
        if nested_function_names.is_empty() {
            return;
        }
        for block in blocks {
            for prop in block_properties(block.node) {
                let mut calls = Vec::new();
                collect_function_calls(prop, &mut calls);
                for call in calls {
                    let Some(callee) = callee_name(call, source) else {
                        continue;
                    };
                    if nested_function_names.iter().any(|n| n == &callee) {
                        self.push_diag(
                            call,
                            "FVNSC",
                            "Use of nested functions is not supported in arguments blocks.",
                            diagnostics,
                        );
                    }
                }
            }
        }
    }

    /// FVOND: Use of name-value arguments in default values is not supported.
    fn check_fvond(
        &self,
        blocks: &[ArgumentsBlockMeta],
        nv_structs: &[String],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVOND") {
            return;
        }
        for block in blocks {
            for prop in block_properties(block.node) {
                let Some(default_node) = find_child_kind(prop, "default_value") else {
                    continue;
                };
                if default_refs_name_value(default_node, source, nv_structs) {
                    self.push_diag(
                        prop,
                        "FVOND",
                        "Use of name-value arguments in default values is not supported.",
                        diagnostics,
                    );
                }
            }
        }
    }

    /// FVONV: Name-value arguments must use the dotted name in validation.
    fn check_fvonv(
        &self,
        blocks: &[ArgumentsBlockMeta],
        nv_fields: &[String],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled("FVONV") {
            return;
        }
        for block in blocks {
            for prop in block_properties(block.node) {
                for call in prop_validator_calls(prop, source) {
                    let uses_bare_field = call.references.iter().any(|r| {
                        matches!(
                            r,
                            CallReference::Identifier(name)
                                if nv_fields.iter().any(|f| f == name)
                        )
                    });
                    if uses_bare_field {
                        self.push_diag(
                            prop,
                            "FVONV",
                            "Use of name-value arguments without dotted name in the validation is not supported.",
                            diagnostics,
                        );
                    }
                }
            }
        }
    }

    /// FVVIN / FVVCON / FVUBD: input validation function rules.
    fn check_fvvin_fvvcon_fvubd(
        &self,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let vin = self.is_check_enabled("FVVIN");
        let vcon = self.is_check_enabled("FVVCON");
        let ubd = self.is_check_enabled("FVUBD");
        if !vin && !vcon && !ubd {
            return;
        }

        let mut input_props: Vec<(tree_sitter::Node, String)> = Vec::new();
        for block in blocks {
            if !(block.role.is_input() || block.is_input_repeating()) {
                continue;
            }
            for prop in block_properties(block.node) {
                if prop_has_ignored(prop) || prop_has_name_value(prop) {
                    continue;
                }
                if let Some(name) = prop_plain_name(prop, source) {
                    if name != "varargin" {
                        input_props.push((prop, name));
                    }
                }
            }
        }

        for (index, (prop, name)) in input_props.iter().enumerate() {
            let calls = prop_validator_calls(*prop, source);
            if calls.is_empty() {
                continue;
            }
            let previously_declared: Vec<&String> =
                input_props[..index].iter().map(|(_, n)| n).collect();
            let declared_later: Vec<&String> =
                input_props[index + 1..].iter().map(|(_, n)| n).collect();
            for call in &calls {
                let uses_arg = call
                    .references
                    .iter()
                    .any(|r| matches!(r, CallReference::Identifier(n) if n == name));
                if vin && !uses_arg {
                    self.push_diag(
                        *prop,
                        "FVVIN",
                        "Validation function must use the argument as an input.",
                        diagnostics,
                    );
                }
                for reference in &call.references {
                    match reference {
                        CallReference::Identifier(ref_name) => {
                            if ref_name == name {
                                continue;
                            }
                            if ubd && declared_later.contains(&ref_name) {
                                self.push_diag(
                                    *prop,
                                    "FVUBD",
                                    "Argument is referenced before it is declared in the arguments block.",
                                    diagnostics,
                                );
                            }
                            if vcon && !previously_declared.contains(&ref_name) {
                                self.push_diag(
                                    *prop,
                                    "FVVCON",
                                    "For input arguments, validation functions must only use previously declared positional arguments, the argument being validated, or literals.",
                                    diagnostics,
                                );
                            }
                        }
                        CallReference::Field => {
                            if vcon {
                                self.push_diag(
                                    *prop,
                                    "FVVCON",
                                    "For input arguments, validation functions must only use previously declared positional arguments, the argument being validated, or literals.",
                                    diagnostics,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    /// FVVIN / FVOCON: output validation function rules.
    fn check_fvvin_fvocon(
        &self,
        blocks: &[ArgumentsBlockMeta],
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let vin = self.is_check_enabled("FVVIN");
        let ocon = self.is_check_enabled("FVOCON");
        if !vin && !ocon {
            return;
        }
        for block in blocks {
            if !(block.role.is_output() || block.is_output_repeating()) {
                continue;
            }
            for prop in block_properties(block.node) {
                if prop_has_ignored(prop) || prop_has_name_value(prop) {
                    continue;
                }
                let Some(name) = prop_plain_name(prop, source) else {
                    continue;
                };
                for call in prop_validator_calls(prop, source) {
                    let uses_arg = call
                        .references
                        .iter()
                        .any(|r| matches!(r, CallReference::Identifier(n) if n == &name));
                    if vin && !uses_arg {
                        self.push_diag(
                            prop,
                            "FVVIN",
                            "Validation function must use the argument as an input.",
                            diagnostics,
                        );
                    }
                    for reference in &call.references {
                        let is_arg = matches!(reference, CallReference::Identifier(n) if n == &name);
                        if ocon && !is_arg {
                            self.push_diag(
                                prop,
                                "FVOCON",
                                "For output arguments, validation functions must only use the argument being validated or literals.",
                                diagnostics,
                            );
                        }
                    }
                }
            }
        }
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

        // USESWNS: Variable must be explicitly defined before first use
        self.check_useswns(symbol_table, ctx, diagnostics);
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
        Self::find_gp_in_nested(root, 0, diagnostics);
    }

    /// Recursively find global/persistent in nested functions.
    fn find_gp_in_nested(
        node: tree_sitter::Node,
        depth: usize,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "function_definition" {
                if depth > 0 {
                    // This is a nested function; check for global/persistent
                    Self::find_gp_statements(child, diagnostics);
                }
                Self::find_gp_in_nested(child, depth + 1, diagnostics);
            } else if child.kind() != "methods" && child.kind() != "class_definition" {
                Self::find_gp_in_nested(child, depth, diagnostics);
            }
        }
    }

    /// Find global/persistent statements within a function definition.
    fn find_gp_statements(
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
                    Self::find_gp_statements(child, diagnostics);
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
        Self::check_break_continue_dfs(root, false, diagnostics);
    }

    /// DFS to find break/continue outside loops.
    fn check_break_continue_dfs(
        node: tree_sitter::Node,
        in_loop: bool,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        match node.kind() {
            "for_statement" | "while_statement" => {
                // Inside a loop now
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    Self::check_break_continue_dfs(child, true, diagnostics);
                }
                return;
            }
            "function_definition" => {
                // Reset loop context for new function scope
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    Self::check_break_continue_dfs(child, false, diagnostics);
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
            Self::check_break_continue_dfs(child, in_loop, diagnostics);
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
        Self::check_end_dfs(root, false, diagnostics);
    }

    /// DFS to find `end` keywords used as values outside of indexing contexts.
    fn check_end_dfs(
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
                    Self::check_end_dfs(child, true, diagnostics);
                }
                return;
            }
            "cell_index" => {
                // Cell indexing — end is valid here
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    Self::check_end_dfs(child, true, diagnostics);
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
            Self::check_end_dfs(child, in_index, diagnostics);
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
// C4 checks: spmd transparency, error/warning messages, construction, spmd bounds
// ---------------------------------------------------------------------------

impl LanguageSpecEngine {
    /// Collect the names of all nested functions in the file.
    ///
    /// Nested functions are `function_definition` nodes whose ancestor is another
    /// `function_definition`. Methods in `methods` blocks are not nested functions.
    fn collect_nested_function_names(&self, root: tree_sitter::Node, source: &str) -> HashSet<String> {
        let mut names = HashSet::new();
        Self::collect_nested_functions_dfs(root, false, source, &mut names);
        names
    }

    /// DFS helper for [`Self::collect_nested_function_names`].
    fn collect_nested_functions_dfs(
        node: tree_sitter::Node,
        inside_function: bool,
        source: &str,
        names: &mut HashSet<String>,
    ) {
        if node.kind() == "function_definition" {
            if inside_function {
                if let Some(name_node) = node.child_by_field_name("name") {
                    names.insert(node_text(name_node, source).to_string());
                }
            }
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                Self::collect_nested_functions_dfs(child, true, source, names);
            }
            return;
        }
        if node.kind() == "class_definition" || node.kind() == "methods" {
            // Methods are not nested functions; do not mark them as such.
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                Self::collect_nested_functions_dfs(child, inside_function, source, names);
            }
            return;
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_nested_functions_dfs(child, inside_function, source, names);
        }
    }

    /// SPDEC / SPDEC3: validate the worker bounds in an `spmd (n)` or `spmd (m, n)` header.
    ///
    /// Fires SPDEC when a bound is a negative literal or a non-integer literal, and
    /// SPDEC3 when more than two bounds are given.
    fn check_spmd_worker_bounds(
        &self,
        node: tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut in_parens = false;
        let mut named_bound_count = 0usize;
        let mut bounds: Vec<tree_sitter::Node> = Vec::new();
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let text = node_text(child, source);
            if text == "(" {
                in_parens = true;
                continue;
            }
            if text == ")" {
                break;
            }
            if in_parens && child.is_named() {
                named_bound_count += 1;
                if child.kind() == "number" || child.kind() == "unary_operator" {
                    bounds.push(child);
                }
            }
        }
        if named_bound_count == 0 {
            return;
        }
        if named_bound_count >= 3 {
            let pos = node.start_position();
            diagnostics.push(Diagnostic {
                rule_id: "SPDEC3",
                message: "An SPMD block can only specify a lower and upper bound for the number of workers"
                    .to_string(),
                severity: Severity::Error,
                byte_range: node.start_byte()..node.end_byte(),
                line: pos.row + 1,
                column: pos.column + 1,
                fix: None,
            });
        }
        for bound in bounds {
            let text = node_text(bound, source);
            let is_negative = text.trim_start().starts_with('-');
            let is_fractional = text.contains('.');
            if is_negative || is_fractional {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    rule_id: "SPDEC",
                    message: "The bounds on the number of workers an SPMD block can use must be a nonnegative integer"
                        .to_string(),
                    severity: Severity::Error,
                    byte_range: node.start_byte()..node.end_byte(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
                break;
            }
        }
    }

    /// SPEVC / SPLD / SPSV / SPWHOS / SPBFN / SPNF: workspace-transparency checks
    /// for `function_call` nodes inside an spmd block.
    fn check_spmd_function_call(
        &self,
        node: tree_sitter::Node,
        source: &str,
        nested_functions: &HashSet<String>,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let Some(name) = callee_name(node, source) else {
            return;
        };
        match name.as_str() {
            "evalin" | "assignin" => {
                let scope = first_string_argument(node, source);
                if scope.as_deref() == Some("caller") {
                    self.push_diag(node, "SPEVC", "EVALIN('caller') and ASSIGNIN('caller') are invalid inside of an SPMD block", diagnostics);
                } else {
                    self.push_diag(node, "SPBFN", "Use of this function is invalid inside an SPMD block because it accesses or modifies the workspace in a non-transparent way", diagnostics);
                }
            }
            "eval" => {
                self.push_diag(node, "SPBFN", "Use of this function is invalid inside an SPMD block because it accesses or modifies the workspace in a non-transparent way", diagnostics);
            }
            "load" => {
                if !is_assignment_rhs(node) {
                    self.push_diag(node, "SPLD", "To avoid a transparency violation, assign the output of LOAD to a variable in SPMD blocks", diagnostics);
                }
            }
            "save" => {
                if !has_string_argument(node, source, "-fromstruct") {
                    self.push_diag(node, "SPSV", "SAVE cannot be called in an SPMD block without the '-fromstruct' option", diagnostics);
                }
            }
            "who" | "whos" => {
                if !has_string_argument(node, source, "-file") {
                    self.push_diag(node, "SPWHOS", "Using \"who\" or \"whos\" without \"-file\" is invalid inside an SPMD block", diagnostics);
                }
            }
            other => {
                if nested_functions.contains(other) {
                    self.push_diag(
                        node,
                        "SPNF",
                        &format!(
                            "The nested function {other} cannot be called from within an SPMD block"
                        ),
                        diagnostics,
                    );
                }
            }
        }
    }

    /// SPLD / SPSV / SPWHOS: command-form transparency checks inside an spmd block.
    fn check_spmd_command(
        &self,
        node: tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let mut command_name: Option<String> = None;
        let mut arguments: Vec<String> = Vec::new();
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "command_name" => {
                    command_name = Some(node_text(child, source).to_string());
                }
                "command_argument" => {
                    arguments.push(node_text(child, source).to_string());
                }
                _ => {}
            }
        }
        let Some(name) = command_name else {
            return;
        };
        match name.as_str() {
            "load" => {
                self.push_diag(node, "SPLD", "To avoid a transparency violation, assign the output of LOAD to a variable in SPMD blocks", diagnostics);
            }
            "save" => {
                if !arguments.iter().any(|a| a == "-fromstruct") {
                    self.push_diag(node, "SPSV", "SAVE cannot be called in an SPMD block without the '-fromstruct' option", diagnostics);
                }
            }
            "who" | "whos" => {
                if !arguments.iter().any(|a| a == "-file") {
                    self.push_diag(node, "SPWHOS", "Using \"who\" or \"whos\" without \"-file\" is invalid inside an SPMD block", diagnostics);
                }
            }
            _ => {}
        }
    }

    /// ERTXT / WTXT: `error`/`warning` called with a single string argument that
    /// is a message identifier (contains `:`) and no message text.
    fn check_error_warning_message(
        &self,
        node: tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let Some(name) = callee_name(node, source) else {
            return;
        };
        let (rule_id, message) = match name.as_str() {
            "error" => (
                "ERTXT",
                "Specify an error message with the message identifier",
            ),
            "warning" => (
                "WTXT",
                "Specify a warning message with the message identifier",
            ),
            _ => return,
        };
        let args = argument_nodes(node);
        if args.len() != 1 {
            return;
        }
        let Some(content) = string_content(args[0], source) else {
            return;
        };
        if content.contains(':') {
            self.push_diag(node, rule_id, message, diagnostics);
        }
    }

    /// NCHKOS: `narginchk`/`nargoutchk` do not return values, so using them on the
    /// right-hand side of an assignment is an error.
    fn check_nchkos_output_use(
        &self,
        node: tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let Some(name) = callee_name(node, source) else {
            return;
        };
        if name != "narginchk" && name != "nargoutchk" {
            return;
        }
        if is_assignment_rhs(node) {
            self.push_diag(
                node,
                "NCHKOS",
                &format!("{name} does not return any values"),
                diagnostics,
            );
        }
    }

    /// CTOINE: a constructed object of a class used as an input to that class's
    /// constructor (e.g., `obj = Foo(Foo(1))` inside the `Foo` constructor).
    fn check_ctoine(
        &self,
        class: &ClassMeta,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        Self::check_ctoine_dfs(ctx.tree.root_node(), &class.name, ctx.source, diagnostics);
    }

    /// DFS helper for [`Self::check_ctoine`].
    fn check_ctoine_dfs(
        node: tree_sitter::Node,
        class_name: &str,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "function_call" {
            if let Some(name) = callee_name(node, source) {
                if name == class_name {
                    for arg in argument_nodes(node) {
                        if arg.kind() == "function_call" {
                            if let Some(inner) = callee_name(arg, source) {
                                if inner == class_name {
                                    let pos = arg.start_position();
                                    diagnostics.push(Diagnostic {
                                        rule_id: "CTOINE",
                                        message: "Use of constructed object as input to constructor is not supported"
                                            .to_string(),
                                        severity: Severity::Error,
                                        byte_range: arg.start_byte()..arg.end_byte(),
                                        line: pos.row + 1,
                                        column: pos.column + 1,
                                        fix: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::check_ctoine_dfs(child, class_name, source, diagnostics);
        }
    }

    /// CTORO: class constructors must declare at least one output argument.
    fn check_ctoro(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        for mb in &class.methods_blocks {
            for func in &mb.methods {
                if func.is_constructor && func.outputs.is_empty() {
                    diagnostics.push(Diagnostic {
                        rule_id: "CTORO",
                        message: "Class constructors must be declared with at least one output argument"
                            .to_string(),
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

    /// MHERIT: deriving from certain built-in classes is not supported.
    fn check_mherit(&self, class: &ClassMeta, diagnostics: &mut Vec<Diagnostic>) {
        for sc in &class.superclasses {
            if NON_SUBCLASSABLE_BUILTINS.contains(&sc.as_str()) {
                diagnostics.push(Diagnostic {
                    rule_id: "MHERIT",
                    message: format!(
                        "Deriving from the built-in MATLAB {sc} class is not supported"
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

    /// USESWNS: in scripts, a variable must be explicitly defined before first use.
    ///
    /// Overlaps with the Warning-severity `SUSENS` from the unset-variables engine;
    /// this is the Error-severity language-specification variant. Users can disable
    /// either engine via configuration.
    fn check_useswns(
        &self,
        symbol_table: &SymbolTable,
        ctx: &FileContext,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let root_scope = symbol_table.root_scope();
        let skipped = Self::collect_function_name_starts(ctx.tree.root_node());
        let mut reported: HashSet<String> = HashSet::new();
        for use_ in &root_scope.uses {
            if skipped.contains(&use_.byte_range.start) {
                continue;
            }
            if reported.contains(&use_.name) {
                continue;
            }
            let earliest_def = root_scope
                .defs
                .iter()
                .filter(|d| d.name == use_.name)
                .min_by_key(|d| d.byte_range.start);
            if let Some(def) = earliest_def {
                if use_.byte_range.start < def.byte_range.start {
                    reported.insert(use_.name.clone());
                    diagnostics.push(Diagnostic {
                        rule_id: "USESWNS",
                        message: format!(
                            "Variable '{}' must be explicitly defined before first use",
                            use_.name
                        ),
                        severity: Severity::Error,
                        byte_range: use_.byte_range.clone(),
                        line: use_.line,
                        column: use_.column,
                        fix: None,
                    });
                }
            }
        }
    }

    /// Collect the byte offsets of identifiers that name functions (function-call
    /// callees and command names) so they are not treated as variable uses.
    fn collect_function_name_starts(root: tree_sitter::Node) -> HashSet<usize> {
        let mut starts = HashSet::new();
        Self::collect_function_name_starts_dfs(root, &mut starts);
        starts
    }

    /// DFS helper for [`Self::collect_function_name_starts`].
    fn collect_function_name_starts_dfs(
        node: tree_sitter::Node,
        starts: &mut HashSet<usize>,
    ) {
        match node.kind() {
            "function_call" => {
                if let Some(callee) = callee_node(node) {
                    starts.insert(callee.start_byte());
                }
            }
            "command" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "command_name" {
                        starts.insert(child.start_byte());
                    }
                }
            }
            _ => {}
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_function_name_starts_dfs(child, starts);
        }
    }

    /// Push a diagnostic at the start of a node.
    fn push_diag(
        &self,
        node: tree_sitter::Node,
        rule_id: &'static str,
        message: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !self.is_check_enabled(rule_id) {
            return;
        }
        let pos = node.start_position();
        diagnostics.push(Diagnostic {
            rule_id,
            message: message.to_string(),
            severity: Severity::Error,
            byte_range: node.start_byte()..node.end_byte(),
            line: pos.row + 1,
            column: pos.column + 1,
            fix: None,
        });
    }
}

/// Built-in MATLAB classes that cannot be subclassed directly.
const NON_SUBCLASSABLE_BUILTINS: &[&str] = &[
    "double", "single", "int8", "int16", "int32", "int64", "uint8", "uint16", "uint32",
    "uint64", "char", "logical", "cell", "struct",
];

/// Get the callee name of a `function_call` node.
fn callee_name(node: tree_sitter::Node, source: &str) -> Option<String> {
    callee_node(node).map(|n| node_text(n, source).to_string())
}

/// Get the callee identifier node of a `function_call`.
fn callee_node(node: tree_sitter::Node) -> Option<tree_sitter::Node> {
    node.child_by_field_name("name")
        .or_else(|| node.child(0))
        .filter(|n| n.kind() == "identifier")
}

/// Get the named argument nodes of a `function_call` (excluding commas).
fn argument_nodes(node: tree_sitter::Node) -> Vec<tree_sitter::Node> {
    let mut cursor = node.walk();
    let mut result = Vec::new();
    for child in node.children(&mut cursor) {
        if child.kind() == "arguments" {
            let mut inner = child.walk();
            for arg in child.children(&mut inner) {
                if arg.is_named() && arg.kind() != "," {
                    result.push(arg);
                }
            }
            break;
        }
    }
    result
}

/// Extract the content of a string argument node (strips the surrounding quotes).
fn string_content(node: tree_sitter::Node, source: &str) -> Option<String> {
    if node.kind() != "string" {
        return None;
    }
    let text = node_text(node, source);
    let trimmed = text.trim();
    if trimmed.len() < 2 {
        return None;
    }
    let content = trimmed[1..trimmed.len() - 1].to_string();
    Some(content)
}

/// Get the first string argument of a `function_call` as its content.
fn first_string_argument(node: tree_sitter::Node, source: &str) -> Option<String> {
    argument_nodes(node)
        .first()
        .and_then(|arg| string_content(*arg, source))
}

/// Check whether any string argument of a `function_call` has the given content.
fn has_string_argument(node: tree_sitter::Node, source: &str, needle: &str) -> bool {
    argument_nodes(node)
        .iter()
        .filter_map(|arg| string_content(*arg, source))
        .any(|content| content.starts_with(needle))
}

/// Check whether a `function_call` is the right-hand side of an assignment.
fn is_assignment_rhs(node: tree_sitter::Node) -> bool {
    node.parent()
        .map(|parent| {
            parent.kind() == "assignment"
                && parent
                    .child_by_field_name("right")
                    .map(|right| right.id() == node.id())
                    .unwrap_or(false)
        })
        .unwrap_or(false)
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

    // ===== Function validation checks (FV* / TIN* / TTOO* group) =====

    const FV_CHECK_IDS: &[&str] = &[
        "FVAPN", "FVATF", "FVBTN", "FVDAN", "FVDAP", "FVDNF", "FVDREP", "FVIDV",
        "FVIOA", "FVMCL", "FVNDE", "FVNIV", "FVNREP", "FVNSC", "FVNVL", "FVOBI",
        "FVOCON", "FVOND", "FVONV", "FVOOD", "FVOOI", "FVOON", "FVORDI", "FVORDN",
        "FVORDO", "FVORDP", "FVORM", "FVOVREP", "FVREPD", "FVREPO", "FVSOR",
        "FVSORO", "FVUBD", "FVVCON", "FVVIN", "FVVREP", "TINVALDIM", "TTOOFEWDIMS",
    ];

    #[test]
    fn test_fv_no_fire_valid_arguments_block() {
        let source = "\
function y = f(x)
    arguments
        x (1,1) double {mustBeReal}
    end
    y = x;
end
";
        let diags = check_source(source, "f.m");
        for id in FV_CHECK_IDS {
            let hits = filter_by_id(&diags, id);
            assert!(
                hits.is_empty(),
                "{id} should NOT fire on a valid arguments block, got: {}",
                hits.len()
            );
        }
    }

    #[test]
    fn test_fv_blocks_extract_roles() {
        let source = "\
function y = f(x)
    arguments (Input)
        x (1,1) double
    end
    arguments (Output)
        y (1,1) double
    end
end
";
        let tree = parse_matlab(source);
        let root = tree.root_node();
        let func_node = root.child(0).unwrap();
        let blocks = extract_arguments_blocks_grouped(func_node, source);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].role, BlockRole::Input);
        assert_eq!(blocks[0].attributes, vec!["Input".to_string()]);
        assert_eq!(blocks[1].role, BlockRole::Output);
        assert_eq!(blocks[1].attributes, vec!["Output".to_string()]);
        assert_eq!(blocks[0].args.len(), 1);
        assert_eq!(blocks[0].args[0].name, "x");
        assert_eq!(blocks[1].args[0].name, "y");
    }

    #[test]
    fn test_fvapn_fires_name_value_before_positional() {
        let source = "\
function f(opts, b)
    arguments
        opts.Name = 'x'
        b (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVAPN").is_empty(),
            "FVAPN should fire when name=value name-value argument precedes a positional"
        );
    }

    #[test]
    fn test_fvapn_no_fire_name_value_last() {
        let source = "\
function f(opts)
    arguments
        opts.Name = 'x'
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVAPN").is_empty(),
            "FVAPN should NOT fire when name-value arguments are last"
        );
    }

    #[test]
    fn test_fvatf_fires_attribute_value() {
        let source = "\
function f(a)
    arguments (foo = 1)
        a (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVATF").is_empty(),
            "FVATF should fire for attribute values in arguments blocks"
        );
    }

    #[test]
    fn test_fvatf_no_fire_bare_attributes() {
        let source = "\
function f(a)
    arguments (Input)
        a (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVATF").is_empty(),
            "FVATF should NOT fire for bare attributes"
        );
    }

    #[test]
    fn test_fvbtn_fires_banned_function() {
        let source = "\
function f(a)
    arguments
        a (1,1) double {mustBeReal(eval('x'))}
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVBTN").is_empty(),
            "FVBTN should fire for eval in an arguments block"
        );
    }

    #[test]
    fn test_fvbtn_no_fire_validator() {
        let source = "\
function f(a)
    arguments
        a (1,1) double {mustBeReal}
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVBTN").is_empty(),
            "FVBTN should NOT fire for ordinary validators"
        );
    }

    #[test]
    fn test_fvdan_fires_shared_name() {
        let source = "\
function f(x, y)
    arguments
        y.Name = 'x'
        y (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVDAN").is_empty(),
            "FVDAN should fire when a name-value structure shares a name with a positional argument"
        );
    }

    #[test]
    fn test_fvdan_no_fire_distinct_names() {
        let source = "\
function f(opts, b)
    arguments
        opts.Name = 'x'
        b (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVDAN").is_empty(),
            "FVDAN should NOT fire for distinct struct and positional names"
        );
    }

    #[test]
    fn test_fvdap_fires_duplicate_positional() {
        let source = "\
function f(a, b)
    arguments
        a (1,1) double
        a (1,1) double = 2
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVDAP").is_empty(),
            "FVDAP should fire when a positional argument is declared twice"
        );
    }

    #[test]
    fn test_fvdap_no_fire_unique_positionals() {
        let source = "\
function f(a, b)
    arguments
        a (1,1) double
        b (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVDAP").is_empty(),
            "FVDAP should NOT fire for unique positional arguments"
        );
    }

    #[test]
    fn test_fvdnf_fires_duplicate_name_value() {
        let source = "\
function f(opts)
    arguments
        opts.Name = 'a'
        opts.Name = 'b'
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVDNF").is_empty(),
            "FVDNF should fire when a name-value argument is declared twice"
        );
    }

    #[test]
    fn test_fvdnf_no_fire_unique_name_values() {
        let source = "\
function f(opts)
    arguments
        opts.Name1 = 'a'
        opts.Name2 = 'b'
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVDNF").is_empty(),
            "FVDNF should NOT fire for unique name-value arguments"
        );
    }

    #[test]
    fn test_fvdrep_fires_multiple_repeating() {
        let source = "\
function f(a2, a3)
    arguments (Repeating)
        a2
    end
    arguments (Repeating)
        a3
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVDREP").is_empty(),
            "FVDREP should fire for multiple Repeating blocks"
        );
    }

    #[test]
    fn test_fvdrep_no_fire_single_repeating() {
        let source = "\
function f(a2)
    arguments (Repeating)
        a2
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVDREP").is_empty(),
            "FVDREP should NOT fire for a single Repeating block"
        );
    }

    #[test]
    fn test_fvidv_fires_validation_on_ignored() {
        let source = "\
function f(a, ~)
    arguments
        a (1,1) double
        ~ (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVIDV").is_empty(),
            "FVIDV should fire for validation on an ignored argument"
        );
    }

    #[test]
    fn test_fvidv_no_fire_bare_ignored() {
        let source = "\
function f(a, ~)
    arguments
        a (1,1) double
        ~
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVIDV").is_empty(),
            "FVIDV should NOT fire for a bare ignored argument"
        );
    }

    #[test]
    fn test_fvioa_fires_both_attributes() {
        let source = "\
function [a] = f(a)
    arguments (Input, Output)
        a (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVIOA").is_empty(),
            "FVIOA should fire when a block has both Input and Output attributes"
        );
    }

    #[test]
    fn test_fvioa_no_fire_single_attribute() {
        let source = "\
function f(a)
    arguments (Input)
        a (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVIOA").is_empty(),
            "FVIOA should NOT fire for a single attribute"
        );
    }

    #[test]
    fn test_fvmcl_fires_multiple_class_properties() {
        let source = "\
function f(pa, pb)
    arguments
        pa.?matlab.graphics.chart.primitive.Bar
        pb.?matlab.graphics.chart.primitive.Line
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVMCL").is_empty(),
            "FVMCL should fire when .? syntax is used for multiple name-value structures"
        );
    }

    #[test]
    fn test_fvmcl_no_fire_single_class_property() {
        let source = "\
function f(pa)
    arguments
        pa.?matlab.graphics.chart.primitive.Bar
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVMCL").is_empty(),
            "FVMCL should NOT fire for a single .? declaration"
        );
    }

    #[test]
    fn test_fvnde_fires_class_name_default() {
        let source = "\
function f(opts)
    arguments
        opts.double = 3
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVNDE").is_empty(),
            "FVNDE should fire for a default value on a class-name name-value argument"
        );
    }

    #[test]
    fn test_fvnde_no_fire_plain_name_value() {
        let source = "\
function f(opts)
    arguments
        opts.Name = 3
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVNDE").is_empty(),
            "FVNDE should NOT fire for a regular name-value argument"
        );
    }

    #[test]
    fn test_fvniv_fires_not_an_input() {
        let source = "\
function f(a)
    arguments
        z (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVNIV").is_empty(),
            "FVNIV should fire for an argument that is not a function input"
        );
    }

    #[test]
    fn test_fvniv_no_fire_declared_input() {
        let source = "\
function f(a)
    arguments
        a (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVNIV").is_empty(),
            "FVNIV should NOT fire for a declared input"
        );
    }

    #[test]
    fn test_fvnrep_fires_name_value_in_repeating() {
        let source = "\
function f(opts)
    arguments (Repeating)
        opts.Name
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVNREP").is_empty(),
            "FVNREP should fire for a name-value argument in a Repeating block"
        );
    }

    #[test]
    fn test_fvnrep_no_fire_name_value_normal() {
        let source = "\
function f(opts)
    arguments
        opts.Name = 'x'
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVNREP").is_empty(),
            "FVNREP should NOT fire for a name-value argument in a normal block"
        );
    }

    #[test]
    fn test_fvnsc_fires_nested_function_call() {
        let source = "\
function outer(a)
    arguments
        a (1,1) double = helper(3)
    end
    b = helper(4);
    function y = helper(x)
        y = x * 2;
    end
end
";
        let diags = check_source(source, "outer.m");
        assert!(
            !filter_by_id(&diags, "FVNSC").is_empty(),
            "FVNSC should fire for a nested function call in an arguments block"
        );
    }

    #[test]
    fn test_fvnsc_no_fire_builtin_call() {
        let source = "\
function f(a)
    arguments
        a (1,1) double = rand(3)
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVNSC").is_empty(),
            "FVNSC should NOT fire for a builtin function call in an arguments block"
        );
    }

    #[test]
    fn test_fvnvl_fires_class_name_validation() {
        let source = "\
function f(opts)
    arguments
        opts.double {mustBeReal}
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVNVL").is_empty(),
            "FVNVL should fire for validation on a class-name name-value argument"
        );
    }

    #[test]
    fn test_fvnvl_no_fire_plain_name_value_validation() {
        let source = "\
function f(opts)
    arguments
        opts.Name {mustBeReal}
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVNVL").is_empty(),
            "FVNVL should NOT fire for a regular name-value argument with validation"
        );
    }

    #[test]
    fn test_fvobi_fires_output_before_input() {
        let source = "\
function y = f(x)
    arguments (Output)
        y (1,1) double
    end
    arguments (Input)
        x (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVOBI").is_empty(),
            "FVOBI should fire when an output block precedes an input block"
        );
    }

    #[test]
    fn test_fvobi_no_fire_input_before_output() {
        let source = "\
function y = f(x)
    arguments (Input)
        x (1,1) double
    end
    arguments (Output)
        y (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVOBI").is_empty(),
            "FVOBI should NOT fire when input blocks precede output blocks"
        );
    }

    #[test]
    fn test_fvocon_fires_output_validator_using_input() {
        let source = "\
function y = f(x)
    arguments (Output)
        y (1,1) double {mustBeGreaterThan(y, x)}
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVOCON").is_empty(),
            "FVOCON should fire when an output validator references an input argument"
        );
    }

    #[test]
    fn test_fvocon_no_fire_output_validator_self_literal() {
        let source = "\
function y = f(x)
    arguments (Output)
        y (1,1) double {mustBeGreaterThan(y, 0)}
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVOCON").is_empty(),
            "FVOCON should NOT fire when an output validator only uses the argument or literals"
        );
    }

    #[test]
    fn test_fvond_fires_name_value_in_default() {
        let source = "\
function f(opts, y)
    arguments
        opts.Name = 3
        y (1,1) double = opts.Name
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVOND").is_empty(),
            "FVOND should fire when a default value uses a name-value argument"
        );
    }

    #[test]
    fn test_fvond_no_fire_default_uses_positional() {
        let source = "\
function f(a, y)
    arguments
        a (1,1) double
        y (1,1) double = a + 1
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVOND").is_empty(),
            "FVOND should NOT fire when a default value uses a positional argument"
        );
    }

    #[test]
    fn test_fvonv_fires_bare_name_value_field() {
        let source = "\
function f(opts, y)
    arguments
        opts.Name {mustBeReal}
        y (1,1) double {mustBeEqual(y, Name)}
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVONV").is_empty(),
            "FVONV should fire when validation uses a name-value field without the dotted name"
        );
    }

    #[test]
    fn test_fvonv_no_fire_dotted_name_value() {
        let source = "\
function f(opts)
    arguments
        opts.Name {mustBeMember(opts.Name, {'a','b'})}
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVONV").is_empty(),
            "FVONV should NOT fire when validation uses the dotted name-value name"
        );
    }

    #[test]
    fn test_fvood_fires_output_default() {
        let source = "\
function y = f(x)
    arguments (Output)
        y (1,1) double = 5
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVOOD").is_empty(),
            "FVOOD should fire for a default value on an output argument"
        );
    }

    #[test]
    fn test_fvood_no_fire_output_without_default() {
        let source = "\
function y = f(x)
    arguments (Output)
        y (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVOOD").is_empty(),
            "FVOOD should NOT fire for an output argument without a default"
        );
    }

    #[test]
    fn test_fvooi_fires_ignored_in_output() {
        let source = "\
function [a, ~] = f(x)
    arguments (Output)
        a (1,1) double
        ~
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVOOI").is_empty(),
            "FVOOI should fire for an ignored argument in an output block"
        );
    }

    #[test]
    fn test_fvooi_no_fire_clean_output() {
        let source = "\
function [a] = f(x)
    arguments (Output)
        a (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVOOI").is_empty(),
            "FVOOI should NOT fire for a clean output block"
        );
    }

    #[test]
    fn test_fvoon_fires_name_value_output() {
        let source = "\
function [a] = f(x)
    arguments (Output)
        opts.Name
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVOON").is_empty(),
            "FVOON should fire for a name-value argument in an output block"
        );
    }

    #[test]
    fn test_fvoon_no_fire_positional_output() {
        let source = "\
function [a] = f(x)
    arguments (Output)
        a (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVOON").is_empty(),
            "FVOON should NOT fire for a positional output argument"
        );
    }

    #[test]
    fn test_fvordi_fires_ignored_after_name_value() {
        let source = "\
function f(opts, ~)
    arguments
        opts.Name = 'x'
        ~
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVORDI").is_empty(),
            "FVORDI should fire for an ignored argument after name-value arguments"
        );
    }

    #[test]
    fn test_fvordi_fires_ignored_after_repeating() {
        let source = "\
function f(varargin, ~)
    arguments (Repeating)
        varargin
    end
    arguments
        ~
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVORDI").is_empty(),
            "FVORDI should fire for an ignored argument after a Repeating block"
        );
    }

    #[test]
    fn test_fvordi_no_fire_ignored_before_name_value() {
        let source = "\
function f(~, opts)
    arguments
        ~
        opts.Name = 'x'
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVORDI").is_empty(),
            "FVORDI should NOT fire for an ignored argument before name-value arguments"
        );
    }

    #[test]
    fn test_fvordn_fires_positional_after_name_value() {
        let source = "\
function f(opts, b)
    arguments
        opts.Name = 'x'
        b (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVORDN").is_empty(),
            "FVORDN should fire when a positional argument follows name-value arguments"
        );
    }

    #[test]
    fn test_fvordn_no_fire_positional_before_name_value() {
        let source = "\
function f(b, opts)
    arguments
        b (1,1) double
        opts.Name = 'x'
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVORDN").is_empty(),
            "FVORDN should NOT fire when positional arguments precede name-value arguments"
        );
    }

    #[test]
    fn test_fvordo_fires_repeating_output_before_required() {
        let source = "\
function [a, varargout] = f(x)
    arguments (Output)
        varargout
        a (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVORDO").is_empty(),
            "FVORDO should fire when a repeating output precedes a required output"
        );
    }

    #[test]
    fn test_fvordo_no_fire_required_before_repeating() {
        let source = "\
function [a, varargout] = f(x)
    arguments (Output)
        a (1,1) double
    end
    arguments (Repeating)
        varargout
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVORDO").is_empty(),
            "FVORDO should NOT fire when required outputs precede repeating outputs"
        );
    }

    #[test]
    fn test_fvordp_fires_optional_before_required() {
        let source = "\
function f(a, b)
    arguments
        b = 3
        a (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVORDP").is_empty(),
            "FVORDP should fire when an optional positional precedes a required one"
        );
    }

    #[test]
    fn test_fvordp_no_fire_required_before_optional() {
        let source = "\
function f(a, b)
    arguments
        a (1,1) double
        b = 3
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVORDP").is_empty(),
            "FVORDP should NOT fire when required positionals precede optional ones"
        );
    }

    #[test]
    fn test_fvorm_fires_multiple_varargout() {
        let source = "\
function [a, varargout] = f(x)
    arguments (Output)
        a (1,1) double
        varargout
    end
    arguments (Repeating)
        varargout
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVORM").is_empty(),
            "FVORM should fire when varargout is declared more than once"
        );
    }

    #[test]
    fn test_fvorm_no_fire_single_varargout() {
        let source = "\
function [a, varargout] = f(x)
    arguments (Output)
        a (1,1) double
    end
    arguments (Repeating)
        varargout
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVORM").is_empty(),
            "FVORM should NOT fire for a single varargout declaration"
        );
    }

    #[test]
    fn test_fvovrep_fires_varargout_in_output_block() {
        let source = "\
function [varargout] = f(x)
    arguments (Output)
        varargout
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVOVREP").is_empty(),
            "FVOVREP should fire for varargout outside a Repeating block"
        );
    }

    #[test]
    fn test_fvovrep_no_fire_varargout_in_repeating() {
        let source = "\
function [varargout] = f(x)
    arguments (Repeating)
        varargout
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVOVREP").is_empty(),
            "FVOVREP should NOT fire for varargout in a Repeating block"
        );
    }

    #[test]
    fn test_fvrepd_fires_default_in_repeating() {
        let source = "\
function f(a)
    arguments (Repeating)
        a = 3
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVREPD").is_empty(),
            "FVREPD should fire for a default value in a Repeating block"
        );
    }

    #[test]
    fn test_fvrepd_no_fire_repeating_without_default() {
        let source = "\
function f(a)
    arguments (Repeating)
        a (1,:) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVREPD").is_empty(),
            "FVREPD should NOT fire for a Repeating block without defaults"
        );
    }

    #[test]
    fn test_fvrepo_fires_varargin_with_others() {
        let source = "\
function f(varargin, x)
    arguments (Repeating)
        varargin
        x (1,:) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVREPO").is_empty(),
            "FVREPO should fire when a Repeating block with varargin has other arguments"
        );
    }

    #[test]
    fn test_fvrepo_no_fire_varargin_alone() {
        let source = "\
function f(varargin)
    arguments (Repeating)
        varargin
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVREPO").is_empty(),
            "FVREPO should NOT fire when a Repeating block contains only varargin"
        );
    }

    #[test]
    fn test_fvsor_fires_order_mismatch() {
        let source = "\
function f(a, b)
    arguments
        b (1,1) double
        a (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVSOR").is_empty(),
            "FVSOR should fire when block declarations do not match the function line"
        );
    }

    #[test]
    fn test_fvsor_no_fire_matching_order() {
        let source = "\
function f(a, b)
    arguments
        a (1,1) double
        b (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVSOR").is_empty(),
            "FVSOR should NOT fire when block declarations match the function line"
        );
    }

    #[test]
    fn test_fvsor_no_fire_partial_declaration() {
        let source = "\
function f(a, b)
    arguments
        a (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVSOR").is_empty(),
            "FVSOR should NOT fire when the block declares a prefix of the inputs"
        );
    }

    #[test]
    fn test_fvsoro_fires_order_mismatch() {
        let source = "\
function [b, a] = f(x)
    arguments (Output)
        a (1,1) double
        b (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVSORO").is_empty(),
            "FVSORO should fire when output block declarations do not match the function line"
        );
    }

    #[test]
    fn test_fvsoro_no_fire_matching_order() {
        let source = "\
function [a, b] = f(x)
    arguments (Output)
        a (1,1) double
        b (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVSORO").is_empty(),
            "FVSORO should NOT fire when output block declarations match the function line"
        );
    }

    #[test]
    fn test_fvubd_fires_reference_before_declaration() {
        let source = "\
function f(a, b)
    arguments
        a (1,1) double {mustBeLessThan(a, b)}
        b (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVUBD").is_empty(),
            "FVUBD should fire when a validator references a later argument"
        );
    }

    #[test]
    fn test_fvubd_no_fire_reference_after_declaration() {
        let source = "\
function f(a, b)
    arguments
        b (1,1) double
        a (1,1) double {mustBeLessThan(a, b)}
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVUBD").is_empty(),
            "FVUBD should NOT fire when a validator references an earlier argument"
        );
    }

    #[test]
    fn test_fvvcon_fires_undeclared_reference() {
        let source = "\
function f(a)
    arguments
        a (1,1) double {mustBeLessThan(a, z)}
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVVCON").is_empty(),
            "FVVCON should fire when a validator references a non-positional value"
        );
    }

    #[test]
    fn test_fvvcon_no_fire_self_and_prior() {
        let source = "\
function f(a, b)
    arguments
        a (1,1) double {mustBeGreaterThan(a, 0)}
        b (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVVCON").is_empty(),
            "FVVCON should NOT fire when validators use the argument or literals"
        );
    }

    #[test]
    fn test_fvvin_fires_argument_not_used() {
        let source = "\
function f(a, b)
    arguments
        a (1,1) double {mustBeGreaterThan(b, 0)}
        b (1,1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVVIN").is_empty(),
            "FVVIN should fire when the validated argument is not used by the validator"
        );
    }

    #[test]
    fn test_fvvin_no_fire_argument_used() {
        let source = "\
function f(a)
    arguments
        a (1,1) double {mustBeReal}
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVVIN").is_empty(),
            "FVVIN should NOT fire when the validator uses the argument (or is bare)"
        );
    }

    #[test]
    fn test_fvvrep_fires_varargin_outside_repeating() {
        let source = "\
function f(varargin)
    arguments
        varargin
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "FVVREP").is_empty(),
            "FVVREP should fire for varargin outside a Repeating block"
        );
    }

    #[test]
    fn test_fvvrep_no_fire_varargin_in_repeating() {
        let source = "\
function f(varargin)
    arguments (Repeating)
        varargin
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "FVVREP").is_empty(),
            "FVVREP should NOT fire for varargin in a Repeating block"
        );
    }

    #[test]
    fn test_tinvaldim_fires_float_dimension() {
        let source = "\
function f(a)
    arguments
        a (1.5, 2) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "TINVALDIM").is_empty(),
            "TINVALDIM should fire for a non-integer dimension"
        );
    }

    #[test]
    fn test_tinvaldim_no_fire_valid_dimensions() {
        let source = "\
function f(a)
    arguments
        a (1, :) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "TINVALDIM").is_empty(),
            "TINVALDIM should NOT fire for integer or colon dimensions"
        );
    }

    #[test]
    fn test_ttoofewdims_fires_single_dimension() {
        let source = "\
function f(a)
    arguments
        a (1) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            !filter_by_id(&diags, "TTOOFEWDIMS").is_empty(),
            "TTOOFEWDIMS should fire for a single dimension"
        );
    }

    #[test]
    fn test_ttoofewdims_no_fire_two_dimensions() {
        let source = "\
function f(a)
    arguments
        a (1, 2) double
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(
            filter_by_id(&diags, "TTOOFEWDIMS").is_empty(),
            "TTOOFEWDIMS should NOT fire for two dimensions"
        );
    }

    #[test]
    fn test_fv_checks_respect_disabled_config() {
        let mut rule_config = LanguageSpecConfig::default();
        rule_config.disabled_checks.push("FVNIV".to_string());
        let engine = LanguageSpecEngine {
            config: rule_config,
        };
        let source = "\
function f(a)
    arguments
        z (1,1) double
    end
end
";
        let tree = parse_matlab(source);
        let path = Path::new("f.m");
        let ctx = FileContext {
            tree: &tree,
            source,
            file_path: path,
        };
        let diags = engine.check_file(&ctx);
        assert!(
            filter_by_id(&diags, "FVNIV").is_empty(),
            "FVNIV should be disabled via config disabled_checks"
        );
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

    // ===== C4: SPMD transparency checks =====

    #[test]
    fn test_spevc_fires_in_spmd() {
        let source = "\
function foo()
    spmd
        evalin('caller', 'x');
        assignin('caller', 'x', 1);
    end
end
";
        let diags = check_source(source, "foo.m");
        let spevc = filter_by_id(&diags, "SPEVC");
        assert_eq!(spevc.len(), 2, "SPEVC should fire for evalin/assignin('caller') in spmd, got {diags:?}");
    }

    #[test]
    fn test_spevc_no_fire_outside_spmd() {
        let source = "\
function foo()
    evalin('caller', 'x');
end
";
        let diags = check_source(source, "foo.m");
        let spevc = filter_by_id(&diags, "SPEVC");
        assert!(spevc.is_empty(), "SPEVC should NOT fire outside spmd");
    }

    #[test]
    fn test_spld_fires_unassigned_load() {
        let source = "\
function foo()
    spmd
        load('data.mat');
    end
end
";
        let diags = check_source(source, "foo.m");
        let spld = filter_by_id(&diags, "SPLD");
        assert!(!spld.is_empty(), "SPLD should fire for unassigned load in spmd");
    }

    #[test]
    fn test_spld_no_fire_assigned_load() {
        let source = "\
function foo()
    spmd
        d = load('data.mat');
    end
end
";
        let diags = check_source(source, "foo.m");
        let spld = filter_by_id(&diags, "SPLD");
        assert!(spld.is_empty(), "SPLD should NOT fire for assigned load");
    }

    #[test]
    fn test_spsv_fires_without_fromstruct() {
        let source = "\
function foo()
    spmd
        save('out.mat');
    end
end
";
        let diags = check_source(source, "foo.m");
        let spsv = filter_by_id(&diags, "SPSV");
        assert!(!spsv.is_empty(), "SPSV should fire for save without -fromstruct");
    }

    #[test]
    fn test_spsv_no_fire_with_fromstruct() {
        let source = "\
function foo()
    spmd
        save('out.mat', '-fromstruct', s);
    end
end
";
        let diags = check_source(source, "foo.m");
        let spsv = filter_by_id(&diags, "SPSV");
        assert!(spsv.is_empty(), "SPSV should NOT fire for save with -fromstruct");
    }

    #[test]
    fn test_spwhos_fires_without_file() {
        let source = "\
function foo()
    spmd
        who;
        whos;
    end
end
";
        let diags = check_source(source, "foo.m");
        let spwhos = filter_by_id(&diags, "SPWHOS");
        assert_eq!(spwhos.len(), 2, "SPWHOS should fire for who/whos without -file");
    }

    #[test]
    fn test_spwhos_no_fire_with_file() {
        let source = "\
function foo()
    spmd
        who('-file', 'x.mat');
    end
end
";
        let diags = check_source(source, "foo.m");
        let spwhos = filter_by_id(&diags, "SPWHOS");
        assert!(spwhos.is_empty(), "SPWHOS should NOT fire for who with -file");
    }

    #[test]
    fn test_spbfn_fires_eval_in_spmd() {
        let source = "\
function foo()
    spmd
        eval('x');
        evalin('base', 'x');
    end
end
";
        let diags = check_source(source, "foo.m");
        let spbfn = filter_by_id(&diags, "SPBFN");
        assert_eq!(spbfn.len(), 2, "SPBFN should fire for eval and non-caller evalin in spmd");
    }

    #[test]
    fn test_spbfn_no_fire_outside_spmd() {
        let source = "\
function foo()
    eval('x');
end
";
        let diags = check_source(source, "foo.m");
        let spbfn = filter_by_id(&diags, "SPBFN");
        assert!(spbfn.is_empty(), "SPBFN should NOT fire outside spmd");
    }

    #[test]
    fn test_spnf_fires_nested_function_call() {
        let source = "\
function outer()
    spmd
        helper(1);
    end
    function helper(a)
    end
end
";
        let diags = check_source(source, "outer.m");
        let spnf = filter_by_id(&diags, "SPNF");
        assert!(!spnf.is_empty(), "SPNF should fire for nested function call in spmd");
    }

    #[test]
    fn test_spnf_no_fire_local_function_call() {
        let source = "\
function main()
    spmd
        helper(1);
    end
end

function helper(a)
end
";
        let diags = check_source(source, "main.m");
        let spnf = filter_by_id(&diags, "SPNF");
        assert!(spnf.is_empty(), "SPNF should NOT fire for top-level local function call");
    }

    #[test]
    fn test_spdec_fires_negative_and_fractional_bounds() {
        let source = "\
function foo()
    spmd (-2, 4)
        x = 1;
    end
    spmd (2.5)
        x = 1;
    end
end
";
        let diags = check_source(source, "foo.m");
        let spdec = filter_by_id(&diags, "SPDEC");
        assert_eq!(spdec.len(), 2, "SPDEC should fire for negative/fractional bounds");
    }

    #[test]
    fn test_spdec_no_fire_valid_bounds() {
        let source = "\
function foo()
    spmd (2)
        x = 1;
    end
    spmd (2, 4)
        x = 1;
    end
end
";
        let diags = check_source(source, "foo.m");
        let spdec = filter_by_id(&diags, "SPDEC");
        assert!(spdec.is_empty(), "SPDEC should NOT fire for valid integer bounds");
    }

    #[test]
    fn test_spdec3_fires_three_bounds() {
        let source = "\
function foo()
    spmd (2, 4, 6)
        x = 1;
    end
end
";
        let diags = check_source(source, "foo.m");
        let spdec3 = filter_by_id(&diags, "SPDEC3");
        assert!(!spdec3.is_empty(), "SPDEC3 should fire for three bounds");
    }

    #[test]
    fn test_spdec3_no_fire_two_bounds() {
        let source = "\
function foo()
    spmd (2, 4)
        x = 1;
    end
end
";
        let diags = check_source(source, "foo.m");
        let spdec3 = filter_by_id(&diags, "SPDEC3");
        assert!(spdec3.is_empty(), "SPDEC3 should NOT fire for two bounds");
    }

    #[test]
    fn test_spbrk_covered_by_spret() {
        // SPBRK (break/continue not fully contained in spmd) is subsumed by SPRET,
        // which fires for every break/continue inside an spmd block.
        let source = "\
function foo()
    spmd
        break;
    end
end
";
        let diags = check_source(source, "foo.m");
        let spret = filter_by_id(&diags, "SPRET");
        assert!(!spret.is_empty(), "SPRET should fire for break inside spmd (covers SPBRK)");
        let spbrk = filter_by_id(&diags, "SPBRK");
        assert!(spbrk.is_empty(), "SPBRK is deferred — no separate diagnostic emitted");
    }

    // ===== C4: error/warning message checks =====

    #[test]
    fn test_ertxt_fires_id_with_no_message() {
        let source = "error('MyTool:badInput');\n";
        let diags = check_source(source, "myscript.m");
        let ertxt = filter_by_id(&diags, "ERTXT");
        assert!(!ertxt.is_empty(), "ERTXT should fire for error ID with no message");
    }

    #[test]
    fn test_ertxt_no_fire_plain_message_or_id_with_message() {
        let source = "\
error('plain message');
error('MyID', 'message here');
";
        let diags = check_source(source, "myscript.m");
        let ertxt = filter_by_id(&diags, "ERTXT");
        assert!(ertxt.is_empty(), "ERTXT should NOT fire for plain message or ID+message");
    }

    #[test]
    fn test_wtxt_fires_id_with_no_message() {
        let source = "warning('MyTool:badInput');\n";
        let diags = check_source(source, "myscript.m");
        let wtxt = filter_by_id(&diags, "WTXT");
        assert!(!wtxt.is_empty(), "WTXT should fire for warning ID with no message");
    }

    #[test]
    fn test_wtxt_no_fire_plain_message() {
        let source = "warning('plain message');\nwarning('MyID', 'msg');\n";
        let diags = check_source(source, "myscript.m");
        let wtxt = filter_by_id(&diags, "WTXT");
        assert!(wtxt.is_empty(), "WTXT should NOT fire for plain message or ID+message");
    }

    #[test]
    fn test_nchkos_fires_assignment_rhs() {
        let source = "\
function foo()
    x = narginchk(1, 2);
    y = nargoutchk(1, 2);
end
";
        let diags = check_source(source, "foo.m");
        let nchkos = filter_by_id(&diags, "NCHKOS");
        assert_eq!(nchkos.len(), 2, "NCHKOS should fire for narginchk/nargoutchk on RHS");
    }

    #[test]
    fn test_nchkos_no_fire_standalone() {
        let source = "\
function foo()
    narginchk(1, 2);
    nargoutchk(1, 2);
end
";
        let diags = check_source(source, "foo.m");
        let nchkos = filter_by_id(&diags, "NCHKOS");
        assert!(nchkos.is_empty(), "NCHKOS should NOT fire for standalone calls");
    }

    // ===== C4: class construction checks =====

    #[test]
    fn test_ctoine_fires_constructed_object_as_input() {
        let source = "\
classdef Foo
    methods
        function obj = Foo(x)
            obj = Foo(Foo(1));
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let ctoine = filter_by_id(&diags, "CTOINE");
        assert!(!ctoine.is_empty(), "CTOINE should fire for Foo(Foo(1)) in constructor");
    }

    #[test]
    fn test_ctoine_no_fire_plain_argument() {
        let source = "\
classdef Foo
    methods
        function obj = Foo(x)
            obj = Foo(x);
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let ctoine = filter_by_id(&diags, "CTOINE");
        assert!(ctoine.is_empty(), "CTOINE should NOT fire for Foo(x)");
    }

    #[test]
    fn test_ctoro_fires_constructor_without_output() {
        let source = "\
classdef Bar
    methods
        function Bar(x)
        end
    end
end
";
        let diags = check_source(source, "Bar.m");
        let ctoro = filter_by_id(&diags, "CTORO");
        assert!(!ctoro.is_empty(), "CTORO should fire for constructor without output");
    }

    #[test]
    fn test_ctoro_no_fire_constructor_with_output() {
        let source = "\
classdef Bar
    methods
        function obj = Bar(x)
        end
    end
end
";
        let diags = check_source(source, "Bar.m");
        let ctoro = filter_by_id(&diags, "CTORO");
        assert!(ctoro.is_empty(), "CTORO should NOT fire for constructor with output");
    }

    #[test]
    fn test_mherit_fires_builtin_superclass() {
        let source = "\
classdef Foo < double
end
";
        let diags = check_source(source, "Foo.m");
        let mherit = filter_by_id(&diags, "MHERIT");
        assert!(!mherit.is_empty(), "MHERIT should fire for 'classdef Foo < double'");
    }

    #[test]
    fn test_mherit_no_fire_supported_superclass() {
        let source = "\
classdef Foo < handle
end

classdef Bar < MyBase
end
";
        let diags = check_source(source, "Foo.m");
        let mherit = filter_by_id(&diags, "MHERIT");
        assert!(mherit.is_empty(), "MHERIT should NOT fire for handle or user superclass");
    }

    // ===== C4: script variable definition check =====

    #[test]
    fn test_useswns_fires_use_before_definition() {
        let source = "y = x + 1;\nx = 5;\n";
        let diags = check_source(source, "myscript.m");
        let useswns = filter_by_id(&diags, "USESWNS");
        assert!(
            !useswns.is_empty(),
            "USESWNS should fire for variable used before definition"
        );
    }

    #[test]
    fn test_useswns_no_fire_defined_before_use() {
        let source = "x = 5;\ny = x + 1;\n";
        let diags = check_source(source, "myscript.m");
        let useswns = filter_by_id(&diags, "USESWNS");
        assert!(
            useswns.is_empty(),
            "USESWNS should NOT fire when variable is defined before use"
        );
    }

    #[test]
    fn test_useswns_no_fire_function_calls() {
        let source = "disp('hello');\nplot(1:10);\n";
        let diags = check_source(source, "myscript.m");
        let useswns = filter_by_id(&diags, "USESWNS");
        assert!(
            useswns.is_empty(),
            "USESWNS should NOT fire for function names in a script"
        );
    }

    // ===== C2 class/method attribute checks =====

    const C2_CHECK_IDS: [&str; 26] = [
        "MABSEAC", "MABSEAM", "MCAPP", "MCCBS", "MCCBU", "MCCMC", "MCCSOP", "MCGSA", "MCMIO",
        "MCMSP", "MCMTP", "MCPIN", "MCPSG", "MCSCC", "MCSCF", "MCSCM", "MCSCN", "MCSCO",
        "MCSCT", "MCSMO", "MCSWA", "MTAGS3", "MTMAT", "MWKCL", "MWKCT", "MWKREF",
    ];

    #[test]
    fn test_c2_no_fire_valid_class() {
        let source = "\
classdef Foo
    properties
        x double = 5
    end
    methods
        function y = f(self)
            y = self.x;
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        for id in C2_CHECK_IDS {
            let hits = filter_by_id(&diags, id);
            assert!(
                hits.is_empty(),
                "{id} should NOT fire on a valid class, got: {}",
                hits.len()
            );
        }
    }

    #[test]
    fn test_c2_checks_respect_disabled_config() {
        let mut rule_config = LanguageSpecConfig::default();
        rule_config.disabled_checks.push("MABSEAC".to_string());
        let engine = LanguageSpecEngine {
            config: rule_config,
        };
        let source = "\
classdef (Sealed, Abstract) Foo
    properties
        x
    end
end
";
        let tree = parse_matlab(source);
        let path = Path::new("Foo.m");
        let ctx = FileContext {
            tree: &tree,
            source,
            file_path: path,
        };
        let diags = engine.check_file(&ctx);
        let hits = filter_by_id(&diags, "MABSEAC");
        assert!(
            hits.is_empty(),
            "MABSEAC should be disabled via config disabled_checks"
        );
    }

    #[test]
    fn test_mabseac_fires_sealed_abstract_instance_members() {
        let source = "\
classdef (Sealed, Abstract) Foo
    properties
        x
    end
    methods
        function y = f(obj)
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MABSEAC");
        assert!(
            !hits.is_empty(),
            "MABSEAC should fire for instance members in a Sealed+Abstract class"
        );
    }

    #[test]
    fn test_mabseac_no_fire_constant_static() {
        let source = "\
classdef (Sealed, Abstract) Foo
    properties (Constant)
        x = 5
    end
    methods (Static)
        function y = f()
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MABSEAC");
        assert!(
            hits.is_empty(),
            "MABSEAC should NOT fire for Constant properties and Static methods"
        );
    }

    #[test]
    fn test_mcswa_fires_sealed_allowed_subclasses() {
        let source = "\
classdef (Sealed, AllowedSubclasses = ?Bar) Foo
    methods
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSWA");
        assert!(
            !hits.is_empty(),
            "MCSWA should fire for Sealed class with AllowedSubclasses"
        );
    }

    #[test]
    fn test_mcswa_no_fire() {
        let source = "\
classdef (Sealed) Foo
    methods
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSWA");
        assert!(hits.is_empty(), "MCSWA should NOT fire for a plain Sealed class");
    }

    #[test]
    fn test_mtmat_fires_duplicate_attribute() {
        let source = "\
classdef (Sealed, Sealed) Foo
    methods
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MTMAT");
        assert!(
            !hits.is_empty(),
            "MTMAT should fire for a duplicate class attribute"
        );
    }

    #[test]
    fn test_mtmat_no_fire() {
        let source = "\
classdef (Sealed, Abstract) Foo
    methods
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MTMAT");
        assert!(hits.is_empty(), "MTMAT should NOT fire for distinct attributes");
    }

    #[test]
    fn test_mtags3_fires_access_with_setaccess() {
        let source = "\
classdef Foo
    properties (Access = private, SetAccess = private)
        x
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MTAGS3");
        assert!(
            !hits.is_empty(),
            "MTAGS3 should fire when Access is combined with SetAccess"
        );
    }

    #[test]
    fn test_mtags3_no_fire() {
        let source = "\
classdef Foo
    properties (Access = private)
        x
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MTAGS3");
        assert!(hits.is_empty(), "MTAGS3 should NOT fire for Access alone");
    }

    #[test]
    fn test_mabseam_fires_abstract_sealed_method() {
        let source = "\
classdef Foo
    methods (Abstract, Sealed)
        function y = f(obj)
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MABSEAM");
        assert!(
            !hits.is_empty(),
            "MABSEAM should fire for a method that is both Abstract and Sealed"
        );
    }

    #[test]
    fn test_mabseam_no_fire() {
        let source = "\
classdef Foo
    methods (Abstract)
        function y = f(obj)
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MABSEAM");
        assert!(hits.is_empty(), "MABSEAM should NOT fire for Abstract only");
    }

    #[test]
    fn test_mcmsp_fires_private_abstract_method() {
        let source = "\
classdef Foo
    methods (Access = private, Abstract)
        function y = f(obj)
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCMSP");
        assert!(
            !hits.is_empty(),
            "MCMSP should fire for a private abstract method"
        );
    }

    #[test]
    fn test_mcmsp_no_fire() {
        let source = "\
classdef Foo
    methods (Access = private)
        function y = f(obj)
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCMSP");
        assert!(hits.is_empty(), "MCMSP should NOT fire for private only");
    }

    #[test]
    fn test_mcapp_fires_private_abstract_property() {
        let source = "\
classdef Foo
    properties (Access = private, Abstract)
        x
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCAPP");
        assert!(
            !hits.is_empty(),
            "MCAPP should fire for a private abstract property"
        );
    }

    #[test]
    fn test_mcapp_no_fire() {
        let source = "\
classdef Foo
    properties (Access = private)
        x
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCAPP");
        assert!(hits.is_empty(), "MCAPP should NOT fire for private only");
    }

    #[test]
    fn test_mcgsa_fires_setter_for_abstract_property() {
        let source = "\
classdef Foo
    properties (Abstract)
        x
    end
    methods
        function set.x(obj, val)
            obj.x = val;
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCGSA");
        assert!(
            !hits.is_empty(),
            "MCGSA should fire when a setter targets an abstract property"
        );
    }

    #[test]
    fn test_mcgsa_no_fire() {
        let source = "\
classdef Foo
    properties
        x
    end
    methods
        function set.x(obj, val)
            obj.x = val;
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCGSA");
        assert!(hits.is_empty(), "MCGSA should NOT fire for a normal property");
    }

    #[test]
    fn test_mcpsg_fires_abstract_setter() {
        let source = "\
classdef Foo
    properties
        x
    end
    methods (Abstract)
        function set.x(obj, val)
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCPSG");
        assert!(
            !hits.is_empty(),
            "MCPSG should fire when a setter is declared abstract"
        );
    }

    #[test]
    fn test_mcpsg_no_fire() {
        let source = "\
classdef Foo
    properties
        x
    end
    methods
        function set.x(obj, val)
            obj.x = val;
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCPSG");
        assert!(hits.is_empty(), "MCPSG should NOT fire for a defined setter");
    }

    // ===== C2 constant property checks =====

    #[test]
    fn test_mccsop_fires_constructor_modifies_constant() {
        let source = "\
classdef Foo
    properties (Constant)
        x = 5
    end
    methods
        function obj = Foo()
            obj.x = 6;
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCCSOP");
        assert!(
            !hits.is_empty(),
            "MCCSOP should fire when the constructor modifies a Constant property"
        );
    }

    #[test]
    fn test_mccsop_no_fire() {
        let source = "\
classdef Foo
    properties
        x = 5
    end
    methods
        function obj = Foo()
            obj.x = 6;
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCCSOP");
        assert!(hits.is_empty(), "MCCSOP should NOT fire for a non-Constant property");
    }

    #[test]
    fn test_mcscn_fires_method_sets_constant() {
        let source = "\
classdef Foo
    properties (Constant)
        x = 5
    end
    methods
        function obj = Foo()
        end
        function f(obj)
            obj.x = 7;
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCN");
        assert!(
            !hits.is_empty(),
            "MCSCN should fire when a method sets a Constant property"
        );
    }

    #[test]
    fn test_mcscn_no_fire() {
        let source = "\
classdef Foo
    properties
        x = 5
    end
    methods
        function obj = Foo()
        end
        function f(obj)
            obj.x = 7;
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCN");
        assert!(hits.is_empty(), "MCSCN should NOT fire for a non-Constant property");
    }

    // ===== C2 WeakHandle property checks =====

    #[test]
    fn test_mwkcl_fires_untyped_weakhandle() {
        let source = "\
classdef Foo
    properties (WeakHandle)
        x
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MWKCL");
        assert!(
            !hits.is_empty(),
            "MWKCL should fire for a WeakHandle property without a class validation"
        );
    }

    #[test]
    fn test_mwkcl_no_fire_typed() {
        let source = "\
classdef Foo
    properties (WeakHandle)
        x SomeClass
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MWKCL");
        assert!(hits.is_empty(), "MWKCL should NOT fire for a typed property");
    }

    #[test]
    fn test_mwkct_fires_weakhandle_constant() {
        let source = "\
classdef Foo
    properties (WeakHandle, Constant)
        x = 5
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MWKCT");
        assert!(
            !hits.is_empty(),
            "MWKCT should fire for WeakHandle combined with Constant"
        );
    }

    #[test]
    fn test_mwkct_no_fire() {
        let source = "\
classdef Foo
    properties (WeakHandle)
        x SomeClass
    end
    properties (Constant)
        y = 5
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MWKCT");
        assert!(hits.is_empty(), "MWKCT should NOT fire for separate blocks");
    }

    #[test]
    fn test_mwkref_fires_weakhandle_dependent() {
        let source = "\
classdef Foo
    properties (WeakHandle, Dependent)
        x
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MWKREF");
        assert!(
            !hits.is_empty(),
            "MWKREF should fire for WeakHandle combined with Dependent"
        );
    }

    #[test]
    fn test_mwkref_no_fire() {
        let source = "\
classdef Foo
    properties (WeakHandle)
        x SomeClass
    end
    properties (Dependent)
        y
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MWKREF");
        assert!(hits.is_empty(), "MWKREF should NOT fire for separate blocks");
    }

    // ===== C2 constructor/superclass checks =====

    #[test]
    fn test_mccbs_fires_undeclared_superclass() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@NotSuper();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCCBS");
        assert!(
            !hits.is_empty(),
            "MCCBS should fire when the superclass constructor is not a declared superclass"
        );
    }

    #[test]
    fn test_mccbs_no_fire() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCCBS");
        assert!(hits.is_empty(), "MCCBS should NOT fire for a declared superclass");
    }

    #[test]
    fn test_mccbu_fires_object_use_before_super() {
        let source = "\
classdef Foo < Bar
    properties
        y
    end
    methods
        function obj = Foo()
            obj.y = 1;
            obj = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCCBU");
        assert!(
            !hits.is_empty(),
            "MCCBU should fire when the object is used before the superclass constructor call"
        );
    }

    #[test]
    fn test_mccbu_no_fire() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCCBU");
        assert!(hits.is_empty(), "MCCBU should NOT fire when super call comes first");
    }

    #[test]
    fn test_mccmc_fires_multiple_super_calls() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@Bar();
            obj = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCCMC");
        assert!(
            !hits.is_empty(),
            "MCCMC should fire when the superclass constructor is called more than once"
        );
    }

    #[test]
    fn test_mccmc_no_fire() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCCMC");
        assert!(hits.is_empty(), "MCCMC should NOT fire for a single super call");
    }

    #[test]
    fn test_mcscf_fires_not_assigned_to_first_output() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            x = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCF");
        assert!(
            !hits.is_empty(),
            "MCSCF should fire when the superclass constructor is assigned to a non-first output"
        );
    }

    #[test]
    fn test_mcscf_no_fire() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCF");
        assert!(hits.is_empty(), "MCSCF should NOT fire for a correct assignment");
    }

    #[test]
    fn test_mcsco_fires_not_first_output_argument() {
        let source = "\
classdef Foo < Bar
    methods
        function [obj, other] = Foo()
            other = other@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCO");
        assert!(
            !hits.is_empty(),
            "MCSCO should fire when the superclass constructor uses a non-first output argument"
        );
    }

    #[test]
    fn test_mcsco_no_fire() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCO");
        assert!(hits.is_empty(), "MCSCO should NOT fire for the first output argument");
    }

    #[test]
    fn test_mcsct_fires_conditional_super_call() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo(x)
            if x > 0
                obj = obj@Bar(1);
            end
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCT");
        assert!(
            !hits.is_empty(),
            "MCSCT should fire when the superclass constructor call is conditionalized"
        );
    }

    #[test]
    fn test_mcsct_fires_super_in_expression() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = 1 + obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCT");
        assert!(
            !hits.is_empty(),
            "MCSCT should fire when the superclass constructor call is part of another expression"
        );
    }

    #[test]
    fn test_mcsct_no_fire() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCT");
        assert!(hits.is_empty(), "MCSCT should NOT fire for a top-level super call");
    }

    #[test]
    fn test_mcsmo_fires_multiple_outputs() {
        let source = "\
classdef Foo < Bar
    methods
        function [obj, x] = Foo()
            [obj, x] = obj@Bar(1);
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSMO");
        assert!(
            !hits.is_empty(),
            "MCSMO should fire when a superclass object initialization has multiple outputs"
        );
    }

    #[test]
    fn test_mcsmo_no_fire() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSMO");
        assert!(hits.is_empty(), "MCSMO should NOT fire for a single output");
    }

    #[test]
    fn test_mscc_fires_super_call_in_non_constructor() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
        end
        function obj = helper()
            obj = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCC");
        assert!(
            !hits.is_empty(),
            "MCSCC should fire when a superclass constructor is called from a non-constructor method"
        );
    }

    #[test]
    fn test_mscc_no_fire() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@Bar();
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCC");
        assert!(hits.is_empty(), "MCSCC should NOT fire for a valid constructor call");
    }

    #[test]
    fn test_mscm_fires_wrong_superclass_method_name() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@Bar();
        end
        function y = mymethod(obj)
            y = other@Bar(obj);
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCM");
        assert!(
            !hits.is_empty(),
            "MCSCM should fire when a superclass method name does not match the enclosing method"
        );
    }

    #[test]
    fn test_mscm_no_fire() {
        let source = "\
classdef Foo < Bar
    methods
        function obj = Foo()
            obj = obj@Bar();
        end
        function y = mymethod(obj)
            y = mymethod@Bar(obj);
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCSCM");
        assert!(hits.is_empty(), "MCSCM should NOT fire for a matching method name");
    }

    // ===== C2 method signature checks =====

    #[test]
    fn test_mcmio_fires_65_outputs() {
        let outputs: Vec<String> = (1..=65).map(|i| format!("a{i}")).collect();
        let source = format!(
            "classdef Foo\n    methods\n        function [{}] = f()\n        end\n    end\nend\n",
            outputs.join(",")
        );
        let diags = check_source(&source, "Foo.m");
        let hits = filter_by_id(&diags, "MCMIO");
        assert!(!hits.is_empty(), "MCMIO should fire for 65 outputs");
    }

    #[test]
    fn test_mcmio_no_fire_64_outputs() {
        let outputs: Vec<String> = (1..=64).map(|i| format!("a{i}")).collect();
        let source = format!(
            "classdef Foo\n    methods\n        function [{}] = f()\n        end\n    end\nend\n",
            outputs.join(",")
        );
        let diags = check_source(&source, "Foo.m");
        let hits = filter_by_id(&diags, "MCMIO");
        assert!(hits.is_empty(), "MCMIO should NOT fire for 64 outputs");
    }

    #[test]
    fn test_mcmtp_fires_non_static_test_parameter_definition() {
        let source = "\
classdef Foo
    methods
        function params = TestParameterDefinition()
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCMTP");
        assert!(
            !hits.is_empty(),
            "MCMTP should fire for a non-Static TestParameterDefinition method"
        );
    }

    #[test]
    fn test_mcmtp_no_fire_static() {
        let source = "\
classdef Foo
    methods (Static)
        function params = TestParameterDefinition()
        end
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCMTP");
        assert!(hits.is_empty(), "MCMTP should NOT fire for a Static method");
    }

    #[test]
    fn test_mcpin_fires_self_instance_default() {
        let source = "\
classdef Foo
    properties
        x Foo = Foo()
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCPIN");
        assert!(
            !hits.is_empty(),
            "MCPIN should fire when a property is initialized to an instance of the class itself"
        );
    }

    #[test]
    fn test_mcpin_no_fire() {
        let source = "\
classdef Foo
    properties
        x double = 5
    end
end
";
        let diags = check_source(source, "Foo.m");
        let hits = filter_by_id(&diags, "MCPIN");
        assert!(hits.is_empty(), "MCPIN should NOT fire for a plain default value");
    }

    // ===== C1: parfor/SPMD variable, slicing, and reduction checks =====

    // -- PFEVC ---------------------------------------------------------------

    #[test]
    fn test_pfevc_fires_evalin_caller_in_parfor() {
        let source = "\
function f()
    parfor i = 1:10
        evalin('caller', 'x');
        x(i) = i;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFEVC").is_empty(), "PFEVC should fire for evalin('caller') in parfor");
    }

    #[test]
    fn test_pfevc_no_fire_not_caller_or_not_parfor() {
        let source = "\
function f()
    parfor i = 1:10
        evalin('base', 'x');
    end
    for i = 1:10
        evalin('caller', 'x');
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFEVC").is_empty(), "PFEVC should NOT fire for evalin('base') or outside parfor");
    }

    // -- PFINPT --------------------------------------------------------------

    #[test]
    fn test_pfinpt_fires_inputname_in_parfor() {
        let source = "\
function f()
    parfor i = 1:10
        inputname(1);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFINPT").is_empty(), "PFINPT should fire for inputname in parfor");
    }

    #[test]
    fn test_pfinpt_no_fire_outside_parfor() {
        let source = "\
function f()
    for i = 1:10
        inputname(1);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFINPT").is_empty(), "PFINPT should NOT fire outside parfor");
    }

    // -- PFNACK --------------------------------------------------------------

    #[test]
    fn test_pfnack_fires_narginchk_in_parfor() {
        let source = "\
function f()
    parfor i = 1:10
        narginchk(1, 2);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFNACK").is_empty(), "PFNACK should fire for narginchk in parfor");
    }

    #[test]
    fn test_pfnack_no_fire_outside_parfor() {
        let source = "\
function f()
    narginchk(1, 2);
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFNACK").is_empty(), "PFNACK should NOT fire outside parfor");
    }

    // -- PFNAIO --------------------------------------------------------------

    #[test]
    fn test_pfnaio_fires_bare_nargin_in_parfor() {
        let source = "\
function f()
    parfor i = 1:10
        nargin;
        nargout();
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFNAIO").is_empty(), "PFNAIO should fire for nargin/nargout without arguments in parfor");
    }

    #[test]
    fn test_pfnaio_no_fire_with_argument_or_outside_parfor() {
        let source = "\
function f()
    for i = 1:10
        nargin;
    end
    nargin();
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFNAIO").is_empty(), "PFNAIO should NOT fire outside parfor");
    }

    // -- PFLD ----------------------------------------------------------------

    #[test]
    fn test_pfld_fires_load_in_parfor() {
        let source = "\
function f()
    parfor i = 1:10
        load('data.mat');
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFLD").is_empty(), "PFLD should fire for load without output in parfor");
    }

    #[test]
    fn test_pfld_fires_load_command_form() {
        let source = "\
function f()
    parfor i = 1:10
        load data.mat
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFLD").is_empty(), "PFLD should fire for command-form load in parfor");
    }

    #[test]
    fn test_pfld_no_fire_load_with_output() {
        let source = "\
function f()
    parfor i = 1:10
        s = load('data.mat');
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFLD").is_empty(), "PFLD should NOT fire when load has an output");
    }

    // -- PFSV ----------------------------------------------------------------

    #[test]
    fn test_pfsv_fires_save_in_parfor() {
        let source = "\
function f()
    parfor i = 1:10
        save('out.mat');
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFSV").is_empty(), "PFSV should fire for save without -fromstruct in parfor");
    }

    #[test]
    fn test_pfsv_no_fire_with_fromstruct() {
        let source = "\
function f()
    parfor i = 1:10
        save('-fromstruct', 's', 'out.mat');
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFSV").is_empty(), "PFSV should NOT fire for save with -fromstruct");
    }

    // -- FPFORP / FWFORP -----------------------------------------------------

    #[test]
    fn test_fpforp_fires_fprintf_to_read_only_handle() {
        let source = "\
function f()
    parfor i = 1:10
        fid = fopen('f.txt', 'r');
        fprintf(fid, 'x');
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "FPFORP").is_empty(), "FPFORP should fire for fprintf to a read-only handle");
    }

    #[test]
    fn test_fpforp_fires_inline_fopen_read_only() {
        let source = "\
function f()
    parfor i = 1:10
        fprintf(fopen('f.txt', 'r'), 'x');
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "FPFORP").is_empty(), "FPFORP should fire for inline read-only fopen");
    }

    #[test]
    fn test_fpforp_no_fire_write_handle_or_stdout() {
        let source = "\
function f()
    parfor i = 1:10
        fid = fopen('f.txt', 'w');
        fprintf(fid, 'x');
        fprintf('hello');
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "FPFORP").is_empty(), "FPFORP should NOT fire for a writable handle or stdout");
    }

    #[test]
    fn test_fwforp_fires_fwrite_to_read_only_handle() {
        let source = "\
function f()
    parfor i = 1:10
        fid = fopen('f.txt', 'r');
        fwrite(fid, 1);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "FWFORP").is_empty(), "FWFORP should fire for fwrite to a read-only handle");
    }

    #[test]
    fn test_fwforp_no_fire_write_handle() {
        let source = "\
function f()
    parfor i = 1:10
        fwrite(fopen('g.txt', 'w'), 1);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "FWFORP").is_empty(), "FWFORP should NOT fire for a writable handle");
    }

    // -- PFCEL ---------------------------------------------------------------

    #[test]
    fn test_pfcel_fires_cell_array_arg() {
        let source = "\
function f()
    parfor i = 1:10
        y = sin({1, 2});
        z = cos(cell(3, 1));
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFCEL").is_empty(), "PFCEL should fire for cell array arguments");
    }

    #[test]
    fn test_pfcel_no_fire_numeric_arg() {
        let source = "\
function f()
    parfor i = 1:10
        y = sin(x(i));
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFCEL").is_empty(), "PFCEL should NOT fire for numeric arguments");
    }

    // -- PFANON --------------------------------------------------------------

    #[test]
    fn test_pfanon_fires_sliced_var_in_lambda() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
        g = @(t) x(t);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFANON").is_empty(), "PFANON should fire when a sliced output is used in a lambda");
    }

    #[test]
    fn test_pfanon_no_fire_lambda_unrelated() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
        g = @(t) t * 2;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFANON").is_empty(), "PFANON should NOT fire for unrelated lambdas");
    }

    // -- PFANSLP / PFANSNS ---------------------------------------------------

    #[test]
    fn test_pfanslp_fires_ans_parfor_variable() {
        let source = "\
function f()
    parfor ans = 1:10
        y = ans;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFANSLP").is_empty(), "PFANSLP should fire for 'ans' as parfor variable");
    }

    #[test]
    fn test_pfanslp_no_fire_normal_parfor_variable() {
        let source = "\
function f()
    parfor i = 1:10
        y = i;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFANSLP").is_empty(), "PFANSLP should NOT fire for a normal parfor variable");
    }

    #[test]
    fn test_pfansns_fires_ans_for_variable_in_parfor() {
        let source = "\
function f()
    parfor i = 1:10
        for ans = 1:5
            x(i, ans) = i;
        end
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFANSNS").is_empty(), "PFANSNS should fire for 'ans' as for variable in parfor");
    }

    #[test]
    fn test_pfansns_no_fire_normal_for_variable() {
        let source = "\
function f()
    parfor i = 1:10
        for j = 1:5
            x(i, j) = i;
        end
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFANSNS").is_empty(), "PFANSNS should NOT fire for a normal for variable");
    }

    // -- PFCTXT --------------------------------------------------------------

    #[test]
    fn test_pfctxt_fires_sliced_access_outside_nested_loop() {
        let source = "\
function f()
    parfor i = 1:10
        for j = 1:5
            x(i, j) = i * j;
        end
        y(i) = x(i, j);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFCTXT").is_empty(), "PFCTXT should fire when a sliced variable is indexed outside the defining for loop");
    }

    #[test]
    fn test_pfctxt_no_fire_access_inside_nested_loop() {
        let source = "\
function f()
    parfor i = 1:10
        for j = 1:5
            x(i, j) = i * j;
        end
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFCTXT").is_empty(), "PFCTXT should NOT fire when accesses are inside the defining for loop");
    }

    // -- PFFRNG --------------------------------------------------------------

    #[test]
    fn test_pffrng_fires_non_constant_nested_range() {
        let source = "\
function f()
    parfor i = 1:10
        for j = 1:i
            x(i, j) = i * j;
        end
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFFRNG").is_empty(), "PFFRNG should fire for a non-constant nested loop range");
    }

    #[test]
    fn test_pffrng_no_fire_constant_positive_range() {
        let source = "\
function f()
    parfor i = 1:10
        for j = 1:5
            x(i, j) = i * j;
        end
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFFRNG").is_empty(), "PFFRNG should NOT fire for a constant positive range");
    }

    // -- PFFSUB --------------------------------------------------------------

    #[test]
    fn test_pffsub_fires_indexing_nested_for_variable() {
        let source = "\
function f()
    parfor i = 1:10
        for j = 1:5
            y = j(k);
        end
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFFSUB").is_empty(), "PFFSUB should fire when a nested for variable is indexed");
    }

    #[test]
    fn test_pffsub_no_fire_nested_for_as_argument() {
        let source = "\
function f()
    parfor i = 1:10
        for j = 1:5
            x(i, j) = i * j;
        end
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFFSUB").is_empty(), "PFFSUB should NOT fire when the nested for variable is an argument");
    }

    // -- PFINCR --------------------------------------------------------------

    #[test]
    fn test_pfincr_fires_mixed_reduction_operators() {
        let source = "\
function f(s)
    parfor i = 1:10
        s = s + x(i);
        s = s * y(i);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFINCR").is_empty(), "PFINCR should fire for mixed reduction operators");
    }

    #[test]
    fn test_pfincr_no_fire_single_reduction_operator() {
        let source = "\
function f(s)
    parfor i = 1:10
        s = s + x(i);
        s = s + y(i);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFINCR").is_empty(), "PFINCR should NOT fire for a single reduction operator");
    }

    // -- PFMLTI --------------------------------------------------------------

    #[test]
    fn test_pfmlti_fires_assigning_nested_for_variable() {
        let source = "\
function f()
    parfor i = 1:10
        for j = 1:5
            j = i;
        end
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFMLTI").is_empty(), "PFMLTI should fire when the nested for variable is assigned");
    }

    #[test]
    fn test_pfmlti_no_fire_no_assignment_to_loop_variable() {
        let source = "\
function f()
    parfor i = 1:10
        for j = 1:5
            x(i, j) = i * j;
        end
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFMLTI").is_empty(), "PFMLTI should NOT fire when the nested for variable is not assigned");
    }

    // -- PFNAR ---------------------------------------------------------------

    #[test]
    fn test_pfnar_fires_subtracting_reduction_variable() {
        let source = "\
function s = f(x)
    s = 0;
    parfor i = 1:10
        s = x(i) - s;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFNAR").is_empty(), "PFNAR should fire when a reduction variable is subtracted from an expression");
    }

    #[test]
    fn test_pfnar_no_fire_valid_reduction_subtraction() {
        let source = "\
function s = f(x)
    s = 0;
    parfor i = 1:10
        s = s - x(i);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFNAR").is_empty(), "PFNAR should NOT fire for a valid reduction subtraction");
    }

    // -- PFRFH ---------------------------------------------------------------

    #[test]
    fn test_pfrfh_fires_unknown_reduction_function() {
        let source = "\
function s = f(x)
    s = 0;
    parfor i = 1:10
        fn = i;
        s = fn(s, x(i));
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFRFH").is_empty(), "PFRFH should fire when the reduction function is not a name or broadcast variable");
    }

    #[test]
    fn test_pfrfh_no_fire_known_reduction_function() {
        let source = "\
function s = f(x)
    s = 0;
    parfor i = 1:10
        s = plus(s, x(i));
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFRFH").is_empty(), "PFRFH should NOT fire for a known reduction function");
    }

    #[test]
    fn test_pfrfh_no_fire_broadcast_reduction_function() {
        let source = "\
function s = f(x, fn)
    s = 0;
    parfor i = 1:10
        s = fn(s, x(i));
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFRFH").is_empty(), "PFRFH should NOT fire for a broadcast reduction function");
    }

    // -- PFRNG ---------------------------------------------------------------

    #[test]
    fn test_pfrng_fires_non_unit_step() {
        let source = "\
function f()
    parfor i = 1:2:10
        x(i) = i;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFRNG").is_empty(), "PFRNG should fire for a non-unit step");
    }

    #[test]
    fn test_pfrng_fires_negative_step() {
        let source = "\
function f()
    parfor i = 10:-1:1
        x(i) = i;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFRNG").is_empty(), "PFRNG should fire for a negative step");
    }

    #[test]
    fn test_pfrng_no_fire_unit_step() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFRNG").is_empty(), "PFRNG should NOT fire for a unit step");
    }

    // -- PFSLO ---------------------------------------------------------------

    #[test]
    fn test_pfslo_fires_temp_indexed_with_loop_variable() {
        let source = "\
function f()
    parfor i = 1:10
        t = i * 2;
        y = t(i);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFSLO").is_empty(), "PFSLO should fire when a temporary is indexed with the loop variable");
    }

    #[test]
    fn test_pfslo_no_fire_valid_sliced_output() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
        y = x(i);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFSLO").is_empty(), "PFSLO should NOT fire for a valid sliced output");
    }

    // -- PFSLRD --------------------------------------------------------------

    #[test]
    fn test_pfslrd_fires_whole_read_of_sliced_variable() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
        y = x;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFSLRD").is_empty(), "PFSLRD should fire for a non-indexed read of a sliced variable");
    }

    #[test]
    fn test_pfslrd_no_fire_indexed_read_only() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
        y = x(i);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFSLRD").is_empty(), "PFSLRD should NOT fire for indexed reads only");
    }

    // -- PFSLW ---------------------------------------------------------------

    #[test]
    fn test_pfslw_fires_different_subscripts() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
        y = x(i, 1);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFSLW").is_empty(), "PFSLW should fire for mismatched subscript lists");
    }

    #[test]
    fn test_pfslw_no_fire_same_subscripts() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
        y = x(i);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFSLW").is_empty(), "PFSLW should NOT fire for consistent subscript lists");
    }

    // -- PFUNK ---------------------------------------------------------------

    #[test]
    fn test_pfunk_fires_sliced_and_temp_mix() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
        x = 5;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFUNK").is_empty(), "PFUNK should fire when a variable is both sliced and a temporary");
    }

    #[test]
    fn test_pfunk_no_fire_pure_sliced() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFUNK").is_empty(), "PFUNK should NOT fire for a pure sliced output");
    }

    // -- PFUTMP --------------------------------------------------------------

    #[test]
    fn test_pfutmp_fires_temp_used_before_set() {
        let source = "\
function f()
    parfor i = 1:10
        y = t + i;
        t = i;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFUTMP").is_empty(), "PFUTMP should fire when a temporary is used before it is set");
    }

    #[test]
    fn test_pfutmp_no_fire_set_before_use() {
        let source = "\
function f()
    parfor i = 1:10
        t = i;
        y = t + 1;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFUTMP").is_empty(), "PFUTMP should NOT fire when the temporary is set before use");
    }

    #[test]
    fn test_pfutmp_no_fire_predefined_variable() {
        let source = "\
function f(t)
    parfor i = 1:10
        y = t + i;
        t = i;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFUTMP").is_empty(), "PFUTMP should NOT fire for a function input");
    }

    // -- PFUTVR --------------------------------------------------------------

    #[test]
    fn test_putvr_fires_uninitialized_reduction() {
        let source = "parfor i = 1:10\n    s = s + x(i);\nend\n";
        let diags = check_source(source, "script.m");
        assert!(!filter_by_id(&diags, "PFUTVR").is_empty(), "PFUTVR should fire for an uninitialized reduction variable");
    }

    #[test]
    fn test_pfutvr_no_fire_initialized_reduction() {
        let source = "\
function s = f(x)
    s = 0;
    parfor i = 1:10
        s = s + x(i);
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFUTVR").is_empty(), "PFUTVR should NOT fire for an initialized reduction variable");
    }

    // -- PFVARS --------------------------------------------------------------

    #[test]
    fn test_pfvars_fires_too_many_variables() {
        let mut body = String::new();
        for k in 0..1100 {
            body.push_str(&format!("v{k} = i;\n"));
        }
        let source = format!("function f()\nparfor i = 1:10\n{body}end\nend\n");
        let diags = check_source(&source, "f.m");
        assert!(!filter_by_id(&diags, "PFVARS").is_empty(), "PFVARS should fire for more than 1024 variables");
    }

    #[test]
    fn test_pfvars_no_fire_few_variables() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFVARS").is_empty(), "PFVARS should NOT fire for a few variables");
    }

    // -- PFVSUB --------------------------------------------------------------

    #[test]
    fn test_pfvsub_fires_indexing_parfor_variable() {
        let source = "\
function f()
    parfor i = 1:10
        for j = 1:5
            y = i(j);
        end
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFVSUB").is_empty(), "PFVSUB should fire when the parfor loop variable is indexed");
    }

    #[test]
    fn test_pfvsub_no_fire_sliced_indexing() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(filter_by_id(&diags, "PFVSUB").is_empty(), "PFVSUB should NOT fire for indexing a sliced variable");
    }

    // -- plan MUST-fire / MUST-NOT-fire examples -----------------------------

    #[test]
    fn test_c1_plan_must_fire_example() {
        let source = "\
function f()
    parfor i = 1:10
        evalin('caller', 'x');
        x(i) = i;
    end
    parfor ans = 1:10
        y = ans;
    end
end
";
        let diags = check_source(source, "f.m");
        assert!(!filter_by_id(&diags, "PFEVC").is_empty(), "PFEVC should fire on the MUST-fire example");
        assert!(!filter_by_id(&diags, "PFANSLP").is_empty(), "PFANSLP should fire on the MUST-fire example");
    }

    #[test]
    fn test_c1_plan_must_not_fire_example() {
        let source = "\
function f()
    parfor i = 1:10
        x(i) = i;
    end
    for i = 1:10
        evalin('base', 'x');
    end
end
";
        let diags = check_source(source, "f.m");
        let c1_diags: Vec<_> = diags
            .iter()
            .filter(|d| {
                matches!(
                    d.rule_id,
                    "FPFORP" | "FWFORP" | "PFANON" | "PFANSLP" | "PFANSNS" | "PFCEL"
                        | "PFCTXT" | "PFEVC" | "PFFRNG" | "PFFSUB" | "PFINCR" | "PFINPT"
                        | "PFLD" | "PFMLTI" | "PFNACK" | "PFNAIO" | "PFNAR" | "PFRFH"
                        | "PFRNG" | "PFSLO" | "PFSLRD" | "PFSLW" | "PFSV" | "PFUNK"
                        | "PFUTMP" | "PFUTVR" | "PFVARS" | "PFVSUB"
                )
            })
            .collect();
        assert!(
            c1_diags.is_empty(),
            "No C1 parfor checks should fire on the MUST-NOT-fire example, got {:?}",
            c1_diags.iter().map(|d| d.rule_id).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_c1_checks_respect_disabled_config() {
        let mut rule_config = LanguageSpecConfig::default();
        rule_config.disabled_checks.push("PFEVC".to_string());
        rule_config.disabled_checks.push("PFANSLP".to_string());
        let engine = LanguageSpecEngine {
            config: rule_config,
        };
        let source = "\
function f()
    parfor i = 1:10
        evalin('caller', 'x');
        x(i) = i;
    end
    parfor ans = 1:10
        y = ans;
    end
end
";
        let tree = parse_matlab(source);
        let path = Path::new("f.m");
        let ctx = FileContext {
            tree: &tree,
            source,
            file_path: path,
        };
        let diags = engine.check_file(&ctx);
        assert!(
            filter_by_id(&diags, "PFEVC").is_empty(),
            "PFEVC should be disabled via config disabled_checks"
        );
        assert!(
            filter_by_id(&diags, "PFANSLP").is_empty(),
            "PFANSLP should be disabled via config disabled_checks"
        );
    }
}
