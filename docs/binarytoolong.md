# BINARYTOOLONG - Binary Literal Has Too Many Digits

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags binary literals without a type suffix that have more than 64 bits. An unsuffixed binary literal is stored as a double, but its digits are limited to what fits in a 64-bit integer (64 bits).

## Why this matters

- More than 64 bits cannot be represented exactly and indicate a typo (usually a repeated digit)
- If the large value is intentional, add a `u64` suffix and use exactly the bits that fit

## Examples

### Correct

```matlab
x = 0b1111111111111111111111111111111111111111111111111111111111111111;  % 64 bits
x = 0b1010;
```

### Incorrect

```matlab
x = 0b11111111111111111111111111111111111111111111111111111111111111111;  % 65 bits
```

### Fixed

```matlab
x = 0b1111111111111111111111111111111111111111111111111111111111111111;
```

## Configuration

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["BINARYTOOLONG"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable; add `"BINARYTOOLONG"` to turn this check off |

## Automatic fixes

This rule does not provide an automatic fix.

## Target node types

This is a file-level check (part of the `SYNTAX_ERRORS_ENGINE`). It inspects `number` nodes in the parse tree.

## Related rules

- `BADHBBT` — Binary literal has too many digits for its type suffix
- `HEXTOOLONG` — Hexadecimal literal has too many digits (no suffix)
- `BADHBB` — Invalid digit in a binary literal
