---
icon: lucide/list-checks
---

# Rules Reference

## Overview

mlt implements lint rules matching MATLAB's Code Analyzer checks for feature parity. Rules use the same check IDs as MATLAB (e.g., `NOSEMI`, `AGROW`, `naming.class.casing`) and are organized into categories.

## Rule Table

| Rule ID | Category | Description | Default Severity | Auto-fix |
| ------- | -------- | ----------- | ---------------- | -------- |
| [NOSEMI](nosemi.md) | Formatting | Statement without trailing semicolon may produce unintended console output | Info | Yes |
| [MFAMB](mfamb.md) | Readability | Cannot determine whether name is a variable or function; assumes function | Info | No |
| [FLUDLR](fludlr.md) | Readability | Nested `flipud(fliplr(x))`/`fliplr(flipud(x))` should use `rot90(x, 2)` | Info | Yes |
| [STLOW](stlow.md) | Readability | Unnecessary UPPER/LOWER call in a comparison | Info | Yes |
| [COMNL](comnl.md) | Readability | Newline following comma acts as a row separator in a matrix; suggest semicolon or ellipsis | Info | Yes |
| [FVINR](fvinr.md) | Readability | For readability, add Input attribute to the input arguments block | Info | Yes |
| [NOFIL](nofil.md) | Incomplete Analysis | File not found | Error | No |
| [RDERR](rderr.md) | Incomplete Analysis | Unable to read file | Error | No |
| [QUIT](quit.md) | Incomplete Analysis | Code analysis did not complete; the analyzer encountered an internal error | Error | No |
| [NOPAR2](nopar2.md) | Syntax Errors | A `(` might be missing a closing `)`, causing invalid syntax at `(` on a line | Error | No |
| [EOLPAR](eolpar.md) | Syntax Errors | A `(` might be missing a closing `)`, causing invalid syntax at end of line | Error | No |
| [ENDPAR](endpar.md) | Syntax Errors | A `(` might be missing a closing `)`, causing invalid syntax at end of file | Error | No |
| [UNSET](unset.md) | Syntax Errors | Invalid use of operator on the left side of an assignment | Error | No |
| [LHROW](lhrow.md) | Syntax Errors | The left side of an assignment cannot have multiple rows (';') | Error | No |
| [RESWD](reswd.md) | Syntax Errors | Invalid use of a reserved word | Error | No |
| [SYNEND](synend.md) | Syntax Errors | Invalid use for END operator | Error | No |
| [MCPLD](mcpld.md) | Syntax Errors | Invalid property syntax | Error | No |
| [BADNOT](badnot.md) | Syntax Errors | Using ~ to ignore a value is not permitted in this context | Error | No |
| [BADNOTLHS](badnotlhs.md) | Syntax Errors | Invalid use of logical not operator (~) on left side of an assignment | Error | No |
| [BADCT](badct.md) | Syntax Errors | Unicode explicit directional formatting characters are not supported | Error | No |
| [ENDCT2](endct2.md) | Syntax Errors | An END might be missing after a block-opening keyword | Error | No |
| [ENDCT3](endct3.md) | Syntax Errors | An END might be missing before a block-opening keyword | Error | No |
| [ENDCT4](endct4.md) | Syntax Errors | A METHODS block or END might be missing before a function definition | Error | No |
| [STRIN](strin.md) | Syntax Errors | A quoted character vector is unterminated | Error | No |
| [DOUQT](douqt.md) | Syntax Errors | A double quoted string is unterminated | Error | No |
| [INBLK](inblk.md) | Syntax Errors | A block comment is unterminated at the end of the file | Error | No |
| [BADFP](badfp.md) | Syntax Errors | Invalid floating-point constant | Error | No |
| [BADHBH](badhbh.md) | Syntax Errors | Invalid digit in hexadecimal literal | Error | No |
| [BADHBB](badhbb.md) | Syntax Errors | Invalid digit in binary literal | Error | No |
| [BADHBHT](badhbht.md) | Syntax Errors | Hexadecimal literal has too many digits for specified type suffix | Error | No |
| [BADHBBT](badhbbt.md) | Syntax Errors | Binary literal has too many digits for specified type suffix | Error | No |
| [HEXTOOLONG](hextoolong.md) | Syntax Errors | Hexadecimal literal has too many digits | Error | No |
| [BINARYTOOLONG](binarytoolong.md) | Syntax Errors | Binary literal has too many digits | Error | No |
| [SBTMP](sbtmp.md) | Syntax Errors | Invalid array indexing or function call; chaining outputs after parenthesis is not supported | Error | No |
| [FVSYN](fvsyn.md) | Syntax Errors | Invalid function argument syntax | Error | No |
| [FVACI](fvaci.md) | Syntax Errors | Use of name-value arguments in cell indexing is not supported | Error | No |
| [FVACS](fvacs.md) | Syntax Errors | Using a character vector or string as a name in name=value syntax is not supported | Error | No |
| [FVAMI](fvami.md) | Syntax Errors | Name in name-value argument syntax must be a valid MATLAB identifier | Error | No |
| [VTPOD](vtpod.md) | Syntax Errors | Specify validation in the following order: size, then class, then functions | Error | No |

## Rule Categories

mlt organizes rules into the same categories as MATLAB's Code Analyzer:

| Category | Config Key | Description |
| -------- | ---------- | ----------- |
| Incomplete Analysis | `incomplete-analysis` | Internal linter limits and analysis failures |
| Syntax Errors | `syntax-errors` | Parser-level syntax validation |
| Language Specification | `language-specification` | Language constraint violations |
| Bugs | `bugs` | Likely bugs and logic errors |
| Custom Checks | `custom-checks` | Configurable code complexity/style metrics |
| Naming | `naming` | Naming convention enforcement |
| Compatibility | `compatibility` | Deprecated/removed functions and APIs |
| Forward Compatibility | `forward-compatibility` | Forward compatibility issues |
| Good Practices | `good-practices` | Common best practices |
| Unset Variables | `unset-variables` | Variables that may not be defined before use |
| Unused Constructions | `unused-constructions` | Dead code and unused constructions |
| Suggested Improvements | `suggested-improvements` | Suggestions for improved code patterns |
| Readability | `readability` | Readability improvements |
| Formatting | `formatting` | Code formatting suggestions |
| Performance | `performance` | Performance improvement suggestions |
| Code Generation | `code-generation` | MATLAB Coder constraints |
| Fixed-Point | `fixed-point` | Fixed-point toolbox specific |
| Deployment | `deployment` | MATLAB Compiler deployment constraints |
| System Objects | `system-objects` | System object validation |
| Unsupported | `unsupported` | Unsupported features |
| Behavior Changes | `behavior-changes` | Behavior changes between MATLAB versions |
| Configuration Issues | `configuration-issues` | Configuration file validation |

## Severity Levels

Rules are categorized into three severity levels:

### Error

Critical issues that likely indicate bugs or broken code:

- Code that will fail at runtime
- Language specification violations
- Syntax errors

### Warning

Issues that affect code quality:

- Common bug patterns
- Good practice violations
- Deprecated function usage

### Info

Low-priority suggestions:

- Formatting preferences
- Performance improvement hints
- Readability suggestions

### Configuring Severity

Override default severities in `.mlt.toml`:

```toml
[lint.rules]
NOSEMI = "error"    # Upgrade from info to error
```

Or in a full table:

```toml
[lint.rules.NOSEMI]
severity = "error"
ignore_functions = ["disp", "fprintf"]
```

### Category-Level Configuration

Enable, disable, or override severity for entire categories:

```toml
[lint.categories]
performance = "off"           # Disable all performance rules
compatibility = "warn"        # Override all compatibility rules to warning
formatting = "info"           # Set all formatting rules to info
```

Per-rule configuration always takes precedence over category-level settings.

See [Configuration](configuration.md) for full details.

## Enabling and Disabling Rules

All rules are enabled by default. Disable individual rules:

```toml
[lint.rules]
NOSEMI = "off"
```

Or disable entire categories:

```toml
[lint.categories]
behavior-changes = "off"
```

## Auto-fix Support

Rules marked with "Yes" in the Auto-fix column provide automatic fixes. Run mlt with `--fix` to apply them:

```bash
mlt --fix src/**/*.m
```

Fixes are applied atomically per file. Overlapping fixes are detected and the conflicting fix is skipped with a warning.

## Adding New Rules

mlt uses auto-registration via the `inventory` crate. To add a new rule:

1. Create a new module in `crates/mlt_rules/src/` (e.g., `agrow.rs`)
2. Implement the `Rule` trait with a `from_config` factory
3. Add `inventory::submit!(crate::RuleRegistration::new("AGROW", Agrow::from_config));` at the bottom
4. Add documentation in `docs/agrow.md`

No manual edits to `lib.rs` are needed beyond adding the `pub mod` declaration.

See the [repository](https://github.com/watermarkhu/mlt) for the full development guide.
