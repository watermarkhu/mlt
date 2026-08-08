# VTPOD - Argument Validation Order

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags `arguments` blocks where argument validation is specified out of order. MATLAB requires validation to be specified in the following order: **size, then class, then functions**.

For example, `x (1,1) {mustBeReal, double}` is wrong because the class `double` appears inside the validation function list. The correct form is `x (1,1) double {mustBeReal}`.

## Why this matters

- **Clarity**: MATLAB parses the property in order size, class, then functions; putting the class inside the validation function braces makes the declaration ambiguous
- **Correctness**: When a class name appears inside the validation function list, MATLAB may not enforce the expected type
- **Consistency**: Arguments blocks are widely used in newer MATLAB code; following the canonical order keeps declarations readable

## Examples

### Incorrect

```matlab
function f(x)
    arguments
        x (1,1) {mustBeReal, double}   % 'double' inside the braces
    end
end

function g(x)
    arguments
        x {mustBeReal, double}         % class inside braces, no dimensions
    end
end

function h(x)
    arguments
        x {double}                     % class as the sole validator
    end
end
```

### Correct

```matlab
function f(x)
    arguments
        x (1,1) double {mustBeReal}    % size, then class, then functions
    end
end

function g(x)
    arguments
        x double {mustBePositive}      % no dimensions
    end
end

function h(x)
    arguments
        x (1,1) double                 % class only
        z {mustBePositive}             % functions only
        w (1,1) double {mustBeReal} = 1
    end
end
```

### Fixed

```matlab
function f(x)
    arguments
        x (1,1) double {mustBeReal}
    end
end
```

## Configuration

This check is part of the `SYNTAX_ERRORS_ENGINE` rule. Disable it via `disabled_checks`:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["VTPOD"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable within the syntax errors engine |

## Automatic fixes

No automatic fix is available for this rule.

## Target node types

This check runs at the file level and inspects `arguments_statement` nodes:

- `arguments_statement` — the `arguments` ... `end` block
- `property` — a single argument declaration (a direct child of `arguments_statement`)
- `validation_functions` — the `{...}` validation function list

## Related rules

- `SYNER` — Generic syntax error detection
- `MCPLD` — Invalid property syntax
