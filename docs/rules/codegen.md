---
icon: lucide/braces
---

# MATLAB for Code Generation

**Default severity:** Error
**Auto-fix:** No
**Category:** Code Generation
**Can be disabled:** Yes

## What this engine does

The `CODEGEN_ENGINE` rule implements the MATLAB Code Analyzer checks in the **MATLAB for Code Generation** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `EMVDF`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `EMVDF` | Variable-size data not supported |
| `EMGRO` | Growing arrays not supported |
| `EMNODEF` | Variable must be defined before use |
| `EMFCN` | Unsupported function for codegen |
| `EMCEL` | Cell arrays not supported |
| `EMTC` | Try-catch not supported |
| `EMIMP` | Import not supported |
| `EMNST` | Nested functions not supported |
| `EMSCR` | Scripts not supported |
| `EMBRK` | Break in unsupported context |
| `EMCNT` | Continue in unsupported context |
| `EMPFR` | Parfor not supported |
| `EMRTN` | Return in unsupported context |
| `EMWHL` | While loops with non-constant bounds |
| `EMRIFAV` | Arguments block feature |
| `EMLOAD` | Load not supported |
| `EMS2N` | str2num not supported |
| `PRMNOIN` | No input validation in codegen |
| `LOOPPRAGMAWITHOUTFOR` | coder.loop pragma without for |
| `FPASE` | Fixed-point: assignment to scaled expression |

## Configuration

```toml
[lint.rules.CODEGEN_ENGINE]
skip_checks = ["AGROW"]   # Turn off specific checks
```

See [Configuration](../configuration.md#per-engine-parameters) for the full parameter list and [../rules.md](../rules.md) for the complete rule inventory.
