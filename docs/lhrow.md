# LHROW - Multiple Rows on Left Side of Assignment

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags assignments where the left side is a multi-output list that contains a row separator (`;`). A multi-output assignment target must be a single row — each output separated by a comma:

```matlab
[a;b] = f();    % ';' creates a second row — invalid LHS
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It triggers on `assignment` nodes whose `left` field is a `multioutput_variable` whose text contains a `;`.

## Why this matters

- **Broken code**: `[a;b] = f()` is not valid MATLAB — the LHS of an assignment must be a single row, so the code fails to run
- **Intent**: The author almost certainly meant `[a, b] = f()` (two outputs) or `[a; b]` on the right side (a matrix literal); the row separator on the LHS is a typo
- **Diagnosis**: The Code Analyzer message ("The left side of an assignment cannot have multiple rows") identifies the exact problem instead of a generic parse error

## Examples

### Correct

```matlab
[a, b] = f();     % comma-separated multi-output LHS
x = f();          % single output
[a, b] = deal(1, 2);
```

### Incorrect

```matlab
[a;b] = f();      % ';' creates multiple rows on the LHS
[a; b] = f();     % same problem, with whitespace
```

### Fixed

```matlab
[a, b] = f();     % comma-separated multi-output LHS
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["LHROW"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"LHROW"` to disable this check) |

## Automatic fixes

None. Replacing `;` with `,` is a plausible fix, but mlt does not apply it automatically because the author's intent (two outputs vs. a matrix) cannot be determined with certainty.

## Target node types

This rule triggers on the following tree-sitter node types:

- `assignment` — where the `left` field is a `multioutput_variable` containing a `;`

## Related rules

- `UNSET` — invalid use of an operator on the left side of an assignment
- `NOLHS` — assignment with an empty left side
