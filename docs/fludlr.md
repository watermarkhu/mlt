# FLUDLR - Nested flips Preferable to rot90

**Default severity:** Info
**Auto-fix:** Yes
**Category:** Readability
**Can be disabled:** Yes

## What this rule does

Flags nested `flipud(fliplr(x))` and `fliplr(flipud(x))` calls — flipping a matrix both vertically and horizontally — and suggests the equivalent 180-degree rotation `rot90(x, 2)` instead.

This check is a sub-check of the `READABILITY_ENGINE` rule.

## Why this matters

- **Clarity**: `rot90(x, 2)` states the intent ("rotate the array 180 degrees") directly, while a nested flip requires the reader to mentally compose two flips
- **Performance**: `rot90(x, 2)` is implemented as a single internal transpose-like operation rather than two passes over the data
- **Conciseness**: One function call with two tokens instead of two nested calls

## Examples

### Correct

```matlab
y = rot90(x, 2);
y = flipud(x);   % single flip — not equivalent to a rotation
y = fliplr(x);
```

### Incorrect

```matlab
y = flipud(fliplr(x));
y = fliplr(flipud(x));
```

### Fixed

```matlab
y = rot90(x, 2);
y = rot90(x, 2);
```

## Configuration

Because `FLUDLR` is a sub-check of the readability engine, it is configured via the `READABILITY_ENGINE` rule table:

```toml title=".mlt.toml"
[lint.rules.READABILITY_ENGINE]
severity = "info"
disabled_checks = ["FLUDLR"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"info"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within the engine; add `"FLUDLR"` to turn this check off |

## Automatic fixes

Replaces the outer call with `rot90(<inner argument>, 2)`:

```diff
- y = flipud(fliplr(x));
+ y = rot90(x, 2);
```

## Target node types

- `function_call` — `flipud(...)` and `fliplr(...)` calls whose single argument is a call to the complementary flip function

## Related rules

- `RPMT1`, `RPMT0`, `RPMTI`, `RPMTN` — Redundant arithmetic (also readability sub-checks)
