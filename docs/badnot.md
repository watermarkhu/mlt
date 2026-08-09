# BADNOT - Invalid Use of ~ to Ignore a Value

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags use of `~` (tilde) as a value or as a standalone statement. In MATLAB, `~` is only permitted as an output-ignoring placeholder on the left side of an assignment (for example, `[~, y] = f()`). Using it anywhere else is an error:

```matlab
x = ~ = 5;      % ~ followed by an assignment
~               % bare ~ statement
y = ~;          % ~ as a value
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It walks `not_operator` nodes and fires when the operator's operand is malformed (an `ERROR` or `MISSING` node), and it walks `ERROR` nodes that consist of a bare `~`.

## Why this matters

- **Broken code**: `~` cannot be computed as a value, so these constructs fail to run
- **Distinguishes intent**: Logical negation (`x = ~y`, `x = ~(y == z)`) is valid MATLAB and is never flagged; only `~` in positions where an *output* is expected is reported
- **Clearer diagnosis**: The Code Analyzer message ("Using ~ to ignore a value is not permitted in this context.") explains exactly what is wrong

## Examples

### Correct

```matlab
x = ~y;            % logical NOT
x = ~(y == z);     % logical NOT of a comparison
[~, y] = f();      % ~ ignores the first output
[~] = f();         % ~ ignores the only output
```

### Incorrect

```matlab
x = ~ = 5;
~
y = ~;
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["BADNOT"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"BADNOT"` to disable this check) |

## Automatic fixes

None. The intended value cannot be inferred.

## Target node types

This rule triggers on the following tree-sitter node types:

- `not_operator` — a `~` operator whose operand contains an `ERROR` or `MISSING` node (e.g., `~ = 5`, `~` followed by nothing)
- `ERROR` — a node consisting of a bare `~` statement

## Related rules

- `BADNOTLHS` — `~` adjacent to an output variable without a comma
- `SYNER` — generic syntax error
