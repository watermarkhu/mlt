# ENDCT4 - A METHODS Block or END Might Be Missing Before a Function Definition

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags a `classdef` that is immediately followed by a function definition without a `methods` block in between. In a MATLAB class file, function definitions must live inside a `methods` block; a function placed directly after `classdef Foo` is invalid. This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule.

```matlab
classdef Foo
    function f()    % ENDCT4 fires here — missing `methods` block (or `end`)
    end
end
```

## Why this matters

- **Broken code**: A method outside a `methods` block is not valid MATLAB and the class fails to load
- **Diagnosis**: The message ("A METHODS block or END might be missing before the function definition. This might be causing additional error messages.") matches the MATLAB Code Analyzer wording exactly
- **Cascade**: A missing `methods` keyword confuses the parser, which can produce several additional spurious errors; fixing it removes the whole cascade

## Examples

### Correct

```matlab
classdef Foo
    methods
        function f()
            x = 1;
        end
    end
end
```

### Incorrect

```matlab
classdef Foo
    function f()    % ENDCT4 fires here
        x = 1;
    end
end
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["ENDCT4"]
```

When a variant such as `ENDCT4` is disabled, the engine falls back to the generic `ENDCT` check so the missing-END condition is still reported. Disabling `ENDCT` itself suppresses all ENDCT family checks (`ENDCT`, `ENDCT2`, `ENDCT3`, `ENDCT4`).

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"ENDCT4"` to disable this check) |

## Automatic fixes

None. Inserting the `methods` block (or an `end`) requires knowing the intended class structure.

## Target node types

This check triggers on the following tree-sitter node types:

- `ERROR` — a node whose text contains `classdef` and which is followed by a `function_definition` sibling with no `methods` between them

## Related rules

- `ENDCT` — generic possible missing `end`
- `ENDCT2` — an END might be missing after a block-opening keyword
- `ENDCT3` — an END might be missing before a block-opening keyword
- `MCPLD` — invalid property syntax
