# DOUQT - Unterminated Double Quoted String

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags a double-quoted string (`"abc"`) that is never closed before the end of the file. Unlike single-quoted character vectors, double-quoted strings in MATLAB can span multiple lines, so the only way a double-quoted string is unterminated is when no closing `"` exists before end-of-file.

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It triggers on `ERROR` nodes that contain an opening `"` token with no matching close.

## Why this matters

- **Broken code**: An unterminated string swallows everything after it until end-of-file, producing cascading parse errors — the code fails to run
- **Diagnosis**: The Code Analyzer message ("A double quoted string is unterminated") pinpoints the opening quote so the author can find where the closing quote is missing
- **Spanning behavior**: Because double-quoted strings span newlines, a missing close is easy to miss when the string content extends across many lines

## Examples

### Correct

```matlab
x = "abc";          % closed on the same line
x = "multi
line";              % double-quoted strings span lines (valid)
x = "a" + "b";      % concatenated strings
```

### Incorrect

```matlab
x = "abc;           % no closing " before end of file
x = "abc            % unterminated at end of file
```

### Fixed

```matlab
x = "abc";
x = "abc";
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["DOUQT"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"DOUQT"` to disable this check) |

## Automatic fixes

None. mlt cannot know where the closing quote was intended to go.

## Target node types

This rule triggers on the following tree-sitter node types:

- `ERROR` — nodes containing an opening `"` token with no closing `"` before end-of-file

Valid closed strings parse as `string` nodes and never trigger this rule, even when they span multiple lines.

## Related rules

- `STRIN` — unterminated single-quoted character vector
- `INBLK` — unterminated block comment
