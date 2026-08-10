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
| `NODEF` | Variable might not be defined before use |
| `USENS` | Variable used but might not be set in all code paths |
| `PSET` | Variable set in one branch but not others |
| `SUSENS` | Script variable used before set |
| `SVNODEF` | Variable in script might not be defined |
| `STOUT` | Output variable might not be assigned |

## Configuration

```toml
[lint.rules.UNSET_VARIABLES_ENGINE]
disabled_checks = ["XXXX"]   # Turn off specific checks
```

See [Configuration](configuration.md#per-engine-parameters) for the full parameter list and [rules.md](rules.md) for the complete rule inventory.
