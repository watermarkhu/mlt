---
icon: lucide/align-left
---

# Formatting Suggestions

**Default severity:** Info
**Auto-fix:** No
**Category:** Formatting
**Can be disabled:** Yes

## What this engine does

The `FORMATTING_ENGINE` rule implements the MATLAB Code Analyzer checks in the **Formatting Suggestions** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `NOCOMMA`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `NOCOMMA` | Use commas to separate elements in a row |
| `NO4LP` | Use 4-space indentation in loop/conditional bodies |
| `ALIGN` | Code alignment suggestion (elseif/else vs. if) |
| `NOPTS` | Add parentheses around condition in if/while |
| `NOPRT` | Remove unnecessary parentheses |
| `PRTCAL` | Consider using command syntax instead of function syntax |
| `NCOMMA` | Use comma to separate input arguments |

## Configuration

```toml
[lint.rules.FORMATTING_ENGINE]
disabled_checks = ["XXXX"]   # Turn off specific checks
```

See [Configuration](configuration.md#per-engine-parameters) for the full parameter list and [rules.md](rules.md) for the complete rule inventory.
