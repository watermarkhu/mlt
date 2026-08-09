# BADHBH - Invalid Digit in Hexadecimal Literal

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags hexadecimal literals (`0x...` / `0X...`) that contain a character that is not a valid hexadecimal digit. Supported hex digits are `0`-`9` and `A`-`F`. Supported type suffixes are `u8`, `u16`, `u32`, `u64` and `s8`, `s16`, `s32`, `s64`.

For example, `0xFFG` is parsed as the literal `0xFF` followed by the invalid digit `G`, and `0xFFu` uses an incomplete/unsupported suffix.

## Why this matters

- A letter outside the range `A`-`F` is almost always a typo for a digit or a hexadecimal letter
- An incomplete or misspelled type suffix (`0xFFu`, `0xFFu9`) silently discards the suffix characters
- MATLAB rejects the statement, so the analyzer pinpoints the offending literal

## Examples

### Correct

```matlab
x = 0xFF;
x = 0x1A2B;
x = 0xFFu8;
```

### Incorrect

```matlab
x = 0xFFG;
x = 0xFFu;
x = 0xFFu9;
```

### Fixed

```matlab
x = 0xFF;
x = 0xFFu8;
```

## Configuration

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["BADHBH"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable; add `"BADHBH"` to turn this check off |

## Automatic fixes

This rule does not provide an automatic fix.

## Target node types

This is a file-level check (part of the `SYNTAX_ERRORS_ENGINE`). It inspects `number` nodes and the `ERROR` node immediately following them in the parse tree.

## Related rules

- `BADHBB` — Invalid digit in a binary literal
- `BADHBHT` — Hexadecimal literal has too many digits for its type suffix
- `HEXTOOLONG` — Hexadecimal literal has too many digits
