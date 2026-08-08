# COMNL - Comma Newline Row Separator

**Default severity:** Info
**Auto-fix:** Yes
**Category:** Readability
**Can be disabled:** Yes

## What this rule does

Flags matrices where a newline immediately follows a trailing comma, causing the newline to act as a row separator:

```matlab
x = [1, 2,
3, 4];
```

In MATLAB, a newline after a comma starts a new row, which is easy to miss. The rule suggests replacing the comma with a semicolon (to make the row separation explicit) or using an ellipsis (`...`) to continue the current row on the next line.

This is a sub-check of the `READABILITY_ENGINE` rule. It triggers on `matrix` nodes that contain two or more rows where a row ends in a comma and the next row begins on a new line.

## Why this matters

- **Readability**: Rows separated only by a newline-after-comma are easy to overlook; a semicolon makes the row boundary obvious
- **Robustness**: Reformatted code (or a trailing-space cleanup) can silently change the number of rows when the separator is an implicit newline
- **Intent**: A trailing comma followed by a newline reads like the row "continues", but it actually terminates it — replacing it with a semicolon or an ellipsis makes the author's intent explicit

## Examples

### Correct

```matlab
x = [1, 2, 3, 4];      % single row — no ambiguity
x = [1, 2;
3, 4];                % explicit row separator (semicolon)
x = [1, 2, ...
3, 4];                % ellipsis continues the row
x = [1, 2, 3
4, 5, 6];            % no trailing comma on row 1
```

### Incorrect

```matlab
x = [1, 2,
3, 4];                % newline after comma is an implicit row separator
```

### Fixed

```matlab
x = [1, 2;
3, 4];                % comma replaced with semicolon
```

or continue the row explicitly:

```matlab
x = [1, 2, ...
3, 4];
```

## Configuration

As a sub-check of the `READABILITY_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.READABILITY_ENGINE]
severity = "info"
disabled_checks = ["COMNL"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"info"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `READABILITY_ENGINE` (add `"COMNL"` to disable this check) |

## Automatic fixes

The fix replaces the trailing comma with a semicolon:

```diff
- x = [1, 2,
- 3, 4];
+ x = [1, 2;
+ 3, 4];
```

The fix is always safe — the newline was already acting as a row separator, so the semicolon makes the existing behavior explicit without changing it.

## Target node types

This rule triggers on the following tree-sitter node types:

- `matrix` — a matrix literal with multiple rows where a row ends in a comma

## Related rules

- `NBRAK2` — unnecessary brackets around a scalar expression
- `NOSEMI` — statement without a trailing semicolon
