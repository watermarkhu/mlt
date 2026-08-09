# FVAMI - Name in Name=Value Must Be a Valid Identifier

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags name=value argument syntax whose name is not a valid MATLAB identifier, such as a numeric literal:

```matlab
f(123 = 1);    % invalid — 123 is not a valid identifier name
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It scans function call argument lists for `=` tokens and reports when the name immediately before the `=` is not a valid MATLAB identifier (e.g., a `number` node, or an identifier that does not start with a letter).

## Why this matters

- **Broken code**: MATLAB requires names in name=value syntax to be valid identifiers
- **Clear guidance**: The message states the requirement directly
- **No false positives**: A valid identifier name (`f(Name = 1)`) is never flagged

## Examples

### Correct

```matlab
f(Name = 1);
f(x1 = 2, y2 = 3);
```

### Incorrect

```matlab
f(123 = 1);
f(1 = 2);
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["FVAMI"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"FVAMI"` to disable this check) |

## Automatic fixes

None. A replacement name cannot be invented automatically.

## Target node types

This rule triggers on the following tree-sitter node types:

- `function_call` — calls whose argument list contains an `=` whose preceding name is not a valid identifier (e.g., a `number`)

## Related rules

- `FVACS` — quoted string used as a name in name=value syntax
- `FVACI` — name-value arguments in cell indexing
- `FVSYN` — invalid function argument syntax
