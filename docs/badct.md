# BADCT - Unicode Directional Formatting Characters

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags Unicode explicit directional formatting characters in MATLAB source code. MATLAB does not support these characters and rejects them at parse time:

- U+202A LRE (Left-to-Right Embedding)
- U+202B RLE (Right-to-Left Embedding)
- U+202C PDF (Pop Directional Formatting)
- U+202D LRO (Left-to-Right Override)
- U+202E RLO (Right-to-Left Override)
- U+2066 LRI (Left-to-Right Isolate)
- U+2067 RLI (Right-to-Left Isolate)
- U+2068 FSI (First Strong Isolate)
- U+2069 PDI (Pop Directional Isolate)

## Why this matters

- **Broken code**: MATLAB cannot parse files containing these characters
- **Security**: directional formatting characters can be used for "bidi text spoofing" — hiding code that appears in a different order than it actually executes
- **Clarity**: these invisible characters are easy to miss and hard to remove

## Examples

### Incorrect

```matlab
% The \u202A character (LRE) appears after the statement
x = 1;‪
```

### Correct

```matlab
x = 1;
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["BADCT"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"BADCT"` to disable this check) |

## Automatic fixes

None. The offending character must be removed manually.

## Target node types

None. This is a source-text scan that detects the Unicode directional formatting character code points anywhere in the file.

## Related rules

- `BADCH` — invalid control characters in source
- `BADSP` — non-ASCII whitespace characters in source
