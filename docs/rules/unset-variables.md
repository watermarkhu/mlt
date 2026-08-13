---
icon: lucide/alert-triangle
---

# Unset Variables

**Default severity:** Warning
**Auto-fix:** No
**Category:** Unset Variables
**Can be disabled:** Yes

## What this engine does

The `UNSET_VARIABLES_ENGINE` rule implements the MATLAB Code Analyzer checks in the **Unset Variables** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `NODEF`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `NODEF` | NODEF |
| `USENS` | USENS |
| `PSET` | PSET |
| `SUSENS` | SUSENS |
| `SVNODEF` | SVNODEF |
| `STOUT` | STOUT |

## Configuration

```toml
[lint.rules.UNSET_VARIABLES_ENGINE]
disabled_checks = ["XXXX"]   # Turn off specific checks
```

See [Configuration](../configuration.md#per-engine-parameters) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
