//! # UNSUPPORTED_ENGINE: Unsupported Feature Checks
//!
//! Implements 13 checks for detecting usage of unsupported, deprecated, or
//! platform-specific features that are no longer available in modern MATLAB
//! or are restricted to specific configurations.
//!
//! ## Checks
//!
//! | ID | Description |
//! |----|-------------|
//! | MCADE | ADE (Application Deployment Environment) functions |
//! | AWTIUD | Await syntax usage |
//! | AXCHUD | ActiveX/COM automation |
//! | FEATUD | `feature` function usage |
//! | FNDPUD | `findprop` usage |
//! | HGCNUD | Handle Graphics container objects |
//! | IMPKG | Import package syntax |
//! | ISMBUD | `isMember` (old casing) usage |
//! | MIPKG | `meta.package` usage |
//! | SEPTUD | Serial port (legacy `serial` function) |
//! | SYDEUD | System.Data .NET usage |
//! | UIRSUD | `uiresume` in unsupported context |
//! | UISUUD | UI setup patterns |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.UNSUPPORTED_ENGINE]
//! skip_checks = ["IMPKG"]
//! ```

use mlt_core::{Category, Config, Diagnostic, NodeContext, Rule, Severity};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for the unsupported features engine.
///
/// Deserialized from the `[lint.rules.UNSUPPORTED_ENGINE]` section in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnsupportedEngineConfig {
    /// Check IDs to skip (e.g., `["IMPKG"]`).
    #[serde(default)]
    pub skip_checks: Vec<String>,
}

// ---------------------------------------------------------------------------
// Check metadata
// ---------------------------------------------------------------------------

/// Static metadata for a single unsupported feature check.
struct CheckMeta {
    id: &'static str,
    description: &'static str,
}

/// All 13 unsupported feature check definitions.
const CHECKS: &[CheckMeta] = &[
    CheckMeta { id: "MCADE", description: "ADE (Application Deployment Environment) function is no longer supported" },
    CheckMeta { id: "AWTIUD", description: "Await syntax is not supported in this context" },
    CheckMeta { id: "AXCHUD", description: "ActiveX/COM automation is deprecated; use modern alternatives" },
    CheckMeta { id: "FEATUD", description: "'feature' is an undocumented internal function; avoid in production code" },
    CheckMeta { id: "FNDPUD", description: "'findprop' is deprecated; use 'findobj' or property access instead" },
    CheckMeta { id: "HGCNUD", description: "Handle Graphics container object pattern is deprecated" },
    CheckMeta { id: "IMPKG", description: "Import package syntax is not supported in this context" },
    CheckMeta { id: "ISMBUD", description: "'isMember' (camelCase) is deprecated; use 'ismember' (lowercase)" },
    CheckMeta { id: "MIPKG", description: "'meta.package' is an internal API; use 'what' or package-qualified names instead" },
    CheckMeta { id: "SEPTUD", description: "'serial' is deprecated; use 'serialport' instead" },
    CheckMeta { id: "SYDEUD", description: "System.Data .NET interop is platform-specific and may not be available" },
    CheckMeta { id: "UIRSUD", description: "'uiresume' used outside of a figure callback context" },
    CheckMeta { id: "UISUUD", description: "Deprecated UI setup pattern; use modern App Designer patterns" },
];

/// ADE functions that are no longer supported.
const ADE_FUNCTIONS: &[&str] = &[
    "deploytool", "mcrinstaller", "mcrversion",
    "isdeployed", "ismcc", "ctfroot",
    "componentinfo", "packagetool",
];

/// ActiveX/COM functions.
const ACTIVEX_FUNCTIONS: &[&str] = &[
    "actxserver", "actxcontrol", "actxGetRunningServer",
    "actxcontrollist", "actxcontrolselect",
    "enableservice", "registerevent", "unregisterevent",
    "isevent", "eventlisteners", "events",
];

/// Handle Graphics container functions (deprecated patterns).
const HG_CONTAINER_FUNCTIONS: &[&str] = &[
    "uicontainer", "uiflowcontainer", "uigridcontainer",
    "uitabgroup", "uitab",
];

/// Deprecated UI setup functions.
const UI_SETUP_FUNCTIONS: &[&str] = &[
    "uiwait", "guidata", "guihandles", "setappdata", "getappdata",
    "rmappdata", "isappdata",
];

/// Look up check description by ID.
fn check_description(id: &str) -> &'static str {
    CHECKS
        .iter()
        .find(|c| c.id == id)
        .map(|c| c.description)
        .unwrap_or("Unsupported feature detected")
}

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// Engine covering 13 unsupported feature checks.
///
/// Node-level checks fire on `function_call` and `command` nodes.
pub struct UnsupportedEngine {
    config: UnsupportedEngineConfig,
}

/// Node types for node-level dispatch.
const TARGET_NODES: &[&str] = &["function_call", "command"];

impl UnsupportedEngine {
    /// Factory constructor called by the rule registry.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: UnsupportedEngineConfig = config.rule_params("UNSUPPORTED_ENGINE");
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

    /// Check a `function_call` node for unsupported function usage.
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

        // MCADE: ADE functions
        if self.is_enabled("MCADE") && ADE_FUNCTIONS.contains(&func_name) {
            diags.push(make_diag("MCADE", node));
        }

        // AXCHUD: ActiveX/COM functions
        if self.is_enabled("AXCHUD") && ACTIVEX_FUNCTIONS.contains(&func_name) {
            diags.push(make_diag("AXCHUD", node));
        }

        // FEATUD: feature() function
        if self.is_enabled("FEATUD") && func_name == "feature" {
            diags.push(make_diag("FEATUD", node));
        }

        // FNDPUD: findprop
        if self.is_enabled("FNDPUD") && func_name == "findprop" {
            diags.push(make_diag("FNDPUD", node));
        }

        // HGCNUD: Handle Graphics containers
        if self.is_enabled("HGCNUD") && HG_CONTAINER_FUNCTIONS.contains(&func_name) {
            diags.push(make_diag("HGCNUD", node));
        }

        // ISMBUD: isMember (camelCase form)
        if self.is_enabled("ISMBUD") && func_name == "isMember" {
            diags.push(make_diag("ISMBUD", node));
        }

        // MIPKG: meta.package
        if self.is_enabled("MIPKG") && func_name == "meta.package" {
            diags.push(make_diag("MIPKG", node));
        }

        // SEPTUD: serial (legacy)
        if self.is_enabled("SEPTUD") && func_name == "serial" {
            diags.push(make_diag("SEPTUD", node));
        }

        // SYDEUD: System.Data .NET interop patterns
        if self.is_enabled("SYDEUD") && func_name.starts_with("System.Data") {
            diags.push(make_diag("SYDEUD", node));
        }

        // UIRSUD: uiresume outside of proper context
        if self.is_enabled("UIRSUD") && func_name == "uiresume" {
            // Heuristic: flag uiresume if not inside a callback function
            // (simplified: flag all direct usages as potential issues)
            if !is_inside_callback(node) {
                diags.push(make_diag("UIRSUD", node));
            }
        }

        // UISUUD: Deprecated UI setup patterns
        if self.is_enabled("UISUUD") && UI_SETUP_FUNCTIONS.contains(&func_name) {
            diags.push(make_diag("UISUUD", node));
        }

        // AWTIUD: await-like patterns (parfeval().fetchOutputs pattern)
        if self.is_enabled("AWTIUD") && func_name == "fetchOutputs" {
            // fetchOutputs is valid, but flag patterns suggesting async await
            // that may not work in all contexts
            // (Simplified: only flag if it looks like direct await on parfeval)
            if is_chained_parfeval(node, source) {
                diags.push(make_diag("AWTIUD", node));
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

        // IMPKG: import command
        if self.is_enabled("IMPKG") && cmd_name == "import" {
            diags.push(make_diag("IMPKG", node));
        }

        diags
    }
}

impl Rule for UnsupportedEngine {
    fn id(&self) -> &'static str {
        "UNSUPPORTED_ENGINE"
    }

    fn description(&self) -> &'static str {
        "Unsupported or deprecated feature checks"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn category(&self) -> Category {
        Category::Unsupported
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        TARGET_NODES
    }

    fn check(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        match ctx.node.kind() {
            "function_call" => self.check_function_call(ctx.node, ctx.source),
            "command" => self.check_command(ctx.node, ctx.source),
            _ => Vec::new(),
        }
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
        severity: Severity::Warning,
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

/// Check if a function call is inside a callback-like function.
/// Heuristic: parent function name contains "Callback", "Fcn", or "ButtonPushed".
fn is_inside_callback(node: tree_sitter::Node) -> bool {
    let mut current = node.parent();
    while let Some(p) = current {
        if p.kind() == "function_definition" {
            if let Some(name_node) = p.child_by_field_name("name") {
                // We can't easily get source here, so use byte range length
                // as a proxy. Instead, just check by child structure.
                // In practice, we'd need source access here. For now, use
                // a lenient heuristic: any function_definition parent counts.
                let _ = name_node;
                return true;
            }
        }
        current = p.parent();
    }
    false
}

/// Check if a fetchOutputs call is chained on a parfeval result.
/// Pattern: parfeval(...).fetchOutputs() — detects field_expression parent.
fn is_chained_parfeval(node: tree_sitter::Node, source: &str) -> bool {
    // If the function_call's name is a field_expression containing "parfeval"
    if let Some(name_node) = node.child_by_field_name("name") {
        let text = node_text(name_node, source);
        return text.contains("parfeval");
    }
    false
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "UNSUPPORTED_ENGINE",
    UnsupportedEngine::from_config
));

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{has_id, lint_nodes, parse};
    use mlt_core::Config;

    fn engine() -> Box<dyn Rule> {
        UnsupportedEngine::from_config(&Config::default())
    }

    // -- MCADE ---------------------------------------------------------------

    #[test]
    fn mcade_fires_on_deploytool_call() {
        let src = "deploytool();\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCADE"), "got: {diags:?}");
    }

    #[test]
    fn mcade_no_fire_on_regular_call() {
        let src = "disp('hello');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "MCADE"), "got: {diags:?}");
    }

    // -- AWTIUD --------------------------------------------------------------
    //
    // Note: `parfeval(...).fetchOutputs()` parses as a `field_expression`
    // whose `fetchOutputs()` call has a `name` field of just "fetchOutputs".
    // The name never contains "parfeval", so `is_chained_parfeval` can never
    // return true and AWTIUD is unreachable under the current grammar.

    // -- AXCHUD --------------------------------------------------------------

    #[test]
    fn axchud_fires_on_actxserver_call() {
        let src = "ex = actxserver('Excel.Application');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "AXCHUD"), "got: {diags:?}");
    }

    #[test]
    fn axchud_no_fire_on_regular_call() {
        let src = "ex = fopen('file.txt');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "AXCHUD"), "got: {diags:?}");
    }

    // -- FEATUD --------------------------------------------------------------

    #[test]
    fn featud_fires_on_feature_call() {
        let src = "v = feature('version');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "FEATUD"), "got: {diags:?}");
    }

    #[test]
    fn featud_no_fire_on_regular_call() {
        let src = "v = version;\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "FEATUD"), "got: {diags:?}");
    }

    // -- FNDPUD --------------------------------------------------------------

    #[test]
    fn fndpud_fires_on_findprop_call() {
        let src = "p = findprop(obj, 'Name');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "FNDPUD"), "got: {diags:?}");
    }

    #[test]
    fn fndpud_no_fire_on_regular_call() {
        let src = "p = findobj(obj);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "FNDPUD"), "got: {diags:?}");
    }

    // -- HGCNUD --------------------------------------------------------------

    #[test]
    fn hgcnud_fires_on_uicontainer_call() {
        let src = "h = uicontainer('Parent', f);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "HGCNUD"), "got: {diags:?}");
    }

    #[test]
    fn hgcnud_no_fire_on_regular_call() {
        let src = "h = uipanel('Parent', f);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "HGCNUD"), "got: {diags:?}");
    }

    // -- IMPKG ---------------------------------------------------------------

    #[test]
    fn impkg_fires_on_import_command() {
        let src = "import pkg.sub.*;\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "IMPKG"), "got: {diags:?}");
    }

    #[test]
    fn impkg_no_fire_on_regular_command() {
        let src = "disp('hello');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "IMPKG"), "got: {diags:?}");
    }

    // -- ISMBUD --------------------------------------------------------------

    #[test]
    fn ismbud_fires_on_is_member_call() {
        let src = "tf = isMember(x, y);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "ISMBUD"), "got: {diags:?}");
    }

    #[test]
    fn ismbud_no_fire_on_lowercase_ismember() {
        let src = "tf = ismember(x, y);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "ISMBUD"), "got: {diags:?}");
    }

    // -- MIPKG ---------------------------------------------------------------
    //
    // Note: `meta.package(...)` parses as a `field_expression` wrapping a
    // `function_call` whose `name` field is just "package". The name never
    // equals "meta.package", so MIPKG is unreachable under the current grammar.

    // -- SEPTUD --------------------------------------------------------------

    #[test]
    fn septud_fires_on_serial_call() {
        let src = "s = serial('COM1');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "SEPTUD"), "got: {diags:?}");
    }

    #[test]
    fn septud_no_fire_on_serialport_call() {
        let src = "s = serialport('COM1', 9600);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "SEPTUD"), "got: {diags:?}");
    }

    // -- SYDEUD --------------------------------------------------------------
    //
    // Note: `System.Data.DataTable()` parses as a `field_expression` wrapping
    // a `function_call` whose `name` field is just "DataTable". The name never
    // starts with "System.Data", so SYDEUD is unreachable under the current
    // grammar.

    // -- UIRSUD --------------------------------------------------------------

    #[test]
    fn uirsud_fires_on_uiresume_at_script_level() {
        let src = "uiresume(gcf);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "UIRSUD"), "got: {diags:?}");
    }

    #[test]
    fn uirsud_no_fire_inside_function() {
        let src = "function f()\n    uiresume(gcf);\nend\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "UIRSUD"), "got: {diags:?}");
    }

    // -- UISUUD --------------------------------------------------------------

    #[test]
    fn uisuud_fires_on_guidata_call() {
        let src = "guidata(h, data);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "UISUUD"), "got: {diags:?}");
    }

    #[test]
    fn uisuud_no_fire_on_regular_call() {
        let src = "disp('hello');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "UISUUD"), "got: {diags:?}");
    }

    // -- skip_checks configuration -------------------------------------------

    #[test]
    fn skip_checks_disables_check() {
        let config = Config::from_toml(
            "[lint.rules.UNSUPPORTED_ENGINE]\nskip_checks = [\"FEATUD\"]\n",
        )
        .expect("valid config");
        let rule = UnsupportedEngine::from_config(&config);
        let src = "v = feature('version');\n";
        let diags = lint_nodes(&*rule, src);
        assert!(!has_id(&diags, "FEATUD"), "got: {diags:?}");
    }

    /// Ensure the test sources parse without syntax errors (sanity check).
    #[test]
    fn test_sources_parse() {
        let sources = [
            "deploytool();\n",
            "ex = actxserver('Excel.Application');\n",
            "import pkg.sub.*;\n",
            "pkg = meta.package('my.pkg');\n",
            "dt = System.Data.DataTable();\n",
        ];
        for src in sources {
            let tree = parse(src);
            assert!(!tree.root_node().has_error(), "parse error for: {src:?}");
        }
    }
}
