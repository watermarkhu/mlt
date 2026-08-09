# ENDPAR - Missing Closing Bracket at End of File

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags a missing closing bracket (`)`, `]`, or `}`) when the unclosed bracket is on the last content line of the file. This is one of three checks (with `NOPAR2` and `EOLPAR`) that replace the former `NOPAR` check.

## Why this matters

- A missing bracket at the end of the file is the most common unclosed-bracket mistake — the file simply stops before the bracket was typed
- MATLAB may report the error on a later (nonexistent) line, confusing the reader
- Naming the exact location (end of file) makes the fix obvious

## Examples

### Correct

```matlab
x = f(1 + 2);
```

### Incorrect

```matlab
x = f(1 + 2;   % missing ) at end of file
```

### Fixed

```matlab
x = f(1 + 2);
```

## Configuration

This check is part of the `SYNTAX_ERRORS_ENGINE` file-level rule. Disable it in `.mlt.toml`:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["ENDPAR"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable, e.g. `["ENDPAR", "EOLPAR"]` |

## Automatic fixes

None. The missing bracket must be inserted manually.

## Target node types

File-level check (`has_file_check`). It inspects `MISSING` nodes of kind `)`, `]`, or `}` where nothing but line terminators, whitespace, `;`, or `,` follows before end of file.

## Related rules

- `NOPAR2` — Missing closing bracket mid-file
- `EOLPAR` — Missing closing bracket at end of line
- `SYNER` — Generic syntax error
- `EOFMI` — File ends with an incomplete construct
