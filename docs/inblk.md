# INBLK - Unterminated Block Comment

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags a block comment opened with `%{` that is never closed with `%}` before the end of the file. Block comments in MATLAB begin with `%{` and end with `%}`; everything between them is ignored, including line breaks.

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It triggers on `comment` nodes whose text starts with `%{` and does not contain `%}`.

## Why this matters

- **Broken code**: An unterminated block comment swallows the rest of the file as comment text — every line after the `%{` is ignored, so the file silently does less than it appears to
- **Diagnosis**: The Code Analyzer message ("A block comment is unterminated at the end of the file") points at the `%{` so the author can add the missing `%}`
- **Silent breakage**: Unlike a parse error, an unterminated block comment parses cleanly, so it is easy to miss without this dedicated check

## Examples

### Correct

```matlab
%{
block comment
closed with %} on its own line
%}

%{ a single-line block comment %}
```

### Incorrect

```matlab
%{
unterminated comment — no %} anywhere before end of file
```

### Fixed

```matlab
%{
unterminated comment — add the closing marker
%}
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["INBLK"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"INBLK"` to disable this check) |

## Automatic fixes

None. mlt cannot know where the closing `%}` was intended to go.

## Target node types

This rule triggers on the following tree-sitter node types:

- `comment` — nodes whose text starts with `%{` and does not contain `%}`

A comment that starts with `%{` and contains `%}` anywhere in its text is treated as closed.

## Related rules

- `STRIN` — unterminated single-quoted character vector
- `DOUQT` — unterminated double-quoted string
