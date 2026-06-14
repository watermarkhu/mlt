---
icon: lucide/list-checks
---

# Rules Reference

## Overview

mlt implements lint rules for catching common MATLAB issues. Rules are organized by category and each has a unique identifier (e.g., `M001`).

## Rule Table

| Rule ID | Name | Description | Default Severity | Auto-fix |
| ------- | ---- | ----------- | ---------------- | -------- |
| [M001](m001.md) | Trailing Semicolon | Statement without trailing semicolon may produce unintended console output | Warning | Yes |

## Rule Categories

### Statement Rules

Rules that check statement-level patterns and conventions.

| Rule ID | Name | Description |
| ------- | ---- | ----------- |
| [M001](m001.md) | Trailing Semicolon | Missing semicolons cause unintended console output |

*More categories will be added as rules are implemented.*

## Severity Levels

Rules are categorized into three severity levels:

### Error

Critical issues that likely indicate bugs or broken code:

- Code that will fail at runtime
- Accessibility or compatibility issues

### Warning

Style and convention issues that affect code quality:

- **M001** — Missing semicolons cause performance degradation and noisy output

### Info

Low-priority suggestions:

- Minor style preferences
- Issues that are subjective

### Configuring Severity

Override default severities in `.mlt.toml`:

```toml
[lint.rules]
M001 = "error"    # Upgrade from warning to error
```

Or in a full table:

```toml
[lint.rules.M001]
severity = "error"
```

See [Configuration](configuration.md) for full details.

## Enabling and Disabling Rules

All rules are enabled by default. Disable rules with:

```toml
[lint.rules]
M001 = "off"
```

## Auto-fix Support

Rules marked with "Yes" in the Auto-fix column provide automatic fixes. Run mlt with `--fix` to apply them:

```bash
mlt --fix src/**/*.m
```

Fixes are applied atomically per file. If a rule produces multiple fixes for the same file, they are applied in reverse byte-offset order to preserve correctness.

## Adding New Rules

mlt's rule system is designed for extensibility. To add a new rule:

1. Create a new module in `crates/mlt_rules/src/` (e.g., `m002_something.rs`)
2. Implement the `Rule` trait
3. Register it in `crates/mlt_rules/src/lib.rs`
4. Add documentation in `docs/m002.md`

See the [repository](https://github.com/watermarkhu/mlt) for the full development guide.
