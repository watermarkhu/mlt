//! `mlt_rules` — Lint rule implementations for mlt.
//!
//! This crate houses all concrete lint rules. It exposes two entry points:
//! - [`all_rules`]: Constructs all available rules from config.
//! - [`active_rules`]: Constructs only enabled rules (respects config filtering).
//!
//! ## Adding a New Rule
//!
//! Rules self-register using the [`inventory`] crate. To add a new rule:
//!
//! 1. Create a new module (e.g., `nosemi.rs`).
//! 2. Implement the `Rule` trait with a `from_config` factory.
//! 3. Register the factory at the bottom of the module:
//!    ```ignore
//!    inventory::submit!(crate::RuleRegistration::new("NOSEMI", Nosemi::from_config));
//!    ```
//!
//! No manual edits to `lib.rs` are needed — the rule is automatically discovered
//! at link time via the `inventory` crate.

#[cfg(test)]
pub mod test_util;

pub mod analysis;
pub mod bugs;
pub mod codegen;
pub mod compatibility;
pub mod config_issues;
pub mod custom_checks;
pub mod deployment;
pub mod formatting;
pub mod good_practices;
pub mod incomplete_analysis;
pub mod language_spec;
pub mod naming;
pub mod nosemi;
pub mod performance;
pub mod readability;
pub mod suggested_improvements;
pub mod syntax_errors;
pub mod system_objects;
pub mod unsupported;
pub mod unused;
pub mod unset_variables;

use mlt_core::{Config, Rule};

/// Type alias for a rule factory function.
pub type RuleFactory = fn(&Config) -> Box<dyn Rule>;

/// A self-registering rule entry collected via `inventory`.
///
/// Each rule module submits one of these at link time using
/// `inventory::submit!`.
pub struct RuleRegistration {
    /// The rule ID (e.g., "NOSEMI").
    pub id: &'static str,
    /// Factory function that constructs the rule from config.
    pub factory: RuleFactory,
}

impl RuleRegistration {
    /// Create a new registration entry.
    pub const fn new(id: &'static str, factory: RuleFactory) -> Self {
        Self { id, factory }
    }
}

inventory::collect!(RuleRegistration);

/// Construct all available lint rules from the given configuration.
///
/// Returns all rules regardless of whether they are enabled in config.
/// Use [`active_rules`] to get only enabled rules.
pub fn all_rules(config: &Config) -> Vec<Box<dyn Rule>> {
    inventory::iter::<RuleRegistration>
        .into_iter()
        .map(|reg| (reg.factory)(config))
        .collect()
}

/// Construct only the enabled lint rules from the given configuration.
///
/// Filters out rules that are set to "off" in the config (either per-rule
/// or per-category). This is the primary entry point used by the CLI.
pub fn active_rules(config: &Config) -> Vec<Box<dyn Rule>> {
    inventory::iter::<RuleRegistration>
        .into_iter()
        .filter_map(|reg| {
            // Construct the rule to check its category for category-level filtering.
            let rule = (reg.factory)(config);
            if config.is_rule_enabled_for_category(reg.id, rule.category()) {
                Some(rule)
            } else {
                None
            }
        })
        .collect()
}
