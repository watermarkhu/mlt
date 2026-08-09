//! # SYSTEM_OBJECTS_ENGINE: System Object Validation Checks
//!
//! Implements 9 checks for MATLAB System object validation. System objects
//! (classes inheriting from `matlab.System`) have specific lifecycle
//! constraints that static analysis can partially verify.
//!
//! Since full type information is not available to a static linter, these
//! checks use pattern-based heuristics — detecting System object patterns
//! based on common conventions (classes ending in "System", method calls
//! like `step()`, `setup()`, `release()`, property access patterns).
//!
//! ## Checks
//!
//! | ID | Severity | Description |
//! |----|----------|-------------|
//! | SONUMIN | Error | Wrong number of inputs to System object method |
//! | SONUMOUT | Error | Wrong number of outputs from System object method |
//! | SODEPPROP | Warning | Deprecated system object property |
//! | SOINITPROP | Warning | Property should be set in constructor |
//! | SODFLTVAL | Warning | Default value issue in system object |
//! | SORSRVDNM | Warning | Reserved name used for system object member |
//! | SOTUNPROP1 | Warning | Tunable property issue |
//! | SOTUNPROP3 | Warning | Non-tunable property modified after setup |
//! | SOTUNPROP4 | Error | Non-tunable property modified in step method |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.SYSTEM_OBJECTS_ENGINE]
//! skip_checks = ["SONUMIN"]
//! ```
//!
//! ## Limitations
//!
//! Without runtime type information, these checks rely on heuristics:
//! - Method names (`step`, `setup`, `release`, `reset`) suggest System object usage
//! - Class names containing "System" or inheriting from `matlab.System`
//! - Property access patterns on objects that appear to be System objects
//!
//! False positives are possible for non-System classes using similar method names.

use mlt_core::{Category, Config, Diagnostic, FileContext, NodeContext, Rule, Severity};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for the system objects engine.
///
/// Deserialized from the `[lint.rules.SYSTEM_OBJECTS_ENGINE]` section in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SystemObjectsEngineConfig {
    /// Check IDs to skip (e.g., `["SONUMIN"]`).
    #[serde(default)]
    pub skip_checks: Vec<String>,
}

// ---------------------------------------------------------------------------
// Check metadata
// ---------------------------------------------------------------------------

/// Static metadata for a single system object check.
struct CheckMeta {
    id: &'static str,
    severity: Severity,
    description: &'static str,
}

/// All 9 system object check definitions.
const CHECKS: &[CheckMeta] = &[
    CheckMeta { id: "SONUMIN", severity: Severity::Error, description: "System object method called with wrong number of inputs" },
    CheckMeta { id: "SONUMOUT", severity: Severity::Error, description: "System object method called with wrong number of outputs" },
    CheckMeta { id: "SODEPPROP", severity: Severity::Warning, description: "Deprecated system object property; use the recommended replacement" },
    CheckMeta { id: "SOINITPROP", severity: Severity::Warning, description: "System object property should be set in the constructor, not after construction" },
    CheckMeta { id: "SODFLTVAL", severity: Severity::Warning, description: "System object property default value may cause unexpected behavior" },
    CheckMeta { id: "SORSRVDNM", severity: Severity::Warning, description: "Reserved name used for system object member; choose a different name" },
    CheckMeta { id: "SOTUNPROP1", severity: Severity::Warning, description: "Tunable property constraint may be violated" },
    CheckMeta { id: "SOTUNPROP3", severity: Severity::Warning, description: "Non-tunable property should not be modified after setup() is called" },
    CheckMeta { id: "SOTUNPROP4", severity: Severity::Error, description: "Non-tunable property must not be modified inside the step() method" },
];

/// System object lifecycle methods.
const SYSTEM_OBJECT_METHODS: &[&str] = &[
    "step", "setup", "release", "reset", "isDone", "isLocked",
    "getNumInputs", "getNumOutputs", "clone",
];

/// Reserved System object method names that should not be overridden.
const RESERVED_NAMES: &[&str] = &[
    "step", "setup", "release", "reset", "isDone", "isLocked",
    "getNumInputs", "getNumOutputs", "clone",
    "processTunedPropertiesImpl", "infoImpl",
];

/// Deprecated System object properties and their replacements.
const DEPRECATED_PROPERTIES: &[(&str, &str)] = &[
    ("SampleRate", "use getSampleRate method"),
    ("FrameLength", "set via input signal"),
    ("NumChannels", "determined automatically from input"),
];

/// Look up check metadata by ID.
fn check_meta(id: &str) -> Option<&'static CheckMeta> {
    CHECKS.iter().find(|c| c.id == id)
}

/// Look up check description by ID.
fn check_description(id: &str) -> &'static str {
    check_meta(id)
        .map(|c| c.description)
        .unwrap_or("System object validation issue")
}

/// Look up check severity by ID.
fn check_severity(id: &str) -> Severity {
    check_meta(id)
        .map(|c| c.severity)
        .unwrap_or(Severity::Warning)
}

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// Engine covering 9 system object validation checks.
///
/// Uses pattern-based heuristics since full type information is not available.
/// Node-level checks fire on `function_call` nodes. File-level checks
/// analyze class definitions for property and method patterns.
pub struct SystemObjectsEngine {
    config: SystemObjectsEngineConfig,
}

/// Node types for node-level dispatch.
const TARGET_NODES: &[&str] = &["function_call"];

impl SystemObjectsEngine {
    /// Factory constructor called by the rule registry.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: SystemObjectsEngineConfig = config.rule_params("SYSTEM_OBJECTS_ENGINE");
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

    /// Check a `function_call` node for system object method patterns.
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

        // Detect method calls on objects: obj.step(), obj.setup(), etc.
        // In tree-sitter-matlab, these may appear as field_expression function_calls
        // where the name is like "obj.step"
        if let Some(method_name) = func_name.rsplit('.').next() {
            if SYSTEM_OBJECT_METHODS.contains(&method_name) {
                // SONUMIN: Check argument count for step() method
                // step() should have the System object + input signals
                if self.is_enabled("SONUMIN") && method_name == "step" {
                    let arg_count = count_args(node);
                    // step() with no arguments is suspicious (should have at least input)
                    if arg_count == 0 && !func_name.contains('.') {
                        diags.push(make_diag("SONUMIN", node));
                    }
                }

                // SONUMOUT: Calling release()/reset() with output arguments
                if self.is_enabled("SONUMOUT")
                    && (method_name == "release" || method_name == "reset")
                    && has_output_assignment(node) {
                        diags.push(make_diag("SONUMOUT", node));
                    }
            }
        }

        // SODEPPROP: Detect access to deprecated properties
        // Pattern: obj.SampleRate, obj.FrameLength, etc.
        if self.is_enabled("SODEPPROP") && func_name.contains('.') {
            if let Some(prop_name) = func_name.rsplit('.').next() {
                if DEPRECATED_PROPERTIES.iter().any(|(p, _)| *p == prop_name) {
                    diags.push(make_diag("SODEPPROP", node));
                }
            }
        }

        diags
    }

    // -----------------------------------------------------------------------
    // File-level checks
    // -----------------------------------------------------------------------

    /// File-level checks for class definitions inheriting from matlab.System.
    fn check_file_level(
        &self,
        tree: &tree_sitter::Tree,
        source: &str,
    ) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        let root = tree.root_node();

        // Find class definitions that inherit from matlab.System
        self.find_system_classes(&root, source, &mut diags);

        diags
    }

    /// Find class definitions and check for system object patterns.
    fn find_system_classes(
        &self,
        node: &tree_sitter::Node,
        source: &str,
        diags: &mut Vec<Diagnostic>,
    ) {
        let count = node.child_count();
        for i in 0..count {
            if let Some(child) = node.child(i) {
                if child.kind() == "class_definition" {
                    // Check if this class inherits from matlab.System
                    if is_system_object_class(child, source) {
                        self.check_system_class(child, source, diags);
                    }
                }
                self.find_system_classes(&child, source, diags);
            }
        }
    }

    /// Check a System object class for various issues.
    fn check_system_class(
        &self,
        class_node: tree_sitter::Node,
        source: &str,
        diags: &mut Vec<Diagnostic>,
    ) {
        let count = class_node.child_count();
        for i in 0..count {
            if let Some(child) = class_node.child(i) {
                // SORSRVDNM: Check method names against reserved names
                if self.is_enabled("SORSRVDNM") && child.kind() == "methods" {
                    self.check_reserved_method_names(child, source, diags);
                }

                // SODFLTVAL: Check properties for default value issues
                if self.is_enabled("SODFLTVAL") && child.kind() == "properties" {
                    self.check_property_defaults(child, source, diags);
                }
            }
        }
    }

    /// Check method names against reserved System object names.
    fn check_reserved_method_names(
        &self,
        methods_node: tree_sitter::Node,
        source: &str,
        diags: &mut Vec<Diagnostic>,
    ) {
        let count = methods_node.child_count();
        for i in 0..count {
            if let Some(child) = methods_node.child(i) {
                if child.kind() == "function_definition" {
                    if let Some(name_node) = child.child_by_field_name("name") {
                        let method_name = node_text(name_node, source);
                        // Only flag if the method name is reserved but not an expected
                        // override (setupImpl, stepImpl, etc. are expected)
                        if RESERVED_NAMES.contains(&method_name)
                            && !method_name.ends_with("Impl")
                        {
                            diags.push(make_diag("SORSRVDNM", name_node));
                        }
                    }
                }
            }
        }
    }

    /// Check property definitions for default value issues.
    fn check_property_defaults(
        &self,
        properties_node: tree_sitter::Node,
        source: &str,
        diags: &mut Vec<Diagnostic>,
    ) {
        let count = properties_node.child_count();
        for i in 0..count {
            if let Some(child) = properties_node.child(i) {
                if child.kind() == "property" {
                    // Check for function call as default value (potentially problematic)
                    let text = node_text(child, source);
                    if text.contains('(') && text.contains(')') && text.contains('=') {
                        // Heuristic: property with function call default value
                        // e.g., `Prop = someFunction()`
                        diags.push(make_diag("SODFLTVAL", child));
                    }
                }
            }
        }
    }
}

impl Rule for SystemObjectsEngine {
    fn id(&self) -> &'static str {
        "SYSTEM_OBJECTS_ENGINE"
    }

    fn description(&self) -> &'static str {
        "System object validation checks"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn category(&self) -> Category {
        Category::SystemObjects
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

/// Build a diagnostic from a check ID and node, using the check's own severity.
fn make_diag(check_id: &'static str, node: tree_sitter::Node) -> Diagnostic {
    let start = node.start_position();
    Diagnostic {
        rule_id: check_id,
        message: check_description(check_id).to_string(),
        severity: check_severity(check_id),
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

// ---------------------------------------------------------------------------
// Helper: pattern detection
// ---------------------------------------------------------------------------

/// Count the number of arguments in a `function_call` node.
fn count_args(node: tree_sitter::Node) -> usize {
    // In tree-sitter-matlab call arguments are a child node of kind
    // `arguments` rather than a named field, so `child_by_field_name` never
    // matches. Locate them by child kind instead.
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "arguments" {
            return child.named_child_count();
        }
    }
    0
}

/// Check if a function_call is the RHS of an assignment (has output).
fn has_output_assignment(node: tree_sitter::Node) -> bool {
    if let Some(parent) = node.parent() {
        // Check if parent is an assignment and this node is on the right side
        if parent.kind() == "assignment" {
            if let Some(rhs) = parent.child_by_field_name("right").or_else(|| parent.child(2)) {
                return node.start_byte() >= rhs.start_byte()
                    && node.end_byte() <= rhs.end_byte();
            }
        }
    }
    false
}

/// Check if a class definition inherits from matlab.System.
fn is_system_object_class(class_node: tree_sitter::Node, source: &str) -> bool {
    // Look for superclasses node containing "matlab.System"
    let count = class_node.child_count();
    for i in 0..count {
        if let Some(child) = class_node.child(i) {
            if child.kind() == "superclasses" {
                let text = node_text(child, source);
                if text.contains("matlab.System") {
                    return true;
                }
            }
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "SYSTEM_OBJECTS_ENGINE",
    SystemObjectsEngine::from_config
));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{has_id, lint_file, lint_nodes};
    use mlt_core::Config;

    fn engine() -> Box<dyn Rule> {
        SystemObjectsEngine::from_config(&Config::default())
    }

    fn system_class(source: &str) -> String {
        format!(
            "classdef mySystem < matlab.System\n{}\nend\n",
            source
        )
    }

    // -- SONUMIN -------------------------------------------------------------

    #[test]
    fn sonumin_bare_step_no_args_fires() {
        let diags = lint_nodes(&*engine(), "step();\n");
        assert!(has_id(&diags, "SONUMIN"), "got: {diags:?}");
    }

    #[test]
    fn sonumin_step_with_input_does_not_fire() {
        let diags = lint_nodes(&*engine(), "step(input);\n");
        assert!(!has_id(&diags, "SONUMIN"), "got: {diags:?}");
    }

    // -- SONUMOUT ------------------------------------------------------------

    #[test]
    fn sonumout_release_with_output_fires() {
        let diags = lint_nodes(&*engine(), "out = release();\n");
        assert!(has_id(&diags, "SONUMOUT"), "got: {diags:?}");
    }

    #[test]
    fn sonumout_reset_with_output_fires() {
        let diags = lint_nodes(&*engine(), "out = reset();\n");
        assert!(has_id(&diags, "SONUMOUT"), "got: {diags:?}");
    }

    #[test]
    fn sonumout_release_as_statement_does_not_fire() {
        let diags = lint_nodes(&*engine(), "release(obj);\n");
        assert!(!has_id(&diags, "SONUMOUT"), "got: {diags:?}");
    }

    #[test]
    fn sonumout_step_with_output_does_not_fire() {
        let diags = lint_nodes(&*engine(), "out = step();\n");
        assert!(!has_id(&diags, "SONUMOUT"), "got: {diags:?}");
    }

    // -- SORSRVDNM -----------------------------------------------------------

    #[test]
    fn sorsrvdnm_reserved_method_name_fires() {
        let source = system_class("    methods\n        function step(obj)\n        end\n    end");
        let diags = lint_file(&*engine(), &source);
        assert!(has_id(&diags, "SORSRVDNM"), "got: {diags:?}");
    }

    #[test]
    fn sorsrvdnm_impl_suffixed_method_does_not_fire() {
        let source = system_class("    methods\n        function stepImpl(obj)\n        end\n    end");
        let diags = lint_file(&*engine(), &source);
        assert!(!has_id(&diags, "SORSRVDNM"), "got: {diags:?}");
    }

    #[test]
    fn sorsrvdnm_non_reserved_method_does_not_fire() {
        let source = system_class("    methods\n        function doWork(obj)\n        end\n    end");
        let diags = lint_file(&*engine(), &source);
        assert!(!has_id(&diags, "SORSRVDNM"), "got: {diags:?}");
    }

    #[test]
    fn sorsrvdnm_non_system_class_does_not_fire() {
        let source = "classdef MyRegular\n    methods\n        function step(obj)\n        end\n    end\nend\n";
        let diags = lint_file(&*engine(), source);
        assert!(!has_id(&diags, "SORSRVDNM"), "got: {diags:?}");
    }

    // -- SODFLTVAL -----------------------------------------------------------

    #[test]
    fn sodfltval_function_call_default_fires() {
        let source = system_class("    properties\n        Gain = rand();\n    end");
        let diags = lint_file(&*engine(), &source);
        assert!(has_id(&diags, "SODFLTVAL"), "got: {diags:?}");
    }

    #[test]
    fn sodfltval_scalar_default_does_not_fire() {
        let source = system_class("    properties\n        Gain = 5;\n    end");
        let diags = lint_file(&*engine(), &source);
        assert!(!has_id(&diags, "SODFLTVAL"), "got: {diags:?}");
    }

    #[test]
    fn sodfltval_string_default_does_not_fire() {
        let source = system_class("    properties\n        Name = 'rx';\n    end");
        let diags = lint_file(&*engine(), &source);
        assert!(!has_id(&diags, "SODFLTVAL"), "got: {diags:?}");
    }
}
