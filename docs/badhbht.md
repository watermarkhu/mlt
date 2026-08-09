# BADHBHT - Hexadecimal Literal Has Too Many Digits for Specified Type Suffix

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags hexadecimal literals whose number of digits exceeds what the specified type suffix can represent. Supported type suffixes are `u8`, `u16`, `u32`, `u64` and `s8`, `s16`, `s32`, `s64`.

For example, `0xFFFFu8` has 4 hexadecimal digits, but a `u8` can hold at most 2.

| Suffix | Max hex digits |
| ------ | -------------- |
| `u8` / `s8` | 2 |
| `u16` / `s16` | 4 |
| `u32` / `s32` | 8 |
| `u64` / `s64` | 16 |

## Why this matters

- A literal with more digits than its suffix supports overflows when MATLAB converts it to the target integer type
- The extra digits indicate a typo in the suffix or in the literal itself
- Using a `u64` (or no suffix) is usually the intended fix

## Examples

### Correct

```matlab
x = 0xFFu8;      % 2 digits, u8 max 2
x = 0xFFFFu16;   % 4 digits, u16 max 4
x = 0xFFu8;
```

### Incorrect

```matlab
x = 0xFFFFu8;    % 4 digits, u8 max 2
x = 0x1FFFFFFFFu32;
```

### Fixed

```matlab
x = 0xFFu8;
x = 0xFFFFFFFFu32;
```

## Configuration

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["BADHBHT"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable; add `"BADHBHT"` to turn this check off |

## Automatic fixes

This rule does not provide an automatic fix.

## Target node types

This is a file-level check (part of the `SYNTAX_ERRORS_ENGINE`). It inspects `number` nodes in the parse tree.

## Related rules

- `BADHBBT` — Binary literal has too many digits for its type suffix
- `HEXTOOLONG` — Hexadecimal literal has too many digits (no suffix)
- `BADHBH` — Invalid digit in a hexadecimal literal
