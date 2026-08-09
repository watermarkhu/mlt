# BADHBBT - Binary Literal Has Too Many Digits for Specified Type Suffix

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags binary literals whose number of bits exceeds what the specified type suffix can represent. Supported type suffixes are `u8`, `u16`, `u32`, `u64` and `s8`, `s16`, `s32`, `s64`.

For example, `0b111111111u8` has 9 bits, but a `u8` can hold at most 8.

| Suffix | Max binary digits |
| ------ | ----------------- |
| `u8` / `s8` | 8 |
| `u16` / `s16` | 16 |
| `u32` / `s32` | 32 |
| `u64` / `s64` | 64 |

## Why this matters

- A literal with more bits than its suffix supports overflows when MATLAB converts it to the target integer type
- The extra bits indicate a typo in the suffix or in the literal itself
- Using a `u64` (or no suffix) is usually the intended fix

## Examples

### Correct

```matlab
x = 0b11111111u8;   % 8 bits, u8 max 8
x = 0b11111111u16;  % 8 bits, u16 max 16
```

### Incorrect

```matlab
x = 0b111111111u8;   % 9 bits, u8 max 8
x = 0b100000000000000000000000000000000u32;  % 33 bits
```

### Fixed

```matlab
x = 0b11111111u8;
x = 0b10000000000000000000000000000000u32;
```

## Configuration

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["BADHBBT"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable; add `"BADHBBT"` to turn this check off |

## Automatic fixes

This rule does not provide an automatic fix.

## Target node types

This is a file-level check (part of the `SYNTAX_ERRORS_ENGINE`). It inspects `number` nodes in the parse tree.

## Related rules

- `BADHBHT` — Hexadecimal literal has too many digits for its type suffix
- `BINARYTOOLONG` — Binary literal has too many digits (no suffix)
- `BADHBB` — Invalid digit in a binary literal
