//! # SUGGESTED_IMPROVEMENTS: Suggested Improvements Lookup Engine
//!
//! A data-driven rule module that handles ~243 MATLAB Code Analyzer checks
//! about functions and patterns that are not recommended, with suggestions
//! for modern replacements. Instead of implementing one struct per check, a
//! single `SuggestedImprovementsEngine` loads a TOML data file at compile
//! time, parses it into a lookup table, and emits diagnostics with the
//! specific check ID from the matched entry.
//!
//! ## Architecture
//!
//! - A `SuggestedEntry` struct represents one check (id, function_name,
//!   message, replacement).
//! - A single `SuggestedImprovementsEngine` rule registers with inventory once.
//! - On each `function_call` or `command` node, the engine extracts the
//!   function name and performs an O(1) HashMap lookup.
//! - If a match is found, a diagnostic is emitted with the entry's specific
//!   check ID (e.g., "CSVRD"), not the meta-ID "SUGGESTED_IMPROVEMENTS".
//!
//! ## Data File
//!
//! The entries are stored in `data/suggested_improvements.toml` and loaded via
//! `include_str!` at compile time. The format is:
//!
//! ```toml
//! [[checks]]
//! id = "CSVRD"
//! function_name = "csvread"
//! message = "'csvread' is not recommended. Use 'readmatrix' instead."
//! replacement = "readmatrix"
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

/// A single suggested improvement check entry from the data file.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct SuggestedEntry {
    /// Check ID matching MATLAB Code Analyzer (e.g., "CSVRD").
    id: String,
    /// The function/class name to match against. Empty string means generic.
    function_name: String,
    /// Human-readable diagnostic message.
    message: String,
    /// Recommended replacement function or syntax.
    replacement: String,
}

/// Top-level structure of the suggested_improvements.toml data file.
#[derive(Debug, Deserialize)]
struct SuggestedData {
    checks: Vec<SuggestedEntry>,
}

// ---------------------------------------------------------------------------
// Static data — parsed once, lives for 'static
// ---------------------------------------------------------------------------

/// Raw TOML source embedded at compile time.
const SUGGESTED_TOML_SOURCE: &str = include_str!("data/suggested_improvements.toml");

/// Parsed suggested improvements data. All strings inside are owned and live
/// for 'static because this is a `LazyLock` static.
static SUGGESTED_DATA: LazyLock<SuggestedData> = LazyLock::new(|| {
    toml::from_str(SUGGESTED_TOML_SOURCE)
        .expect("failed to parse data/suggested_improvements.toml")
});

/// Lookup table: function_name → reference to `SuggestedEntry`.
///
/// Only entries with a non-empty `function_name` are included. For function
/// names that map to multiple check IDs (e.g., "maketform" maps to MTFA1,
/// MTFA2, MTFP1, MTFP2, MTFB), only the first entry is stored — the engine
/// will emit the first matching diagnostic.
static SUGGESTED_TABLE: LazyLock<HashMap<&'static str, &'static SuggestedEntry>> =
    LazyLock::new(|| {
        let data = &*SUGGESTED_DATA;
        let mut map = HashMap::with_capacity(data.checks.len());
        for entry in &data.checks {
            if !entry.function_name.is_empty() {
                // Use the first entry for a given function_name (don't overwrite).
                map.entry(entry.function_name.as_str()).or_insert(entry);
            }
        }
        map
    });

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// The suggested improvements engine — a single rule instance that checks all
/// not-recommended function usage via a data-driven lookup table.
///
/// This is registered once with inventory. Each emitted diagnostic carries the
/// specific check ID from the data file (e.g., "CSVRD"), not a generic ID.
pub struct SuggestedImprovementsEngine;

/// Target node types for the suggested improvements engine.
const TARGET_NODES: &[&str] = &["function_call", "command"];

impl SuggestedImprovementsEngine {
    /// Factory constructor called by the rule registry.
    pub fn from_config(_config: &Config) -> Box<dyn Rule> {
        // Force initialization of static data at construction time so that any
        // parse errors surface early rather than on first lint invocation.
        let _ = &*SUGGESTED_TABLE;
        Box::new(Self)
    }
}

impl Rule for SuggestedImprovementsEngine {
    fn id(&self) -> &'static str {
        "SUGGESTED_IMPROVEMENTS"
    }

    fn description(&self) -> &'static str {
        "Not-recommended function or pattern with suggested replacement"
    }

    fn severity(&self) -> Severity {
        Severity::Info
    }

    fn category(&self) -> Category {
        Category::SuggestedImprovements
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

        // Lookup in the suggested improvements table.
        let entry = match SUGGESTED_TABLE.get(func_name) {
            Some(e) => e,
            None => return Vec::new(),
        };

        let start = ctx.node.start_position();

        // The entry's `id` field is a &str pointing into the static
        // SuggestedData. Since SUGGESTED_DATA is a LazyLock static, the
        // String's backing memory lives for 'static, so we can safely
        // transmute the &str lifetime.
        // SAFETY: SUGGESTED_DATA is a static LazyLock; its contents are never
        // deallocated, so the &str reference is valid for 'static.
        let rule_id: &'static str =
            unsafe { &*(entry.id.as_str() as *const str) };

        vec![Diagnostic {
            rule_id,
            message: entry.message.clone(),
            severity: Severity::Info,
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
/// and dotted names like `sigwin.hamming`).
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

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "SUGGESTED_IMPROVEMENTS",
    SuggestedImprovementsEngine::from_config
));
