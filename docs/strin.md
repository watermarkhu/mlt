# STRIN - Unterminated Quoted Character Vector

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags a single-quoted character vector (`'abc'`) that is not terminated before the end of the line. In MATLAB, a character vector delimited by single quotes must be closed with a second `'` on the same line — it cannot span lines.

This is a sub-check of the `SYNTAX_ERRORS_ENGINE` rule. It triggers on `ERROR` nodes that start with an unmatched `'` (an opening quote with no closing quote before end-of-line) or that contain a stray `'` token.

## Why this matters

- **Broken code**: An unterminated character vector is a syntax error — the code fails to parse and will not run
- **Diagnosis**: The Code Analyzer message ("A quoted character vector is unterminated") identifies the exact problem instead of a generic parse error
- **Ambiguity**: Without this check, an unmatched `'` is easy to miss among transpose operators (`A'`), which is why the rule only fires when the quote is not in transpose position

## Examples

### Correct

```matlab
x = 'abc';          % closed on the same line
x = 'it''s ok';     % doubled '' is an escaped quote
x = A';             % ' is the transpose operator, not a string
x = A.';            % element-wise transpose
```

### Incorrect

```matlab
x = 'abc;           % opening ' never closed before end of line
x = 'don''t stop    % escaped quote still needs a closing '
```

### Fixed

```matlab
x = 'abc';
x = 'don''t stop';
```

## Configuration

As a sub-check of the `SYNTAX_ERRORS_ENGINE`, disable it via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
severity = "error"
disabled_checks = ["STRIN"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"error"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Sub-check IDs to disable within `SYNTAX_ERRORS_ENGINE` (add `"STRIN"` to disable this check) |

## Automatic fixes

None. mlt cannot know where the closing quote was intended to go.

## Target node types

This rule triggers on the following tree-sitter node types:

- `ERROR` — nodes starting with an unmatched `'`, or containing a stray `'` token in a non-transpose position

A `'` is treated as the transpose operator (and ignored) when the character before it is a letter, digit, `)`, `]`, `}`, or `.`. Valid transposes parse as `postfix_operator` nodes and never trigger this rule.

## Related rules

- `DOUQT` — unterminated double-quoted string
- `INBLK` — unterminated block comment
