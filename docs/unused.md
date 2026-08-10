---
icon: lucide/trash-2
---

# Unused Constructions

**Default severity:** Warning
**Auto-fix:** No
**Category:** Unused Constructions
**Can be disabled:** Yes

## What this engine does

The `UNUSED_ENGINE` rule implements the MATLAB Code Analyzer checks in the **Unused Constructions** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `NASGU`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `NASGU` | Variable is assigned but never used |
| `NUSED` | Variable is defined (input arg) but never used |
| `NOEFF` | Statement has no effect (expression result discarded) |
| `EQEFF` | Comparison has no effect (result not used) |
| `ASGLU` | Assignment to a variable that is immediately overwritten |
| `SETNU` | Output of function assigned but never used |
| `PUSE` | Persistent/global variable set but not used |
| `PREALL` | Variable preallocated but unused |
| `INUSA` | Input argument not used in function |
| `INUSD` | Input argument defined but could be removed |
| `VANUS` | Value assigned to ans is unused |
| `DEFNU` | Local function defined but never called |
| `UNRCH` | Unreachable code after return/break/continue |
| `MANU` | Method defined but never called |
| `VUNUS` | Variable assigned in all branches but unused after |
| `MSNU` | Struct field set but never read |
| `MSNE` | Struct field doesn't exist (assigned but typo) |

## Configuration

```toml
[lint.rules.UNUSED_ENGINE]
disabled_checks = ["XXXX"]   # Turn off specific checks
```

See [Configuration](configuration.md#per-engine-parameters) for the full parameter list and [rules.md](rules.md) for the complete rule inventory.
