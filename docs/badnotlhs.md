# BADNOTLHS - Invalid ~ on Left Side of an Assignment

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags use of `~` inside a multi-output assignment list where the `~` is adjacent to an output variable without a separating comma. `~` in the left side of an assignment must stand alone as an output placeholder, separated from other outputs by commas:

```matlab
[x ~ y] = f();    % ~ must be separated from x and y by commas
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It walks `multioutput_variable` nodes and fires when an `ignored_argument` (`~`) has an `identifier` immediately before or after it in the output list.

## Why this matters

- **Broken code**: `[x ~ y] = f()` is not valid MATLAB and fails at parse time
- **Common typo**: Forgetting the comma after (or before) `~` when calling functions with multiple outputs
- **No false positives**: `[~, y] = f()` and `[~] = f()` are valid output-ignoring patterns and are never flagged

## Examples

### Correct

```matlab
[~, y] = f();      % ~ separated from y by a comma
[~] = f();         % ~ is the only output
[x, ~, y] = f();   % commas everywhere
[x, ~] = f();
```

### Incorrect

```matlab
[x ~ y] = f();
[x ~] = f();
[~ y] = f();
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["BADNOTLHS"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"BADNOTLHS"` to disable this check) |

## Automatic fixes

None. Inserting a comma would change which outputs are captured.

## Target node types

This rule triggers on the following tree-sitter node types:

- `multioutput_variable` — where an `ignored_argument` child has an `identifier` as its immediate previous or next sibling

## Related rules

- `BADNOT` — using `~` to ignore a value is not permitted in this context
- `SYNER` — generic syntax error
