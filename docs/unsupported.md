---
icon: lucide/ban
---

# Unsupported Features

**Default severity:** Warning
**Auto-fix:** No
**Category:** Unsupported
**Can be disabled:** Yes

## What this engine does

The `UNSUPPORTED_ENGINE` rule implements the MATLAB Code Analyzer checks in the **Unsupported Features** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `MCADE`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `MCADE` | ADE (Application Deployment Environment) functions |
| `AWTIUD` | Await syntax usage |
| `AXCHUD` | ActiveX/COM automation |
| `FEATUD` | `feature` function usage |
| `FNDPUD` | `findprop` usage |
| `HGCNUD` | Handle Graphics container objects |
| `IMPKG` | Import package syntax |
| `ISMBUD` | `isMember` (old casing) usage |
| `MIPKG` | `meta.package` usage |
| `SEPTUD` | Serial port (legacy `serial` function) |
| `SYDEUD` | System.Data .NET usage |
| `UIRSUD` | `uiresume` in unsupported context |
| `UISUUD` | UI setup patterns |

## Configuration

```toml
[lint.rules.UNSUPPORTED_ENGINE]
skip_checks = ["AGROW"]   # Turn off specific checks
```

See [Configuration](configuration.md#per-engine-parameters) for the full parameter list and [rules.md](rules.md) for the complete rule inventory.
