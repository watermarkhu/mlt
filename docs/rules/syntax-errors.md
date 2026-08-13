---
icon: lucide/x-circle
---

# Syntax Errors

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this engine does

The `SYNTAX_ERRORS_ENGINE` rule implements the MATLAB Code Analyzer checks in the **Syntax Errors** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `SYNER`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `SYNER` | ERROR node detected in parse tree |
| `BDFIL` | File name doesn't follow MATLAB naming rules |
| `BADNE` | Source contains `!=` (MATLAB uses `~=`) |
| `BADOT` | Source contains `..` not part of `...` |
| `TWOCM` | Source contains `,,` |
| `CLIS` | class_definition in a script file |
| `CLTWO` | Multiple class_definition nodes in one file |
| `SOFOC` | Statements outside a class_definition in a class file |
| `SEMFU` | File has only empty statements (only `;` and whitespace) |
| `FNDOT` | Function name contains dots outside class methods block |
| `FNSWA` | Function name doesn't start with alphabetic character |
| `NOPAR2` | Missing closing bracket mid-file |
| `EOLPAR` | Missing closing bracket at end of line |
| `ENDPAR` | Missing closing bracket at end of file |
| `ENDCT` | ERROR node suggesting missing END |
| `ENDCT2` | An END might be missing after a block-opening keyword |
| `ENDCT3` | An END might be missing before a block-opening keyword |
| `ENDCT4` | A METHODS block or END might be missing before a function definition |
| `EOFMI` | File ends with ERROR node (incomplete) |
| `NOLHS` | Assignment with empty left side |
| `BADCH` | Invalid control characters in source |
| `BADSP` | Non-ASCII whitespace characters in source |
| `BADCT` | Unicode explicit directional formatting characters |
| `REDEF` | Same identifier used as both function name and variable |
| `SEPEXR` | Missing newline/semicolon between statements |
| `SBTMP` | Chaining outputs after parenthesis is not supported |
| `FVSYN` | Invalid function argument syntax |
| `FVACI` | Name-value arguments in cell indexing not supported |
| `FVACS` | Quoted string used as name in name=value syntax |
| `FVAMI` | Name in name=value syntax is not a valid identifier |
| `UNSET` | Invalid use of operator on the left side of an assignment |
| `LHROW` | Assignment left side cannot have multiple rows (';') |
| `RESWD` | Invalid use of a reserved word |
| `SYNEND` | Invalid use for END operator |
| `MCPLD` | Invalid property syntax |
| `BADNOT` | Using ~ to ignore a value is not permitted |
| `BADNOTLHS` | Invalid use of logical not operator (~) on LHS |
| `STRIN` | A quoted character vector is unterminated |
| `DOUQT` | A double quoted string is unterminated |
| `INBLK` | A block comment is unterminated at the end of the file |
| `BADFP` | Invalid floating-point constant (e.g., truncated `1.2.3`) |
| `BADHBH` | Invalid digit in a hexadecimal literal |
| `BADHBB` | Invalid digit in a binary literal |
| `BADHBHT` | Hex literal has too many digits for its type suffix |
| `BADHBBT` | Binary literal has too many digits for its type suffix |
| `HEXTOOLONG` | Hex literal has too many digits (max 16 without suffix) |
| `BINARYTOOLONG` | Binary literal has too many digits (max 64 without suffix) |
| `VTPOD` | Specify validation in the following order: size, then class, then functions |

## Configuration

```toml
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["XXXX"]   # Turn off specific checks
```

See [Configuration](../configuration.md#per-engine-parameters) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
