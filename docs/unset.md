# UNSET - Invalid Operator on Left Side of Assignment

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags assignments where the left side of `=` is an operator expression, such as a comparison, arithmetic operation, logical operation, or unary operator. MATLAB requires the left side of an assignment to be a valid target (a variable, indexed element, field, or output list):

```matlab
x == 5 = 3;    % comparison_operator on the LHS
x + 1 = 2;     % binary_operator on the LHS
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It triggers on `assignment` nodes whose left side is an operator, and on `ERROR` nodes that follow a statement-level operator expression and begin with `=` (which is how the parser represents `x == 5 = 3;` and `x + 1 = 2;`).

## Why this matters

- **Broken code**: `x == 5 = 3` is not valid MATLAB and fails to run — the error is emitted at parse time
- **Diagnosis**: The Code Analyzer message ("Invalid use of operator on the left side of an assignment") is clearer than a generic syntax error, pointing directly at the offending operator expression
- **Typo detection**: `x == x + 1` is a common typo for `x = x + 1`; a `+` or `==` on the left of `=` almost always indicates a missing variable name

## Examples

### Correct

```matlab
x = 5;
y = x == 5;      % operator on the right side is fine
x(1) = 5;        % indexed assignment is valid
obj.field = 5;   % field assignment is valid
[a, b] = f();    % multi-output list is valid
```

### Incorrect

```matlab
x == 5 = 3;   % comparison operator on the left
x + 1 = 2;    % binary operator on the left
-x = 3;       % unary operator on the left
x' = 3;       % transpose operator on the left
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["UNSET"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"UNSET"` to disable this check) |

## Automatic fixes

None. An operator expression on the left side of an assignment cannot be fixed automatically because the intended target variable is unknown.

## Target node types

This rule triggers on the following tree-sitter node types:

- `assignment` — where the `left` field is a `comparison_operator`, `binary_operator`, `boolean_operator`, `unary_operator`, or `postfix_operator`
- `ERROR` — a node whose text starts with `=` and which follows a statement-level operator expression

## Related rules

- `LHROW` — left side of an assignment has multiple rows
- `NOLHS` — assignment with an empty left side
