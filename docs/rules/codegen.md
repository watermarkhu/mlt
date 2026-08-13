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
| `EMVDF` | EMVDF |
| `EMGRO` | EMGRO |
| `EMNODEF` | EMNODEF |
| `EMFCN` | EMFCN |
| `EMCEL` | EMCEL |
| `EMTC` | EMTC |
| `EMIMP` | EMIMP |
| `EMNST` | EMNST |
| `EMSCR` | EMSCR |
| `EMBRK` | EMBRK |
| `EMCNT` | EMCNT |
| `EMPFR` | EMPFR |
| `EMRTN` | EMRTN |
| `EMWHL` | EMWHL |
| `EMRIFAV` | EMRIFAV |
| `EMLOAD` | EMLOAD |
| `EMS2N` | EMS2N |
| `PRMNOIN` | PRMNOIN |
| `LOOPPRAGMAWITHOUTFOR` | LOOPPRAGMAWITHOUTFOR |
| `FPASE` | FPASE |

## Configuration

```toml
[lint.rules.CODEGEN_ENGINE]
skip_checks = ["AGROW"]   # Turn off specific checks
```

See [Configuration](../configuration.md#per-engine-parameters) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
