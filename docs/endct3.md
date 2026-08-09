# ENDCT3 - An END Might Be Missing Before a Block-Opening Keyword

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags an `ERROR` node that is immediately followed by a sibling beginning with a block-opening keyword (`if`, `for`, `while`, `switch`, `try`, `function`, `classdef`, `properties`, `methods`, `events`, or `enumeration`). The pattern indicates that a continuation keyword such as `else`, `elseif`, `case`, or `catch` appears outside its parent block, so an `end` is missing before the following block opener. This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule.

```matlab
else
if x > 0        % the `else` has no matching `if`/`end`
    y = 1;
```

## Why this matters

- **Broken code**: A stray `else`/`elseif`/`case`/`catch` outside a block is a syntax error and prevents the file from running
- **Diagnosis**: The message ("An END might be missing (before ELSE on line 1), possibly matching IF.") identifies exactly which keyword is misplaced and which block it should match
- **Recovery**: When an `end` is dropped, the parser often consumes the next block opener into an error region; this check surfaces the boundary

## Examples

### Correct

```matlab
if x > 0
    y = 1;
else
    y = 2;
end
```

### Incorrect

```matlab
else            % ENDCT3 fires here
if x > 0
    y = 1;
end
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["ENDCT3"]
```

When a variant such as `ENDCT3` is disabled, the engine falls back to the generic `ENDCT` check so the missing-END condition is still reported. Disabling `ENDCT` itself suppresses all ENDCT family checks (`ENDCT`, `ENDCT2`, `ENDCT3`, `ENDCT4`).

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"ENDCT3"` to disable this check) |

## Automatic fixes

None. The correct position of the missing `end` depends on the intended block structure.

## Target node types

This check triggers on the following tree-sitter node types:

- `ERROR` — a node whose next meaningful sibling begins with a block-opening keyword

## Related rules

- `ENDCT` — generic possible missing `end`
- `ENDCT2` — an END might be missing after a block-opening keyword
- `ENDCT4` — a METHODS block or END might be missing before a function definition
