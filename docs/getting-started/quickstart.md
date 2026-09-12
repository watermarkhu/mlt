---
icon: lucide/play
---

# Quick Start

Get up and running with mlt in minutes.

## Basic Usage

### Lint MATLAB files

```bash
# Lint a single file
mlt src/compute.m

# Lint multiple files
mlt src/compute.m src/utils.m lib/helpers.m
```

### Auto-fix issues

```bash
# Apply automatic fixes
mlt --fix src/compute.m
```

## Create a Configuration File

Create a `.mlt.toml` file in your project root to customize behavior:

```toml title=".mlt.toml"
[lint]
exclude = ["vendor/**", "test/fixtures/**"]

[lint.rules]
M001 = "warn"
```

mlt automatically discovers `.mlt.toml` in the current working directory. You can also specify a config file explicitly:

```bash
mlt --config path/to/.mlt.toml src/compute.m
```

## Example Configuration

```toml title=".mlt.toml"
[lint]
# Exclude directories from linting
exclude = ["vendor/**", "third_party/**"]

[lint.rules]
# Set severity per rule
M001 = "error"

[lint.rules.M001]
# Rule-specific parameters
severity = "warn"
ignore_functions = ["disp", "fprintf", "warning", "error"]
```

## Understanding Output

mlt outputs diagnostics in a clear, grep-friendly format:

```text
src/compute.m:2:5 [W] M001: Statement without trailing semicolon may produce unintended console output
src/compute.m:4:5 [W] M001: Statement without trailing semicolon may produce unintended console output

Found 2 issues in 1 file.
```

Each line shows:

- **File path** and **line:column** position
- **Severity** in brackets (`[E]` error, `[W]` warning, `[I]` info)
- **Rule ID** (e.g., M001)
- **Description** of the issue

## Exit Codes

| Code | Meaning                     |
| ---- | --------------------------- |
| `0`  | No issues found             |
| `1`  | Diagnostics found           |
| `2`  | Configuration or runtime error |

## Next Steps

- [CLI Reference](../usage/cli.md) — Full command-line options
- [Configuration](../configuration.md) — Detailed config file reference
- [Rules Reference](../rules.md) — Explore all available rules
