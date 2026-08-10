---
icon: lucide/bug
---

# Bugs

**Default severity:** Error
**Auto-fix:** No
**Category:** Bugs
**Can be disabled:** Yes

## What this engine does

The `BUGS_ENGINE` rule implements the MATLAB Code Analyzer checks in the **Bugs** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `IFBDUP`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `IFBDUP` | Duplicate if-branch bodies |
| `IFCDUP` | Duplicate if-branch conditions |
| `CTRUE` | Condition is always true (`if true`, `while 1`) |
| `CFALSE` | Condition is always false (`if false`, `while 0`) |
| `SHOCIRT` | Short-circuit `&&` with non-scalar LHS |
| `SHOCIRF` | Short-circuit `\\\|\\\|` with non-scalar LHS |
| `DEBUGFUN` | Debug function in code (keyboard, dbstop, etc.) |
| `INCR` | Suspicious self-increment `x = x + 1` |
| `DECR` | Suspicious self-decrement `x = x - 1` |
| `CMDAND` | `&` used where `&&` intended (boolean context) |
| `CMDOR` | `\\\|` used where `\\\|\\\|` intended (boolean context) |
| `RHSFN` | Function name used on RHS without `@` |
| `FNAN` | Comparison with NaN (use `isnan` instead) |
| `LOGEMP` | `length(x) == 0` instead of `isempty(x)` |
| `STCUL` | `strcmpi` with same-case arguments |
| `LBODUP` | Duplicate case values in switch |
| `FUNFUN` | Passing function name as string instead of handle |
| `DEFSIZE` | `size(x) == [m n]` instead of `isequal(size(x), [m n])` |
| `VARARG` | Misuse of varargin/varargout |
| `STRCMPCSTR` | `strcmp` with single-char comparison |
| `ASSRT` | `assert` with constant true condition |
| `BDSCA2` | Suspicious scalar/array operation |
| `NOPRC` | No `otherwise` in switch |
| `MOCUP` | Operator precedence issue |
| `MDUPC` | Duplicate case in switch |
| `MNANC` | Comparison with NaN (alternate form) |
| `MULCC` | Multiple conditions could be simplified |
| `MEXCEP` | Catch without identifier |
| `PFUIXE` | Parfor index used in eval |
| `PFBFN` | Builtin function in parfor |
| `PFWHOS` | who/whos in parfor |
| `PFTUSE` | Temporary variable misuse in parfor |
| `PFRNC` | Reduction not consistent in parfor |
| `FWPARF` | For loop could be parfor |
| `PFTRIV` | Parfor could be for |

## Configuration

```toml
[lint.rules.BUGS_ENGINE]
disabled_checks = ["XXXX"]   # Turn off specific checks
```

See [Configuration](configuration.md#per-engine-parameters) for the full parameter list and [rules.md](rules.md) for the complete rule inventory.
