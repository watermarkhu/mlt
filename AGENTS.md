# AGENTS.md

This document describes how to develop and document the `mlt` codebase for AI coding agents and human contributors.

## Project Overview

`mlt` (pronounced "melt") is an ultra-fast, extensible linter for MATLAB written in Rust. It uses `tree-sitter-matlab` for parsing and is architecturally inspired by [ruff](https://github.com/astral-sh/ruff) and [rumdl](https://github.com/rvben/rumdl).

## Workspace Structure

```
mlt/
├── Cargo.toml              # Workspace root (resolver = "2")
├── zensical.toml           # Documentation site config (Zensical/MkDocs)
├── crates/
│   ├── mlt_cli/            # CLI binary (clap, config discovery, output formatting)
│   ├── mlt_core/           # Core engine (Rule trait, RuleRegistry, Linter, Config, Diagnostic)
│   └── mlt_rules/          # Individual lint rule implementations
└── docs/                   # Zensical documentation site (Markdown)
```

### Crate Responsibilities

| Crate | Purpose |
|-------|---------|
| `mlt_core` | Defines `Rule` trait, `NodeContext`/`FileContext`, `RuleRegistry`, `Linter` engine, `Config` parsing, `Diagnostic`/`Fix`/`Severity` types. No concrete rules live here. |
| `mlt_rules` | All concrete rule implementations. Exports `all_rules(&Config)` and `active_rules(&Config)`. Depends on `mlt_core`. |
| `mlt_cli` | Binary entry point. Discovers `.mlt.toml`, builds registry, invokes linter, formats output. Depends on both `mlt_core` and `mlt_rules`. |

### Dependency Direction

```
mlt_cli → mlt_rules → mlt_core
```

`mlt_core` has zero internal dependencies. `mlt_rules` depends only on `mlt_core`. `mlt_cli` depends on both.

## Architecture

### Lint Engine Flow

1. CLI loads `.mlt.toml` → parses into `Config`
2. `mlt_rules::active_rules(&config)` → constructs only enabled rules (filtered by config)
3. `RuleRegistry::new(rules, &config)` → indexes rules by `target_node_types()`, resolves effective severity per rule
4. `Linter::new(registry)` → initializes tree-sitter parser with MATLAB grammar
5. `linter.lint(source, file_path)` → single-pass DFS traversal:
   - For each node: O(1) lookup of subscribed rules, dispatch `rule.check(&NodeContext)`
   - After traversal: call `rule.check_file(&FileContext)` on all rules
   - Stamp effective severity (config override) onto all diagnostics
   - Return sorted diagnostics

### Rule Trait

```rust
pub trait Rule: Send + Sync {
    fn id(&self) -> &'static str;                          // e.g., "M001"
    fn description(&self) -> &'static str;                 // One-line summary
    fn severity(&self) -> Severity;                        // Default severity
    fn target_node_types(&self) -> &'static [&'static str]; // Node types to subscribe to

    fn check(&self, ctx: &NodeContext) -> Vec<Diagnostic> { vec![] }      // Per-node
    fn check_file(&self, ctx: &FileContext) -> Vec<Diagnostic> { vec![] } // Per-file
}
```

- **Node-level rules**: Implement `check()`, return non-empty `target_node_types()`.
- **File-level rules**: Implement `check_file()`, return empty `target_node_types()`.
- **Hybrid**: Implement both (rare).

### Configuration System

Config file: `.mlt.toml` (discovered in CWD, or explicit `--config <path>`).

```toml
[lint]
exclude = ["vendor/**"]

[lint.rules]
M001 = "warn"           # Shorthand: severity only
M002 = "off"            # Disable rule

[lint.rules.M001]       # Full table: severity + rule-specific params
severity = "error"
ignore_functions = ["disp", "fprintf"]
```

Rules access typed parameters via `config.rule_params::<T>(rule_id)`, which deserializes the rule's TOML table into a `#[derive(Deserialize, Default)]` struct.

## Adding a New Rule

This is the most common task. Follow these steps exactly:

### 1. Create the rule module

Create `crates/mlt_rules/src/m<NNN>_<snake_name>.rs`:

```rust
use mlt_core::{Config, Diagnostic, Fix, NodeContext, Rule, Severity};
use serde::Deserialize;

/// Rule-specific configuration (deserialized from .mlt.toml).
#[derive(Debug, Clone, Deserialize, Default)]
pub struct M<NNN>Config {
    // Add rule-specific parameters here with #[serde(default)]
}

pub struct M<NNN><RuleName> {
    config: M<NNN>Config,
}

impl M<NNN><RuleName> {
    pub fn from_config(config: &Config) -> Box<dyn Rule> {
        let rule_config: M<NNN>Config = config.rule_params("M<NNN>");
        Box::new(Self { config: rule_config })
    }
}

impl Rule for M<NNN><RuleName> {
    fn id(&self) -> &'static str { "M<NNN>" }
    fn description(&self) -> &'static str { "..." }
    fn severity(&self) -> Severity { Severity::Warning }
    fn target_node_types(&self) -> &'static [&'static str] { &["..."] }

    fn check(&self, ctx: &NodeContext) -> Vec<Diagnostic> {
        // Implementation here
        Vec::new()
    }
}
```

### 2. Register the rule

In `crates/mlt_rules/src/lib.rs`:

1. Add `pub mod m<NNN>_<snake_name>;`
2. Add `use m<NNN>_<snake_name>::M<NNN><RuleName>;`
3. Add entry to `RULE_FACTORIES`:
   ```rust
   ("M<NNN>", M<NNN><RuleName>::from_config),
   ```

### 3. Add documentation

Create `docs/m<NNN>.md` following the format of `docs/m001.md`:

- Title with rule ID and name
- Default severity and auto-fix status
- "What this rule does" section
- "Why this matters" section
- Examples: Correct / Incorrect / Fixed (with MATLAB code blocks)
- Configuration table with parameters
- Automatic fixes description
- Target node types
- Related rules

### 4. Update docs/rules.md

Add a row to the rule table in `docs/rules.md`.

### 5. Update zensical.toml nav

Add the rule page to the `Rules` section in `zensical.toml`:

```toml
{ "M<NNN> - <Name>" = "m<NNN>.md" },
```

### 6. Verify

```bash
cargo build
cargo clippy
cargo test
```

## Key Patterns and Conventions

### Tree-sitter Node Types

The MATLAB grammar defines these important node types:

- **Statements**: `assignment`, `function_call`, `command`, `for_statement`, `if_statement`, `while_statement`, `switch_statement`, `try_statement`, `return_statement`, `break_statement`, `continue_statement`
- **Expressions**: `binary_operator`, `boolean_operator`, `comparison_operator`, `unary_operator`, `postfix_operator`, `number`, `string`, `identifier`, `function_call`, `field_expression`, `cell`, `matrix`, `range`, `lambda`
- **Structural**: `source_file`, `block`, `function_definition`, `class_definition`, `properties`, `methods`, `arguments_statement`
- **Statement parents**: `source_file`, `block` (use these to determine if a node is at statement level)

### Checking Statement-Level Context

Many rules need to know if a node is a statement (top-level in a block) vs. a sub-expression. Pattern:

```rust
let is_statement_level = node
    .parent()
    .map(|p| ["source_file", "block"].contains(&p.kind()))
    .unwrap_or(false);
```

### Fix Construction

```rust
// Simple insertion
Fix::insert(byte_offset, ";")

// Replacement
Fix::new(start..end, "replacement_text")

// Multi-edit (atomic)
Fix::with_additional(primary_range, primary_replacement, vec![
    Fix::new(other_range, other_replacement),
])
```

### Rule Config Pattern

Every rule should define a `#[derive(Deserialize, Default)]` config struct, even if currently empty. This makes adding parameters non-breaking:

```rust
#[derive(Debug, Clone, Deserialize, Default)]
pub struct M<NNN>Config {
    #[serde(default)]
    pub some_param: Vec<String>,
}
```

## Commands

| Command | Purpose |
|---------|---------|
| `cargo build` | Build all crates |
| `cargo clippy` | Lint Rust code (must pass with zero warnings) |
| `cargo test` | Run all tests (unit tests in `mlt_core::config`) |
| `cargo run -- <file.m>` | Run the linter on a MATLAB file |
| `cargo run -- --fix <file.m>` | Apply auto-fixes |
| `cargo run -- --config path/.mlt.toml <file.m>` | Lint with explicit config |

## Quality Standards

- Zero clippy warnings (CI gate)
- All public types and functions must have doc comments
- Rule modules must have module-level `//!` documentation with examples
- Every rule must have a corresponding `docs/m<NNN>.md` documentation page
- Tests should cover: rule fires correctly, rule does NOT fire on valid code, config parameters are respected

## Documentation System

Documentation uses [Zensical](https://zensical.org) (successor to Material for MkDocs):

- Config: `zensical.toml` at repo root
- Content: `docs/` directory (Markdown)
- Preview: `zensical serve` (requires `pip install zensical`)
- Build: `zensical build`

### Documentation Structure

```
docs/
├── index.md                    # Landing page
├── getting-started/
│   ├── installation.md         # How to install
│   └── quickstart.md           # Basic usage
├── usage/
│   ├── cli.md                  # CLI reference
│   ├── editors.md              # Editor integration
│   └── ci-cd.md                # CI/CD integration
├── configuration.md            # .mlt.toml schema reference
├── rules.md                    # Rules overview + table
└── m<NNN>.md                   # One page per rule (flat, not nested)
```

### Rule Documentation Template

Every rule page follows this structure (see `docs/m001.md` as the canonical example):

1. `# M<NNN> - <Human Name>`
2. Default severity + auto-fix badge
3. "What this rule does"
4. "Why this matters" (bullet points)
5. "Examples" → Correct / Incorrect / Fixed (MATLAB code blocks)
6. "Configuration" (TOML example + parameters table)
7. "Automatic fixes" (describe what the fix does)
8. "Target node types" (tree-sitter nodes)
9. "Related rules"
