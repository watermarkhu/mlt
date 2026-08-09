# BADFP - Invalid Floating-Point Constant

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags floating-point literals that are truncated by an invalid trailing character, so the value is not the literal the author intended. For example, `1.2.3` is parsed as the constant `1.2` followed by the invalid remainder `.3`.

## Why this matters

- A truncated constant silently changes the value of the expression
- The remaining characters are usually a typo (a second decimal point, or a digit pasted onto the end)
- MATLAB rejects the statement, so the error is caught at parse time — but the analyzer pinpoints the offending constant

## Examples

### Correct

```matlab
x = 1.2;
x = 0.5;
x = 1.2e3;
```

### Incorrect

```matlab
x = 1.2.3;
x = 1.2e3.4;
```

### Fixed

```matlab
x = 1.2;
x = 1.2e3;
```

## Configuration

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["BADFP"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable; add `"BADFP"` to turn this check off |

## Automatic fixes

This rule does not provide an automatic fix.

## Target node types

This is a file-level check (part of the `SYNTAX_ERRORS_ENGINE`). It inspects `number` nodes and the `ERROR` node immediately following them in the parse tree.

## Related rules

- `BADHBH` — Invalid digit in a hexadecimal literal
- `BADHBB` — Invalid digit in a binary literal
- `SYNER` — Generic parse error
