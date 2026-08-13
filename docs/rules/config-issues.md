---
icon: lucide/cog
---

# Code Analyzer Configuration Issues

**Default severity:** Error
**Auto-fix:** No
**Category:** Configuration Issues
**Can be disabled:** Yes

## What this engine does

The `CONFIG_ISSUES_ENGINE` rule implements the MATLAB Code Analyzer checks in the **Code Analyzer Configuration Issues** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `BDCFG`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `BDCFG` | BDCFG |
| `CFERR` | CFERR |
| `BDOPT` | BDOPT |
| `CFIG` | CFIG |

## Configuration

```toml
[lint.rules.CONFIG_ISSUES_ENGINE]
disabled_checks = ["XXXX"]   # Turn off specific checks
```

See [Configuration](../configuration.md#per-engine-parameters) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
