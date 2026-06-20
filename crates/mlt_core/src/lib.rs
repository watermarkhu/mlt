//! `mlt_core` — The core engine for the mlt MATLAB linter.
//!
//! This crate provides:
//! - [`Diagnostic`], [`Severity`], and [`Fix`] types for representing lint results.
//! - The [`Rule`] trait that all lint rules implement.
//! - [`Category`] enum for grouping rules by MATLAB Code Analyzer categories.
//! - [`NodeContext`] and [`FileContext`] for passing analysis context to rules.
//! - [`RuleRegistry`] for efficient rule indexing by node type.
//! - [`Linter`] for single-pass AST traversal and rule dispatch.
//! - [`Config`] and related types for TOML-based configuration.

pub mod config;
pub mod diagnostic;
pub mod linter;
pub mod registry;
pub mod rule;

// Re-export primary types at crate root for ergonomic imports.
pub use config::{Config, ConfigError, RuleConfig};
pub use diagnostic::{Diagnostic, Fix, Severity};
pub use linter::{LintError, Linter};
pub use registry::RuleRegistry;
pub use rule::{Category, FileContext, NodeContext, Rule};
