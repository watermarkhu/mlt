# BADHBB - Invalid Digit in Binary Literal

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags binary literals (`0b...` / `0B...`) that contain a character that is not a valid binary digit. Supported binary digits are `0` and `1`. Supported type suffixes are `u8`, `u16`, `u32`, `u64` and `s8`, `s16`, `s32`, `s64`.

For example, `0b102` is parsed as the literal `0b10` followed by the invalid digit `2`.

## Why this matters

- A digit other than `0` or `1` in a binary literal is a typo that changes the intended bit pattern
- MATLAB rejects the statement, so the analyzer pinpoints the offending literal

## Examples

### Correct

```matlab
x = 0b10;
x = 0b10101010;
x = 0b11110000u8;
```

### Incorrect

```matlab
x = 0b102;
x = 0b1012;
```

### Fixed

```matlab
x = 0b1010;
```

## Configuration

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["BADHBB"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable; add `"BADHBB"` to turn this check off |

## Automatic fixes

This rule does not provide an automatic fix.

## Target node types

This is a file-level check (part of the `SYNTAX_ERRORS_ENGINE`). It inspects `number` nodes and the `ERROR` node immediately following them in the parse tree.

## Related rules

- `BADHBH` — Invalid digit in a hexadecimal literal
- `BADHBBT` — Binary literal has too many digits for its type suffix
- `BINARYTOOLONG` — Binary literal has too many digits
