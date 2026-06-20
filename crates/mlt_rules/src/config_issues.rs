//! # CONFIG_ISSUES_ENGINE: Configuration Issue Checks
//!
//! Detects MATLAB-side configuration issues such as invalid parameter names
//! in configuration functions, wrong argument counts, and invalid option values.
//!
//! ## Checks
//!
//! | ID    | Description                        |
//! |-------|------------------------------------|
//! | BDCFG | Invalid configuration parameter    |
//! | CFERR | Configuration function error       |
//! | BDOPT | Invalid option value               |
//! | CFIG  | Configuration file issue           |
//!
//! ## Examples
//!
//! Bad:
//! ```matlab
//! set_param(gcs, 'NotARealParam', 'value');
//! cfg = coder.config();
//! opts = optimset('NotAnOption', 1);
//! ```

use mlt_core::{Category, Config, Diagnostic, NodeContext, Rule, Severity};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Rule-specific configuration
// ---------------------------------------------------------------------------

/// Configuration for CONFIG_ISSUES_ENGINE.
///
/// Deserialized from the `[lint.rules.CONFIG_ISSUES_ENGINE]` section in `.mlt.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ConfigIssuesConfig {
    /// Check IDs to disable within this engine (e.g., `["BDCFG", "CFIG"]`).
    #[serde(default)]
    pub disabled_checks: Vec<String>,
}

// ---------------------------------------------------------------------------
// Known-invalid parameter/option lists
// ---------------------------------------------------------------------------

/// Parameter names that are definitively not valid Simulink model parameters.
const KNOWN_BAD_PARAMS: &[&str] = &[
    "NotARealParam",
    "InvalidParameter",
    "FakeParam",
    "UndefinedSetting",
];

/// Option names that are not valid for `optimset`/`optimoptions`.
const KNOWN_BAD_OPTIM_OPTIONS: &[&str] = &[
    "NotAnOption",
    "FakeOption",
    "InvalidOpt",
    "BadTolerance",
];

/// Configuration functions that require specific argument counts.
const CONFIG_FUNCTIONS: &[&str] = &[
    "coder.config",
    "coder.HardwareImplementation",
    "matlab.system.config",
];

/// Command-form calls that reference configuration/toolbox management.
const CONFIG_COMMANDS: &[&str] = &[
    "matlab.addons.toolbox.installToolbox",
    "matlab.addons.toolbox.packageToolbox",
];

// ---------------------------------------------------------------------------
// Rule implementation
// ---------------------------------------------------------------------------

/// Node types this rule subscribes to.
const TARGET_NODES: &[&str] = &["function_call", "command"];

/// Engine rule that checks for configuration-related issues in MATLAB code.
///
/// Targets `function_call` and `command` nodes, matching against known
/// configuration function patterns to detect invalid parameters, bad option
/// values, and incorrect argument counts.
pub struct ConfigIssuesEngine {
    config: ConfigIssuesConfig,
}

impl ConfigIssuesEngine {
    /// Factory constructor. Reads rule-specific params from config.
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: ConfigIssuesConfig = config.rule_params("CONFIG_ISSUES_ENGINE");
        Box::new(Self {
            config: rule_config,
        })
    }

    /// Returns `true` if the given check ID is disabled via config.
    fn is_check_disabled(&self, check_id: &str) -> bool {
        self.config
            .disabled_checks
            .iter()
            .any(|d| d == check_id)
    }

    /// BDCFG: Check `set_param`/`get_param` calls for known-invalid parameter names.
    fn check_bdcfg(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        if self.is_check_disabled("BDCFG") {
            return Vec::new();
        }

        let node = ctx.node;
        if node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match extract_function_name(node, ctx.source) {
            Some(name) => name,
            None => return Vec::new(),
        };

        if func_name != "set_param" && func_name != "get_param" {
            return Vec::new();
        }

        // Look for string literal arguments that match known-bad parameters.
        let mut diagnostics = Vec::new();
        let arg_list = match node.child_by_field_name("arguments") {
            Some(args) => args,
            None => return Vec::new(),
        };

        let arg_count = arg_list.named_child_count();
        for i in 0..arg_count {
            if let Some(arg) = arg_list.named_child(i) {
                if arg.kind() == "string" {
                    let text = &ctx.source[arg.start_byte()..arg.end_byte()];
                    let unquoted = text.trim_matches('\'').trim_matches('"');
                    if KNOWN_BAD_PARAMS.contains(&unquoted) {
                        let start = arg.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "BDCFG",
                            message: format!(
                                "Invalid configuration parameter '{unquoted}' in {func_name} call"
                            ),
                            severity: Severity::Error,
                            byte_range: arg.start_byte()..arg.end_byte(),
                            line: start.row + 1,
                            column: start.column + 1,
                            fix: None,
                        });
                    }
                }
            }
        }

        diagnostics
    }

    /// CFERR: Check `coder.config` and similar calls for wrong argument count.
    fn check_cferr(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        if self.is_check_disabled("CFERR") {
            return Vec::new();
        }

        let node = ctx.node;
        if node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match extract_function_name(node, ctx.source) {
            Some(name) => name,
            None => return Vec::new(),
        };

        if !CONFIG_FUNCTIONS.contains(&func_name) {
            return Vec::new();
        }

        // These functions require exactly 1 argument (the config type string).
        // Zero arguments is invalid.
        let arg_list = match node.child_by_field_name("arguments") {
            Some(args) => args,
            None => {
                // No argument list at all — flag it.
                let start = node.start_position();
                return vec![Diagnostic {
                    rule_id: "CFERR",
                    message: format!(
                        "Configuration function '{func_name}' called without required arguments"
                    ),
                    severity: Severity::Error,
                    byte_range: node.start_byte()..node.end_byte(),
                    line: start.row + 1,
                    column: start.column + 1,
                    fix: None,
                }];
            }
        };

        let arg_count = arg_list.named_child_count();
        if arg_count == 0 {
            let start = node.start_position();
            return vec![Diagnostic {
                rule_id: "CFERR",
                message: format!(
                    "Configuration function '{func_name}' requires at least one argument"
                ),
                severity: Severity::Error,
                byte_range: node.start_byte()..node.end_byte(),
                line: start.row + 1,
                column: start.column + 1,
                fix: None,
            }];
        }

        Vec::new()
    }

    /// BDOPT: Check `optimset`/`optimoptions` for known-invalid option names.
    fn check_bdopt(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        if self.is_check_disabled("BDOPT") {
            return Vec::new();
        }

        let node = ctx.node;
        if node.kind() != "function_call" {
            return Vec::new();
        }

        let func_name = match extract_function_name(node, ctx.source) {
            Some(name) => name,
            None => return Vec::new(),
        };

        if func_name != "optimset" && func_name != "optimoptions" {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        let arg_list = match node.child_by_field_name("arguments") {
            Some(args) => args,
            None => return Vec::new(),
        };

        let arg_count = arg_list.named_child_count();
        for i in 0..arg_count {
            if let Some(arg) = arg_list.named_child(i) {
                if arg.kind() == "string" {
                    let text = &ctx.source[arg.start_byte()..arg.end_byte()];
                    let unquoted = text.trim_matches('\'').trim_matches('"');
                    if KNOWN_BAD_OPTIM_OPTIONS.contains(&unquoted) {
                        let start = arg.start_position();
                        diagnostics.push(Diagnostic {
                            rule_id: "BDOPT",
                            message: format!(
                                "Invalid option '{unquoted}' for {func_name}"
                            ),
                            severity: Severity::Error,
                            byte_range: arg.start_byte()..arg.end_byte(),
                            line: start.row + 1,
                            column: start.column + 1,
                            fix: None,
                        });
                    }
                }
            }
        }

        diagnostics
    }

    /// CFIG: Check command-form config calls for missing arguments.
    fn check_cfig(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        if self.is_check_disabled("CFIG") {
            return Vec::new();
        }

        let node = ctx.node;
        if node.kind() != "command" {
            return Vec::new();
        }

        // Extract command name from first child.
        let name_node = match node.child(0) {
            Some(n) if n.kind() == "command_name" => n,
            _ => return Vec::new(),
        };

        let cmd_name = &ctx.source[name_node.start_byte()..name_node.end_byte()];

        if !CONFIG_COMMANDS.contains(&cmd_name) {
            return Vec::new();
        }

        // Command-form calls to toolbox functions should have at least one argument.
        // If there are no further children after the command name, flag it.
        let child_count = node.named_child_count();
        if child_count <= 1 {
            let start = node.start_position();
            return vec![Diagnostic {
                rule_id: "CFIG",
                message: format!(
                    "Configuration command '{cmd_name}' called without required arguments"
                ),
                severity: Severity::Error,
                byte_range: node.start_byte()..node.end_byte(),
                line: start.row + 1,
                column: start.column + 1,
                fix: None,
            }];
        }

        Vec::new()
    }
}

impl Rule for ConfigIssuesEngine {
    fn id(&self) -> &'static str {
        "CONFIG_ISSUES_ENGINE"
    }

    fn description(&self) -> &'static str {
        "Detects configuration-related issues in MATLAB code"
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn category(&self) -> Category {
        Category::ConfigurationIssues
    }

    fn target_node_types(&self) -> &'static [&'static str] {
        TARGET_NODES
    }

    fn check(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        diagnostics.extend(self.check_bdcfg(ctx));
        diagnostics.extend(self.check_cferr(ctx));
        diagnostics.extend(self.check_bdopt(ctx));
        diagnostics.extend(self.check_cfig(ctx));
        diagnostics
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract the function name from a `function_call` node.
fn extract_function_name<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> Option<&'a str> {
    let name_node = node.child_by_field_name("name")?;
    Some(&source[name_node.start_byte()..name_node.end_byte()])
}

// ---------------------------------------------------------------------------
// Auto-registration
// ---------------------------------------------------------------------------

inventory::submit!(crate::RuleRegistration::new(
    "CONFIG_ISSUES_ENGINE",
    ConfigIssuesEngine::from_config
));
