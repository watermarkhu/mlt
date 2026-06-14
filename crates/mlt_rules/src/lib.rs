//! `mlt_rules` — Lint rule implementations for mlt.
//!
//! This crate houses all concrete lint rules. It exposes two entry points:
//! - [`all_rules`]: Constructs all available rules from config.
//! - [`active_rules`]: Constructs only enabled rules (respects config filtering).
//!
//! The CLI and core engine consume these to build the
//! [`RuleRegistry`](mlt_core::RuleRegistry).

pub mod m001_trailing_semicolon;

use mlt_core::{Config, Rule};

use m001_trailing_semicolon::M001TrailingSemicolon;

/// Type alias for a rule factory function.
type RuleFactory = fn(&Config) -> Box<dyn Rule>;

/// Static registry of all available rules and their factory functions.
///
/// To add a new rule:
/// 1. Create a new module (e.g., `m002_something.rs`).
/// 2. Implement the `Rule` trait on your struct with a `from_config` factory.
/// 3. Add an entry to this array.
const RULE_FACTORIES: &[(&str, RuleFactory)] = &[
    ("M001", M001TrailingSemicolon::from_config),
];

/// Construct all available lint rules from the given configuration.
///
/// Returns all rules regardless of whether they are enabled in config.
/// Use [`active_rules`] to get only enabled rules.
pub fn all_rules(config: &Config) -> Vec<Box<dyn Rule>> {
    RULE_FACTORIES
        .iter()
        .map(|(_, factory)| factory(config))
        .collect()
}

/// Construct only the enabled lint rules from the given configuration.
///
/// Filters out rules that are set to "off" in the config. This is the
/// primary entry point used by the CLI.
pub fn active_rules(config: &Config) -> Vec<Box<dyn Rule>> {
    RULE_FACTORIES
        .iter()
        .filter(|(id, _)| config.is_rule_enabled(id))
        .map(|(_, factory)| factory(config))
        .collect()
}
