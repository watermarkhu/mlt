# ENDCT2 - An END Might Be Missing After a Block-Opening Keyword

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags an unterminated block: a block-opening keyword (`if`, `for`, `while`, `switch`, `try`, `function`, `classdef`, `properties`, `methods`, `events`, or `enumeration`) appears without a matching `end`. This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule.

The check triggers on `ERROR` nodes produced by the parser when a block opener is not closed with `end`. The opener can appear in the error text itself (the common case — the parser wraps the whole unterminated construct in an `ERROR` node) or in the error's previous sibling.

```matlab
if x > 0
    y = 1;       % missing end for the if block
```

## Why this matters

- **Broken code**: Every block in MATLAB must be closed with `end`; an unterminated block fails to parse and the file does not run
- **Hidden damage**: A missing `end` often swallows subsequent code into the block, so errors appear far from the actual problem
- **Diagnosis**: The message ("An END might be missing...") points directly at the unterminated construct instead of a generic syntax error

## Examples

### Correct

```matlab
if x > 0
    y = 1;
end

for i = 1:10
    disp(i);
end

while x
    y = 1;
end
```

### Incorrect

```matlab
if x > 0
    y = 1;        % ENDCT2 fires here

for i = 1:10
    disp(i);      % ENDCT2 fires here

switch x
    case 1
        y = 1;    % ENDCT2 fires here
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["ENDCT2"]
```

When a variant such as `ENDCT2` is disabled, the engine falls back to the generic `ENDCT` check so the missing-END condition is still reported. Disabling `ENDCT` itself suppresses all ENDCT family checks (`ENDCT`, `ENDCT2`, `ENDCT3`, `ENDCT4`).

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"ENDCT2"` to disable this check) |

## Automatic fixes

None. A missing `end` cannot be fixed automatically because the intended position of the `end` depends on the surrounding block structure.

## Target node types

This check triggers on the following tree-sitter node types:

- `ERROR` — a node whose text (or previous sibling) contains a block-opening keyword and which does not contain an `end`

## Related rules

- `ENDCT` — generic possible missing `end`
- `ENDCT3` — an END might be missing before a block-opening keyword
- `ENDCT4` — a METHODS block or END might be missing before a function definition
