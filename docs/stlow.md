# STLOW - Unnecessary UPPER/LOWER Call in a Comparison

**Default severity:** Info
**Auto-fix:** Yes
**Category:** Readability
**Can be disabled:** Yes

## What this rule does

Flags comparisons that apply `upper(x)` or `lower(x)` to one side while the other side is a string literal already entirely in that case. The case-conversion call is then redundant:

- `strcmp(upper(str), 'ABC')` — the literal is already uppercase, so `upper` is unnecessary
- `strcmp(lower(str), 'abc')` — the literal is already lowercase, so `lower` is unnecessary

This is a sub-check of the `READABILITY_ENGINE` rule. It triggers on `strcmp`-family comparisons where one argument is a single-argument `upper`/`lower` call and the other argument is a string literal whose case already matches the conversion.

## Why this matters

- **Clarity**: `upper(x)` next to an already-uppercase literal makes the reader wonder whether the case conversion actually affects the outcome
- **Performance**: Every `upper`/`lower` call allocates a new character array; removing redundant calls saves work in hot loops
- **Intent**: Dropping the redundant call makes the comparison's real behavior obvious

## Examples

### Correct

```matlab
strcmp(upper(str), 'AbC');   % literal is mixed case; upper() matters
strcmp(upper(str), 'abc');   % literal is lowercase; lower() would be needed
strcmp(str, 'ABC');          % no case conversion at all
strcmp(upper(x), y);         % other side is not a literal
```

### Incorrect

```matlab
strcmp(upper(str), 'ABC');   % 'ABC' is already uppercase
strcmp(lower(str), 'abc');   % 'abc' is already lowercase
```

### Fixed

```matlab
strcmp(str, 'ABC');          % upper() removed
strcmp(str, 'abc');          % lower() removed
```

## Configuration

As a sub-check of the `READABILITY_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.READABILITY_ENGINE]
severity = "info"
disabled_checks = ["STLOW"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"info"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `READABILITY_ENGINE` (add `"STLOW"` to disable this check) |

## Automatic fixes

The fix replaces the redundant `upper(x)` / `lower(x)` call with its inner argument:

```diff
- strcmp(upper(x), 'ABC');
+ strcmp(x, 'ABC');
```

The fix is always safe — removing a redundant case conversion does not change the comparison's result.

## Target node types

This rule triggers on the following tree-sitter node types:

- `function_call` — the `strcmp` call whose argument is `upper(x)`/`lower(x)`

## Related rules

- `STREMP` — `strcmp(s, '')` should be `strlength(s)==0`
- `STRIFCND` — `strcmp` in an `if` condition should be `matches`
- `STRCL1` — `strncmp`/`strncmpi` should be `startsWith`/`endsWith`
