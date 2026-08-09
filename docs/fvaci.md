# FVACI - Name-Value Arguments in Cell Indexing

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags use of name=value argument syntax inside cell indexing (`{}`), which MATLAB does not support:

```matlab
c{key = 'Name'} = 5;    % invalid name=value inside {}
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It walks `function_call` nodes whose argument delimiters are braces (`{}` — cell indexing) and fires when the argument list contains an `=` token at argument-list level.

## Why this matters

- **Broken code**: Name=value syntax is only valid for function calls `f(Name=value)`, not for cell indexing
- **Confusing intent**: The code is likely a typo for `c{key} = 'Name';`
- **No false positives**: Comma-separated cell indexing (`c{key, 'Name'}`) and scalar cell indexing (`y{1}`) do not contain `=` and are never flagged

## Examples

### Correct

```matlab
c{key, 'Name'} = 5;
x = y{1};
c{key} = 'Name';
```

### Incorrect

```matlab
c{key = 'Name'} = 5;
c{other = 2} = 1;
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["FVACI"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"FVACI"` to disable this check) |

## Automatic fixes

None. The intended index expression cannot be inferred reliably.

## Target node types

This rule triggers on the following tree-sitter node types:

- `function_call` — brace-delimited calls (`{}` indexing) whose `arguments` child contains an `=` token

## Related rules

- `FVACS` — quoted string used as a name in name=value syntax
- `FVAMI` — invalid name in name=value argument syntax
- `SBTMP` — chaining outputs after parenthesis
