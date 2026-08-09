# FVSYN - Invalid Function Argument Syntax

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags malformed argument lists inside function calls, such as arguments that are not separated by commas:

```matlab
f(1 2);     % missing comma between arguments
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. When the parser produces an `ERROR` node inside a function call's argument list, `FVSYN` is reported instead of the generic `SYNER` message. Because the ERROR node is handled by the FVSYN branch, only one diagnostic is produced (no double-fire with `SYNER`).

## Why this matters

- **Broken code**: The argument list cannot be parsed
- **Prioritized message**: `FVSYN` ("Invalid function argument syntax.") is more actionable than the generic `SYNER`
- **No double-fire**: Errors inside call argument lists are never also reported as `SYNER`

## Examples

### Correct

```matlab
f(1, 2);
f(Name = 1);
f(1, 2, 3);
```

### Incorrect

```matlab
f(1 2);
f(1, 2 3);
f('a' 'b');
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["FVSYN"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"FVSYN"` to disable this check) |

## Automatic fixes

None. The intended comma placement cannot be inferred reliably.

## Target node types

This rule triggers on the following tree-sitter node types:

- `ERROR` — nodes that appear inside a function call's `arguments` list (e.g., the stray `2` in `f(1 2)`)

## Related rules

- `SYNER` — generic syntax error
- `FVAMI` — invalid name in name=value argument syntax
- `FVACS` — quoted string used as a name in name=value syntax
