# SYNEND - Invalid Use for END Operator

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags use of the `end` keyword outside its two legal contexts: as a terminator for block statements (`if`, `for`, `while`, `switch`, `try`, `function`, `classdef`, `properties`, `methods`, `events`, `enumeration`) and as an index expression (`x(end)`, `v(1:end)`). Using `end` as a variable or as a value produces a parse error:

```matlab
end = 5;      % end as an assignment target
y = end;      % end as a value
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It walks `ERROR` nodes in the parse tree and fires when the error contains an `end_keyword` token, which is how the parser represents an `end` that appears in an invalid position.

## Why this matters

- **Broken code**: `end` is a keyword, not a variable — assigning to it or reading it as a value fails at parse time
- **Prioritized message**: `SYNEND` ("Invalid use for END operator.") is reported instead of the more generic `RESWD` reserved-word message
- **No false positives**: Valid uses — block terminators and index expressions — never produce an `ERROR` node containing `end_keyword`, so they are never flagged

## Examples

### Correct

```matlab
x(end) = 5;        % end as an index
y = x(end);        % end as an index
for i = 1:10
    y(i) = i;      % end terminates the block
end
```

### Incorrect

```matlab
end = 5;
y = end;
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["SYNEND"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"SYNEND"` to disable this check) |

## Automatic fixes

None. There is no automatic way to know what variable name was intended.

## Target node types

This rule triggers on the following tree-sitter node types:

- `ERROR` — nodes whose subtree contains an `end_keyword` token (e.g., `end = `, `= end`)

## Related rules

- `RESWD` — invalid use of a reserved word
- `ENDCT` — possible missing `end` keyword
- `SYNER` — generic syntax error
