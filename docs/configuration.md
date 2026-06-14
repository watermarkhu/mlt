---
icon: lucide/settings
---

# Configuration

mlt is configured via a `.mlt.toml` file in your project directory. This page is the complete reference for the configuration schema.

## Config File Discovery

mlt looks for `.mlt.toml` in the current working directory by default. You can override this with the `--config` flag:

```bash
mlt --config path/to/config.toml src/**/*.m
```

If no config file is found and `--config` is not specified, mlt uses default settings (all rules enabled at their default severity).

## Schema Overview

```toml title=".mlt.toml"
[lint]
# Global lint settings
exclude = ["vendor/**", "third_party/**"]

[lint.rules]
# Per-rule configuration (shorthand or full table)
M001 = "warn"           # Shorthand: just set severity
M002 = "off"            # Disable a rule
M003 = "error"          # Escalate to error

[lint.rules.M001]       # Full table: severity + rule parameters
severity = "warn"
ignore_functions = ["disp", "fprintf"]
```

## `[lint]` Section

### `exclude`

A list of glob patterns for files and directories to skip when linting.

```toml
[lint]
exclude = [
    "vendor/**",
    "third_party/**",
    "test/fixtures/**",
]
```

**Type:** Array of strings (glob patterns)
**Default:** `[]` (no exclusions)

## `[lint.rules]` Section

Configure individual rules. Each key is a rule ID (e.g., `M001`). Values can be either a **severity shorthand** (string) or a **full configuration table**.

### Severity Shorthand

Set a rule's severity with a single string value:

```toml
[lint.rules]
M001 = "warn"       # Override severity to warning
M002 = "error"      # Override severity to error
M003 = "info"       # Override severity to info
M004 = "off"        # Disable the rule entirely
```

### Valid Severity Values

| Value | Aliases | Effect |
| ----- | ------- | ------ |
| `"error"` | `"err"` | Rule violations are errors |
| `"warn"` | `"warning"` | Rule violations are warnings |
| `"info"` | `"note"` | Rule violations are informational |
| `"off"` | `"false"`, `"disabled"` | Rule is completely disabled |

### Full Table Configuration

For rules that accept parameters, use a full TOML table:

```toml
[lint.rules.M001]
severity = "error"
ignore_functions = ["disp", "fprintf", "warning", "error"]
```

Or equivalently as an inline table:

```toml
[lint.rules]
M001 = { severity = "error", ignore_functions = ["disp", "fprintf"] }
```

The `severity` key is always optional in a full table. If omitted, the rule uses its default severity.

All other keys in the table are rule-specific parameters. See each rule's documentation page for available options.

## Default Behavior

When no `.mlt.toml` is present:

- All rules are **enabled**
- Each rule uses its **default severity** (typically `"warn"`)
- No files are excluded
- No rule-specific parameters are set

## Severity Override Precedence

The effective severity for each rule is resolved as:

1. Config override (if `[lint.rules.XXXX]` specifies `severity` or shorthand)
2. Rule's built-in default (if no config override)

The config override applies to all diagnostics produced by that rule — rules themselves always emit their default severity, and the engine stamps the override afterward.

## Complete Example

```toml title=".mlt.toml"
[lint]
# Skip generated and vendored code
exclude = [
    "codegen/**",
    "vendor/**",
    "*.generated.m",
]

[lint.rules]
# Disable rules that don't apply to this project
# M002 = "off"   # (example: when M002 exists)

# Escalate missing semicolons to errors in production code
[lint.rules.M001]
severity = "error"
ignore_functions = ["disp", "fprintf", "warning", "error", "assert"]
```

## Per-Rule Parameters

Each rule can define its own configuration parameters. These are deserialized from the rule's config table into a typed struct. See individual rule documentation for available options:

- [M001 - Trailing Semicolon](m001.md#configuration)

## Next Steps

- [Rules Reference](rules.md) — Browse all available rules
- [CLI Reference](usage/cli.md) — Command-line options
- [Quick Start](getting-started/quickstart.md) — Get up and running
