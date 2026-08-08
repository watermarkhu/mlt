# HEXTOOLONG - Hexadecimal Literal Has Too Many Digits

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags hexadecimal literals without a type suffix that have more than 16 digits. An unsuffixed hexadecimal literal is stored as a double, but its digits are limited to what fits in a 64-bit integer (16 hex digits).

For example, `0xFFFFFFFFFFFFFFFFFFFF` has 20 digits.

## Why this matters

- More than 16 hex digits cannot be represented exactly and indicate a typo (usually a repeated digit)
- If the large value is intentional, add a `u64` suffix and use exactly the digits that fit

## Examples

### Correct

```matlab
x = 0xFFFFFFFFFFFFFFFF;   % 16 digits
x = 0xFF;
```

### Incorrect

```matlab
x = 0xFFFFFFFFFFFFFFFFFFFF;   % 20 digits
```

### Fixed

```matlab
x = 0xFFFFFFFFFFFFFFFF;
```

## Configuration

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["HEXTOOLONG"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable; add `"HEXTOOLONG"` to turn this check off |

## Automatic fixes

This rule does not provide an automatic fix.

## Target node types

This is a file-level check (part of the `SYNTAX_ERRORS_ENGINE`). It inspects `number` nodes in the parse tree.

## Related rules

- `BADHBHT` — Hexadecimal literal has too many digits for its type suffix
- `BINARYTOOLONG` — Binary literal has too many digits (no suffix)
- `BADHBH` — Invalid digit in a hexadecimal literal
