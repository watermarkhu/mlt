# RESWD - Invalid Use of a Reserved Word

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags code that uses a MATLAB reserved word (keyword) as an identifier or as a standalone expression. MATLAB keywords such as `else`, `if`, `for`, `while`, `switch`, `case`, `classdef`, `function`, and `return` cannot be used as variable names, function names, or values:

```matlab
else           % reserved word as a statement
y = for;       % reserved word as a value
case = 5;      % reserved word as a variable name
```

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It walks `ERROR` nodes in the parse tree and looks for a whole-word match against the reserved keyword list, so `foo_else` (an identifier that merely contains a keyword) is not flagged.

## Why this matters

- **Broken code**: Reserved words are part of the MATLAB grammar; using them as identifiers produces parse errors and the code cannot run
- **Clearer diagnosis**: The Code Analyzer message ("Invalid use of a reserved word.") is more actionable than a generic syntax error
- **Substring safety**: Only whole-word matches are reported, so valid identifiers like `for_loop` or `end_index` are never flagged

## Examples

### Correct

```matlab
else_value = 5;        % contains "else" but is a valid identifier
for x = 1:10           % "for" used as a block keyword
    y = x;
end
switch x
    case 1
        y = 1;
end
```

### Incorrect

```matlab
else
y = for;
case = 5;
x = while;
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["RESWD"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"RESWD"` to disable this check) |

## Automatic fixes

None. The intended replacement identifier is unknown.

## Target node types

This rule triggers on the following tree-sitter node types:

- `ERROR` — nodes whose text contains a reserved keyword as a whole word (e.g., `else`, `y = for`, `case = `)

The keyword `end` is handled by `SYNEND` instead; `SYNEND` takes priority when both could apply.

## Related rules

- `SYNEND` — invalid use of the `END` operator
- `MCPLD` — invalid property syntax
- `SYNER` — generic syntax error
