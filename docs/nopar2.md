# NOPAR2 - Missing Closing Bracket (Mid-File)

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags a missing closing bracket (`)`, `]`, or `}`) that causes invalid syntax mid-file — that is, when the unclosed bracket is not the last thing on its line and not at the end of the file. This is one of three checks (with `EOLPAR` and `ENDPAR`) that replace the former `NOPAR` check.

This fires both when the parser recovers by inserting a missing `)` node and when the whole statement becomes an `ERROR` node because of the unclosed bracket.

## Why this matters

- Unclosed brackets are a common typo that produces confusing cascading parse errors
- MATLAB cannot parse the statement, so the surrounding code is also likely to be mis-parsed
- Splitting the check into mid-file / end-of-line / end-of-file variants pinpoints where the fix belongs

## Examples

### Correct

```matlab
x = f(1);
y = [1, 2];
z = {1, 2};
```

### Incorrect

```matlab
y = [1 2;
x = f(1;  % missing ) before the rest of the line
z = 3;
```

### Fixed

```matlab
y = [1 2];
x = f(1);
z = 3;
```

## Configuration

This check is part of the `SYNTAX_ERRORS_ENGINE` file-level rule. Disable it in `.mlt.toml`:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["NOPAR2"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable, e.g. `["NOPAR2", "EOLPAR"]` |

## Automatic fixes

None. The missing bracket must be inserted manually.

## Target node types

File-level check (`has_file_check`). It inspects `ERROR` nodes whose text has more opening than closing brackets, and `MISSING` nodes of kind `)`, `]`, or `}`.

## Related rules

- `EOLPAR` — Missing closing bracket at end of line
- `ENDPAR` — Missing closing bracket at end of file
- `SYNER` — Generic syntax error
- `ENDCT` — Possible missing `end` keyword
