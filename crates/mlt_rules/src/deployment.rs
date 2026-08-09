//! # DEPLOYMENT_ENGINE: MATLAB Compiler / Deployment Checks
//!
//! Implements 10 checks for MATLAB Compiler deployment constraints.
//! Deployed MATLAB applications (compiled with `mcc`) run in a restricted
//! environment where certain functions are unavailable or behave differently.
//!
//! ## Checks
//!
//! | ID | Severity | Description |
//! |----|----------|-------------|
//! | MCCD | Error | `cd` in deployed code |
//! | MCPRD | Error | Path modification in deployed code |
//! | MCHLP | Warning | `help`/`doc` in deployed code |
//! | MCKBD | Warning | `keyboard` in deployed code |
//! | MCSVP | Warning | `savepath` in deployed code |
//! | MCMLR | Warning | `matlabroot` in deployed code |
//! | MCABF | Error | `addpath` with absolute path |
//! | MCMFL | Warning | `mfilename` in deployed code |
//! | MCTBX | Warning | Toolbox function in deployed code |
//! | MCLL | Error | License check in deployed code |
//!
//! ## Configuration
//!
//! ```toml
//! [lint.rules.DEPLOYMENT_ENGINE]
//! skip_checks = ["MCTBX"]
//! ```

use mlt_core::{Category, Config, Diagnostic, NodeContext, Rule, Severity};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for the deployment engine.
///
/// Deserialized from the `[lint.rules.DEPLOYMENT_ENGINE]` section in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeploymentEngineConfig {
    /// Check IDs to skip (e.g., `["MCTBX"]`).
    #[serde(default)]
    pub skip_checks: Vec<String>,
}

// ---------------------------------------------------------------------------
// Check metadata
// ---------------------------------------------------------------------------

/// Static metadata for a single deployment check.
struct CheckMeta {
    id: &'static str,
    severity: Severity,
    description: &'static str,
}

/// All 10 deployment check definitions.
const CHECKS: &[CheckMeta] = &[
    CheckMeta { id: "MCCD", severity: Severity::Error, description: "'cd' should not be used in deployed applications" },
    CheckMeta { id: "MCPRD", severity: Severity::Error, description: "Path modification functions should not be used in deployed applications" },
    CheckMeta { id: "MCHLP", severity: Severity::Warning, description: "'help'/'doc' are not available in deployed applications" },
    CheckMeta { id: "MCKBD", severity: Severity::Warning, description: "'keyboard' is not available in deployed applications" },
    CheckMeta { id: "MCSVP", severity: Severity::Warning, description: "'savepath' is not available in deployed applications" },
    CheckMeta { id: "MCMLR", severity: Severity::Warning, description: "'matlabroot' returns the MCR root in deployed applications, not MATLAB root" },
    CheckMeta { id: "MCABF", severity: Severity::Error, description: "'addpath' with absolute path will fail in deployed applications" },
    CheckMeta { id: "MCMFL", severity: Severity::Warning, description: "'mfilename' behaves differently in deployed applications" },
    CheckMeta { id: "MCTBX", severity: Severity::Warning, description: "Toolbox function may not be available in deployed applications without proper toolbox compilation" },
    CheckMeta { id: "MCLL", severity: Severity::Error, description: "License checking is not available in deployed applications" },
];

/// Common toolbox-specific functions that require special compilation support.
const TOOLBOX_FUNCTIONS: &[&str] = &[
    // Signal Processing Toolbox
    "fft2", "ifft2", "fftshift", "butter", "cheby1", "filter", "filtfilt",
    "spectrogram", "pwelch",
    // Image Processing Toolbox
    "imread", "imshow", "imresize", "imfilter", "edge", "regionprops",
    "bwlabel", "rgb2gray",
    // Statistics and Machine Learning
    "fitlm", "predict", "kmeans", "pca", "corr", "ttest",
    "anova1", "normpdf", "normcdf",
    // Optimization Toolbox
    "fmincon", "linprog", "quadprog", "fsolve", "lsqnonlin",
    // Control System Toolbox
    "tf", "ss", "zpk", "bode", "step", "nyquist", "margin",
    // Curve Fitting Toolbox
    "fit", "cfit", "fittype",
    // Symbolic Math Toolbox (never deployable)
    "sym", "syms", "simplify", "expand", "solve", "diff", "int",
    "limit", "taylor",
];

/// Look up check metadata by ID.
fn check_meta(id: &str) -> Option<&'static CheckMeta> {
    CHECKS.iter().find(|c| c.id == id)
}

/// Look up check description by ID.
fn check_description(id: &str) -> &'static str {
    check_meta(id)
        .map(|c| c.description)
        .unwrap_or("Deployment constraint violation")
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

/// Engine covering 10 deployment checks for MATLAB Compiler constraints.
///
/// Node-level checks fire on `function_call` and `command` nodes.
pub struct DeploymentEngine {
    config: DeploymentEngineConfig,
}

/// Node types for node-level dispatch.
const TARGET_NODES: &[&str] = &["function_call", "command"];

impl DeploymentEngine {
    /// Factory constructor called by the rule registry.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: DeploymentEngineConfig = config.rule_params("DEPLOYMENT_ENGINE");
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

    /// Check a `function_call` node for deployment-restricted functions.
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

        // MCPRD: addpath / rmpath / path modification
        if self.is_enabled("MCPRD")
            && (func_name == "addpath"
                || func_name == "rmpath"
                || func_name == "path"
                || func_name == "pathtool"
                || func_name == "restoredefaultpath")
        {
            diags.push(make_diag("MCPRD", node));

            // MCABF: addpath with absolute path (additional check)
            if self.is_enabled("MCABF")
                && func_name == "addpath"
                && has_absolute_path_arg(node, source)
            {
                diags.push(make_diag("MCABF", node));
            }
        }

        // MCHLP: help / doc
        if self.is_enabled("MCHLP") && (func_name == "help" || func_name == "doc") {
            diags.push(make_diag("MCHLP", node));
        }

        // MCKBD: keyboard
        if self.is_enabled("MCKBD") && func_name == "keyboard" {
            diags.push(make_diag("MCKBD", node));
        }

        // MCSVP: savepath
        if self.is_enabled("MCSVP") && func_name == "savepath" {
            diags.push(make_diag("MCSVP", node));
        }

        // MCMLR: matlabroot
        if self.is_enabled("MCMLR") && func_name == "matlabroot" {
            diags.push(make_diag("MCMLR", node));
        }

        // MCMFL: mfilename
        if self.is_enabled("MCMFL") && func_name == "mfilename" {
            diags.push(make_diag("MCMFL", node));
        }

        // MCTBX: Toolbox functions
        if self.is_enabled("MCTBX") && TOOLBOX_FUNCTIONS.contains(&func_name) {
            diags.push(make_diag("MCTBX", node));
        }

        // MCLL: license check
        if self.is_enabled("MCLL") && func_name == "license" {
            diags.push(make_diag("MCLL", node));
        }

        diags
    }

    /// Check a `command` node for deployment-restricted commands.
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

        // MCCD: cd command
        if self.is_enabled("MCCD") && cmd_name == "cd" {
            diags.push(make_diag("MCCD", node));
        }

        // MCHLP: help / doc as commands
        if self.is_enabled("MCHLP") && (cmd_name == "help" || cmd_name == "doc") {
            diags.push(make_diag("MCHLP", node));
        }

        // MCPRD: addpath / rmpath as commands
        if self.is_enabled("MCPRD") && (cmd_name == "addpath" || cmd_name == "rmpath") {
            diags.push(make_diag("MCPRD", node));
        }

        diags
    }
}

impl Rule for DeploymentEngine {
    fn id(&self) -> &'static str {
        "DEPLOYMENT_ENGINE"
    }

    fn description(&self) -> &'static str {
        "MATLAB Compiler deployment constraint checks"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn category(&self) -> Category {
        Category::Deployment
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

/// Check if a function_call has an argument that looks like an absolute path.
/// Detects strings starting with '/' (Unix) or drive letters like 'C:\' (Windows).
fn has_absolute_path_arg(node: tree_sitter::Node, source: &str) -> bool {
    let count = node.child_count();
    for i in 0..count {
        if let Some(child) = node.child(i) {
            if child.kind() == "string" {
                let text = node_text(child, source);
                let inner = text.trim_matches('\'').trim_matches('"');
                // Check for Unix absolute path or Windows drive letter
                if inner.starts_with('/')
                    || (inner.len() >= 3
                        && inner.as_bytes()[0].is_ascii_alphabetic()
                        && inner.as_bytes()[1] == b':'
                        && (inner.as_bytes()[2] == b'\\' || inner.as_bytes()[2] == b'/'))
                {
                    return true;
                }
            }
            // Recurse into arguments node
            if child.kind() == "arguments" && has_absolute_path_arg(child, source) {
                return true;
            }
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "DEPLOYMENT_ENGINE",
    DeploymentEngine::from_config
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
        DeploymentEngine::from_config(&Config::default())
    }

    // -- MCCD ----------------------------------------------------------------

    #[test]
    fn mccd_fires_on_cd_command() {
        let src = "cd /tmp\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCCD"), "got: {diags:?}");
    }

    #[test]
    fn mccd_no_fire_on_regular_command() {
        let src = "ls\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "MCCD"), "got: {diags:?}");
    }

    // -- MCPRD ---------------------------------------------------------------

    #[test]
    fn mcprd_fires_on_addpath_call() {
        let src = "addpath('src');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCPRD"), "got: {diags:?}");
    }

    #[test]
    fn mcprd_fires_on_rmpath_command() {
        let src = "rmpath src\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCPRD"), "got: {diags:?}");
    }

    #[test]
    fn mcprd_no_fire_on_regular_call() {
        let src = "disp('hello');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "MCPRD"), "got: {diags:?}");
    }

    // -- MCHLP ---------------------------------------------------------------

    #[test]
    fn mchlp_fires_on_help_call() {
        let src = "help('plot');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCHLP"), "got: {diags:?}");
    }

    #[test]
    fn mchlp_fires_on_doc_command() {
        let src = "doc plot\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCHLP"), "got: {diags:?}");
    }

    #[test]
    fn mchlp_no_fire_on_regular_call() {
        let src = "plot(x, y);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "MCHLP"), "got: {diags:?}");
    }

    // -- MCKBD ---------------------------------------------------------------

    #[test]
    fn mckbd_fires_on_keyboard_call() {
        let src = "keyboard();\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCKBD"), "got: {diags:?}");
    }

    #[test]
    fn mckbd_no_fire_on_regular_call() {
        let src = "disp('x');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "MCKBD"), "got: {diags:?}");
    }

    // -- MCSVP ---------------------------------------------------------------

    #[test]
    fn mcsvp_fires_on_savepath_call() {
        let src = "savepath();\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCSVP"), "got: {diags:?}");
    }

    #[test]
    fn mcsvp_no_fire_on_regular_call() {
        let src = "save('data.mat');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "MCSVP"), "got: {diags:?}");
    }

    // -- MCMLR ---------------------------------------------------------------

    #[test]
    fn mcmlr_fires_on_matlabroot_call() {
        let src = "r = matlabroot();\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCMLR"), "got: {diags:?}");
    }

    #[test]
    fn mcmlr_no_fire_on_regular_call() {
        let src = "r = pwd;\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "MCMLR"), "got: {diags:?}");
    }

    // -- MCABF ---------------------------------------------------------------

    #[test]
    fn mcabf_fires_on_absolute_path_addpath() {
        let src = "addpath('/home/user/mytools');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCABF"), "got: {diags:?}");
    }

    #[test]
    fn mcabf_fires_on_windows_drive_addpath() {
        let src = "addpath('C:\\mytools');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCABF"), "got: {diags:?}");
    }

    #[test]
    fn mcabf_no_fire_on_relative_path_addpath() {
        let src = "addpath('mytools');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "MCABF"), "got: {diags:?}");
    }

    // -- MCMFL ---------------------------------------------------------------

    #[test]
    fn mcmfl_fires_on_mfilename_call() {
        let src = "name = mfilename();\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCMFL"), "got: {diags:?}");
    }

    #[test]
    fn mcmfl_no_fire_on_regular_call() {
        let src = "name = mfile;\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "MCMFL"), "got: {diags:?}");
    }

    // -- MCTBX ---------------------------------------------------------------

    #[test]
    fn mctbx_fires_on_toolbox_function() {
        let src = "y = fft2(x);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCTBX"), "got: {diags:?}");
    }

    #[test]
    fn mctbx_no_fire_on_builtin_function() {
        let src = "y = sin(x);\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "MCTBX"), "got: {diags:?}");
    }

    // -- MCLL ----------------------------------------------------------------

    #[test]
    fn mcll_fires_on_license_call() {
        let src = "license('checkout', 'Signal_Toolbox');\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(has_id(&diags, "MCLL"), "got: {diags:?}");
    }

    #[test]
    fn mcll_no_fire_on_regular_call() {
        let src = "lic = 'abc';\n";
        let diags = lint_nodes(&*engine(), src);
        assert!(!has_id(&diags, "MCLL"), "got: {diags:?}");
    }

    // -- skip_checks configuration -------------------------------------------

    #[test]
    fn skip_checks_disables_check() {
        let config = Config::from_toml(
            "[lint.rules.DEPLOYMENT_ENGINE]\nskip_checks = [\"MCTBX\"]\n",
        )
        .expect("valid config");
        let rule = DeploymentEngine::from_config(&config);
        let src = "y = fft2(x);\n";
        let diags = lint_nodes(&*rule, src);
        assert!(!has_id(&diags, "MCTBX"), "got: {diags:?}");
    }

    /// Ensure the test source parses without syntax errors (sanity check).
    #[test]
    fn test_sources_parse() {
        let sources = ["cd /tmp\n", "addpath('src');\n", "doc plot\n", "r = matlabroot;\n"];
        for src in sources {
            let tree = parse(src);
            assert!(!tree.root_node().has_error(), "parse error for: {src:?}");
        }
    }
}
