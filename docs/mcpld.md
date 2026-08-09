# MCPLD - Invalid Property Syntax

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags property declarations inside a class `properties` block that contain a syntax error, such as two consecutive `=` operators:

```matlab
classdef Foo
    properties
        x = 1 = 2    % invalid: two assignments
    end
end
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It walks `ERROR` nodes in the parse tree and fires when the error occurs inside a `property` node within a `properties` block. The diagnostic names the property whose declaration is malformed.

## Why this matters

- **Broken code**: An invalid property declaration prevents the entire class from loading
- **Clearer diagnosis**: The Code Analyzer message ("Invalid property syntax at ...") points at the offending property rather than surfacing a generic parse error
- **Scoped check**: The same double-equals pattern outside a `properties` block (e.g., in a script) is not reported here — this check is specific to property declarations

## Examples

### Correct

```matlab
classdef Foo
    properties
        x = 1;
        y = [2 3];
    end
end
```

### Incorrect

```matlab
classdef Foo
    properties
        x = 1 = 2
    end
end
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["MCPLD"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"MCPLD"` to disable this check) |

## Automatic fixes

None. The intended property default value is ambiguous.

## Target node types

This rule triggers on the following tree-sitter node types:

- `ERROR` — nodes whose ancestors include a `property` node inside a `properties` block

## Related rules

- `SYNER` — generic syntax error
- `RESWD` — invalid use of a reserved word
