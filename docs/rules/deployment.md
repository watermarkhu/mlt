---
icon: lucide/package
---

# MATLAB Compiler (Deployment)

**Default severity:** Warning
**Auto-fix:** No
**Category:** Deployment
**Can be disabled:** Yes

## What this engine does

The `DEPLOYMENT_ENGINE` rule implements the MATLAB Code Analyzer checks in the **MATLAB Compiler (Deployment)** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `MCCD`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `MCCD` | MCCD |
| `MCPRD` | MCPRD |
| `MCHLP` | MCHLP |
| `MCKBD` | MCKBD |
| `MCSVP` | MCSVP |
| `MCMLR` | MCMLR |
| `MCABF` | MCABF |
| `MCMFL` | MCMFL |
| `MCTBX` | MCTBX |
| `MCLL` | MCLL |

## Configuration

```toml
[lint.rules.DEPLOYMENT_ENGINE]
skip_checks = ["AGROW"]   # Turn off specific checks
```

See [Configuration](../configuration.md#per-engine-parameters) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
