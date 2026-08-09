# SBTMP - Chaining Outputs After Parenthesis

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags chaining of calls or indexing onto the result of a parenthesized function call, such as `f()(1)`. MATLAB does not support indexing or calling the output of a function call directly:

```matlab
x = f()(1);     % invalid
x = f(1)(2);    % invalid
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It walks `function_call` nodes and fires when a call's `name` is itself a `function_call` — the grammar represents `f()(1)` as an outer call whose name is the inner call `f()`.

## Why this matters

- **Broken code**: MATLAB does not support chaining outputs after parenthesis
- **Confusing intent**: The code is likely a typo for `x = f(1)` or `y = f(); x = y(1);`
- **No false positives**: Plain calls (`f(1)`), cell indexing (`y{1}`), and method calls (`obj.method()`) do not have a `function_call` as their name

## Examples

### Correct

```matlab
x = f(1);
x = y{1};
obj.method();
y = f();
x = y(1);
```

### Incorrect

```matlab
x = f()(1);
x = f(1)(2);
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["SBTMP"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"SBTMP"` to disable this check) |

## Automatic fixes

None. There is no way to know whether `f(1)` or `y = f(); y(1)` was intended.

## Target node types

This rule triggers on the following tree-sitter node types:

- `function_call` — nodes whose `name` field is itself a `function_call` (e.g., `f()(1)`)

## Related rules

- `FVACI` — name-value arguments in cell indexing
- `FVSYN` — invalid function argument syntax
- `SYNER` — generic syntax error
