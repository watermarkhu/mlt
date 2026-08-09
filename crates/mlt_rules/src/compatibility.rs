//! # COMPAT: Compatibility Lookup Engine
//!
//! A data-driven rule module that handles ~1,803 MATLAB Code Analyzer checks
//! about deprecated/removed functions and behavior changes. Instead of
//! implementing one struct per check, a single `CompatibilityEngine` loads a
//! TOML data file at compile time, parses it into a lookup table, and emits
//! diagnostics with the specific check ID from the matched entry.
//!
//! ## Architecture
//!
//! - A `CompatEntry` struct represents one check (id, severity, message,
//!   function_name, category sub-type).
//! - A single `CompatibilityEngine` rule registers with inventory once.
//! - On each `function_call` or `command` node, the engine extracts the
//!   function name and performs an O(1) HashMap lookup.
//! - If a match is found, a diagnostic is emitted with the entry's specific
//!   check ID (e.g., "DPSD"), not the meta-ID "COMPAT".
//!
//! ## Data File
//!
//! The entries are stored in `data/compatibility.toml` and loaded via
//! `include_str!` at compile time. The format is:
//!
//! ```toml
//! [[checks]]
//! id = "DPSD"
//! function_name = "psd"
//! severity = "error"
//! message = "'psd' has been removed. Use 'periodogram' or 'pwelch' instead."
//! category = "compatibility"
//! ```
//!
//! Entries with `function_name = ""` are "generic" checks that cannot be
//! matched by function name alone (they require AST pattern matching) and
//! are skipped during the lookup-based check.

use std::collections::HashMap;
use std::sync::LazyLock;

use mlt_core::{Category, Config, Diagnostic, NodeContext, Rule, Severity};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Data types for TOML deserialization
// ---------------------------------------------------------------------------

/// A single compatibility check entry from the data file.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct CompatEntry {
    /// Check ID matching MATLAB Code Analyzer (e.g., "DPSD").
    id: String,
    /// The function/class name to match against. Empty string means generic.
    function_name: String,
    /// Severity level: "error" or "warning".
    severity: String,
    /// Human-readable diagnostic message.
    message: String,
    /// Category: "compatibility", "forward-compatibility", or "behavior-changes".
    category: String,
    /// Optional pattern marker. "generic" means this check cannot be matched
    /// by function name alone.
    #[serde(default)]
    pattern: String,
}

/// Top-level structure of the compatibility.toml data file.
#[derive(Debug, Deserialize)]
struct CompatData {
    checks: Vec<CompatEntry>,
}

// ---------------------------------------------------------------------------
// Static data — parsed once, lives for 'static
// ---------------------------------------------------------------------------

/// Raw TOML source embedded at compile time.
const COMPAT_TOML_SOURCE: &str = include_str!("data/compatibility.toml");

/// Parsed compatibility data. All strings inside are owned and live for 'static
/// because this is a `LazyLock` static.
static COMPAT_DATA: LazyLock<CompatData> = LazyLock::new(|| {
    toml::from_str(COMPAT_TOML_SOURCE).expect("failed to parse data/compatibility.toml")
});

/// Lookup table: function_name → index into `COMPAT_DATA.checks`.
///
/// Only entries with a non-empty `function_name` and no "generic" pattern are
/// included. For function names that map to multiple check IDs (e.g., "tcpip"
/// maps to both TCPC and TCPS), only the first entry is stored — the engine
/// will emit the first matching diagnostic.
static COMPAT_TABLE: LazyLock<HashMap<&'static str, &'static CompatEntry>> = LazyLock::new(|| {
    let data = &*COMPAT_DATA;
    let mut map = HashMap::with_capacity(data.checks.len());
    for entry in &data.checks {
        if !entry.function_name.is_empty() && entry.pattern != "generic" {
            // Use the first entry for a given function_name (don't overwrite).
            map.entry(entry.function_name.as_str()).or_insert(entry);
        }
    }
    map
});

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// The compatibility engine — a single rule instance that checks all
/// deprecated/removed function usage via a data-driven lookup table.
///
/// This is registered once with inventory. Each emitted diagnostic carries the
/// specific check ID from the data file (e.g., "DPSD"), not a generic ID.
pub struct CompatibilityEngine;

/// Target node types for the compatibility engine.
const TARGET_NODES: &[&str] = &["function_call", "command"];

impl CompatibilityEngine {
    /// Factory constructor called by the rule registry.
    pub fn from_config(_config: &Config) -> Box<dyn Rule> {
        // Force initialization of static data at construction time so that any
        // parse errors surface early rather than on first lint invocation.
        let _ = &*COMPAT_TABLE;
        Box::new(Self)
    }
}

impl Rule for CompatibilityEngine {
    fn id(&self) -> &'static str {
        "COMPAT"
    }

    fn description(&self) -> &'static str {
        "Deprecated or removed function usage"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn category(&self) -> Category {
        Category::Compatibility
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        TARGET_NODES
    }

    fn check(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        // Extract the function/command name from the node.
        let func_name = match extract_func_name(ctx.node, ctx.source) {
            Some(name) => name,
            None => return Vec::new(),
        };

        // Lookup in the compatibility table.
        let entry = match COMPAT_TABLE.get(func_name) {
            Some(e) => e,
            None => return Vec::new(),
        };

        let start = ctx.node.start_position();
        let severity = parse_severity(&entry.severity);

        // The entry's `id` field is a &str pointing into the static CompatData.
        // Since COMPAT_DATA is a LazyLock static, the String's backing memory
        // lives for 'static, so we can safely transmute the &str lifetime.
        // SAFETY: COMPAT_DATA is a static LazyLock; its contents are never
        // deallocated, so the &str reference is valid for 'static.
        let rule_id: &'static str =
            unsafe { &*(entry.id.as_str() as *const str) };

        vec![Diagnostic {
            rule_id,
            message: entry.message.clone(),
            severity,
            byte_range: ctx.node.start_byte()..ctx.node.end_byte(),
            line: start.row + 1,
            column: start.column + 1,
            fix: None,
        }]
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract the function/command name from a `function_call` or `command` node.
///
/// For `function_call`: extracts the `name` field (handles simple identifiers
/// and dotted names like `dsp.FIRFilter`).
///
/// For `command`: extracts the first child (the command name).
fn extract_func_name<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> Option<&'a str> {
    match node.kind() {
        "function_call" => {
            let name_node = node.child_by_field_name("name")?;
            Some(&source[name_node.start_byte()..name_node.end_byte()])
        }
        "command" => {
            let name_node = node.child(0)?;
            if name_node.kind() == "command_name" {
                Some(&source[name_node.start_byte()..name_node.end_byte()])
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Parse a severity string from the TOML data into a `Severity` enum value.
fn parse_severity(s: &str) -> Severity {
    match s {
        "error" => Severity::Error,
        "warning" => Severity::Warning,
        "info" => Severity::Info,
        _ => Severity::Warning,
    }
}

/// Determine the category from the entry's category string.
///
/// This is used internally for documentation/reference but the engine always
/// reports `Category::Compatibility` as its own category for registry purposes.
/// Individual diagnostics carry the specific check ID.
#[allow(dead_code)]
fn parse_category(s: &str) -> Category {
    match s {
        "compatibility" => Category::Compatibility,
        "forward-compatibility" => Category::ForwardCompatibility,
        "behavior-changes" => Category::BehaviorChanges,
        _ => Category::Compatibility,
    }
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "COMPAT",
    CompatibilityEngine::from_config
));

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{has_id, lint_nodes};
    use mlt_core::Config;

    fn engine() -> Box<dyn Rule> {
        CompatibilityEngine::from_config(&Config::default())
    }

    /// Assert that calling `name` (function-call form) emits a diagnostic with
    /// the expected check ID and nothing else.
    fn assert_call_fires(name: &str, expected_id: &str) {
        let source = format!("{name}(x);\n");
        let diags = lint_nodes(&*engine(), &source);
        assert!(
            has_id(&diags, expected_id),
            "expected {expected_id} for `{name}(x)`; got: {diags:?}"
        );
        assert!(
            !has_id(&diags, "COMPAT"),
            "diagnostic should carry the specific check ID, not COMPAT; got: {diags:?}"
        );
    }

    /// Assert that `name` in command form (`name arg`) emits the expected ID.
    fn assert_command_fires(name: &str, expected_id: &str) {
        let source = format!("{name} arg\n");
        let diags = lint_nodes(&*engine(), &source);
        assert!(
            has_id(&diags, expected_id),
            "expected {expected_id} for `{name} arg`; got: {diags:?}"
        );
        assert!(
            !has_id(&diags, "COMPAT"),
            "diagnostic should carry the specific check ID, not COMPAT; got: {diags:?}"
        );
    }

    // -- function_call form: deprecated signal-processing names ---------------

    #[test]
    fn psd_fires_dpsd() {
        assert_call_fires("psd", "DPSD");
    }

    #[test]
    fn inline_fires_dinln() {
        assert_call_fires("inline", "DINLN");
    }

    #[test]
    fn fcnchk_fires_dfcnchk() {
        assert_call_fires("fcnchk", "DFCNCHK");
    }

    #[test]
    fn matlabpool_fires_matpool() {
        assert_call_fires("matlabpool", "MATPOOL");
    }

    #[test]
    fn treedisp_fires_treedisp() {
        assert_call_fires("treedisp", "TREEDISP");
    }

    #[test]
    fn treefit_fires_treefit() {
        assert_call_fires("treefit", "TREEFIT");
    }

    #[test]
    fn bitmax_fires_dbitmax() {
        assert_call_fires("bitmax", "DBITMAX");
    }

    #[test]
    fn colordef_fires_colordef() {
        assert_call_fires("colordef", "COLORDEF");
    }

    #[test]
    fn whitebg_fires_whitebg() {
        assert_call_fires("whitebg", "WHITEBG");
    }

    #[test]
    fn textread_fires_dtextread() {
        assert_call_fires("textread", "DTEXTREAD");
    }

    // -- command form --------------------------------------------------------

    #[test]
    fn command_form_matlabpool_fires_matpool() {
        assert_command_fires("matlabpool", "MATPOOL");
    }

    #[test]
    fn command_form_whitebg_fires_whitebg() {
        assert_command_fires("whitebg", "WHITEBG");
    }

    #[test]
    fn command_form_colordef_fires_colordef() {
        assert_command_fires("colordef", "COLORDEF");
    }

    #[test]
    fn command_form_mupad_fires_mupad() {
        assert_command_fires("mupad", "MUPAD");
    }

    // -- negative tests: modern replacements do not fire ---------------------

    #[test]
    fn modern_periodogram_not_flagged() {
        let diags = lint_nodes(&*engine(), "periodogram(x);\n");
        assert!(!has_id(&diags, "DPSD"), "got: {diags:?}");
        assert!(diags.is_empty(), "got: {diags:?}");
    }

    #[test]
    fn modern_strcmp_not_flagged() {
        let diags = lint_nodes(&*engine(), "strcmp('a', 'b');\n");
        assert!(diags.is_empty(), "got: {diags:?}");
    }

    #[test]
    fn modern_plot_not_flagged() {
        let diags = lint_nodes(&*engine(), "plot(x, y);\n");
        assert!(diags.is_empty(), "got: {diags:?}");
    }

    #[test]
    fn modern_fitctree_not_flagged() {
        let diags = lint_nodes(&*engine(), "fitctree(x, y);\n");
        assert!(!has_id(&diags, "TREEFIT"), "got: {diags:?}");
    }

    #[test]
    fn modern_command_disp_not_flagged() {
        let diags = lint_nodes(&*engine(), "disp hello\n");
        assert!(diags.is_empty(), "got: {diags:?}");
    }

    #[test]
    fn deprecated_name_as_variable_not_flagged() {
        let diags = lint_nodes(&*engine(), "psd = 5;\n");
        assert!(!has_id(&diags, "DPSD"), "got: {diags:?}");
    }
}
