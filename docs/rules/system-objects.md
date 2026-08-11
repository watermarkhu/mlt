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
| `SONUMIN` | Wrong number of inputs to System object method |
| `SONUMOUT` | Wrong number of outputs from System object method |
| `SODEPPROP` | Deprecated system object property |
| `SOINITPROP` | Property should be set in constructor |
| `SODFLTVAL` | Default value issue in system object |
| `SORSRVDNM` | Reserved name used for system object member |
| `SOTUNPROP1` | Tunable property issue |
| `SOTUNPROP3` | Non-tunable property modified after setup |
| `SOTUNPROP4` | Non-tunable property modified in step method |

## Configuration

```toml
[lint.rules.SYSTEM_OBJECTS_ENGINE]
skip_checks = ["AGROW"]   # Turn off specific checks
```

See [Configuration](../configuration.md#per-engine-parameters) for the full parameter list and [../rules.md](../rules.md) for the complete rule inventory.
