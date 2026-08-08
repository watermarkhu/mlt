# FVACS - Quoted String as Name in Name=Value

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags use of a character vector or string as the name in name=value argument syntax. The name in `Name=value` must be a bare identifier, not a quoted string:

```matlab
f('Bad Name' = 1);    % invalid — remove the quotes around the name
f('Name' = 'x');      % invalid
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It scans function call argument lists for `=` tokens and reports when the name immediately before the `=` is a `string` node (or appears inside an ERROR node that contains a quoted string).

## Why this matters

- **Broken code**: MATLAB does not accept quoted names in name=value syntax
- **Clear guidance**: The message explains exactly what to do — remove the quotes around the name
- **No false positives**: A bare identifier name (`f(Name = 1)`) is valid and never flagged

## Examples

### Correct

```matlab
f(Name = 1);
f(BadName = 1);
f('Bad Name', 1);
```

### Incorrect

```matlab
f('Bad Name' = 1);
f('Name' = 'x');
f("Name" = 1);
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["FVACS"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"FVACS"` to disable this check) |

## Automatic fixes

None. Removing the quotes changes the value type, so the fix is left to the user.

## Target node types

This rule triggers on the following tree-sitter node types:

- `function_call` — calls whose argument list contains an `=` whose preceding name is a `string` node

## Related rules

- `FVAMI` — name in name=value syntax is not a valid MATLAB identifier
- `FVACI` — name-value arguments in cell indexing
- `FVSYN` — invalid function argument syntax
