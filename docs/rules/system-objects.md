---
icon: lucide/boxes
---

# System Objects

**Default severity:** Warning
**Auto-fix:** No
**Category:** System Objects
**Can be disabled:** Yes

## What this engine does

The `SYSTEM_OBJECTS_ENGINE` rule implements the MATLAB Code Analyzer checks in the **System Objects** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `SONUMIN`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `SONUMIN` | SONUMIN |
| `SONUMOUT` | SONUMOUT |
| `SODEPPROP` | SODEPPROP |
| `SOINITPROP` | SOINITPROP |
| `SODFLTVAL` | SODFLTVAL |
| `SORSRVDNM` | SORSRVDNM |
| `SOTUNPROP1` | SOTUNPROP1 |
| `SOTUNPROP3` | SOTUNPROP3 |
| `SOTUNPROP4` | SOTUNPROP4 |

## Configuration

```toml
[lint.rules.SYSTEM_OBJECTS_ENGINE]
skip_checks = ["AGROW"]   # Turn off specific checks
```

See [Configuration](../configuration.md#per-engine-parameters) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
